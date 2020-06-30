package main

import (
	"bytes"
	"encoding/json"
	"io"
	"net/http"
	"text/template"

	"github.com/BurntSushi/toml"
	tgbotapi "github.com/go-telegram-bot-api/telegram-bot-api"
	log "github.com/sirupsen/logrus"
	"gopkg.in/alecthomas/kingpin.v2"
)

var conf struct {
	ListenPort string `toml:"listen_port"`
	TgBotToken string `toml:"tg_bot_token"`
	TgChatID   int64  `toml:"tg_chat_id"`
}

var confPath = kingpin.Flag("conf", "Path to config file.").Short('c').Default("alerter.conf.toml").String()
var debug = kingpin.Flag("debug", "Run in debug mode").Short('d').Default("false").Bool()

func main() {
	kingpin.Parse()

	if _, err := toml.DecodeFile(*confPath, &conf); err != nil {
		log.Fatalf("Error decoding config file from %s", *confPath)
	}

	bot, err := tgbotapi.NewBotAPI(conf.TgBotToken)
	if err != nil {
		log.Panic(err)
	}

	if *debug {
		bot.Debug = true
	}

	log.Printf("Authorized on account %s", bot.Self.UserName)

	alertChan := make(chan Alert, 1000)

	go tgBotHandleAlerts(bot, alertChan)

	processAlerts := makeHandler(alertChan)
	http.HandleFunc("/", processAlerts)
	if err := http.ListenAndServe(conf.ListenPort, nil); err != nil {
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

const tmpl = `
{{ .Status }}
{{ range .Labels }}
Name: {{ .alertname }}
{{ end }}
{{ range .Annotations }}
Description: {{ .description }}
{{ end }}
`

func tgBotHandleAlerts(bot *tgbotapi.BotAPI, alertChan <-chan Alert) {
	t := template.Must(template.New("template").Parse(tmpl))
	for a := range alertChan {
		var bytesBuff bytes.Buffer
		writer := io.Writer(&bytesBuff)
		err := t.Execute(writer, a)
		if err != nil {
			log.Errorf("failed to parse alert: %s", err)
		}

		alert := bytesBuff.String()

		msg := tgbotapi.NewMessage(-1001453885388, alert)
		_, err = bot.Send(msg)
		if err != nil {
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
