# Alerter

[![Crates.io](https://img.shields.io/crates/v/alerter?style=flat-square)](https://crates.io/crates/alerter) [![Docker Image Size (latest by date)](https://img.shields.io/docker/image-size/ultram4rine/alerter?logo=docker&style=flat-square)](https://hub.docker.com/r/ultram4rine/alerter) [![GitHub Workflow Status](https://img.shields.io/github/workflow/status/ultram4rine/alerter/CICD?label=CI%2FCD&logo=github&style=flat-square)](https://github.com/ultram4rine/alerter/actions/workflows/cicd.yml)

Telegram and Matrix bot for alerts from [Alertmanager](https://github.com/prometheus/alertmanager)

## Usage

### Telegram preparations

1. Create bot with [BotFather](https://t.me/BotFather) and get token.

2. Get your, group or channel chat id with [Get My ID](https://t.me/getmyid_bot) bot.

### Matrix preparations

1. Create user under which `alerter` will work.

2. Add this user to room for alerts and get `room_id`.

After preparations:

1. [Configure](#configuration) `alerter`.

2. Optionally check that messages are being sent and properly formatted:

   ```sh
   ./test.sh tg
   ```

   Or

   ```sh
   ./test.sh matrix
   ```

3. Add `alerter` to `receivers` in your alertmanager.yml:

   ```text
   receivers:
   - name: alerter_tg
     webhook_configs:
     - send_resolved: true
       url: 'http://127.0.0.1:48655/tg'
   - name: alerter_matrix
     webhook_configs:
     - send_resolved: true
       url: 'http://127.0.0.1:48655/matrix'
   ```

### Templating

You can modify default templates for your needs, they are in [Handlebars](https://handlebarsjs.com/guide/) format. For Telegram `alerter` uses [HTML](https://core.telegram.org/bots/api#html-style) style and for Matrix [MarkDown](https://doc.matrix.tu-dresden.de/en/messaging/formatting/) style.

## Installation

### Prebuilt packages

You can download prebuilt archive or Deb/RPM package from [releases](https://github.com/ultram4rine/alerter/releases).

### Docker

Pull image from [DockerHub](https://hub.docker.com/r/ultram4rine/alerter):

```sh
docker pull ultram4rine/alerter
```

In other cases, you need [Rust](https://www.rust-lang.org/tools/install) installed.

### Crates.io

You can install alerter from [crates.io](https://crates.io/crates/alerter):

```sh
cargo install alerter
```

Then download [template](./templates/default.hbs) and run.

### Build from source

1. Clone repository:

   ```sh
   git clone https://github.com/ultram4rine/alerter.git
   cd alerter
   ```

2. Build binary:

   ```sh
   cargo build --release
   ```

3. Binary should be in `target/release/alerter`.

### Build Deb or RPM package

See [cargo-deb](https://github.com/kornelski/cargo-deb#usage) or [cargo-generate-rpm](https://github.com/cat-in-136/cargo-generate-rpm#usage) instructions respectively.

## Configuration

Use environment variables or command-line flags to configure `alerter`:

| Environment variable     | Command-line flag      | Default                      | Description                                                               |
| ------------------------ | ---------------------- | ---------------------------- | ------------------------------------------------------------------------- |
| ALERTER_LISTEN_PORT      | --port (-p)            | 48655                        | Port to listen.                                                           |
|                          | --tg                   |                              | Enable Telegram support.                                                  |
| ALERTER_TG_BOT_TOKEN     | --tg-token             |                              | Telegram bot token. Required for Telegram support.                        |
| ALERTER_TG_CHAT_ID       | --chat-id              |                              | Telegram chat ID. Required for Telegram support.                          |
| ALERTER_TG_TMPL_PATH     | --template-path        | templates/default.tg.hbs     | Path to handlebars template file. Required for Telegram support.          |
|                          | --matrix               |                              | Enable Matrix support.                                                    |
| ALERTER_MATRIX_USERNAME  | --matrix-user          |                              | Matrix username. Required for Matrix support.                             |
| ALERTER_MATRIX_PASSWORD  | --matrix-pass          |                              | Matrix password. Required for Matrix support.                             |
| ALERTER_MATRIX_ROOM_ID   | --matrix-room-id       |                              | Matrix room id. Required for Matrix support.                              |
| ALERTER_MATRIX_TMPL_PATH | --matrix-template-path | templates/default.matrix.hbs | Path to handlebars template file for Matrix. Required for Matrix support. |
