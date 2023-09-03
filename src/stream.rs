use std::sync::mpsc;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SizedSample,
};
use cpal::{Sample, FromSample};
use crate::oscillator::{Oscillator, Waveform};


fn main() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel();
    let stream = stream_setup_for(rx)?;
    let _ = stream.play();
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let tokens: Vec<String> = input.trim().split(' ').map(String::from).collect();
        println!("Got input = {}", tokens.len());
        tx.send(tokens).unwrap();
    }
}

pub fn stream_setup_for(rx: mpsc::Receiver<Vec<String>>) -> Result<cpal::Stream, anyhow::Error> {
    let (_host, device, config) = host_device_setup()?;

    match config.sample_format() {
        cpal::SampleFormat::I8  => make_stream::<i8> (&device, &config.into(), rx),
        cpal::SampleFormat::I16 => make_stream::<i16>(&device, &config.into(), rx),
        cpal::SampleFormat::I32 => make_stream::<i32>(&device, &config.into(), rx),
        cpal::SampleFormat::I64 => make_stream::<i64>(&device, &config.into(), rx),
        cpal::SampleFormat::U8  => make_stream::<u8> (&device, &config.into(), rx),
        cpal::SampleFormat::U16 => make_stream::<u16>(&device, &config.into(), rx),
        cpal::SampleFormat::U32 => make_stream::<u32>(&device, &config.into(), rx),
        cpal::SampleFormat::U64 => make_stream::<u64>(&device, &config.into(), rx),
        cpal::SampleFormat::F32 => make_stream::<f32>(&device, &config.into(), rx),
        cpal::SampleFormat::F64 => make_stream::<f64>(&device, &config.into(), rx),
        sample_format => Err(anyhow::Error::msg(format!(
            "Unsupported sample format '{sample_format}'"
        ))),
    }
}

pub fn host_device_setup() -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::Error::msg("Default output device is not available"))?;
    println!("Output device : {}", device.name()?);

    let config = device.default_output_config()?;
    println!("Default output config : {:?}", config);

    Ok((host, device, config))
}

pub fn make_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    rx: mpsc::Receiver<Vec<String>>
) -> Result<cpal::Stream, anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
{
    let num_channels = config.channels as usize;
    let mut oscillator = Oscillator {
        waveform: Waveform::Sine,
        sample_rate: config.sample_rate.0 as f32,
        current_sample_index: 0.0,
        frequency_hz: 440.0,
    };
    let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
            match rx.try_recv() {
                Ok(message) => {
                    oscillator.set_waveform( match message.first().unwrap().as_str() {
                        "sine" => Waveform::Sine,
                        "sawtooth" => Waveform::Saw,
                        _ => Waveform::Square,
                    })
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    println!("Channel disconnected");
                    return;
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
            process_frame(output, &mut oscillator, num_channels)
        },
        err_fn,
        None
    )?;

    Ok(stream)
}

fn process_frame<SampleType>(
    output: &mut [SampleType],
    oscillator: &mut Oscillator,
    num_channels: usize,
) where
    SampleType: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(num_channels) {
        let value: SampleType = SampleType::from_sample(oscillator.tick());

        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

#[cfg(test)]
mod test{
    #[test]
    pub fn test() {
        super::main();
    }
}
