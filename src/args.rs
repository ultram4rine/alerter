use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("req_flags")
        .required(true)
        .multiple(true)
        .args(&["tg", "matrix"]),
))]
pub struct Args {
    #[clap(
        short,
        long,
        env = "ALERTER_LISTEN_PORT",
        default_value_t = 3030,
        help = "Port to listen."
    )]
    pub port: u16,

    #[clap(
        long,
        requires = "tg-token",
        requires = "tg-chat-id",
        requires = "tg-template-path",
        help = "Enable Telegram support"
    )]
    pub tg: bool,

    #[clap(long, env = "ALERTER_TG_BOT_TOKEN", help = "Telegram bot token.")]
    pub tg_token: Option<String>,

    #[clap(long, env = "ALERTER_TG_CHAT_ID", help = "Telegram chat ID.")]
    pub tg_chat_id: Option<i64>,

    #[clap(
        long,
        env = "ALERTER_TG_TMPL_PATH",
        default_value = "templates/default.tg.hbs",
        help = "Path to handlebars template file for Telegram."
    )]
    pub tg_template_path: String,

    #[clap(
        long,
        requires = "matrix-user",
        requires = "matrix-pass",
        requires = "matrix-room-id",
        requires = "matrix-template-path",
        help = "Enable Matrix support"
    )]
    pub matrix: bool,

    #[clap(long, env = "ALERTER_MATRIX_USERNAME", help = "Matrix username")]
    pub matrix_user: Option<String>,

    #[clap(long, env = "ALERTER_MATRIX_PASSWORD", help = "Matrix password")]
    pub matrix_pass: Option<String>,

    #[clap(long, env = "ALERTER_MATRIX_ROOM_ID", help = "Matrix room id")]
    pub matrix_room_id: Option<String>,

    #[clap(
        long,
        env = "ALERTER_MATRIX_TMPL_PATH",
        default_value = "templates/default.matrix.hbs",
        help = "Path to handlebars template file for Matrix."
    )]
    pub matrix_template_path: String,
}
