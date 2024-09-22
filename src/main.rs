use audio_player::alsa::{self, AlsaPlayer};
use audio_player::player::{AudioError, Note, Player};
use regex::Regex;
use reqwest;
use std::error::Error;
use std::fmt::Display;
use std::{env, fs};

mod audio_player;

#[derive(Debug)]
enum AppError {
    ShortArgs,
    InvalidArgs,
    Http,
    FileNotFound,
    Juicing,
    Audio(AudioError),
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
            RunMode::Play(st) => play_from_source(st).await,
            RunMode::Wav(st) => play_from_source(st).await,
            RunMode::Invalid => Err(AppError::InvalidArgs),
        },
        Err(err) => Err(err),
    }
}

async fn play_from_source(src: SourceType) -> Result<(), AppError> {
    let freqs = match src {
        SourceType::Url(url) => juice_url(url.as_str()).await,
        SourceType::File(path) => juice_file(path.as_str()),
        SourceType::Invalid => Err(AppError::InvalidArgs),
    };
    if freqs.is_err() {
        return Err(AppError::Juicing);
    }

    let bitrate = 44100;
    let note_duration = 0.9f32;
    let player = AlsaPlayer::new(bitrate);

    let result = player.play_sound(
        freqs
            .unwrap()
            .iter()
            .map(|f| Note::new(*f as f32, note_duration))
            .collect(),
    );
    if result.is_err() {
        return Err(AppError::Audio(result.err().unwrap()));
    }

    Ok(())
}

fn juice_file(path: &str) -> Result<Vec<u32>, AppError> {
    let file_content = fs::read_to_string(path);
    if file_content.is_err() {
        return Err(AppError::FileNotFound);
    }
    Ok(file_content
        .unwrap()
        .replace("\n", "")
        .bytes()
        .map(|f| f as u32)
        .collect())
}

async fn juice_url(url: &str) -> Result<Vec<u32>, AppError> {
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

    Ok::<Vec<u32>, AppError>(
        replaced_body
            .replace("\n", "")
            .bytes()
            .map(|f| f as u32)
            .collect(),
    )
}
