# Alerter

Telegram bot for alerts from Prometheus

## Configuration

Use environment variables to configure `Alerter`. All configuration options are required.

| Name                 | Description                       |
| -------------------- | --------------------------------- |
| ALERTER_LISTEN_PORT  | Port to listen.                   |
| ALERTER_TMPL_PATH    | Path to handlebars template file. |
| ALERTER_TG_BOT_TOKEN | Telegram bot token.               |
| ALERTER_TG_CHAT_ID   | Telegram chat ID.                 |

## Setup

You need [Rust](https://www.rust-lang.org/tools/install) installed.

### RPM

1. Install [cargo-generate-rpm](https://crates.io/crates/cargo-generate-rpm):

   ```sh
   cargo install cargo-generate-rpm
   ```

2. Build binary and RPM package:

   ```sh
   cargo build --release
   strip -s target/release/alerter
   cargo generate-rpm
   ```

3. Package should be in `target/generate-rpm/alerter-VERSION.ARCH.rpm`.

### Build from source

1. Build binary

   ```sh
   cargo build --release
   ```

2. Binary should be in `target/release/alerter`.
