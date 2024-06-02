use std::process::Command;
use anyhow::{Result, anyhow};

pub fn play_with_mpv(url: &str) -> Result<()> {
    let status = Command::new("mpv")
        .arg(url)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("Failed to play with mpv"))
    }
}
