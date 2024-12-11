use image::{GenericImageView, ImageReader, Pixel, Rgba};
use std::error::Error;

use crate::DensityChar;

#[derive(Debug, Clone)]
pub struct AsciiImg {
    pub height: usize,
    pub width: usize,
    pub pixels: Vec<Vec<char>>,
}
impl AsciiImg {
    pub fn new(pixels: Vec<Vec<Rgba<u8>>>, height: usize, width: usize, invert: bool) -> AsciiImg {
        let pixels = pixels
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|l| {
                        let x = l.channels();
                        (x[0], x[3])
                    })
                    .collect::<Vec<(u8, u8)>>()
            }) /*.skip_while(|p| p.is_empty())*/
            .collect::<Vec<Vec<(u8, u8)>>>();

        let len_v = pixels.len();
        let len_h = pixels[0].len();

        let source_height = len_v;
        let source_width = len_h;

        assert!(
            source_height > 0 && source_width > 0,
            "Input grid must not be empty."
        );
        assert!(
            width > 0 && height > 0,
            "Target dimensions must be greater than zero."
        );

        let mut output = Vec::new();

        // Scaling factors
        // let scale_x = (source_width as f32 / width as f32).floor() as u32;
        // let scale_y = (source_height as f32 / height as f32).floor() as u32;

        let scale_x = ((source_width as f32 / width as f32).ceil() as usize).max(1);
        let scale_y = ((source_height as f32 / height as f32).ceil() as usize).max(1);

        for h in 0..height {
            let mut out_row = Vec::new();
            for w in 0..width {
                let mut buffer = Vec::new();
                for y in 0..scale_y {
                    for x in 0..scale_x {
                        let indx_v = ((w * scale_x) + x) as usize;
                        let indx_h = ((h * scale_y) + y) as usize;

                        if indx_v >= source_width || indx_h >= source_height {
                            continue;
                        }
                        buffer.push(
                            pixels[indx_h][indx_v],
                        );
                        // buffer.push(pixels[indx_v][indx_h]);
                        // buffer.push(pixels[((y * scale_y) + h) as usize][((x * scale_x) + w) as usize]);
                    }
                }
                out_row.push(
                    buffer
                        .into_iter()
                        .map(|pair| pair.calc_penality())
                        .collect::<Vec<u8>>()
                        .average(),
                );
            }
            output.push(out_row);
        }

        // println!("{:?}", output);

        AsciiImg {
            height,
            width,
            pixels: output
                .iter()
                .map(|vec| {
                    vec.iter()
                        .map(|px| DensityChar::get_char_from_u8(*px, invert))
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>(),
        }
    }
}

pub trait Average {
    type Output;
    fn average(&self) -> Self::Output;
}

impl Average for Vec<u8> {
    type Output = u8;
    fn average(&self) -> Self::Output {
        let len = self.len() as f32;
        ((self.iter().map(|l| Into::<u32>::into(*l)).sum::<u32>() as f32) / len) as u8
    }
}

pub trait Something {
    fn gen_ascii(&self) -> String;
}

impl Something for Vec<Vec<char>> {
    fn gen_ascii(&self) -> String {
        self.iter()
            .map(|vec| {
                vec.iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub trait Penality {
    fn calc_penality(&self) -> u8;
}

impl Penality for (u8, u8) {
    fn calc_penality(&self) -> u8 {
        let jkl = 255;
        let x = self.0 as i32 - (jkl as i32 - self.1 as i32);
        x.max(0) as u8
    }
}

pub fn convert(path: String, target_height: Option<usize>, target_width: Option<usize>, invert: bool) -> Result<String, Box<dyn Error>> {
    let target_image = path;

    let img = ImageReader::open(target_image)?.decode()?.grayscale();

    // let (height, width) = img.dimensions();
    let (width, height) = img.dimensions();

    // println!("{}, {}", width, height);

    let mut pixels: Vec<Vec<Rgba<u8>>> = Vec::new();

    for y in 0..height {
        let mut inner_vec = vec![];
        for x in 0..width {
            // Get the pixel as an RGBA tuple
            let pixel = img.get_pixel(x, y);

            inner_vec.push(pixel);
        }
        pixels.push(inner_vec);
    }

    // println!("{}, {}", pixels.len(), pixels[0].len());

    let ascii = AsciiImg::new(pixels, target_height.unwrap_or(height as usize), target_width.unwrap_or(width as usize), invert);

    Ok(ascii.pixels.gen_ascii())
}
