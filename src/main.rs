mod duration;
mod server;

extern crate dotenv;

use std::{fs, sync::Arc};

use anyhow::Result;
use chrono::{DateTime, Local};
use clap::Parser;
use dotenv::dotenv;
use handlebars::{handlebars_helper, Handlebars};
use teloxide::Bot;
use warp::Filter;

use crate::duration::format_duration;
use crate::server::send_message;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(
        short,
        long,
        env = "ALERTER_LISTEN_PORT",
        default_value_t = 48655,
        help = "Port to listen."
    )]
    port: u16,

    #[clap(
        long,
        env = "ALERTER_TMPL_PATH",
        default_value = "templates/default.hbs",
        help = "Path to handlebars template file."
    )]
    template_path: String,

    #[clap(long, env = "ALERTER_TG_BOT_TOKEN", help = "Telegram bot token.")]
    token: String,

    #[clap(long, env = "ALERTER_TG_CHAT_ID", help = "Telegram chat ID.")]
    chat_id: i64,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let args = Args::parse();

    let listen_port = args.port;
    let tmpl_path = args.template_path;
    let token = args.token;
    let chat_id = args.chat_id;

    let bot = Bot::new(token);

    let tpl_str = fs::read_to_string(tmpl_path)?;
    let mut hb = Handlebars::new();
    hb.register_template_string("default", tpl_str)?;
    handlebars_helper!(eq: |x: str, { compare: str = "firing" }| x == compare);
    handlebars_helper!(since: |x: str| {
        let time = DateTime::parse_from_rfc3339(x).unwrap();
        let now = Local::now();
        format_duration(now.signed_duration_since(time))
    });
    handlebars_helper!(duration: |x: str, {y: str = ""}| {
        let from = DateTime::parse_from_rfc3339(x).unwrap();
        let to = DateTime::parse_from_rfc3339(y).unwrap();
        format_duration(to.signed_duration_since(from))
    });
    hb.register_helper("eq", Box::new(eq));
    hb.register_helper("since", Box::new(since));
    hb.register_helper("duration", Box::new(duration));

    let hb = Arc::new(hb);

    warp::serve(
        warp::path::end()
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map(move || bot.clone()))
            .and(warp::any().map(move || hb.clone()))
            .and(warp::any().map(move || chat_id))
            .and_then(send_message),
    )
    .run(([0, 0, 0, 0], listen_port))
    .await;

    Ok(())
}
