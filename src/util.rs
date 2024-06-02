use std::process::Command as StdCommand;
use anyhow::{Result, anyhow};

pub fn die(message: &str) -> ! {
    eprintln!("\x1b[1;31m{}\x1b[0m", message);
    std::process::exit(1);
}

pub fn external_menu(command: &str, prompt: &str) -> Result<String> {
    let output = StdCommand::new("rofi")
        .arg(command)
        .arg("-sort")
        .arg("-dmenu")
        .arg("-i")
        .arg("-width")
        .arg("1500")
        .arg("-p")
        .arg(prompt)
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(anyhow!("Failed to run external menu"))
    }
}

pub fn launcher(use_external_menu: bool, command: &str, prompt: &str) -> Result<String> {
    if !use_external_menu {
        let output = StdCommand::new("fzf")
            .arg(command)
            .arg("--reverse")
            .arg("--cycle")
            .arg("--prompt")
            .arg(prompt)
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?)
        } else {
            Err(anyhow!("Failed to run launcher"))
        }
    } else {
        external_menu(command, prompt)
    }
}
