use std::{collections::HashMap, fs};

use serde::Deserialize;

use super::scales;

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
    pub fn new(name: String, charset: HashMap<char, f32>) -> Self {
        Self { name, charset }
    }

    pub fn get_scales() -> Vec<Scale> {
        scales::get_scales()
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
