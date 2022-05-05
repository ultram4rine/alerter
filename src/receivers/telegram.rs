use log::warn;
use std::{convert::Infallible, sync::Arc};

use handlebars::Handlebars;
use teloxide::{
    requests::{Request, Requester},
    types::{ChatId, ParseMode},
    Bot,
};
use warp::http::StatusCode;

use crate::webhook::WebHook;

/// Sends message to Telegram chat.
///
/// # Arguments
///
/// * `webhook` - A `WebHook` from Alertmanager.
/// * `maybe_bot` - An optional `Bot` instance. If `None`, status code `503` would be returned.
/// * `hb` - A handlebars registry with `default_tg` template.
/// * `chat_id` - A `ChatId` of chat to send message to.
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
