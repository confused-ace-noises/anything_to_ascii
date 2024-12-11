#![allow(dead_code, non_upper_case_globals)]

use std::u8;

static char_empty: char = ' ';
static char0_17: char = '.';
static char17_34: char = '\'';
static char34_51: char = ',';
static char51_68: char = '-';
static char68_85: char = '~';
static char85_102: char = ':';
static char102_119: char = ';';
static char119_136: char = '=';
static char136_153: char = '!';
static char153_170: char = 'o';
static char170_187: char = '*';
static char187_204: char = '#';
static char204_221: char = '%';
static char221_238: char = '@';
static char238_255: char = '$';

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

    pub fn get_char_from_u8(n: u8, invert: bool) -> char {
        if !invert {
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
}
