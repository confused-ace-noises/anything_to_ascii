use clap::builder::styling::Color;
use colored::{ColoredString, CustomColor};
use image::{GenericImageView, ImageReader, Pixel, Rgba};
use std::error::Error;

use crate::{ColorChar, DensityChar, JoinColored};

#[derive(Debug, Clone)]
pub struct AsciiImg {
    pub height: Option<usize>,
    pub width: Option<usize>,
    pub pixels: Vec<Vec<ColorChar>>,
}
impl AsciiImg {
    pub fn new(
        path: String,
        target_height: Option<usize>,
        target_width: Option<usize>,
        invert: bool,
        grayscale: bool,
    ) -> Result<AsciiImg, Box<dyn Error>> {
        let target_image = path;

        let img: image::DynamicImage;

        if grayscale {
            img = ImageReader::open(target_image)?.decode()?.grayscale();
        } else {
            img = ImageReader::open(target_image)?.decode()?;
        }

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

        let (height, width) = (
            target_height.unwrap_or(height as usize),
            target_width.unwrap_or(width as usize),
        );

        // // -------------------------------------------------------------

        // let pixels = pixels
        //     .into_iter()
        //     .map(|v| {
        //         v.into_iter()
        //             .map(|l| {
        //                 let x = l.channels();
        //                 (x[0], x[3])
        //             })
        //             .collect::<Vec<(u8, u8)>>()
        //     }) /*.skip_while(|p| p.is_empty())*/
        //     .collect::<Vec<Vec<(u8, u8)>>>();

        // let len_v = pixels.len();
        // let len_h =

        let source_height = pixels.len();
        let source_width = pixels[0].len();

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
                        buffer.push(pixels[indx_h][indx_v]);
                        // buffer.push(pixels[indx_v][indx_h]);
                        // buffer.push(pixels[((y * scale_y) + h) as Option<usize>][((x * scale_x) + w) as Option<usize>]);
                    }
                }
                out_row.push(
                    buffer
                        .into_iter()
                        .map(|pair| {if grayscale {(pair.calc_penalty(), CustomColor::new(255, 255, 255))} else { let x = pair.channels(); (pair.calc_penalty(), CustomColor::new(x[0], x[1], x[2]))}})
                        .collect::<Vec<(u8, CustomColor)>>()
                        .average(),
                );
            }
            output.push(out_row);
        }

        // println!("{:?}", output);
        // todo!();

        Ok(AsciiImg {
            height: target_height,
            width: target_width,
            pixels: output
                .iter()
                .map(|vec| {
                    vec.iter()
                        .map(|px| DensityChar::get_char_from_u8(px.0, invert, px.1))
                        .collect::<Vec<ColorChar>>()
                })
                .collect::<Vec<Vec<ColorChar>>>(),
        })
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

impl Average for Vec<CustomColor> {
    type Output = CustomColor;

    fn average(&self) -> Self::Output {
        let len = self.len() as f32;
        let mut r: usize = 0;
        let mut g: usize = 0;
        let mut b: usize= 0;

        self.iter().for_each(|k| {
            r += k.r as usize;
            g += k.g as usize;
            b += k.b as usize;
        });

        CustomColor::new((r as f32 / len).round_ties_even() as u8, (g as f32 / len).round_ties_even() as u8,  (b as f32 / len).round_ties_even() as u8)
    }
}

impl Average for Vec<(u8, CustomColor)> {
    type Output = (u8, CustomColor);

    fn average(&self) -> Self::Output {
        let (x, y): (Vec<u8>, Vec<CustomColor>) = self.clone().into_iter().unzip();

        let x = x.average();
        let y = y.average();

        (x, y)
    }
}

pub trait GenAscii {
    fn gen_ascii(&self) -> String;
}

impl GenAscii for Vec<Vec<char>> {
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

impl GenAscii for Vec<Vec<ColorChar>> {
    fn gen_ascii(&self) -> String {
        self.iter()
            .map(|vec| {
                vec.iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<ColoredString>>()
                    .custom_join("")
            })
            .collect::<Vec<String>>()
            .custom_join("\n")
    }
}

pub trait Penalty {
    fn calc_penalty(&self) -> u8;
}

impl Penalty for (u8, u8) {
    fn calc_penalty(&self) -> u8 {
        let jkl = 255;
        let x = self.0 as i32 - (jkl as i32 - self.1 as i32);
        x.max(0) as u8
    }
}

impl Penalty for Rgba<u8> {
    fn calc_penalty(&self) -> u8 {
        let things = self.channels();

        (things[0], things[3]).calc_penalty()
    }
}

// pub fn convert(
//     path: String,
//     target_height: Option<Option<usize>>,
//     target_width: Option<Option<usize>>,
//     invert: bool,
//     grayscale: bool,
// ) -> Result<String, Box<dyn Error>> {
//     let target_image = path;

//     let img = ImageReader::open(target_image)?.decode()?;

//     // let (height, width) = img.dimensions();
//     let (width, height) = img.dimensions();

//     // println!("{}, {}", width, height);

//     let mut pixels: Vec<Vec<Rgba<u8>>> = Vec::new();

//     for y in 0..height {
//         let mut inner_vec = vec![];
//         for x in 0..width {
//             // Get the pixel as an RGBA tuple
//             let pixel = img.get_pixel(x, y);

//             inner_vec.push(pixel);
//         }
//         pixels.push(inner_vec);
//     }

//     let ascii = AsciiImg::new(
//         pixels,
//         target_height.unwrap_or(height as Option<usize>),
//         target_width.unwrap_or(width as Option<usize>),
//         invert,
//         grayscale,
//     );

//     // -------------------------------------------------------------

//     Ok(ascii.pixels.gen_ascii())
// }

pub fn convert(
    path: String,
    target_height: Option<usize>,
    target_width: Option<usize>,
    invert: bool,
    grayscale: bool,
) -> Result<String, Box<dyn Error>> {
    let ascii = AsciiImg::new(path, target_height, target_width, invert, grayscale)?;
    //todo!()
    Ok(ascii.pixels.gen_ascii())
}

// pub fn convert_color(path: String, target_height: Option<Option<usize>>, target_width: Option<Option<usize>>) -> Result<String, Box<dyn Error>> {
//     let target_image = path;

//     let img = ImageReader::open(target_image)?.decode()?;

//     let (width, height) = img.dimensions();

// }
