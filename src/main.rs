use audio_player::alsa;
use audio_player::player::{Note, Player};
use futures::executor;
use regex::Regex;
use reqwest;
use std::env;
use std::error::Error;
use std::fmt::Display;

mod audio_player;

#[derive(Debug)]
enum AppError {
    ShortArgs,
    InvalidArgs,
    Http,
}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

fn print_usage() {
    println!(
        "Usage of Grievous:

    ; grievous [? RUN MODE] [SOURCE TYPE] [SOURCE PATH]

        RUN MODE: (optional) either \"play\" or \"wav\", and defaults to play
            - play: reads the input and blasts it out of a speaker.
            - wav: saves it into a file of the format <orig_file_name>_grievous.wav
        SOURCE TYPE: input file type, it can be either \"url\", \"file\"
            - url: reads the input from a url
            - file: reads the input from a file
        SOURCE PATH: a valid url or a file path

Examples:
    ; grievous play url https://rustup.rs
    ; grievous play file ./README.md
"
    );
}

#[derive(PartialEq)]
enum SourceType {
    Url(String),
    File(String),
    Invalid,
}

impl SourceType {
    fn new(st: String, value: String) -> Self {
        match st.to_lowercase().as_str() {
            "url" => Self::Url(value),
            "file" => Self::File(value),
            _ => Self::Invalid,
        }
    }
}

#[derive(PartialEq)]
enum RunMode {
    Play(SourceType),
    Wav(SourceType),
    Invalid,
}

impl RunMode {
    fn new(rm: String, st: SourceType) -> Self {
        match rm.to_lowercase().as_str() {
            "play" => Self::Play(st),
            "wav" => Self::Wav(st),
            _ if st.eq(&SourceType::Invalid) => Self::Invalid,
            _ => Self::Invalid,
        }
    }
}

fn get_run_mode_from_args() -> Result<RunMode, AppError> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => Ok(RunMode::Play(SourceType::new(
            args[1].clone(),
            args[2].clone(),
        ))),
        4 => Ok(RunMode::new(
            args[1].clone(),
            SourceType::new(args[2].clone(), args[3].clone()),
        )),
        _ => {
            print_usage();
            Err(AppError::ShortArgs)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    match get_run_mode_from_args() {
        Ok(rm) => match rm {
            RunMode::Play(st) => {
                match st {
                    SourceType::Url(url) => println!("{}", url),
                    SourceType::File(path) => println!("{}", path),
                    SourceType::Invalid => {}
                }
                todo!("impl play")
            }
            RunMode::Wav(st) => {
                match st {
                    SourceType::Url(url) => println!("{}", url),
                    SourceType::File(path) => println!("{}", path),
                    SourceType::Invalid => {}
                }
                todo!("impl wav")
            }
            RunMode::Invalid => Err(AppError::InvalidArgs),
        },
        Err(err) => Err(err),
    }
}

async fn juice_url(url: &str) -> Result<Vec<u16>, AppError> {
    let ml_pat = Regex::new(r"<[^>]*>").unwrap();
    let resp = reqwest::get(url).await;
    if resp.is_err() {
        return Err(AppError::Http);
    }
    let resp_body = resp.unwrap().text().await;
    if resp_body.is_err() {
        return Err(AppError::Http);
    }
    let resp_body_unwrapped = resp_body.unwrap();
    let replaced_body = ml_pat.replace_all(resp_body_unwrapped.as_str(), "");

    Ok::<Vec<u16>, AppError>(
        replaced_body
            .replace("\n", "")
            .bytes()
            .map(|f| f as u16)
            .collect(),
    )
}
