use audio_player::alsa;
use audio_player::player::{Note, Player};
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

    let player = alsa::AlsaPlayer::new(44100);
    let freqs = juice_url(args[1].as_str()).await?;
    player.play_sound(
        vec![
            Note::new(392.00, 0.8),
            Note::new(392.00, 0.8),
            Note::new(440.00, 0.8),
            Note::new(392.00, 0.8),
            Note::new(329.63, 0.8),
            Note::new(349.23, 0.8),
            Note::new(392.00, 0.8),
            Note::new(440.00, 0.8),
            Note::new(493.88, 0.8),
            Note::new(523.25, 0.8),
            Note::new(587.33, 0.8),
            Note::new(523.25, 0.8),
            Note::new(493.88, 0.8),
            Note::new(440.00, 0.8),
            Note::new(392.00, 0.8),
            Note::new(440.00, 0.8),
            Note::new(392.00, 0.8),
            Note::new(349.23, 0.8),
            Note::new(329.63, 0.8),
            Note::new(293.66, 0.8),
        ]
        .iter()
        .map(|n| *n)
        .collect(),
    )?;
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
