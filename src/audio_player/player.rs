use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum AudioError {
    Hardware,
}

impl Error for AudioError {}

impl Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Clone)]
pub struct Note {
    pub freq: f32,
    pub duration: f32, // seconds
}

impl Note {
    pub fn new(freq: f32, duration: f32) -> Self {
        Self { freq, duration }
    }
}

impl Copy for Note {}

pub trait Player {
    fn play_sound(&self, freqs: Vec<Note>) -> Result<(), AudioError>;
}
