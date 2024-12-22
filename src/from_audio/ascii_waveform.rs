use std::error::Error;

use colored::CustomColor;
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
    ascii_img::GenAscii,
    core::chars::DensityChar,
};

pub struct AsciiWaveform {
    pub pixels: Vec<Vec<char>>,
}
impl AsciiWaveform {
    pub fn new_parallel(
        path: String,
        media_type: String,
        max_height: u32,
        uniform: bool,
        invert: bool,
    ) -> Result<AsciiWaveform, Box<dyn Error>> {
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

        let downscaled_samples: Vec<(u16, bool)> = samples
        .into_par_iter()
        .map(|sample| {
            let sign = sample >= 0; // true if positive or 0
            let magnitude = sample.abs() as u16;
            (magnitude, sign)
        })
        .collect();
    let height = max_height;

    let midpoint = ((height*2) - 1) / 2;

    let columns = downscaled_samples
        .into_par_iter()
        .map(|x| {
            let x = (
                (x.0 as f32 * (255.0 / height as f32)).round_ties_even() as u8,
                x.1,
            );

            let char_used = {
                let ch = DensityChar::get_char_from_u8(
                    x.0,
                    invert,
                    CustomColor::new(255, 255, 255),
                    uniform,
                )
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
                    column[(midpoint - y as u32) as usize] = char_used;
                }
            } else {
                for y in 0..x.0 {
                    column[(midpoint + y as u32) as usize] = char_used;
                }
            }

            column
        })
        .collect::<Vec<_>>();

    let transposed = transpose(columns);

    Ok(AsciiWaveform { pixels: transposed })
    }

    fn new_no_parallel(
        path: String,
        media_type: String,
        max_height: u32,
        uniform: bool,
        invert: bool,
    ) -> Result<AsciiWaveform, Box<dyn Error>> {
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

        let downscaled_samples: Vec<(u16, bool)> = samples
            .iter()
            .map(|&sample| {
                let sign = sample >= 0; // true if positive or 0
                let magnitude = sample.abs() as u16;
                (magnitude, sign)
            })
            .collect();
        let height = max_height;

        let midpoint = ((height*2) - 1) / 2;

        let columns = downscaled_samples
            .into_iter()
            .map(|x| {
                let x = (
                    (x.0 as f32 * (255.0 / height as f32)).round_ties_even() as u8,
                    x.1,
                );

                let char_used = {
                    let ch = DensityChar::get_char_from_u8(
                        x.0,
                        invert,
                        CustomColor::new(255, 255, 255),
                        uniform,
                    )
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
                        column[(midpoint - y as u32) as usize] = char_used;
                    }
                } else {
                    for y in 0..x.0 {
                        column[(midpoint + y as u32) as usize] = char_used;
                    }
                }

                column
            })
            .collect::<Vec<_>>();

        let transposed = transpose(columns);

        Ok(AsciiWaveform { pixels: transposed })
    }
}

fn transpose<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut transposed = vec![vec![]; cols];

    for i in 0..cols {
        for j in 0..rows {
            transposed[i].push(matrix[j][i].clone());
        }
    }

    transposed
}

#[test]
fn test() {
    let x = convert_audio(
        "picts/beep-sound-short-237619.mp3".to_string(),
        "mp3".to_string(),
        500,        
        false,
        false,
        false,
    );
    std::fs::write("pic.txt", x.unwrap()).unwrap()
}

pub fn convert_audio(
    path: String,
    media_type: String,
    max_height: u32,
    uniform: bool,
    invert: bool,
    paralleled: bool,
) -> Result<String, Box<dyn Error>> {
    let pixels: Vec<Vec<char>>;

    if paralleled {
        pixels = AsciiWaveform::new_parallel(path, media_type, max_height, uniform, invert)?.pixels;
    } else {
        pixels = AsciiWaveform::new_no_parallel(path, media_type, max_height,uniform, invert)?.pixels;
    }
    Ok(pixels.gen_ascii())
}
