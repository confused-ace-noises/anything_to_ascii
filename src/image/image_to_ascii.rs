use std::fmt::Display;
use image::{DynamicImage, GenericImageView, ImageReader, Rgba};
use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use crate::core::algo::algo_sequential;
use crate::core::char::Concat;
use crate::{core::{algo::algo_parallel, char::ColoredChar, flat_matrix::FlatMatrix}, utils::utils::DemureUnwrap, Error};

pub struct AsciiImg(pub FlatMatrix<ColoredChar>);

impl AsciiImg {
    pub fn new_parallel(
        image: DynamicImage,
        target_height: Option<usize>,
        target_width: Option<usize>,
        invert: bool,
        grayscale: bool,
        uniform: bool,
    ) -> Result<Self, Error> {
        // let image = if grayscale {
        //     ImageReader::open(path)?.decode()?.grayscale()
        // } else {
        //     ImageReader::open(path)?.decode()?
        // };

        let (width, height) = image.dimensions();
        let (height, width) = (height as usize, width as usize);

        let mut pixels = FlatMatrix::new_fill(height, width, Rgba::<u8>::from([0,0,0,0]));

        pixels.par_chunks_mut().enumerate().for_each(|(row, chunk)| {
            for column in 0..width {
                chunk[column] = image.get_pixel(column as u32, row as u32);
            }
        });

        let (target_width, target_height) = (target_width, target_height).demure_unwrap(width, height);

        let flat_matrix = algo_parallel(pixels, target_height, target_width, grayscale, invert, uniform);

        Ok(Self(flat_matrix))
    }
    
    pub fn new_sequential(
        image: DynamicImage,
        target_height: Option<usize>,
        target_width: Option<usize>,
        invert: bool,
        grayscale: bool,
        uniform: bool,
    ) -> Result<Self, Error> {
        // let image = if grayscale {
        //     ImageReader::open(path)?.decode()?.grayscale()
        // } else {
        //     ImageReader::open(path)?.decode()?
        // };

        let (width, height) = image.dimensions();
        let (height, width) = (height as usize, width as usize);

        let mut pixels = FlatMatrix::new_fill(height, width, Rgba::<u8>::from([0,0,0,0]));

        pixels.chunks_mut().enumerate().for_each(|(row, chunk)| {
            for column in 0..width {
                chunk[column] = image.get_pixel(column as u32, row as u32);
            }
        });

        let (target_width, target_height) = (target_width, target_height).demure_unwrap(width, height);

        let flat_matrix = algo_sequential(pixels, target_height, target_width, grayscale, invert, uniform);

        Ok(Self(flat_matrix))
    }

    pub fn new_parallel_file(
        path: String,
        target_height: Option<usize>,
        target_width: Option<usize>,
        invert: bool,
        grayscale: bool,
        uniform: bool,
    ) -> Result<Self, Error> {
        let image = if grayscale {
            ImageReader::open(path)?.decode()?.grayscale()
        } else {
            ImageReader::open(path)?.decode()?
        };

        Self::new_parallel(image, target_height, target_width, invert, grayscale, uniform)
    }

    pub fn new_sequential_file(
        path: String,
        target_height: Option<usize>,
        target_width: Option<usize>,
        invert: bool,
        grayscale: bool,
        uniform: bool,
    ) -> Result<Self, Error> {
        let image = if grayscale {
            ImageReader::open(path)?.decode()?.grayscale()
        } else {
            ImageReader::open(path)?.decode()?
        };

        Self::new_sequential(image, target_height, target_width, invert, grayscale, uniform)
    }
}

impl Display for AsciiImg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pic = self.0.into_iter_vecs().map(|x| x.concat()).collect::<Vec<_>>().join("\n");

        write!(f, "{}", pic)
    }
}

#[test]
fn test() {
    let path = "picts/idk_anymore.png";
    let image = AsciiImg::new_parallel_file(path.to_string(), Some(100), None, false, false, false).unwrap();
    println!("{}", image);
    std::fs::write("thing", image.to_string()).unwrap();
}