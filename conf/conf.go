package conf

import (
	log "github.com/sirupsen/logrus"
	"github.com/spf13/viper"
)

// Config is the configuration.
var Config struct {
	ListenPort   string `mapstructure:"listen_port"`
	TemplatePath string `mapstructure:"tmpl_path"`
	TgBotToken   string `mapstructure:"tg_bot_token"`
	TgChatID     int64  `mapstructure:"tg_chat_id"`
}

// Load parses the config from file or from ENV variables into a Config.
func Load(confName string) error {
	viper.SetConfigName(confName)
	viper.AddConfigPath(".")
	if err := viper.ReadInConfig(); err != nil {
		log.Warnf("Error decoding config file from %s: %s", confName, err)
	}

	viper.SetEnvPrefix("alerter")
	if err := viper.BindEnv("listen_port"); err != nil {
		return err
	}
	if err := viper.BindEnv("tmpl_path"); err != nil {
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
