use std::process::Command;
use anyhow::{Result, anyhow};

pub fn run_rofi(prompt: &str) -> Result<String> {
    let output = Command::new("rofi")
        .arg("-dmenu")
        .arg("-p")
        .arg(prompt)
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(anyhow!("Failed to run rofi"))
    }
}
