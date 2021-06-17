mod server;

extern crate dotenv;

use std::env;
use std::fs;
use std::sync::Arc;

use anyhow::Result;
use chrono::prelude::*;
use dotenv::dotenv;
use handlebars::{handlebars_helper, Handlebars};
use telegram_bot::Api;
use tokio::sync::Mutex;
use warp::Filter;

use crate::server::send_message;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let listen_port = env::var("LISTEN_PORT")
        .expect("LISTEN_PORT not set")
        .parse::<u16>()?;
    let tmpl_path = env::var("TMPL_PATH").expect("TMPL_PATH not set");
    let token = env::var("TG_BOT_TOKEN").expect("TG_BOT_TOKEN not set");
    let chat_id = env::var("TG_CHAT_ID")
        .expect("TG_CHAT_ID not set")
        .parse::<i64>()?;

    let bot = Arc::new(Mutex::new(Api::new(token)));

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

fn format_duration(duration: chrono::Duration) -> String {
    let weeks = duration.num_weeks();
    let days = duration.num_days() - weeks * 7;
    let hours = duration.num_hours() - (weeks * 7 + days) * 24;
    let minutes = duration.num_minutes() - (((weeks * 7 + days) * 24) + hours) * 60;
    let seconds =
        duration.num_seconds() - (((((weeks * 7 + days) * 24) + hours) * 60) + minutes) * 60;
    let milliseconds = duration.num_milliseconds()
        - ((((((weeks * 7 + days) * 24) + hours) * 60) + minutes) * 60 + seconds) * 1000;

    let f_weeks = format_unit("week".to_string(), "weeks".to_string(), weeks);
    let f_days = format_unit("day".to_string(), "days".to_string(), days);
    let f_hours = format_unit("hour".to_string(), "hours".to_string(), hours);
    let f_minutes = format_unit("minute".to_string(), "minutes".to_string(), minutes);
    let f_seconds = format_unit("second".to_string(), "seconds".to_string(), seconds);
    let f_milliseconds = if seconds == 0 {
        format_unit(
            "millisecond".to_string(),
            "milliseconds".to_string(),
            milliseconds,
        )
    } else {
        "".to_string()
    };

    format!(
        "{} {} {} {} {} {}",
        f_weeks, f_days, f_hours, f_minutes, f_seconds, f_milliseconds
    )
    .trim()
    .to_string()
}

fn format_unit(unit: String, units: String, count: i64) -> String {
    match count {
        0 => "".to_string(),
        1 => format!("1 {}", unit),
        _ => format!("{} {}", count, units),
    }
}
