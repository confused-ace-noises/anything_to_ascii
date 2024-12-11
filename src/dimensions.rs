use std::{fs::File, io::{self, Read}};

pub fn get_png_dimension(path: &str) -> io::Result<(u32, u32)> {
    let mut file = File::open(path)?;
    let mut buffer = [0_u8; 24];
    file.read_exact(&mut buffer)?;

    let png_signature = b"\x89PNG\r\n\x1a\n";
    if &buffer[0..8] != png_signature {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a PNG file"));
    }

    let width = u32::from_be_bytes([buffer[16], buffer[17], buffer[18], buffer[19]]);
    let height = u32::from_be_bytes([buffer[20], buffer[21], buffer[22], buffer[23]]);
    Ok((width, height))
}