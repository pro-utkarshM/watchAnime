use reqwest::Client;
use anyhow::Result;
use std::fs::File;
use std::io::Write;

pub async fn update_script() -> Result<()> {
    let client = Client::new();
    let response = client.get("https://raw.githubusercontent.com/pystardust/ani-cli/master/ani-cli")
        .header("User-Agent", "ani-cli")
        .send()
        .await?
        .text()
        .await?;

    let mut file = File::create("ani-cli.sh")?;
    file.write_all(response.as_bytes())?;
    println!("Script updated successfully.");
    Ok(())
}
