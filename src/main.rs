mod server;

extern crate dotenv;

use std::env;
use std::fs;
use std::sync::Arc;
use tokio::sync::Mutex;

use dotenv::dotenv;
use handlebars::{handlebars_helper, Handlebars};
use telegram_bot::{Api, Error};
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
    hb.register_helper("eq", Box::new(eq));

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
