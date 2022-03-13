mod duration;
mod server;

extern crate dotenv;
extern crate pretty_env_logger;

use std::sync::Arc;

use anyhow::Result;
use chrono::{DateTime, Local};
use clap::{ArgGroup, Parser};
use dotenv::dotenv;
use handlebars::{handlebars_helper, Handlebars};
use matrix_sdk::{ruma::UserId, Client};
use teloxide::Bot;
use warp::Filter;

use crate::duration::format_duration;
use crate::server::{send_message_matrix, send_message_tg};

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("req_flags")
        .required(true)
        .multiple(true)
        .args(&["tg", "matrix"]),
))]
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
        requires = "tg-token",
        requires = "tg-chat-id",
        requires = "tg-template-path",
        help = "Enable Telegram support"
    )]
    tg: bool,

    #[clap(long, env = "ALERTER_TG_BOT_TOKEN", help = "Telegram bot token.")]
    tg_token: Option<String>,

    #[clap(long, env = "ALERTER_TG_CHAT_ID", help = "Telegram chat ID.")]
    tg_chat_id: Option<i64>,

    #[clap(
        long,
        env = "ALERTER_TG_TMPL_PATH",
        default_value = "templates/default.tg.hbs",
        help = "Path to handlebars template file for Telegram."
    )]
    tg_template_path: String,

    #[clap(
        long,
        requires = "matrix-user",
        requires = "matrix-pass",
        requires = "matrix-room-id",
        requires = "matrix-template-path",
        help = "Enable Matrix support"
    )]
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
    matrix_template_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    dotenv().ok();
    let args = Args::parse();

    let mut tg_token: String = "".to_owned();
    let mut tg_chat_id: i64 = 0;

    if args.tg {
        tg_token = args.tg_token.unwrap();
        tg_chat_id = args.tg_chat_id.unwrap();
    }

    let bot = Bot::new(tg_token);

    let mut matrix_user: String = "".to_owned();
    let mut matrix_pass: String = "".to_owned();
    let mut matrix_room_id: String = "".to_owned();

    if args.matrix {
        matrix_user = args.matrix_user.unwrap();
        matrix_pass = args.matrix_pass.unwrap();
        matrix_room_id = args.matrix_room_id.unwrap();
    }

    let matrix_user_id = UserId::try_from(matrix_user)?;
    let matrix_client = Client::new_from_user_id(matrix_user_id.clone()).await?;

    matrix_client
        .login(
            matrix_user_id.localpart(),
            &matrix_pass,
            Some("Alerter bot"),
            None,
        )
        .await?;

    let mut hb = Handlebars::new();
    hb.register_template_file("default_tg", args.tg_template_path)?;
    hb.register_template_file("default_matrix", args.matrix_template_path)?;
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
        .and(warp::any().map(move || matrix_room_id.clone()))
        .and_then(send_message_matrix);

    let server = warp::serve(tg_route.or(matrix_route));

    server.run(([0, 0, 0, 0], args.port)).await;

    Ok(())
}
