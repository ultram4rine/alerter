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
    types::{ChatId, ParseMode},
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
    maybe_bot: Option<Bot>,
    hb: Arc<Handlebars<'_>>,
    chat_id: ChatId,
) -> Result<impl warp::Reply, Infallible> {
    match maybe_bot {
        Some(bot) => {
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
        None => Ok(StatusCode::SERVICE_UNAVAILABLE),
    }
}

/// Sends message to Matrix room.
pub async fn send_message_matrix(
    webhook: WebHook,
    maybe_client: Option<Client>,
    hb: Arc<Handlebars<'_>>,
    room_id: String,
) -> Result<impl warp::Reply, Infallible> {
    match maybe_client {
        Some(client) => {
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
                        &AnyMessageEventContent::RoomMessage(MessageEventContent::new(
                            MessageType::Text(TextMessageEventContent::markdown(msg_text)),
                        )),
                    ),
                    None,
                )
                .await
                .unwrap();

            Ok(StatusCode::OK)
        }
        None => Ok(StatusCode::SERVICE_UNAVAILABLE),
    }
}
