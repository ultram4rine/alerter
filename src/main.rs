mod duration;
mod server;

extern crate dotenv;

use std::{env, fs, sync::Arc};

use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use dotenv::dotenv;
use handlebars::{handlebars_helper, Handlebars};
use teloxide::Bot;
use warp::Filter;

use crate::duration::format_duration;
use crate::server::send_message;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let listen_port = match env::var("ALERTER_LISTEN_PORT")
        .expect("LISTEN_PORT not set")
        .parse::<u16>()
    {
        Ok(v) => v,
        Err(err) => {
            return Err(anyhow!(format!(
                "failed to parse LISTEN_PORT to integer: {}",
                err
            )));
        }
    };
    let tmpl_path = env::var("ALERTER_TMPL_PATH").expect("TMPL_PATH not set");
    let token = env::var("ALERTER_TG_BOT_TOKEN").expect("TG_BOT_TOKEN not set");
    let chat_id = match env::var("ALERTER_TG_CHAT_ID")
        .expect("TG_CHAT_ID not set")
        .parse::<i64>()
    {
        Ok(v) => v,
        Err(err) => {
            return Err(anyhow!(format!(
                "failed to parse TG_CHAT_ID to integer: {}",
                err
            )));
        }
    };

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
            .and(warp::any().map(move || chat_id.clone()))
            .and_then(send_message),
    )
    .run(([0, 0, 0, 0], listen_port))
    .await;

    Ok(())
}
