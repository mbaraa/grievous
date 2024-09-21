use super::player::{AudioError, Note, Player};

#[link(name = "alsa", kind = "static")]
extern "C" {
    fn init_alsa() -> i32;
    fn destroy_alsa() -> i32;
    fn play_frequency(freq: f32, rate: u32, duration: f32) -> i32;
}

pub struct AlsaPlayer {
    rate: u32,
}

impl AlsaPlayer {
    pub fn new(rate: u32) -> Self {
        unsafe {
            if init_alsa() != 0 {
                panic!("{}", AudioError::Hardware.to_string());
            }
        }
        Self { rate }
    }
}

impl Player for AlsaPlayer {
    fn play_sound(&self, freqs: Vec<Note>) -> Result<(), AudioError> {
        unsafe {
            freqs
                .iter()
                .map(
                    |note| match play_frequency(note.freq, self.rate, note.duration) {
                        0 => Ok(()),
                        _ => Err(AudioError::Hardware),
                    },
                )
                .collect()
        }
    }
}

impl Drop for AlsaPlayer {
    fn drop(&mut self) {
        println!("alsa destructed!");
        unsafe {
            if destroy_alsa() != 0 {
                panic!("{}", AudioError::Hardware.to_string());
            }
        }
    }
}