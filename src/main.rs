mod cli;
mod commands;

use crate::cli::build_cli;
use crate::commands::{update, search, play, download, history};

#[tokio::main]
async fn main() {
    let matches = build_cli().get_matches();

    if matches.get_flag("version") {
        println!("watchAnime version 1.0");
        return;
    }

    if matches.get_flag("help") {
        build_cli().print_help().unwrap();
        return;
    }

    if matches.get_flag("update") {
        update::update_script().await;
        return;
    }

    if let Some(query) = matches.get_one::<String>("query") {
        search::search_anime(query).await;
    }

    if matches.get_flag("continue") {
        history::continue_watching().await;
    }

    // Other command handling here
}
