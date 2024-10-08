use audio_file::{AudioFileGenerator, SaveFileError, WavFileGenerator};
use audio_player::AlsaPlayer;
use audio_player::{AudioError, Player};
use music::Note;
use music::{Scale, ScaleError};
use rand::Rng;
use regex::Regex;
use reqwest;
use std::error::Error;
use std::fmt::Display;
use std::{env, fs};

mod audio_file;
mod audio_player;
mod music;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let scales = Scale::get_scales();
    match get_run_mode_from_args() {
        Ok(rm) => match rm {
            RunMode::Play(st, scale_name) => play_from_source(st, scales, scale_name).await,
            RunMode::Wav(st, scale_name) => generate_file_from_source(st, scales, scale_name).await,
            RunMode::Invalid => Err(AppError::InvalidArgs),
        },
        Err(err) => Err(err),
    }
}

#[derive(Debug)]
enum AppError {
    ShortArgs,
    InvalidArgs,
    Http,
    FileNotFound,
    Juicing,
    Audio(AudioError),
    Scale(ScaleError),
    SaveFile(SaveFileError),
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

    ; grievous [? RUN MODE] [SOURCE TYPE] [SOURCE PATH] [? SCALE NAME]

        RUN MODE: (optional) either \"play\" or \"wav\", and defaults to play
            - play: reads the input and blasts it out of a speaker.
            - wav: saves it into a file of the format <orig_file_name>_grievous.wav
        SOURCE TYPE: input file type, it can be either \"url\", \"file\"
            - url: reads the input from a url
            - file: reads the input from a file
        SOURCE PATH: a valid url or a file path
        SCALE NAME: (optional) either a scale from the list under \"./scales.json\", or without a scale if not specified.

Examples:
    ; grievous play url https://rustup.rs
    ; grievous play file ./README.md
    ; grievous play file ./README.md saba
    ; grievous wav url https://rustup.rs saba
    ; grievous wav file ./README.md saba
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
    Play(SourceType, String),
    Wav(SourceType, String),
    Invalid,
}

impl RunMode {
    fn new(rm: String, st: SourceType, scale_name: String) -> Self {
        match rm.to_lowercase().as_str() {
            "play" => Self::Play(st, scale_name),
            "wav" => Self::Wav(st, scale_name),
            _ if st.eq(&SourceType::Invalid) => Self::Invalid,
            _ => Self::Invalid,
        }
    }
}

fn get_run_mode_from_args() -> Result<RunMode, AppError> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => Ok(RunMode::Play(
            SourceType::new(args[1].clone(), args[2].clone()),
            "".to_string(),
        )),
        4 => Ok(RunMode::new(
            args[1].clone(),
            SourceType::new(args[2].clone(), args[3].clone()),
            "".to_string(),
        )),
        5 => Ok(RunMode::new(
            args[1].clone(),
            SourceType::new(args[2].clone(), args[3].clone()),
            args[4].clone(),
        )),
        _ => {
            print_usage();
            Err(AppError::ShortArgs)
        }
    }
}

fn get_random_duration() -> f32 {
    rand::thread_rng().gen_range(0.10..=0.69)
}

async fn play_from_source(
    src: SourceType,
    scales: Vec<Scale>,
    scale_name: String,
) -> Result<(), AppError> {
    let freqs = match src {
        SourceType::Url(url) => juice_url(url.as_str()).await,
        SourceType::File(path) => juice_file(path.as_str()),
        SourceType::Invalid => Err(AppError::InvalidArgs),
    };
    if freqs.is_err() {
        return Err(AppError::Juicing);
    }

    let bitrate = 44100;
    let player = AlsaPlayer::new(bitrate);
    let notes = vec![
        vec![Note::new(0.0, 1.69)],
        freqs
            .unwrap()
            .iter()
            .map(|f| Note::new(*f as f32, get_random_duration()))
            .collect(),
    ]
    .concat();

    let scale = Scale::find_scale(scales, scale_name);
    let result = match scale {
        Ok(scale) => player.play_sound_with_scale(notes, &scale),
        Err(_err) => player.play_sound(notes),
    };

    if result.is_err() {
        return Err(AppError::Audio(result.err().unwrap()));
    }

    Ok(())
}

async fn generate_file_from_source(
    src: SourceType,
    scales: Vec<Scale>,
    scale_name: String,
) -> Result<(), AppError> {
    let (src_name, freqs) = match src {
        SourceType::Url(url) => (
            url.clone().split("//").collect::<Vec<&str>>()[1].to_string(),
            juice_url(url.clone().as_str()).await,
        ),
        SourceType::File(path) => (path.clone(), juice_file(path.clone().as_str())),
        SourceType::Invalid => ("".to_string(), Err(AppError::InvalidArgs)),
    };
    if freqs.is_err() {
        return Err(AppError::Juicing);
    }

    let bitrate = 44100;
    let generator = WavFileGenerator::new(bitrate);
    let notes = vec![
        vec![Note::new(0.0, 1.69)],
        freqs
            .unwrap()
            .iter()
            .map(|f| Note::new(*f as f32, get_random_duration()))
            .collect(),
    ]
    .concat();

    let scale = Scale::find_scale(scales, scale_name);
    let result = match scale {
        Ok(scale) => generator.generate_with_scale(src_name, notes, &scale),
        Err(_err) => generator.generate(src_name, notes),
    };

    if result.is_err() {
        return Err(AppError::SaveFile(result.err().unwrap()));
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
