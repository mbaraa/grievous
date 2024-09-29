use std::f32::consts;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

use crate::music::Note;

use super::AudioFileGenerator;

pub struct WavFileGenerator {
    rate: u32,
}

impl WavFileGenerator {
    pub fn new(rate: u32) -> Self {
        Self { rate }
    }
}

impl AudioFileGenerator for WavFileGenerator {
    fn generate(&self, file_name: String, notes: Vec<Note>) -> Result<(), super::SaveFileError> {
        match self.write_wave_file(file_name, notes) {
            Ok(_) => Ok(()),
            Err(_err) => Err(super::SaveFileError::FileError),
        }
    }

    fn generate_with_scale(
        &self,
        file_name: String,
        notes: Vec<Note>,
        scale: &crate::music::Scale,
    ) -> Result<(), super::SaveFileError> {
        match self.write_wave_file(
            file_name,
            notes
                .iter()
                .map(|note| Note::new(scale.map_frequency(note.freq), note.duration))
                .collect(),
        ) {
            Ok(_) => Ok(()),
            Err(_err) => Err(super::SaveFileError::FileError),
        }
    }
}

impl WavFileGenerator {
    fn write_wave_file(&self, file_name: String, notes: Vec<Note>) -> std::io::Result<()> {
        const BIT_DEPTH: i32 = 32;
        const CHUNK_ID: &str = "RIFF";
        const CHUNK_SIZE_PLACEHOLDER: &str = "----";
        const FORMAT: &str = "WAVE";
        const SUB_CHUNK1_ID: &str = "fmt ";
        const SUB_CHUNK1_SIZE: u32 = 16;
        const AUDIO_FORMAT: u16 = 1;
        const CHANNELS: u16 = 2;
        let byte_rate: u32 = self.rate * (CHANNELS as u32) * (SUB_CHUNK1_SIZE / 8);
        let block_align: u16 = CHANNELS * (SUB_CHUNK1_SIZE / 8) as u16;
        let bits_per_sample: u16 = BIT_DEPTH as u16;
        const SUB_CHUNK2_ID: &str = "data";
        const SUB_CHUNK2_SIZE_PLACEHOLDER: &str = "----";

        // open file
        let mut output_file = File::create(file_name + "_grievous.wav")?;

        // Header
        output_file.write_all(CHUNK_ID.as_bytes())?;
        let pos_cksize = output_file.stream_position()?;
        output_file.write_all(CHUNK_SIZE_PLACEHOLDER.as_bytes())?;
        output_file.write_all(FORMAT.as_bytes())?;

        //  Format
        output_file.write_all(SUB_CHUNK1_ID.as_bytes())?;
        output_file.write_all(&SUB_CHUNK1_SIZE.to_le_bytes())?;
        output_file.write_all(&AUDIO_FORMAT.to_le_bytes())?;
        output_file.write_all(&CHANNELS.to_le_bytes())?;
        output_file.write_all(&self.rate.to_le_bytes())?;
        output_file.write_all(&byte_rate.to_le_bytes())?;
        output_file.write_all(&block_align.to_le_bytes())?;
        output_file.write_all(&bits_per_sample.to_le_bytes())?;

        // Data
        output_file.write_all(SUB_CHUNK2_ID.as_bytes())?;
        let pos_data_placeholder = output_file.stream_position()?;
        output_file.write_all(SUB_CHUNK2_SIZE_PLACEHOLDER.as_bytes())?;
        let pos_data_start = output_file.stream_position()?;

        notes
            .iter()
            .map(|note| -> std::io::Result<()> {
                let max_amplitude: i64 = (1 << (BIT_DEPTH - 1)) - 1;

                let mut i = 0.5;
                while i < (self.rate as f64) * (note.duration as f64) {
                    let amplitude = i / (self.rate as f64) * max_amplitude as f64;
                    let sample = (2.0 * (consts::PI as f64) * i * (note.freq as f64)
                        / (self.rate as f64))
                        .sin();

                    let channel1 = amplitude * sample;
                    let channel2 = amplitude * sample;

                    output_file.write_all(&(channel1 as f32).to_le_bytes())?;
                    output_file.write_all(&(channel2 as f32).to_le_bytes())?;

                    i += 1.0;
                }

                Ok(())
            })
            .collect::<std::io::Result<()>>()?;

        let mut pos_end = output_file.stream_position()?;

        let chunk_size_data: u32 = (pos_end - pos_data_start) as u32;
        if chunk_size_data % 2 != 0 {
            output_file.write_all(&[0x00])?;
            pos_end = output_file.stream_position()?;
        }
        output_file.seek(SeekFrom::Start(pos_data_placeholder))?;

        output_file.write_all(&chunk_size_data.to_le_bytes())?;

        output_file.seek(SeekFrom::Start(pos_cksize))?;
        let chunk_size_header: u32 = (pos_end - 8) as u32;
        output_file.write_all(&chunk_size_header.to_le_bytes())?;

        output_file.sync_all()?;

        Ok(())
    }
}
