use colored::CustomColor;
use image::{Pixel, Rgba};
use rayon::prelude::*;
use crate::ascii_img::{Average, Penalty};

use super::chars::*;

#[rustfmt::skip]
pub fn algo_parallel(pixels: Vec<Vec<Rgba<u8>>>, src_height: u32, src_width: u32, final_height: usize, final_width: usize, grayscale: bool, invert: bool, uniform: bool) -> Vec<Vec<ColorChar>> {
    let scale_x = ((src_width as f32 / final_width as f32).ceil() as usize).max(1);
    let scale_y = ((src_height as f32 / final_height as f32).ceil() as usize).max(1);
    
    (0..final_height)
            .into_par_iter()
            .map(|big_px_h| {
                (0..final_width)
                    .into_par_iter()
                    .map(|big_px_w| {
                        let thing = (0..scale_y).into_par_iter().map(|inner_y| {
                            (0..scale_x).into_par_iter().map(|inner_x| {
                                let indx_height = ((big_px_h * scale_y) + inner_y) as usize;
                                let indx_width = ((big_px_w * scale_x) + inner_x) as usize;
                                
                                if indx_height < src_height as usize && indx_width < src_width as usize {
                                    pixels[indx_height][indx_width]
                                } else {
                                    Rgba::from([128, 128, 128, 0])
                                }
                            }).collect::<Vec<Rgba<u8>>>()
                        }).flatten().map(|x| if grayscale { (x.calc_penalty(), CustomColor::new(255, 255, 255))} else { let z = x.channels(); (x.calc_penalty(), CustomColor::new(z[0], z[1], z[2]))}).collect::<Vec<(u8, CustomColor)>>().average();

                        DensityChar::get_char_from_u8(thing.0, invert, thing.1, uniform)
                    })
                    .collect::<Vec<ColorChar>>() // Collect the row
            })
            .collect::<Vec<Vec<ColorChar>>>()
}