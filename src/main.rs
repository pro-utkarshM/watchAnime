mod cli;
mod config;
mod util;
mod commands;
mod external;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli::build_cli().get_matches();

    if matches.is_present("version") {
        cli::version_info();
        return Ok(());
    }

    if matches.is_present("help") {
        cli::help_info();
        return Ok(());
    }

    if matches.is_present("update") {
        commands::update::update_script().await?;
        return Ok(());
    }

    // Rest of the command handling goes here

    Ok(())
}
