extern crate colorful;

use colorful::{Color, Colorful};

#[tokio::main]
pub async fn check(proxy: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all(proxy)?)
        .build()?;
    let res = client.get("https://httpbin.org/ip").send().await?;
    if res.status().is_success() {
        println!("GOOD - {}", proxy.gradient(Color::Green));
    } else {
        println!("BAD - {}", proxy.gradient(Color::Red));
    }
    Ok(())
}