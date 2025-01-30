use std::fmt::Display;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use symphonia::{
    core::{
        audio::Signal,
        codecs::DecoderOptions,
        formats::FormatOptions,
        io::{MediaSourceStream, MediaSourceStreamOptions},
        meta::MetadataOptions,
        probe::Hint,
    },
    default::{get_codecs, get_probe},
};

use crate::{
    core::{char::ColoredChar, flat_matrix::FlatMatrix},
    Error,
};

pub struct AsciiAudio(pub FlatMatrix<char>);

impl AsciiAudio {
    pub fn new_parallel(
        path: &String,
        media_type: String,
        max_height: usize,
        uniform: bool,
        invert: bool,
    ) -> Result<Self, Error> {
        let file = std::fs::File::open(path)?;

        let media_src_stream =
            MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());

        let hint = Hint::new();
        // hint.with_extension(&media_type);

        let probed = get_probe().format(
            &hint,
            media_src_stream,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )?;
        let mut format = probed.format;

        let track = format
            .tracks()
            .iter()
            .next()
            .ok_or("No supported audio tracks found")?;
        let track_id = track.id;
        let mut decoder = get_codecs().make(&track.codec_params, &DecoderOptions::default())?;

        // Storage for samples
        let mut samples = Vec::new();

        // Decode packets and collect samples
        while let Ok(packet) = format.next_packet() {
            if packet.track_id() == track_id {
                if let Ok(decoded) = decoder.decode(&packet) {
                    match decoded {
                        symphonia::core::audio::AudioBufferRef::S16(buf) => {
                            samples.extend(buf.chan(0)); // Only take the first channel
                        }
                        symphonia::core::audio::AudioBufferRef::F32(buf) => {
                            samples.extend(
                                buf.chan(0)
                                    .iter()
                                    .map(|&sample| (sample * i16::MAX as f32) as i16),
                            ); // Convert float to i16
                        }
                        _ => {
                            eprintln!("Unsupported audio buffer format");
                        }
                    }
                }
            }
        }

        println!("xx");
        let downscaled_samples: Vec<(u16, bool)> = samples
            .into_par_iter()
            .map(|sample| {
                let sign = sample >= 0; // true if positive or 0
                let magnitude = sample.abs() as u16;
                (magnitude, sign)
            })
            .collect();
        let height = max_height;

        let midpoint = ((height * 2) - 1) / 2;

        let columns = downscaled_samples
            .into_par_iter()
            .map(|x| {
                let x = (
                    (((x.0 as f32 / 255.0) * midpoint as f32).round_ties_even() as u8),
                    x.1,
                );

                println!("{:?}", x);

                let char_used = {
                    let ch =
                        ColoredChar::from_everything(x.0, (255, 255, 255), true, invert, uniform)
                            .ch;

                    if ch == ' ' {
                        '.'
                    } else {
                        ch
                    }
                };
                let mut column: Vec<char> = vec![' '; height as usize * 2];

                if x.1 {
                    for y in 0..x.0 {
                        column[midpoint - y as usize] = char_used;
                    }
                } else {
                    for y in 0..x.0 {
                        column[midpoint + y as usize] = char_used;
                    }
                }

                column
            })
            .collect::<FlatMatrix<_>>();

        // todo!()

        let transposed = columns.transpose();

        Ok(Self(transposed))
    }

    pub fn new_sequential(
        path: &String,
        media_type: String,
        max_height: usize,
        uniform: bool,
        invert: bool,
    ) -> Result<Self, Error> {
        let file = std::fs::File::open(path)?;

        let media_src_stream =
            MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());

        let mut hint = Hint::new();
        hint.with_extension(&media_type);

        let probed = get_probe().format(
            &hint,
            media_src_stream,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )?;
        let mut format = probed.format;

        let track = format
            .tracks()
            .iter()
            .next()
            .ok_or("No supported audio tracks found")?;
        let track_id = track.id;
        let mut decoder = get_codecs().make(&track.codec_params, &DecoderOptions::default())?;

        // Storage for samples
        let mut samples = Vec::new();

        // Decode packets and collect samples
        while let Ok(packet) = format.next_packet() {
            if packet.track_id() == track_id {
                if let Ok(decoded) = decoder.decode(&packet) {
                    match decoded {
                        symphonia::core::audio::AudioBufferRef::S16(buf) => {
                            samples.extend(buf.chan(0)); // Only take the first channel
                        }
                        symphonia::core::audio::AudioBufferRef::F32(buf) => {
                            samples.extend(
                                buf.chan(0)
                                    .iter()
                                    .map(|&sample| (sample * i16::MAX as f32) as i16),
                            ); // Convert float to i16
                        }
                        _ => {
                            eprintln!("Unsupported audio buffer format");
                        }
                    }
                }
            }
        }

        let mut max = 0;
        samples.iter().for_each(|x| {
            if *x > max {
                max = *x
            }
        });

        let downscaled_samples: Vec<(u8, bool)> = samples
            .into_iter()
            .map(|sample| {
                let sign = sample >= 0; // true if positive or 0
                let magnitude =
                    ((sample.abs() as f32 / max as f32) * 255.0).round_ties_even() as u8;
                (magnitude, sign)
            })
            .collect();
        let height = max_height;

        let midpoint = ((height * 2) - 1) / 2;

        let columns = downscaled_samples
            .into_iter()
            .map(|x| {
                let char_used = {
                    let ch =
                        ColoredChar::from_everything(x.0, (255, 255, 255), true, invert, uniform)
                            .ch;

                    if ch == ' ' {
                        '.'
                    } else {
                        ch
                    }
                };

                print!("(");
                print!("{:?}, ", x);
                let x = (
                    (((x.0 as f32 / 255.0) * midpoint as f32).round_ties_even() as u8),
                    x.1,
                );

                print!("{:?}", x);
                println!(")");
                let mut column: Vec<char> = vec![' '; height as usize * 2];

                if x.1 {
                    for y in 0..x.0 {
                        column[midpoint - y as usize] = char_used;
                    }
                } else {
                    for y in 0..x.0 {
                        column[midpoint + y as usize] = char_used;
                    }
                }

                column
            })
            .collect::<FlatMatrix<_>>();

        // todo!()

        let transposed = columns.transpose();

        Ok(Self(transposed))
    }
}

impl Display for AsciiAudio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self
            .0
            .into_iter_vecs()
            .map(|x| {
                x.into_iter()
                    .map(|y| y.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", string)
    }
}

#[test]
fn test() {
    let ascii_wave = AsciiAudio::new_sequential(
        &"picts/beep-sound-short-237619.mp3".to_string(),
        "mp3".to_string(),
        255,
        false,
        false,
    )
    .unwrap();

    println!("{}", ascii_wave)
} // ......
