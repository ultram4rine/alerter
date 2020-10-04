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
	"github.com/spf13/viper"
	"gopkg.in/alecthomas/kingpin.v2"
)

var confName = kingpin.Flag("conf", "Name of config file.").Short('c').Default("alerter.conf").String()
var debug = kingpin.Flag("debug", "Run in debug mode").Short('d').Default("false").Bool()

func main() {
	kingpin.Parse()

	conf.GetConfig(*confName)

	bot, err := tgbotapi.NewBotAPI(viper.GetString("tg_bot_token"))
	if err != nil {
		log.Fatalf("Failed to create bot: %s", err)
	}

	if *debug {
		bot.Debug = true
	}

	log.Printf("Authorized on account %s", bot.Self.UserName)

	alertChan := make(chan Alert, 1000)

	go tgBotHandleAlerts(bot, alertChan)

	processAlerts := makeHandler(alertChan)
	http.HandleFunc("/", processAlerts)
	if err := http.ListenAndServe(viper.GetString("listen_port"), nil); err != nil {
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
{{- if isMap .Labels -}}
{{ range $k, $v := .Labels }}
{{ $k }}: {{ $v }}
{{- end -}}
{{- end -}}

{{- if isMap .Annotations -}}
{{ range $k, $v := .Annotations }}
{{ $k }}: {{ $v }}
{{- end -}}
{{- end -}}
`

func tgBotHandleAlerts(bot *tgbotapi.BotAPI, alertChan <-chan Alert) {
	funcMap := template.FuncMap{"isMap": func(i interface{}) bool {
		v := reflect.ValueOf(i)
		switch v.Kind() {
		case reflect.Map:
			return true
		default:
			return false
		}
	}}
	t := template.Must(template.New("template").Funcs(funcMap).Parse(tmpl))

	for a := range alertChan {
		var bytesBuff bytes.Buffer
		writer := io.Writer(&bytesBuff)
		err := t.Execute(writer, a)
		if err != nil {
			log.Errorf("failed to parse alert: %s", err)
		}

		alert := bytesBuff.String()

		msg := tgbotapi.NewMessage(viper.GetInt64("tg_chat_id"), alert)
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
