#![allow(dead_code, non_upper_case_globals)]

use colored::{ColoredString, Colorize, CustomColor};
use std::u8;

static char_empty: char = ' ';
static char0_17: char = '.';
static char17_34: char = '-';
static char34_51: char = '=';
static char51_68: char = 'r';
static char68_85: char = 'z';
static char85_102: char = 'L';
static char102_119: char = 'T';
static char119_136: char = '3';
static char136_153: char = 'y';
static char153_170: char = 'Y';
static char170_187: char = 'V';
static char187_204: char = 'K';
static char204_221: char = '8';
static char221_238: char = 'Q';
static char238_255: char = '@';

//  '.,-~:;=!o*#%@$

pub enum DensityChar {
    Empty,
    Char0_17,
    Char17_34,
    Char34_51,
    Char51_68,
    Char68_85,
    Char85_102,
    Char102_119,
    Char119_136,
    Char136_153,
    Char153_170,
    Char170_187,
    Char187_204,
    Char204_221,
    Char221_238,
    Char238_255,
}
impl DensityChar {
    pub fn get_char(self) -> char {
        match self {
            DensityChar::Empty => char_empty,
            DensityChar::Char0_17 => char0_17,
            DensityChar::Char17_34 => char17_34,
            DensityChar::Char34_51 => char34_51,
            DensityChar::Char51_68 => char51_68,
            DensityChar::Char68_85 => char68_85,
            DensityChar::Char85_102 => char85_102,
            DensityChar::Char102_119 => char102_119,
            DensityChar::Char119_136 => char119_136,
            DensityChar::Char136_153 => char136_153,
            DensityChar::Char153_170 => char153_170,
            DensityChar::Char170_187 => char170_187,
            DensityChar::Char187_204 => char187_204,
            DensityChar::Char204_221 => char204_221,
            DensityChar::Char221_238 => char221_238,
            DensityChar::Char238_255 => char238_255,
        }
    }

    pub fn get_char_from_u8(n: u8, invert: bool, color: CustomColor, uniform: bool) -> ColorChar {
        let ch = {
            if !invert {
                if uniform {
                    if n == 0 {
                        char_empty
                    } else {
                        char238_255
                    }
                } else {
                    match n {
                        0 => char_empty,
                        1..17 => char0_17,
                        17..34 => char17_34,
                        34..51 => char34_51,
                        51..68 => char51_68,
                        68..85 => char68_85,
                        85..102 => char85_102,
                        102..119 => char102_119,
                        119..136 => char119_136,
                        136..153 => char136_153,
                        153..170 => char153_170,
                        170..187 => char170_187,
                        187..204 => char187_204,
                        204..221 => char204_221,
                        221..238 => char221_238,
                        238..=255 => char238_255,
                    }
                }
            } else {
                if uniform {
                    if n == 255 {
                        char_empty
                    } else {
                        char0_17
                    }
                } else {
                    match n {
                        0..17 => char238_255,
                        17..34 => char221_238,
                        34..51 => char204_221,
                        51..68 => char187_204,
                        68..85 => char170_187,
                        85..102 => char153_170,
                        102..119 => char136_153,
                        119..136 => char119_136,
                        136..153 => char102_119,
                        153..170 => char85_102,
                        170..187 => char68_85,
                        187..204 => char51_68,
                        204..221 => char34_51,
                        221..238 => char17_34,
                        238..255 => char0_17,
                        255 => char_empty,
                    }
                }
            }
        };

        ColorChar::new(ch, color)
    }
}

#[derive(Debug, Clone)]
pub struct ColorChar {
    ch: char,
    color: CustomColor,
}

impl ColorChar {
    pub fn new(ch: char, color: CustomColor) -> Self {
        ColorChar { ch, color }
    }

    pub fn to_string(&self) -> String {
        let ch = self.ch;
        let color = self.color;

        if color.r == 255 && color.g == 255 && color.b == 255 {
            return ch.to_string();
        } else {
            format!("{}", ch.to_string().custom_color(color))
        }
    }
}

pub trait JoinColored {
    fn custom_join(&self, separator: &str) -> String;
}

impl JoinColored for Vec<ColoredString> {
    fn custom_join(&self, separator: &str) -> String {
        let x = String::from(separator);
        let mut final_string = String::new();

        for y in self.iter() {
            final_string = format!("{}{}{}", final_string, x.clone(), y);
        }

        final_string
    }
}

impl JoinColored for Vec<String> {
    fn custom_join(&self, separator: &str) -> String {
        let x = String::from(separator);
        let mut final_string = String::new();

        for y in self.iter() {
            final_string = format!("{}{}{}", final_string, x.clone(), y);
        }

        final_string
    }
}

#[test]
fn something() {
    let red_text = "Red".red();
    let green_text = "Green".green();
    let blue_text = "Blue".blue();

    // Concatenate while preserving colors
    let concatenated = format!("{} {} {}", red_text, green_text, blue_text);

    println!("{}", concatenated);
}
