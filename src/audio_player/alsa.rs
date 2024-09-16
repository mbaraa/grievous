use super::player::{AudioError, Player};

#[link(name = "alsa", kind = "static")]
extern "C" {
    fn init() -> i32;
    fn destroy() -> i32;
    fn play_frequency_with_custom_params(freq: u16, rate: u16, latency: f32, duration: f32) -> i32;
}

pub struct AlsaPlayer {
    rate: u16,
    latency: f32,
    duration: f32,
}

impl AlsaPlayer {
    pub fn default() -> Self {
        unsafe {
            if init() != 0 {
                panic!("{}", AudioError::Hardware.to_string());
            }
        }
        Self {
            rate: 44100,
            latency: 0.2,
            duration: 0.5,
        }
    }

    pub fn new(rate: u16, latency: f32, duration: f32) -> Self {
        unsafe {
            if init() != 0 {
                panic!("{}", AudioError::Hardware.to_string());
            }
        }
        Self {
            rate,
            latency,
            duration,
        }
    }
}

impl Player for AlsaPlayer {
    fn play_sound(&self, freqs: Vec<u16>) -> Result<(), AudioError> {
        unsafe {
            freqs
                .iter()
                // .map(|freq| (*freq + 400))
                .map(|freq| {
                    match play_frequency_with_custom_params(
                        *freq,
                        self.rate,
                        self.latency,
                        self.duration,
                    ) {
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
            if destroy() != 0 {
                panic!("{}", AudioError::Hardware.to_string());
            }
        }
    }
}
