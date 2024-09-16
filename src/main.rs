use audio_player::{alsa, player::Player};
use regex::Regex;
use reqwest;
use std::error::Error;

mod audio_player;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ml_pat = Regex::new(r"<[^>]*>").unwrap();

    let client = reqwest::Client::new();

    // let resp_body = reqwest::get("https://mbaraa.com").await?.text().await?;

    // let replaced_body = ml_pat.replace_all(resp_body.as_str(), "");
    // print!("{:?}", replaced_body);

    let player = alsa::AlsaPlayer::new(44100, 160.0, 0.09);
    let res = player.play_sound(vec![440, 1000, 440, 1000, 440, 1000]);
    Ok(())
}
