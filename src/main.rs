use audio_player::{alsa, player::Player};
use regex::Regex;
use reqwest;
use std::env;
use std::error::Error;

mod audio_player;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        // return Err("Not enough args".to_string());
    }
    println!("{}", args[1]);

    let player = alsa::AlsaPlayer::new(44100, 0.1, 0.20);
    let freqs = juice_url(args[1].as_str()).await?;
    player.play_sound(freqs)?;
    Ok(())
}

async fn juice_url(url: &str) -> Result<Vec<u16>, Box<dyn Error>> {
    let ml_pat = Regex::new(r"<[^>]*>").unwrap();
    let resp_body = reqwest::get(url).await?.text().await?;
    let replaced_body = ml_pat.replace_all(resp_body.as_str(), "");

    Ok(replaced_body
        .replace("\n", "")
        .bytes()
        .map(|f| f as u16)
        .collect())
}
