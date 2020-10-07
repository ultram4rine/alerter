package conf

import "github.com/spf13/viper"

var Config struct {
	ListenPort string `mapstructure:"listen_port"`
	TgBotToken string `mapstructure:"tg_bot_token"`
	ChatID     int64  `mapstructure:"tg_chat_id"`
}

func prepareConfig(confName string) error {
	viper.SetConfigName(confName)
	viper.AddConfigPath(".")
	viper.AddConfigPath("/etc/alerter/")

	if err := viper.ReadInConfig(); err != nil {
		return err
	}

	viper.SetEnvPrefix("alerter")
	viper.AutomaticEnv()

	return nil
}

func GetConfig(confName string) error {
	if err := prepareConfig(confName); err != nil {
		return err
	}

	if err := viper.Unmarshal(&Config); err != nil {
		return err
	}

	return nil
}
