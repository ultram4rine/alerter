package main

import (
	"bytes"
	"encoding/json"
	"io"
	"net/http"
	"path"
	"strings"
	"text/template"
	"time"

	"git.sgu.ru/ultramarine/alerter/conf"

	tgbotapi "github.com/go-telegram-bot-api/telegram-bot-api"
	"github.com/hako/durafmt"
	log "github.com/sirupsen/logrus"
	"gopkg.in/alecthomas/kingpin.v2"
)

var confName = kingpin.Flag("conf", "Name of config file.").Short('c').Default("alerter.conf").String()
var debug = kingpin.Flag("debug", "Run in debug mode").Short('d').Default("false").Bool()

func main() {
	kingpin.Parse()

	if err := conf.Load(*confName); err != nil {
		log.Fatalf("Failed to load configuration: %s", err)
	}

	var funcsMap = template.FuncMap{"duration": func(start time.Time, end time.Time) string {
		return durafmt.Parse(end.Sub(start)).String()
	}, "durationNow": func(start time.Time) string {
		return durafmt.Parse(time.Now().Sub(start)).String()
	}, "since": func(t time.Time) string {
		return durafmt.Parse(time.Since(t)).String()
	}}

	tmpl, err := template.New(path.Base(conf.Config.TemplatePath)).Funcs(funcsMap).ParseFiles(conf.Config.TemplatePath)
	if err != nil {
		log.Fatalf("Failed to parse template: %s", err)
	}

	bot, err := tgbotapi.NewBotAPI(conf.Config.TgBotToken)
	if err != nil {
		log.Fatalf("Failed to create bot: %s", err)
	}

	if *debug {
		bot.Debug = true
	}

	log.Infof("Authorized on account %s", bot.Self.UserName)

	whChan := make(chan WebHook, 1000)

	go tgBotHandleWebHooks(bot, tmpl, whChan)

	processAlerts := makeHandler(whChan)
	http.HandleFunc("/", processAlerts)
	if err := http.ListenAndServe(conf.Config.ListenPort, nil); err != nil {
		log.Fatal(err)
	}
}

func makeHandler(whChan chan<- WebHook) func(http.ResponseWriter, *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/" {
			http.Error(w, "404 Not Found", http.StatusNotFound)
			return
		}

		switch r.Method {
		case "POST":
			decoder := json.NewDecoder(r.Body)
			var wh WebHook
			err := decoder.Decode(&wh)
			if err != nil {
				log.Errorf("failed to decode request body: %s", err)
			} else {
				whChan <- wh
			}

		default:
			http.Error(w, "405 Method Not Allowed", http.StatusMethodNotAllowed)
		}
	}
}

func tgBotHandleWebHooks(bot *tgbotapi.BotAPI, tmpl *template.Template, whChan <-chan WebHook) {
	for wh := range whChan {
		var bytesBuff bytes.Buffer
		writer := io.Writer(&bytesBuff)

		if err := tmpl.Execute(writer, wh); err != nil {
			log.Errorf("failed to parse alert: %s", err)
		}

		msgTxt := strings.TrimSuffix(bytesBuff.String(), "\n")
		msg := tgbotapi.NewMessage(conf.Config.TgChatID, msgTxt)
		msg.ParseMode = tgbotapi.ModeHTML
		if _, err := bot.Send(msg); err != nil {
			log.Errorf("failed to send message: %s", err)
			log.Infof("message itself: %s", msgTxt)
		}
	}
}

// WebHook is a POST request from alertmanager.
type WebHook struct {
	Status string  `json:"status"`
	Alerts []Alert `json:"alerts"`
}

// Alert is a representation of an alert in the Prometheus eco-system.
type Alert struct {
	Labels      map[string]interface{} `json:"labels"`
	Annotations map[string]interface{} `json:"annotations"`
	StartsAt    time.Time              `json:"startsAt"`
	EndsAt      time.Time              `json:"endsAt"`
}
