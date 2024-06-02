use std::process::Command;
use anyhow::{Result, anyhow};

pub fn play_with_vlc(url: &str) -> Result<()> {
    let status = Command::new("vlc")
        .arg(url)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("Failed to play with vlc"))
    }
}
