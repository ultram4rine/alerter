mod server;

extern crate dotenv;

use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

use dotenv::dotenv;
use telegram_bot::{Api, Error};
use warp::Filter;

use crate::server::send_message;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let bot = Arc::new(Mutex::new(Api::new(token)));

    warp::serve(
        warp::path::end()
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map(move || bot.clone()))
            .and_then(send_message),
    )
    .run(([0, 0, 0, 0], 3030))
    .await;

    Ok(())
}
