# Alerter

Нотификатор алертов в телеграм канал для AlertManager

## Установка

Зависимости:
* Cargo


### Вручную

Для того что бы забилдить:

```
git clone https://git.sgu.ru/git/ultramarine/alerter.git
cd alerter
cargo build --release
```

Может быть будет не хватать некоторых библиотек (например openssl), тогда сборщик поругается и скажет чего не хватает.
Соотвественно надо установить (например ```dnf install openssl```).

После бинарник нам становится доступен по пути ```target/release/alerter```

### RPM

Предварительно устанавливаем нужный для этого пакет ```cargo install cargo-generate-rpm```

После:
```
cargo build --release
strip -s target/release/alerter
cargo generate-rpm
```

Готовый rpm-файл лежит в ```target/generate-rpm/alerter.rpm```

## Конфигурационные файлы

Alerter берет конфиги из переменного окружения:

Имя | Описание
:---|:--------
ALERTER_LISTEN_PORT | HTTP порт на котором принимает сообщения от Alertmanager
ALERTER_TMPL_PATH | Шаблон отправляемых сообщений в телеграм
ALERTER_TG_BOT_TOKEN | Токен бота в телеграме
ALERTER_TG_CHAT_ID | ID чата в телеграме