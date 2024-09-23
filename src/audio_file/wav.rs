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
        const BITDEPTH: u16 = 16;
        const CHANNELS: u16 = 1;
        const BLOCKALIGN: u16 = BITDEPTH / 2;
        let BYTERATE: u32 = self.rate * BITDEPTH as u32 / 8;
        const FORMAT: u16 = 1; // WAVE_FORMAT_PCM
        const CHUNKSIZE: u32 = 16;

        // open file
        let mut output_file = File::create(file_name + "_grievous.wav")?;

        // Header
        // - RIFF
        output_file.write_all(&[0x52, 0x49, 0x46, 0x46])?;
        // - ---- place holder
        let pos_cksize = output_file.stream_position()?;
        output_file.write_all("----".as_bytes())?;
        output_file.write_all("WAVE".as_bytes())?;

        //  Format
        output_file.write_all("fmt ".as_bytes())?;
        output_file.write_all(&CHUNKSIZE.to_le_bytes())?;
        output_file.write_all(&FORMAT.to_le_bytes())?;
        output_file.write_all(&CHANNELS.to_le_bytes())?;
        output_file.write_all(&self.rate.to_le_bytes())?;
        output_file.write_all(&BYTERATE.to_le_bytes())?;
        output_file.write_all(&BLOCKALIGN.to_le_bytes())?;
        output_file.write_all(&BITDEPTH.to_le_bytes())?;

        // Data
        output_file.write_all("data".as_bytes())?;
        let pos_data_placeholder = output_file.stream_position()?;
        output_file.write_all("----".as_bytes())?;
        let pos_data_start = output_file.stream_position()?;

        notes.iter().for_each(|note| {
            let max_amplitude = (1 << (BITDEPTH - 1)) - 1;

            let mut i = 0.5;
            while i < (self.rate as f64) {
                let amplitude = i / (self.rate as f64) * max_amplitude as f64;
                let sample =
                    (2.0 * (consts::PI as f64) * i * (note.freq as f64) / (self.rate as f64)).sin();
                output_file.write_all(&((amplitude * sample) as i16).to_be_bytes());

                i += 1.0;
            }
        });

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
