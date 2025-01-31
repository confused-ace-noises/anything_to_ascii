use colored::CustomColor;
use image::Rgba;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use indicatif::{ProgressBar, ProgressStyle};
use crate::{report, utils::utils::Verbosity, timestamp};

use super::{char::ColoredChar, flat_matrix::FlatMatrix};

pub fn algo_parallel(pixels: FlatMatrix<Rgba<u8>>, target_height: usize, target_width: usize, grayscale: bool, uniform: bool, invert: bool, verbosity: Verbosity, progress: bool) -> FlatMatrix<ColoredChar> {
    let src_height = pixels.rows; 
    let src_width = pixels.columns;

    let progress: Option<ProgressBar> = if progress {
        let progress = ProgressBar::new((target_height*target_width) as u64);
        progress.set_style(ProgressStyle::default_bar()
            .template("pixel processing progress: [{bar:40.cyan/blue}] {pos:>3}/{len}")
            .unwrap());
        Some(progress)
        } else {
            None
        };

    report!(verbosity, @verbose "calculating height and width conversion scales...");
    let scale_height = ((src_height as f32 / target_height as f32).ceil() as usize).max(1);
    let scale_width = ((src_width as f32 / target_width as f32).ceil() as usize).max(1);
    report!(verbosity, @verbose "finished calculating height and width conversion scales; height scale: {}, width scale: {}", scale_height, scale_width);

    // report!(verbosity, @normal "executing parallel conversion algorithm on image or video frame...");
    let final_matrix = (0..target_height).into_par_iter().map(|big_px_height| {
        (0..target_width).into_par_iter().map(|big_px_width| {
            
            report!(verbosity, @verbose "current character index: (row: {}, column: {})", big_px_height, big_px_width);
            let big_px_to_average = (0..scale_height).into_par_iter().map(|small_px_height| {
                (0..scale_width).into_par_iter().map(|small_px_width| {
                    let index = (big_px_height*scale_height + small_px_height, big_px_width*scale_width + small_px_width);

                    if index.0 >= pixels.rows || index.1 >= pixels.columns {
                        ColoredChar {
                            color: CustomColor::new(255, 255, 255),
                            ch: ' ',
                            density: 0,
                            display: false,
                        }
                    } else {
                        let pixel = pixels[index];
                        ColoredChar::from_color(pixel, grayscale, invert, uniform)
                    }
                    // let density = pixel.calc_penalty();

                    // (density as f32, !((density == 0 && !invert) || (density == 255 && invert)))
                }).collect::<Vec<_>>()
            }).flatten()
                .map(|x| ((x.color.r as f32, x.color.g as f32, x.color.b as f32, x.density as f32, x.display), 1))
                .reduce(|| ((0.0, 0.0, 0.0, 0.0, false), 0_usize), |(sum1, count1), (sum2, count2)| ((sum1.0 + sum2.0, sum1.1 + sum2.1, sum1.2 + sum2.2, sum1.3 + sum2.3, sum1.4 || sum2.4), count1 + count2));

            let big_px_average = {
                let dividend = big_px_to_average.1 as f32;

                let r = (big_px_to_average.0.0 / dividend).round_ties_even() as u8;
                let g = (big_px_to_average.0.1 / dividend).round_ties_even() as u8;
                let b = (big_px_to_average.0.2 / dividend).round_ties_even() as u8;
                
                let density = (big_px_to_average.0.3 / dividend).round_ties_even() as u8;
                
                let display = big_px_to_average.0.4;
                
                ColoredChar::from_everything(density, (r, g, b), display, invert, uniform)
            };

            if let Some(prog) = &progress {prog.inc(1)};

            if let Verbosity::Verbose = verbosity {
                let r = big_px_average.color.r;
                let g = big_px_average.color.g;
                let b = big_px_average.color.b;
            
                // progress;

                report!(verbosity, @verbose "finished working on character: (row: {}, column: {}), {}, (r: {}, g: {}, b: {})", big_px_height, big_px_width, big_px_average, r, g, b);
            }
            big_px_average
        }).collect::<Vec<_>>()
    }).collect::<FlatMatrix<_>>();
    
    if let Some(prog) = progress {prog.finish();};
    
    // report!(verbosity, @normal "finished executing parallel conversion algorithm on image or video frame");
    final_matrix
}

