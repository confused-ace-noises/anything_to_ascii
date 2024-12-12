pub mod chars;
pub use chars::*;

pub mod ascii_img;

pub mod dimensions;

pub mod cli;

use colored::{Color, Colorize, CustomColor};

// 0.299 ∙ Red + 0.587 ∙ Green + 0.114 ∙ Blue
pub fn convert_grayscale(rgba: [u8; 4]) -> [u8; 4] {
    let x = (0.2989 * rgba[0] as f32 + 0.5870 * rgba[1] as f32 + 0.1140 * rgba[2] as f32).round_ties_even() as u8;
    [x, x, x, rgba[3]]
}


// #[test]
// fn thing() {
//     let x = "something".blue().on_bright_magenta();
//     let y = "something_else".custom_color(CustomColor {});
//     println!("{}", x);
//     println!("{}", y);
// }