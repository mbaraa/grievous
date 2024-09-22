use std::f32::consts;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

use crate::audio_player::player::Note;

const PI: f64 = 3.1415926535;
const BITDEPTH: u16 = 16;
const SAMPLERATE: u32 = 44100;
const CHANNELS: u16 = 1;
const BLOCKALIGN: u16 = BITDEPTH / 2;
const BYTERATE: u32 = SAMPLERATE * BITDEPTH as u32 / 8;
const FORMAT: u16 = 1; // WAVE_FORMAT_PCM
const CHUNKSIZE: u32 = 16;
const DURATION: u8 = 2;
const FREQUENCY: f64 = 150.0;

pub fn write_wav_file(file_name: String, notes: Vec<Note>) -> std::io::Result<()> {
    // open file
    let mut output_file = File::create(file_name + ".wav")?;

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
    output_file.write_all(&SAMPLERATE.to_le_bytes())?;
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
        while i < (SAMPLERATE as f64) {
            let amplitude = i / (SAMPLERATE as f64) * max_amplitude as f64;
            let sample =
                (2.0 * (consts::PI as f64) * i * (note.freq as f64) / (SAMPLERATE as f64)).sin();
            output_file.write_all(&((amplitude * sample) as i16).to_be_bytes());

            i += 1.0;
        }
    });

    //    // generate some sine wave
    //    let amplitude: f64 = 0.5;
    //    let offset: f64 = 2.0 * PI * FREQUENCY / (SAMPLERATE as f64);
    //    let mut angle: f64 = 0.0;
    //    let samples_required: u64 = SAMPLERATE as u64 * DURATION as u64;
    //
    //    let mut sample: f64;
    //    let mut sample_to_write: i16;
    //    let max_amplitude: f64 = 2.0f64.powi((BITDEPTH - 1).into()) - 1.0;
    //
    //    for _ in 1..samples_required {
    //        sample = amplitude * angle.sin();
    //        angle += offset;
    //        sample_to_write = (sample * max_amplitude) as i16;
    //        output_file.write_all(&sample_to_write.to_le_bytes())?;
    //    }
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
