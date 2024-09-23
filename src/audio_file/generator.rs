use std::{error::Error, fmt::Display};

use crate::music::{Note, Scale};

#[derive(Debug)]
pub enum SaveFileError {
    FileError,
}

impl Error for SaveFileError {}

impl Display for SaveFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub trait AudioFileGenerator {
    fn generate(&self, file_name: String, notes: Vec<Note>) -> Result<(), SaveFileError>;
    fn generate_with_scale(
        &self,
        file_name: String,
        notes: Vec<Note>,
        scale: &Scale,
    ) -> Result<(), SaveFileError>;
}
