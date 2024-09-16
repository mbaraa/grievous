pub enum AudioError {
    Hardware,
}

impl ToString for AudioError {
    fn to_string(&self) -> String {
        match self {
            AudioError::Hardware => "Hardware error".to_string(),
        }
    }
}

pub trait Player {
    fn play_sound(&self, freqs: Vec<u16>) -> Result<(), AudioError>;
}
