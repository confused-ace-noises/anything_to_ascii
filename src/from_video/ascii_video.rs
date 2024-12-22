use image::Rgba;
use rayon::prelude::*;
use std::{
    error::Error,
    path::Path,
};
use video_rs;

use crate::ascii_img::{AsciiImg, DemureUnwrap, GenAscii};

pub struct AsciiVideo {
    pub frames: Vec<AsciiImg>,
    pub n_frames: usize, // in frames/second
}
impl AsciiVideo {
    fn new_parallel_fixed(
        path: String,
        n_frames: Option<u32>,
        final_height: Option<u32>,
        final_width: Option<u32>,
        invert: bool,
        grayscale: bool,
        uniform: bool,
    ) -> Result<AsciiVideo, Box<dyn Error>> {
        video_rs::init().unwrap();
        let mut decoder = video_rs::Decoder::new(Path::new(&path))?;

        let images = select_spaced_items(decoder
            .decode_iter()
            .take_while(|f| f.is_ok())
            .map(|l| l.unwrap())
            .collect::<Vec<_>>(), n_frames.and_then(|x| Some(x as usize)));

        let ascii_images = images
            .into_par_iter()
            .map(|frame| {
                let frame = frame.1;

                let x = (0..frame.shape()[0])
                    .into_par_iter()
                    .map(|px_outer| {
                        (0..frame.shape()[1])
                            .into_par_iter()
                            .map(|px_inner| {
                                let x = frame
                                    .slice(ndarray::s![px_outer, px_inner, ..])
                                    .to_slice()
                                    .unwrap();
                                Rgba::<u8>::from([x[0], x[1], x[2], 255])
                            })
                            .collect::<Vec<Rgba<u8>>>()
                    })
                    .collect::<Vec<Vec<Rgba<u8>>>>();

                let (final_width, final_height) = (final_width, final_height)
                    .demure_unwrap(frame.shape()[1] as u32, frame.shape()[0] as u32);

                crate::core::algo::algo_parallel(
                    x,
                    frame.shape()[0] as u32,
                    frame.shape()[1] as u32,
                    final_height as usize,
                    final_width as usize,
                    grayscale,
                    invert,
                    uniform,
                )
            })
            .collect::<Vec<_>>();

        let len = ascii_images.len() as u32;

        Ok(AsciiVideo {
            frames: ascii_images
                .into_iter()
                .map(|x| AsciiImg {
                    height: final_height.and_then(|y| Some(y as usize)),
                    width: final_width.and_then(|y| Some(y as usize)),
                    pixels: x,
                })
                .collect(),
            n_frames: n_frames.unwrap_or(len) as usize,
        })
    }
}

fn select_spaced_items<I>(iter: Vec<I>, n_frames_to_keep: Option<usize>) -> Vec<I>
where
    I: Send + Sync + Clone,
{
    let len = iter.len();
    //let iter = iter.into_par_iter();

    if n_frames_to_keep.is_none() {
        return iter;
    } else if n_frames_to_keep.unwrap() >= len {
        return iter;
    } else {
        let n_frames_to_keep = n_frames_to_keep.unwrap();

        let ratio = len as f32 / n_frames_to_keep as f32;

        (0..n_frames_to_keep)
            .into_par_iter() // Use parallel iterator from rayon
            .map(|i| {
                let index = (i as f32*ratio).floor() as usize;
                iter[index].clone()
            })
            .collect()
    }
}

pub fn convert_video(
    path: String,
    n_frames: Option<u32>,
    final_height: Option<u32>,
    final_width: Option<u32>,
    invert: bool,
    grayscale: bool,
    uniform: bool,
    paralleled: bool,
) -> Result<Vec<String>, Box<dyn Error>> {
    let ascii_video: AsciiVideo;

    if paralleled {
        ascii_video = AsciiVideo::new_parallel_fixed(
            path,
            n_frames,
            final_height,
            final_width,
            invert,
            grayscale,
            uniform,
        )?;
    } else {
        todo!()
    }

    Ok(ascii_video.frames.iter().map(|k| k.pixels.gen_ascii()).collect())
}

#[test]
fn test() {
    convert_video(
        "gender-bender-gender.mp4".to_string(),
        None,
        Some(200),
        None,
        false,
        false,
        false,
        true,
    )
    .unwrap();
}
