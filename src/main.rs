mod server;

extern crate dotenv;

use std::env;
use std::fs;
use std::sync::Arc;

use chrono::prelude::*;
use dotenv::dotenv;
use handlebars::{handlebars_helper, Handlebars};
use telegram_bot::{Api, Error};
use tokio::sync::Mutex;
use warp::Filter;

use crate::server::send_message;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let bot = Arc::new(Mutex::new(Api::new(token)));

    let tpl_str = fs::read_to_string("templates/default.hbs").unwrap();
    let mut hb = Handlebars::new();
    hb.register_template_string("default", tpl_str).unwrap();
    handlebars_helper!(eq: |x: str, { compare: str = "firing" }| x == compare);
    handlebars_helper!(since:|x:str| {
        let time=DateTime::parse_from_rfc3339(x).unwrap();
        let now=Local::now();
        format!("{}", now.signed_duration_since(time).num_days())
    });
    handlebars_helper!(duration:|x:str, {y: str = ""}| {
        let from=DateTime::parse_from_rfc3339(x).unwrap();
        println!("{}", y);
        let to=DateTime::parse_from_rfc3339(y).unwrap();
        format!("{}", to.signed_duration_since(from).num_days())
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
            .and_then(send_message),
    )
    .run(([0, 0, 0, 0], 3030))
    .await;

    Ok(())
}
