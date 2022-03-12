use log::warn;
use std::{
    convert::{Infallible, TryFrom},
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use handlebars::Handlebars;
use matrix_sdk::{
    ruma::{
        api::client::r0::message::send_message_event,
        events::{
            room::message::{MessageEventContent, MessageType, TextMessageEventContent},
            AnyMessageEventContent,
        },
        identifiers::RoomId,
    },
    Client,
};
use teloxide::{
    requests::{Request, Requester},
    types::ParseMode,
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

/// Sends message to Telegram chat.
pub async fn send_message_tg(
    webhook: WebHook,
    bot: Bot,
    hb: Arc<Handlebars<'_>>,
    chat_id: i64,
) -> Result<impl warp::Reply, Infallible> {
    let msg_text = match hb.render("default_tg", &webhook) {
        Ok(v) => v,
        Err(err) => {
            warn!("failed to render message: {}", err);
            return Ok(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let mut msg = bot.send_message(chat_id, msg_text.clone());
    msg.parse_mode = Some(ParseMode::Html);

    match msg.send().await {
        Ok(_) => Ok(StatusCode::OK),
        Err(err) => {
            warn!("failed to send message: {}", err);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Sends message to Matrix room.
pub async fn send_message_matrix(
    webhook: WebHook,
    client: Client,
    hb: Arc<Handlebars<'_>>,
    room_id: String,
) -> Result<impl warp::Reply, Infallible> {
    let msg_text = match hb.render("default_matrix", &webhook) {
        Ok(v) => v,
        Err(err) => {
            warn!("failed to render message: {}", err);
            return Ok(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    client
        .send(
            send_message_event::Request::new(
                &RoomId::try_from(room_id).unwrap(),
                "1",
                &AnyMessageEventContent::RoomMessage(MessageEventContent::new(MessageType::Text(
                    TextMessageEventContent::markdown(msg_text),
                ))),
            ),
            None,
        )
        .await
        .unwrap();

    Ok(StatusCode::OK)
}
