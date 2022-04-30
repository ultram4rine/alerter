use clap::{crate_authors, crate_description, crate_version, Arg, ArgGroup};

fn main() -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from("pkg/doc");

    let cmd = clap::Command::new("alerter")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .group(
            ArgGroup::new("req_flags")
                .required(true)
                .multiple(true)
                .args(&["tg", "matrix"]),
        )
        .arg(
            clap::Arg::new("port")
                .short('p')
                .long("port")
                .env("ALERTER_LISTEN_PORT")
                .value_name("PORT")
                .default_value("3030")
                .help("Port to listen")
                .required(false),
        )
        .arg(
            clap::Arg::new("tg")
                .long("tg")
                .help("Enable Telegram support")
                .required(true)
                .requires("tg_token")
                .requires("tg_chat_id")
                .requires("tg_template_path"),
        )
        .arg(
            clap::Arg::new("tg_token")
                .long("tg-token")
                .env("ALERTER_TG_BOT_TOKEN")
                .value_name("TG_TOKEN")
                .help("Telegram bot token")
                .required(false),
        )
        .arg(
            clap::Arg::new("tg_chat_id")
                .long("tg-chat-id")
                .env("ALERTER_TG_CHAT_ID")
                .value_name("TG_CHAT_ID")
                .help("Telegram chat ID")
                .required(false),
        )
        .arg(
            clap::Arg::new("tg_template_path")
                .long("tg-template-path")
                .env("ALERTER_TG_TMPL_PATH")
                .value_name("TG_TEMPLATE_PATH")
                .default_value("templates/default.tg.hbs")
                .help("Path to handlebars template file for Telegram")
                .required(false),
        )
        .arg(
            Arg::new("matrix")
                .long("matrix")
                .help("Enable Matrix support")
                .required(true)
                .requires("matrix_user")
                .requires("matrix_pass")
                .requires("matrix_room_id")
                .requires("matrix_template_path"),
        )
        .arg(
            Arg::new("matrix_user")
                .long("matrix-user")
                .env("ALERTER_MATRIX_USERNAME")
                .value_name("MATRIX_USER")
                .help("Matrix username")
                .required(false),
        )
        .arg(
            Arg::new("matrix_pass")
                .long("matrix-pass")
                .env("ALERTER_MATRIX_PASSWORD")
                .value_name("MATRIX_PASS")
                .help("Matrix password")
                .required(false),
        )
        .arg(
            Arg::new("matrix_room_id")
                .long("matrix-room-id")
                .env("ALERTER_MATRIX_ROOM_ID")
                .value_name("MATRIX_ROOM_ID")
                .help("Matrix room id")
                .required(false),
        )
        .arg(
            Arg::new("matrix_template_path")
                .long("matrix-template-path")
                .env("ALERTER_MATRIX_TMPL_PATH")
                .value_name("MATRIX_TEMPLATE_PATH")
                .default_value("templates/default.matrix.hbs")
                .help("Path to handlebars template file for Matrix")
                .required(false),
        );

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join("alerter.1"), buffer)?;

    Ok(())
}
