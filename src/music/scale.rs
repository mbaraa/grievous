use std::{collections::HashMap, fs};

use serde::Deserialize;

#[derive(Debug)]
pub enum ScaleError {
    ScalesFileNotFound,
    NoScalesAvailable,
    ScaleNotFound,
}

#[derive(Deserialize, Debug)]
pub struct Scale {
    pub name: String,
    pub charset: HashMap<char, f32>,
}

impl Scale {
    pub fn load_from_file() -> Result<Vec<Scale>, ScaleError> {
        let scales_raw = fs::read_to_string("./scales.json");
        if scales_raw.is_err() {
            return Err(ScaleError::ScalesFileNotFound);
        }
        let scales = serde_json::from_str::<Vec<Scale>>(&scales_raw.unwrap());
        if scales.is_err() {
            return Err(ScaleError::NoScalesAvailable);
        }
        let scales_unwrapped = scales.unwrap();
        if scales_unwrapped.len() == 0 {
            return Err(ScaleError::NoScalesAvailable);
        }
        Ok(scales_unwrapped)
    }

    pub fn find_scale(scales: Vec<Scale>, name: String) -> Result<Scale, ScaleError> {
        let scale = scales.into_iter().find(|s| s.name == name);
        if scale.is_none() {
            return Err(ScaleError::ScaleNotFound);
        }
        Ok(scale.unwrap())
    }

    pub fn map_frequency(&self, freq: f32) -> f32 {
        let mapped_freq = self.charset.get(&(freq as u32 as u8 as char));
        if !mapped_freq.is_none() {
            return *mapped_freq.unwrap();
        }
        let mut new_freq = ((freq as i32 - ('a' as i32) % 26).abs() as u32 + ('a' as u32)) as f32;
        if [0f32, 123f32].contains(&new_freq) {
            new_freq = *self.charset.get(&'a').unwrap();
        }

        new_freq
    }
}
