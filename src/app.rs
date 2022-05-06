use std::sync::Arc;

use chrono::{DateTime, Local};
use handlebars::{handlebars_helper, Handlebars};
use matrix_sdk::Client;
use teloxide::{types::ChatId, Bot};
use warp::Filter;

use crate::{
    duration::format_duration,
    receivers::{matrix::send_message_matrix, telegram::send_message_tg},
    webhook::WebHook,
};

pub fn create_hb(
    tg_template_path: String,
    matrix_template_path: String,
) -> Result<Arc<Handlebars<'static>>, anyhow::Error> {
    let mut hb = Handlebars::new();
    hb.register_template_file("default_tg", tg_template_path)?;
    hb.register_template_file("default_matrix", matrix_template_path)?;
    handlebars_helper!(eq: |x: str, { compare: str = "firing" }| x == compare);
    handlebars_helper!(since: |x: str| {
        let time = DateTime::parse_from_rfc3339(x).unwrap();
        let now = Local::now();
        format_duration(now.signed_duration_since(time))
    });
    handlebars_helper!(duration: |x: str, {y: str = ""}| {
        let from = DateTime::parse_from_rfc3339(x).unwrap();
        let to = DateTime::parse_from_rfc3339(y).unwrap();
        format_duration(to.signed_duration_since(from))
    });
    hb.register_helper("eq", Box::new(eq));
    hb.register_helper("since", Box::new(since));
    hb.register_helper("duration", Box::new(duration));

    Ok(Arc::new(hb))
}

pub fn create_filter(
    maybe_tg_bot: Option<Bot>,
    maybe_matrix_client: Option<Client>,
    hb: Arc<Handlebars<'_>>,
    tg_chat_id: ChatId,
    matrix_room_id: String,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone + '_ {
    let tg_route = warp::path!("tg")
        .and(warp::post())
        .and(json_body())
        .and(warp::any().map(move || maybe_tg_bot.clone()))
        .and(with_hb(hb.clone()))
        .and(warp::any().map(move || tg_chat_id))
        .and_then(send_message_tg);

    let matrix_route = warp::path!("matrix")
        .and(warp::post())
        .and(json_body())
        .and(warp::any().map(move || maybe_matrix_client.clone()))
        .and(with_hb(hb))
        .and(warp::any().map(move || matrix_room_id.clone()))
        .and_then(send_message_matrix);

    tg_route.or(matrix_route)
}

fn json_body() -> impl Filter<Extract = (WebHook,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn with_hb(
    hb: Arc<Handlebars>,
) -> impl Filter<Extract = (Arc<Handlebars>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || hb.clone())
}
