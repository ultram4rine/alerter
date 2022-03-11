# Alerter

[![Crates.io](https://img.shields.io/crates/v/alerter?style=flat-square)](https://crates.io/crates/alerter) [![GitHub Workflow Status](https://img.shields.io/github/workflow/status/ultram4rine/alerter/CICD?label=CI%2FCD&logo=github&style=flat-square)](https://github.com/ultram4rine/alerter/actions/workflows/cicd.yml)

Telegram bot for alerts from [Alertmanager](https://github.com/prometheus/alertmanager)

## Usage

1. Create bot with [BotFather](https://t.me/BotFather) and get token.

2. Get your, group or channel chat id with [Get My ID](https://t.me/getmyid_bot) bot.

3. [Configure](#configuration) `alerter`.

4. Add `alerter` to `receivers` in your alertmanager.yml:

   ```text
   receivers:
   - name: alerter
     webhook_configs:
     - send_resolved: true
       url: 'http://127.0.0.1:48655'
   ```

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

| Environment variable | Command-line flag | Default               | Description                       |
| -------------------- | ----------------- | --------------------- | --------------------------------- |
| ALERTER_LISTEN_PORT  | --port (-p)       | 48655                 | Port to listen.                   |
| ALERTER_TMPL_PATH    | --template-path   | templates/default.hbs | Path to handlebars template file. |
| ALERTER_TG_BOT_TOKEN | --token           |                       | Telegram bot token.               |
| ALERTER_TG_CHAT_ID   | --chat-id         |                       | Telegram chat ID.                 |
