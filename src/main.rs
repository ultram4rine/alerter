mod duration;
mod server;

extern crate dotenv;
extern crate pretty_env_logger;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use clap::Parser;
use dotenv::dotenv;
use handlebars::{handlebars_helper, Handlebars};
use matrix_sdk::{ruma::UserId, Client};
use teloxide::Bot;
use warp::Filter;

use crate::duration::format_duration;
use crate::server::{send_message_matrix, send_message_tg};

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

    #[clap(long, env = "ALERTER_TG", help = "Enable matrix support")]
    tg: bool,

    #[clap(long, env = "ALERTER_TG_BOT_TOKEN", help = "Telegram bot token.")]
    tg_token: String,

    #[clap(long, env = "ALERTER_TG_CHAT_ID", help = "Telegram chat ID.")]
    tg_chat_id: i64,

    #[clap(
        long,
        env = "ALERTER_TG_TMPL_PATH",
        default_value = "templates/default.tg.hbs",
        help = "Path to handlebars template file for Telegram."
    )]
    template_tg_path: String,

    #[clap(long, env = "ALERTER_MATRIX", help = "Enable matrix support")]
    matrix: bool,

    #[clap(long, env = "ALERTER_MATRIX_USERNAME", help = "Matrix username")]
    matrix_user: Option<String>,

    #[clap(long, env = "ALERTER_MATRIX_PASSWORD", help = "Matrix password")]
    matrix_pass: Option<String>,

    #[clap(long, env = "ALERTER_MATRIX_ROOM_ID", help = "Matrix room id")]
    matrix_room_id: Option<String>,

    #[clap(
        long,
        env = "ALERTER_MATRIX_TMPL_PATH",
        default_value = "templates/default.matrix.hbs",
        help = "Path to handlebars template file for Matrix."
    )]
    template_matrix_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    dotenv().ok();
    let args = Args::parse();

    if !args.tg && !args.matrix {
        return Err(anyhow!("Either tg or matrix flag should be specified"));
    }

    let tg_token = args.tg_token;
    let tg_chat_id = args.tg_chat_id;

    let bot = Bot::new(tg_token);

    let mut hb = Handlebars::new();
    hb.register_template_file("default_tg", args.template_tg_path)?;
    hb.register_template_file("default_matrix", args.template_matrix_path)?;
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

    let hb_tg = Arc::new(hb);
    let hb_matrix = hb_tg.clone();

    let matrix_user = UserId::try_from(args.matrix_user.unwrap())?;
    let matrix_client = Client::new_from_user_id(matrix_user.clone()).await?;

    // First we need to log in.
    matrix_client
        .login(
            matrix_user.localpart(),
            &args.matrix_pass.unwrap(),
            None,
            None,
        )
        .await?;

    let tg_route = warp::post()
        .and(warp::path!("tg"))
        .and(warp::body::json())
        .and(warp::any().map(move || bot.clone()))
        .and(warp::any().map(move || hb_tg.clone()))
        .and(warp::any().map(move || tg_chat_id))
        .and_then(send_message_tg);

    let matrix_route = warp::post()
        .and(warp::path!("matrix"))
        .and(warp::body::json())
        .and(warp::any().map(move || matrix_client.clone()))
        .and(warp::any().map(move || hb_matrix.clone()))
        .and(warp::any().map(move || args.matrix_room_id.clone().unwrap()))
        .and_then(send_message_matrix);

    let server = warp::serve(tg_route.or(matrix_route));

    server.run(([0, 0, 0, 0], args.port)).await;

    Ok(())
}
