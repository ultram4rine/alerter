# Alerter

Telegram bot for alerts from Prometheus

## Configuration

Use environment variables or command-line flags to configure `alerter`:

| Name                 | Default               | Description                       |
| -------------------- | --------------------- | --------------------------------- |
| ALERTER_LISTEN_PORT  | 48655                 | Port to listen.                   |
| ALERTER_TMPL_PATH    | templates/default.hbs | Path to handlebars template file. |
| ALERTER_TG_BOT_TOKEN |                       | Telegram bot token.               |
| ALERTER_TG_CHAT_ID   |                       | Telegram chat ID.                 |

Flags:

```sh
$ ./alerter --help
alerter 0.3.0
Telegram bot for alerts from Prometheus

USAGE:
    alerter [OPTIONS] --token <TOKEN> --chat-id <CHAT_ID>

OPTIONS:
        --chat-id <CHAT_ID>
            Telegram chat ID. [env: ALERTER_TG_CHAT_ID=]

    -h, --help
            Print help information

    -p, --port <PORT>
            Port to listen. [env: ALERTER_LISTEN_PORT=] [default: 48655]

        --template-path <TEMPLATE_PATH>
            Path to handlebars template file. [env: ALERTER_TMPL_PATH=] [default:
            templates/default.hbs]

        --token <TOKEN>
            Telegram bot token. [env: ALERTER_TG_BOT_TOKEN=]

    -V, --version
            Print version information
```

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

3. Package should be in `target/generate-rpm/alerter-VERSION-RELEASE.ARCH.rpm`.

### Build from source

1. Build binary

   ```sh
   cargo build --release
   ```

2. Binary should be in `target/release/alerter`.
