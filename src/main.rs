use audio_player::alsa;
use audio_player::player::{Note, Player};
use regex::Regex;
use reqwest;
use std::env;
use std::error::Error;
use std::fmt::Display;

mod audio_player;

#[derive(Debug)]
enum AppError {
    ShortArgs,
}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<impl Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(Box::new(AppError::ShortArgs));
        // return Err("Not enough args".to_string());
    }

    let freqs = juice_url("file:///home/b/foo.c").await?;
    freqs.iter().for_each(|f| {
        println!("{}", *f);
    });

    Ok(())
}

async fn juice_url(url: &str) -> Result<Vec<u16>, Box<impl Error>> {
    let ml_pat = Regex::new(r"<[^>]*>").unwrap();
    let resp_body = reqwest::get(url).await?.text().await?;
    let replaced_body = ml_pat.replace_all(resp_body.as_str(), "");

    Ok(replaced_body
        .replace("\n", "")
        .bytes()
        .map(|f| f as u16)
        .collect())
}
