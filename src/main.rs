mod app;
mod args;
mod duration;
mod receivers;
mod webhook;

extern crate dotenv;
extern crate pretty_env_logger;

use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use matrix_sdk::{ruma::UserId, Client};
use teloxide::{types::ChatId, Bot};

use crate::{
    app::{create_filter, create_hb},
    args::Args,
};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    dotenv().ok();
    let args = Args::parse();

    let tg_token: String;
    let mut tg_chat_id: ChatId = ChatId(0);
    let mut tg_bot: Option<Bot> = None;

    if args.tg {
        tg_token = args.tg_token.unwrap();
        tg_chat_id = ChatId(args.tg_chat_id.unwrap());

        tg_bot = Some(Bot::new(tg_token));
    }

    let matrix_user: String;
    let matrix_pass: String;
    let mut matrix_room_id: String = "".to_owned();
    let mut matrix_client: Option<Client> = None;

    if args.matrix {
        matrix_user = args.matrix_user.unwrap();
        matrix_pass = args.matrix_pass.unwrap();
        matrix_room_id = args.matrix_room_id.unwrap();

        let matrix_user_id = <&UserId>::try_from(matrix_user.as_str())?;

        matrix_client = Some(
            Client::builder()
                .user_id(matrix_user_id.clone())
                .build()
                .await?,
        );

        matrix_client
            .clone()
            .unwrap()
            .login(
                matrix_user_id.localpart(),
                matrix_pass.as_str(),
                Some("Alerter bot"),
                None,
            )
            .await?;
    }

    let hb = create_hb(args.tg_template_path, args.matrix_template_path)?;
    let filter = create_filter(tg_bot, matrix_client, hb, tg_chat_id, matrix_room_id);

    let server = warp::serve(filter);
    server.run(([0, 0, 0, 0], args.port)).await;

    Ok(())
}
