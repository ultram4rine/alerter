package main

import (
	"bytes"
	"encoding/json"
	"io"
	"net/http"
	"reflect"
	"text/template"

	"git.sgu.ru/ultramarine/alerter/conf"

	tgbotapi "github.com/go-telegram-bot-api/telegram-bot-api"
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

	var funcsMap = template.FuncMap{"isMap": func(i interface{}) bool {
		v := reflect.ValueOf(i)
		switch v.Kind() {
		case reflect.Map:
			return true
		default:
			return false
		}
	}}

	tmpl, err := template.New("default").Funcs(funcsMap).ParseFiles(conf.Config.TemplatePath)
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

	alertChan := make(chan Alert, 1000)

	go tgBotHandleAlerts(bot, tmpl, alertChan)

	processAlerts := makeHandler(alertChan)
	http.HandleFunc("/", processAlerts)
	if err := http.ListenAndServe(conf.Config.ListenPort, nil); err != nil {
		log.Fatal(err)
	}
}

func makeHandler(alertChan chan<- Alert) func(http.ResponseWriter, *http.Request) {
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
				for _, a := range wh.Alerts {
					alertChan <- a
				}
			}

		default:
			http.Error(w, "405 Method Not Allowed", http.StatusMethodNotAllowed)
		}
	}
}

func tgBotHandleAlerts(bot *tgbotapi.BotAPI, tmpl *template.Template, alertChan <-chan Alert) {
	for a := range alertChan {
		var bytesBuff bytes.Buffer
		writer := io.Writer(&bytesBuff)

		if err := tmpl.Execute(writer, a); err != nil {
			log.Errorf("failed to parse alert: %s", err)
		}

		alert := bytesBuff.String()

		msg := tgbotapi.NewMessage(conf.Config.TgChatID, alert)
		msg.ParseMode = "markdown"
		if _, err := bot.Send(msg); err != nil {
			log.Errorf("failed to send message: %s", err)
		}
	}
}

type WebHook struct {
	Alerts []Alert `json:"alerts"`
}

type Alert struct {
	Status       string                 `json:"status"`
	Labels       map[string]interface{} `json:"labels"`
	Annotations  map[string]interface{} `json:"annotations"`
	StartsAt     string                 `json:"startsAt"`
	EndAt        string                 `json:"endsAt"`
	GeneratorURL string                 `json:"generatorURL"`
}