pub fn algo_sequential(pixels: FlatMatrix<Rgba<u8>>, target_height: usize, target_width: usize, grayscale: bool, uniform: bool, invert: bool, verbosity: Verbosity, progress: bool) -> FlatMatrix<ColoredChar> {
    let src_height = pixels.rows; 
    let src_width = pixels.columns;

    let progress: Option<ProgressBar> = if progress {
        let progress = ProgressBar::new((target_height*target_width) as u64);
        progress.set_style(ProgressStyle::default_bar()
            .template("image/frame progress: [{bar:40.cyan/blue}] {pos:>3}/{len}")
            .unwrap());
        Some(progress)
        } else {
            None
        };


    report!(verbosity, @verbose "calculating height and width conversion scales...");
    let scale_height = ((src_height as f32 / target_height as f32).ceil() as usize).max(1);
    let scale_width = ((src_width as f32 / target_width as f32).ceil() as usize).max(1);
    report!(verbosity, @verbose "finished calculating height and width conversion scales; height scale: {}, width scale: {}", scale_height, scale_width);

    // report!(verbosity, @normal "executing conversion sequential algorithm on image or video frame...");
    let final_matrix = (0..target_height).into_iter().map(|big_px_height| {
        (0..target_width).into_iter().map(|big_px_width| {
            
            report!(verbosity, @verbose "current character index: (row: {}, column: {})", big_px_height, big_px_width);
            let big_px_to_average = (0..scale_height).into_iter().map(|small_px_height| {
                (0..scale_width).into_iter().map(|small_px_width| {
                    let index = (big_px_height*scale_height + small_px_height, big_px_width*scale_width + small_px_width);

                    if index.0 >= pixels.rows || index.1 >= pixels.columns {
                        ColoredChar {
                            color: CustomColor::new(255, 255, 255),
                            ch: ' ',
                            density: 0,
                            display: false,
                        }
                    } else {
                        let pixel = pixels[index];
                        ColoredChar::from_color(pixel, grayscale, invert, uniform)
                    }
                    // let density = pixel.calc_penalty();

                    // (density as f32, !((density == 0 && !invert) || (density == 255 && invert)))
                }).collect::<Vec<_>>()
            }).flatten()
                .map(|x| ((x.color.r as f32, x.color.g as f32, x.color.b as f32, x.density as f32, x.display), 1))
                .fold(((0.0, 0.0, 0.0, 0.0, false), 0_usize), |(sum1, count1), (sum2, count2)| ((sum1.0 + sum2.0, sum1.1 + sum2.1, sum1.2 + sum2.2, sum1.3 + sum2.3, sum1.4 || sum2.4), count1 + count2));


            let big_px_average = {
                let dividend = big_px_to_average.1 as f32;

                let r = (big_px_to_average.0.0 / dividend).round_ties_even() as u8;
                let g = (big_px_to_average.0.1 / dividend).round_ties_even() as u8;
                let b = (big_px_to_average.0.2 / dividend).round_ties_even() as u8;

                let density = (big_px_to_average.0.3 / dividend).round_ties_even() as u8;

                let display = big_px_to_average.0.4;

                ColoredChar::from_everything(density, (r, g, b), display, invert, uniform)
            };

            if let Some(prog) = &progress {prog.inc(1)};

            if let Verbosity::Verbose = verbosity {
                let r = big_px_average.color.r;
                let g = big_px_average.color.g;
                let b = big_px_average.color.b;
            
                report!(verbosity, @verbose "finished working on character: (row: {}, column: {}), {}, (r: {}, g: {}, b: {})", big_px_height, big_px_width, big_px_average, r, g, b);
            }
            big_px_average
        }).collect::<Vec<_>>()
    }).collect::<FlatMatrix<_>>();
    if let Some(prog) = progress {prog.finish();};
    // report!(verbosity, @normal "finished executing parallel conversion algorithm on image or video frame");
    final_matrix
}