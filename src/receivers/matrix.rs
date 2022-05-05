use log::warn;
use std::{
    convert::{Infallible, TryFrom},
    sync::Arc,
};

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
