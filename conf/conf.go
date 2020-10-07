package conf

import (
	log "github.com/sirupsen/logrus"
	"github.com/spf13/viper"
)

var Config struct {
	ListenPort string `mapstructure:"listen_port"`
	TgBotToken string `mapstructure:"tg_bot_token"`
	ChatID     int64  `mapstructure:"tg_chat_id"`
}

func GetConfig(confName string) error {
	viper.SetConfigName(confName)
	viper.AddConfigPath(".")

	if err := viper.ReadInConfig(); err != nil {
		log.Warn("Failed to read config file: %s", err)
	}

	viper.SetEnvPrefix("alerter")
	if err := viper.BindEnv("listen_port"); err != nil {
		return err
	}
	if err := viper.BindEnv("tg_bot_token"); err != nil {
		return err
	}
	if err := viper.BindEnv("tg_chat_id"); err != nil {
		return err
	}

	if err := viper.Unmarshal(&Config); err != nil {
		return err
	}

	return nil
}
