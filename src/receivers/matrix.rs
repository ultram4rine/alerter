use log::warn;
use std::{convert::Infallible, sync::Arc};

use handlebars::Handlebars;
use matrix_sdk::{
    ruma::{
        api::client::message::send_message_event,
        events::{
            room::message::{MessageType, RoomMessageEventContent, TextMessageEventContent},
            AnyMessageLikeEventContent,
        },
        RoomId, TransactionId,
    },
    Client,
};
use warp::http::StatusCode;

use crate::webhook::WebHook;

/// Sends message to Matrix room.
///
/// # Arguments
///
/// * `webhook` - A `WebHook` from Alertmanager.
/// * `maybe_client` - An optional `Client` instance. If `None`, status code `503` would be returned.
/// * `hb` - A handlebars registry with `default_matrix` template.
/// * `room_id` - An ID of room to send message to.
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

            match client
                .send(
                    match send_message_event::v3::Request::new(
                        &RoomId::parse(room_id).unwrap(),
                        &TransactionId::new(),
                        &AnyMessageLikeEventContent::RoomMessage(RoomMessageEventContent::new(
                            MessageType::Text(TextMessageEventContent::markdown(msg_text)),
                        )),
                    ) {
                        Ok(req) => req,
                        Err(err) => {
                            warn!("failed to create request: {}", err);
                            return Ok(StatusCode::INTERNAL_SERVER_ERROR);
                        }
                    },
                    None,
                )
                .await
            {
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
