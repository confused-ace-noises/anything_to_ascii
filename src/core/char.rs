use colored::{Color, Colorize, CustomColor};
use image::Rgba;

#[derive(Debug, Clone)]
pub struct ColoredChar {
    pub color: CustomColor,
    pub ch: char,
    pub density: u8, 
    pub display: bool,
}

impl ColoredChar {
    pub const CHAR_EMPTY: char = ' ';
    pub const CHAR0_17: char = '.';
    pub const CHAR17_34: char = '-';
    pub const CHAR34_51: char = '=';
    pub const CHAR51_68: char = 'r';
    pub const CHAR68_85: char = 'z';
    pub const CHAR85_102: char = 'L';
    pub const CHAR102_119: char = 'T';
    pub const CHAR119_136: char = '3';
    pub const CHAR136_153: char = 'y';
    pub const CHAR153_170: char = 'Y';
    pub const CHAR170_187: char = 'V';
    pub const CHAR187_204: char = 'K';
    pub const CHAR204_221: char = '8';
    pub const CHAR221_238: char = 'Q';
    pub const CHAR238_255: char = '@';

    pub fn from_everything(density: u8, color: (u8, u8, u8), mut display: bool, invert: bool, uniform: bool) -> ColoredChar {
        let ch: char = {
            if uniform && invert {
                Self::CHAR_EMPTY
            } else if uniform {
                Self::CHAR238_255
            } else {
                if !invert {
                    match density {
                        0 => {
                            display = false;
                            Self::CHAR_EMPTY
                        }
                        1..17 => Self::CHAR0_17,
                        17..34 => Self::CHAR17_34,
                        34..51 => Self::CHAR34_51,
                        51..68 => Self::CHAR51_68,
                        68..85 => Self::CHAR68_85,
                        85..102 => Self::CHAR85_102,
                        102..119 => Self::CHAR102_119,
                        119..136 => Self::CHAR119_136,
                        136..153 => Self::CHAR136_153,
                        153..170 => Self::CHAR153_170,
                        170..187 => Self::CHAR170_187,
                        187..204 => Self::CHAR187_204,
                        204..221 => Self::CHAR204_221,
                        221..238 => Self::CHAR221_238,
                        238..=255 => Self::CHAR238_255,
                    }
                } else {
                    match density {
                        0..17 => Self::CHAR238_255,
                        17..34 => Self::CHAR221_238,
                        34..51 => Self::CHAR204_221,
                        51..68 => Self::CHAR187_204,
                        68..85 => Self::CHAR170_187,
                        85..102 => Self::CHAR153_170,
                        102..119 => Self::CHAR136_153,
                        119..136 => Self::CHAR119_136,
                        136..153 => Self::CHAR102_119,
                        153..170 => Self::CHAR85_102,
                        170..187 => Self::CHAR68_85,
                        187..204 => Self::CHAR51_68,
                        204..221 => Self::CHAR34_51,
                        221..238 => Self::CHAR17_34,
                        238..255 => Self::CHAR0_17,
                        255 => {
                            display = false;
                            Self::CHAR_EMPTY
                        }
                    }
                }
            }
        };

        ColoredChar { color: CustomColor { r: color.0, g: color.1, b: color.2 }, ch, density, display }
    }

    pub fn from_color(color: Rgba<u8>, grayscale: bool, invert: bool, uniform: bool) -> Self {
        let mut display = true;

        let brightness = color.calc_penalty();

        let ch: char = {
            if uniform && invert {
                Self::CHAR_EMPTY
            } else if uniform {
                Self::CHAR238_255
            } else {
                if !invert {
                    match brightness {
                        0 => {
                            display = false;
                            Self::CHAR_EMPTY
                        }
                        1..17 => Self::CHAR0_17,
                        17..34 => Self::CHAR17_34,
                        34..51 => Self::CHAR34_51,
                        51..68 => Self::CHAR51_68,
                        68..85 => Self::CHAR68_85,
                        85..102 => Self::CHAR85_102,
                        102..119 => Self::CHAR102_119,
                        119..136 => Self::CHAR119_136,
                        136..153 => Self::CHAR136_153,
                        153..170 => Self::CHAR153_170,
                        170..187 => Self::CHAR170_187,
                        187..204 => Self::CHAR187_204,
                        204..221 => Self::CHAR204_221,
                        221..238 => Self::CHAR221_238,
                        238..=255 => Self::CHAR238_255,
                    }
                } else {
                    match brightness {
                        0..17 => Self::CHAR238_255,
                        17..34 => Self::CHAR221_238,
                        34..51 => Self::CHAR204_221,
                        51..68 => Self::CHAR187_204,
                        68..85 => Self::CHAR170_187,
                        85..102 => Self::CHAR153_170,
                        102..119 => Self::CHAR136_153,
                        119..136 => Self::CHAR119_136,
                        136..153 => Self::CHAR102_119,
                        153..170 => Self::CHAR85_102,
                        170..187 => Self::CHAR68_85,
                        187..204 => Self::CHAR51_68,
                        204..221 => Self::CHAR34_51,
                        221..238 => Self::CHAR17_34,
                        238..255 => Self::CHAR0_17,
                        255 => {
                            display = false;
                            Self::CHAR_EMPTY
                        }
                    }
                }
            }
        };

        let color = if grayscale {
            let gray_color = grayscale_calc(color.0[0], color.0[1], color.0[2]);

            CustomColor::new(gray_color, gray_color, gray_color)
        } else {
            CustomColor::new(color.0[0], color.0[1], color.0[2])
        };

        ColoredChar { color, ch, display, density: brightness }
    }

    pub fn is_grayscale(&self) -> bool {
        self.color.r == self.color.g && self.color.g == self.color.b
    }

    pub fn is_white(&self) -> bool {
        self.color.r == 255 && self.is_grayscale()
    }
}

pub trait Penalty {
    fn calc_penalty(&self) -> u8;
}

impl Penalty for Rgba<u8> {
    fn calc_penalty(&self) -> u8 {
        let alpha = self.0[3];
        let grayscale = grayscale_calc(self.0[0], self.0[1], self.0[2]);

        let val = grayscale.saturating_sub(u8::MAX - alpha);

        val
    }
}

pub trait Concat {
    fn concat(self) -> String;
}

impl Concat for Vec<ColoredChar> {
    fn concat(self) -> String {
        self.into_iter().map(|c| c.to_string()).collect::<Vec<_>>().concat()
    }
}

impl std::fmt::Display for ColoredChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.display {
            let color = self.color;
            let char = self.ch;

            if self.is_grayscale() { write!(f, "{}", char) } else { write!(f, "{}", char.to_string().color(Color::TrueColor { r: color.r, g: color.g, b: color.b })) }
        } else {
            Ok(())
        }
    }
}

pub fn grayscale_calc(r: u8, g: u8, b: u8) -> u8 {
    (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32).round_ties_even() as u8
}
