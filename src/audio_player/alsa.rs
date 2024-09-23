use crate::music::{Note, Scale};

use super::player::{AudioError, Player};

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
    fn play_sound(&self, notes: Vec<Note>) -> Result<(), AudioError> {
        unsafe {
            notes
                .iter()
                .map(|note| {
                    println!("{}", note.freq);
                    match play_frequency(note.freq, self.rate, note.duration) {
                        0 => Ok(()),
                        _ => Err(AudioError::Hardware),
                    }
                })
                .collect()
        }
    }

    fn play_sound_with_scale(&self, notes: Vec<Note>, scale: &Scale) -> Result<(), AudioError> {
        unsafe {
            notes
                .iter()
                .map(|note| {
                    let freq = scale.map_frequency(note.freq);
                    println!("{}", freq);
                    match play_frequency(freq, self.rate, note.duration) {
                        0 => Ok(()),
                        _ => Err(AudioError::Hardware),
                    }
                })
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
