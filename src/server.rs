use log::error;
use std::{convert::Infallible, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use handlebars::Handlebars;
use teloxide::{
    requests::{Request, Requester},
    types::ParseMode::Html,
    Bot,
};
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
    bot: Bot,
    hb: Arc<Handlebars<'_>>,
    chat_id: i64,
) -> Result<impl warp::Reply, Infallible> {
    let msg_text = match hb.render("default", &webhook) {
        Ok(v) => v,
        Err(err) => {
            error!("failed to render message: {}", err);
            return Ok(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let mut msg = bot.send_message(chat_id, msg_text);
    msg.parse_mode = Some(Html);

    match msg.send().await {
        Ok(_) => Ok(StatusCode::OK),
        Err(err) => {
            error!("failed to send message: {}", err);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
