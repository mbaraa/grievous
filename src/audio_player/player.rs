use crate::music::{Note, Scale};

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

pub trait Player {
    fn play_sound(&self, notes: Vec<Note>) -> Result<(), AudioError>;
    fn play_sound_with_scale(&self, notes: Vec<Note>, scale: &Scale) -> Result<(), AudioError>;
}
