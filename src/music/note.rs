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
