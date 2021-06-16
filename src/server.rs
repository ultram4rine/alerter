use std::{convert::Infallible, sync::Arc};

use serde::Deserialize;
use serde_json::Value;

use telegram_bot::{Api, ChatId, ParseMode, SendMessage};
use tokio::sync::Mutex;
use warp::http::StatusCode;

#[derive(Deserialize)]
pub struct WebHook {
    pub status: String,
    pub alerts: Vec<Alert>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub labels: Value,
    pub annotations: Value,
    pub starts_at: String,
    pub ends_at: String,
}

pub async fn send_message(
    webhook: WebHook,
    bot: Arc<Mutex<Api>>,
) -> Result<impl warp::Reply, Infallible> {
    let b = bot.lock().await;
    match b
        .send(SendMessage::new(ChatId::from(101814676), webhook.status).parse_mode(ParseMode::Html))
        .await
    {
        Ok(_) => return Ok(StatusCode::OK),
        Err(e) => {
            println!("{:?}", e);
            return Ok(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
}
