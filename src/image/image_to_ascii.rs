use std::fmt::Display;
use image::{DynamicImage, GenericImageView, ImageReader, Rgba};
use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use crate::core::algo::algo_sequential;
use crate::core::char::Concat;
use crate::report;
use crate::utils::utils::Verbosity;
use crate::{core::{algo::algo_parallel, char::ColoredChar, flat_matrix::FlatMatrix}, utils::utils::DemureUnwrap, Error};
use crate::timestamp;

pub struct AsciiImg(pub FlatMatrix<ColoredChar>);

impl AsciiImg {
    pub fn new_parallel(
        image: DynamicImage,
        target_height: Option<usize>,
        target_width: Option<usize>,
        invert: bool,
        grayscale: bool,
        uniform: bool,
        verbosity: Verbosity,
    ) -> Result<Self, Error> {
        report!(verbosity, @verbose "running image with mode: parallel");

        report!(verbosity, @verbose "getting width and height");
        let (width, height) = image.dimensions();
        let (height, width) = (height as usize, width as usize);
        report!(verbosity, @verbose "finished getting width and height: width: {}, height: {}", height, width);

        let mut pixels = FlatMatrix::new_fill(height, width, Rgba::<u8>::from([0,0,0,0]));

        report!(verbosity, @normal "loading pixels...");
        pixels.par_chunks_mut().enumerate().for_each(|(row, chunk)| {
            for column in 0..width {
                chunk[column] = image.get_pixel(column as u32, row as u32);
            }
        });
        report!(verbosity, @normal "finished loading pixels");

        report!(verbosity, @verbose "calculating final height and width");
        let (target_width, target_height) = (target_width, target_height).demure_unwrap(width, height);
        report!(verbosity, @verbose "finished calculating height and width");

        let show_progress = {
            if let Verbosity::Normal = verbosity {
                true
            } else {
                false
            }
        };

        report!(verbosity, @normal "executing conversion parallel algorithm on image...");
        let flat_matrix = algo_parallel(pixels, target_height, target_width, grayscale, invert, uniform, verbosity, show_progress);
        report!(verbosity, @normal "finished executing parallel conversion algorithm on image");

        Ok(Self(flat_matrix))
    }
    
    pub fn new_sequential(
        image: DynamicImage,
        target_height: Option<usize>,
        target_width: Option<usize>,
        invert: bool,
        grayscale: bool,
        uniform: bool,
        verbosity: Verbosity
    ) -> Result<Self, Error> {
        report!(verbosity, @verbose "running image with mode: sequential");

        report!(verbosity, @verbose "getting width and height");
        let (width, height) = image.dimensions();
        let (height, width) = (height as usize, width as usize);
        report!(verbosity, @verbose "finished getting width and height: width: {}, height: {}", height, width);

        let mut pixels = FlatMatrix::new_fill(height, width, Rgba::<u8>::from([0,0,0,0]));

        report!(verbosity, @normal "loading pixels");
        pixels.chunks_mut().enumerate().for_each(|(row, chunk)| {
            for column in 0..width {
                chunk[column] = image.get_pixel(column as u32, row as u32);
            }
        });
        report!(verbosity, @normal "finished loading pixels");

        report!(verbosity, @verbose "calculating final height and width");
        let (target_width, target_height) = (target_width, target_height).demure_unwrap(width, height);
        report!(verbosity, @verbose "finished calculating height and width");

        let show_progress = {
            if let Verbosity::Normal = verbosity {
                true
            } else {
                false
            }
        };

        report!(verbosity, @normal "executing conversion sequential algorithm on image...");
        let flat_matrix = algo_sequential(pixels, target_height, target_width, grayscale, invert, uniform, verbosity, show_progress);
        report!(verbosity, @normal "finished executing sequential conversion algorithm on image");

        Ok(Self(flat_matrix))
    }

    pub fn new_parallel_file(
        path: String,
        target_height: Option<usize>,
        target_width: Option<usize>,
        invert: bool,
        grayscale: bool,
        uniform: bool,
        verbosity: Verbosity
    ) -> Result<Self, Error> {
        report!(verbosity, @verbose "opening image");
        let image = if grayscale {
            ImageReader::open(path)?.decode()?.grayscale()
        } else {
            ImageReader::open(path)?.decode()?
        };
        report!(verbosity, @verbose "finished opening image");

        Self::new_parallel(image, target_height, target_width, invert, grayscale, uniform, verbosity)
    }

    pub fn new_sequential_file(
        path: String,
        target_height: Option<usize>,
        target_width: Option<usize>,
        invert: bool,
        grayscale: bool,
        uniform: bool,
        verbosity: Verbosity
    ) -> Result<Self, Error> {
        report!(verbosity, @verbose "opening image");
        let image = if grayscale {
            ImageReader::open(path)?.decode()?.grayscale()
        } else {
            ImageReader::open(path)?.decode()?
        };
        report!(verbosity, @verbose "finished opening image");

        Self::new_sequential(image, target_height, target_width, invert, grayscale, uniform, verbosity)
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
    let image = AsciiImg::new_parallel_file(path.to_string(), Some(100), None, false, false, false, Verbosity::Verbose).unwrap();
    println!("{}", image);
    std::fs::write("thing", image.to_string()).unwrap();
}