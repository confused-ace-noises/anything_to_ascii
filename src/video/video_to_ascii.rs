use std::path::Path;

use image::Rgba;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{core::{algo::{algo_parallel, algo_sequential}, flat_matrix::FlatMatrix}, image::image_to_ascii::AsciiImg, utils::utils::DemureUnwrap, Error};

pub struct AsciiVid(pub Vec<AsciiImg>);

impl AsciiVid {
    pub fn new_paralleled(
        path: &String,
        n_frames: Option<usize>,
        final_height: Option<usize>,
        final_width: Option<usize>,
        invert: bool,
        grayscale: bool,
        uniform: bool,
    ) -> Result<Self, Error> {
        video_rs::init().unwrap();
        let mut decoder = video_rs::Decoder::new(Path::new(&path))?;

        let images = par_select_spaced_items(decoder
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

                                // let index = (px_outer*frame.width() + px_inner) as usize;

                                // let x = frame.data(index);


                                Rgba::<u8>::from([x[0], x[1], x[2], 255])
                            })
                            .collect::<Vec<Rgba<u8>>>()
                    })
                    .collect::<FlatMatrix<Rgba<u8>>>();

                let (final_width, final_height) = (final_width, final_height)
                    .demure_unwrap(frame.shape()[1] as usize, frame.shape()[0] as usize);


                println!("xxx");
                algo_parallel(
                    x,
                    final_height as usize,
                    final_width as usize,
                    grayscale,
                    invert,
                    uniform,
                )
            })
            .collect::<Vec<_>>();

        Ok(Self(
            ascii_images
                .into_iter()
                .map(|x| AsciiImg(x))
                .collect()))
    }

    pub fn new_sequential(
        path: &String,
        n_frames: Option<usize>,
        final_height: Option<usize>,
        final_width: Option<usize>,
        invert: bool,
        grayscale: bool,
        uniform: bool,
    ) -> Result<Self, Error> {
        video_rs::init().unwrap();
        let mut decoder = video_rs::Decoder::new(Path::new(&path))?;

        let images = seq_select_spaced_items(decoder
            .decode_iter()
            .take_while(|f| f.is_ok())
            .map(|l| l.unwrap())
            .collect::<Vec<_>>(), n_frames.and_then(|x| Some(x as usize)));

        let ascii_images = images
            .into_iter()
            .map(|frame| {
                let frame = frame.1;
                let x = (0..frame.shape()[0])
                    .into_iter()
                    .map(|px_outer| {
                        (0..frame.shape()[1])
                            .into_iter()
                            .map(|px_inner| {
                                let x = frame
                                    .slice(ndarray::s![px_outer, px_inner, ..])
                                    .to_slice()
                                    .unwrap();

                                // let index = (px_outer*frame.width() + px_inner) as usize;

                                // let x = frame.data(index);


                                Rgba::<u8>::from([x[0], x[1], x[2], 255])
                            })
                            .collect::<Vec<Rgba<u8>>>()
                    })
                    .collect::<FlatMatrix<Rgba<u8>>>();

                let (final_width, final_height) = (final_width, final_height)
                    .demure_unwrap(frame.shape()[1] as usize, frame.shape()[0] as usize);


                println!("xxx");
                algo_sequential(
                    x,
                    final_height as usize,
                    final_width as usize,
                    grayscale,
                    invert,
                    uniform,
                )
            })
            .collect::<Vec<_>>();

        Ok(Self(
            ascii_images
                .into_iter()
                .map(|x| AsciiImg(x))
                .collect()))
    }

    // pub fn new_parallel_file(
    //     path: String,
    //     target_height: Option<usize>,
    //     target_width: Option<usize>,
    //     invert: bool,
    //     grayscale: bool,
    //     uniform: bool,
    // ) -> Result<Self, Error> {
    //     let image = if grayscale {
    //         ImageReader::open(path)?.decode()?.grayscale()
    //     } else {
    //         ImageReader::open(path)?.decode()?
    //     };

    //     Self::new_parallel(image, target_height, target_width, invert, grayscale, uniform)
    // }

    // pub fn new_sequential_file(
    //     path: String,
    //     target_height: Option<usize>,
    //     target_width: Option<usize>,
    //     invert: bool,
    //     grayscale: bool,
    //     uniform: bool,
    // ) -> Result<Self, Error> {
    //     let image = if grayscale {
    //         ImageReader::open(path)?.decode()?.grayscale()
    //     } else {
    //         ImageReader::open(path)?.decode()?
    //     };

    //     Self::new_sequential(image, target_height, target_width, invert, grayscale, uniform)
    // }
}

fn par_select_spaced_items<I>(iter: Vec<I>, n_frames_to_keep: Option<usize>) -> Vec<I>
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

fn seq_select_spaced_items<I>(iter: Vec<I>, n_frames_to_keep: Option<usize>) -> Vec<I>
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
            .into_iter() // Use parallel iterator from rayon
            .map(|i| {
                let index = (i as f32*ratio).floor() as usize;
                iter[index].clone()
            })
            .collect()
    }
}

// #[test]
// fn test() {
//     let path = "picts/gender-bender-gender.mp4".to_string();
//     let vid = AsciiVid::new_paralleled(path, None, Some(200), None, false, true, false).unwrap();
//     let picts = vid.0;

//     for pic in picts.into_iter().enumerate() {
//         fs::write(format!("yyy/{}", pic.0), pic.1.to_string()).unwrap();
//     }
// }