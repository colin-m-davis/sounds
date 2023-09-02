use std::fs::File;
use std::io::Write;
use byteorder::{WriteBytesExt, LittleEndian};

#[repr(C)]
struct WAVHeader {
    riff: [u8; 4],
    chunk_size: u32,
    wave: [u8; 4],
    fmt: [u8; 4],
    sub_chunk1_size: u32,
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
    data: [u8; 4],
    sub_chunk2_size: u32,
}

impl WAVHeader {
    fn new(data_size: u32, sample_rate: u32, bits_per_sample: u16, num_channels: u16) -> Self {
        let byte_rate = sample_rate * (bits_per_sample / 8) as u32 * num_channels as u32;
        let block_align = (bits_per_sample / 8) * num_channels;
        
        Self {
            riff: [b'R', b'I', b'F', b'F'],
            chunk_size: data_size + 36,
            wave: [b'W', b'A', b'V', b'E'],
            fmt: [b'f', b'm', b't', b' '],
            sub_chunk1_size: 16,
            audio_format: 1,
            num_channels,
            sample_rate,
            byte_rate,
            block_align,
            bits_per_sample,
            data: [b'd', b'a', b't', b'a'],
            sub_chunk2_size: data_size,
        }
    }
}

fn write_header(file: &mut File, header: &WAVHeader) -> std::io::Result<()> {
    file.write_all(&header.riff)?;
    file.write_u32::<LittleEndian>(header.chunk_size)?;
    file.write_all(&header.wave)?;
    file.write_all(&header.fmt)?;
    file.write_u32::<LittleEndian>(header.sub_chunk1_size)?;
    file.write_u16::<LittleEndian>(header.audio_format)?;
    file.write_u16::<LittleEndian>(header.num_channels)?;
    file.write_u32::<LittleEndian>(header.sample_rate)?;
    file.write_u32::<LittleEndian>(header.byte_rate)?;
    file.write_u16::<LittleEndian>(header.block_align)?;
    file.write_u16::<LittleEndian>(header.bits_per_sample)?;
    file.write_all(&header.data)?;
    file.write_u32::<LittleEndian>(header.sub_chunk2_size)?;
    Ok(())
}

fn write_body(file: &mut File, num_samples: u32, sample_rate: u32, frequencies: &[f32]) -> std::io::Result<()> {
    let amplitude = i16::MAX as f32;
    for n in 0..num_samples {
        let t = n as f32 / sample_rate as f32;
        let mut sample_f32: f32 = 0.0;

        for &freq in frequencies {
            sample_f32 += (t * freq * 2.0 * std::f32::consts::PI).sin();
        }

        sample_f32 /= frequencies.len() as f32;
        let sample_i16 = (sample_f32 * amplitude) as i16;

        file.write_all(&sample_i16.to_le_bytes())?;
    }
    Ok(())
}

fn write_wav(filename: &str, sample_rate: u32, duration: f32, frequencies: &[f32]) -> std::io::Result<()> {
    let num_channels = 1;
    let bits_per_sample = 16;
    let num_samples = (sample_rate as f32 * duration) as u32;
    let data_size = num_samples * (bits_per_sample / 8) as u32 * num_channels as u32;
    let header = WAVHeader::new(data_size, sample_rate, bits_per_sample, num_channels);

    let mut file = File::create(filename)?;
    write_header(&mut file, &header)?;
    write_body(&mut file, num_samples, sample_rate, frequencies)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let sample_rate = 44100;
    let duration = 2.0; // seconds
    let frequencies = [440.0, 554.37, 659.26]; // A4, C#5, E5 for A major chord

    write_wav("chord.wav", sample_rate, duration, &frequencies)?;
    println!("WAV file written successfully!");

    Ok(())
}