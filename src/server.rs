use std::{convert::Infallible, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use handlebars::Handlebars;
use telegram_bot::{Api, ChatId, ParseMode, SendMessage};
use tokio::sync::Mutex;
use warp::http::StatusCode;

#[derive(Serialize, Deserialize)]
pub struct WebHook {
    pub status: String,
    pub alerts: Vec<Alert>,
}

#[derive(Serialize, Deserialize)]
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
    hb: Arc<Handlebars<'_>>,
    chat_id: i64,
) -> Result<impl warp::Reply, Infallible> {
    let b = bot.lock().await;

    let msg_text = match hb.render("default", &webhook) {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            return Ok(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match b
        .send(SendMessage::new(ChatId::from(chat_id), msg_text).parse_mode(ParseMode::Html))
        .await
    {
        Ok(_) => return Ok(StatusCode::OK),
        Err(e) => {
            println!("{:?}", e);
            return Ok(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
}
