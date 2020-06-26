package main

import (
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

	msg := tgbotapi.NewMessage(conf.TgChatID, "hello test")
	bot.Send(msg)

	log.Printf("Authorized on account %s", bot.Self.UserName)

	alertChan := make(chan string, 1000)

	go tgBotHandleUpdates(bot, alertChan)

	processAlerts := makeHandler(alertChan)
	http.HandleFunc("/", processAlerts)
	if err := http.ListenAndServe(conf.ListenPort, nil); err != nil {
		log.Fatal(err)
	}
}

func makeHandler(alertChan chan string) func(http.ResponseWriter, *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/" {
			http.Error(w, "404 Not Found", http.StatusNotFound)
			return
		}

		switch r.Method {
		case "POST":
			alertChan <- r.Host
			log.Println(r.Host)
		default:
			http.Error(w, "405 Method Not Allowed", http.StatusMethodNotAllowed)
		}
	}
}

func tgBotHandleUpdates(bot *tgbotapi.BotAPI, alertChan <-chan string) {
	u := tgbotapi.NewUpdate(0)
	u.Timeout = 60

	alert := <-alertChan
	log.Println(alert)
	msg := tgbotapi.NewMessage(-1001453885388, alert)
	bot.Send(msg)

	updates, err := bot.GetUpdatesChan(u)
	if err != nil {
		log.Fatal(err)
	}

	for update := range updates {
		if update.Message == nil {
			continue
		}

		log.Printf("[%s] %s", update.Message.From.UserName, update.Message.Text)

		msg := tgbotapi.NewMessage(update.Message.Chat.ID, update.Message.Text)
		msg.ReplyToMessageID = update.Message.MessageID

		bot.Send(msg)
	}
}
