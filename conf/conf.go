package conf

import "github.com/spf13/viper"

func GetConfig(confName string) error {
	viper.SetConfigName(confName)
	viper.AddConfigPath(".")
	viper.AddConfigPath("/etc/alerter/")

	if err := viper.ReadInConfig(); err != nil {
		return err
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

	return nil
}
