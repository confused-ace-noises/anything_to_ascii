use std::{error::Error, fs};

use clap::Parser;
use image_to_ascii::{ascii_img, cli::Cli};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let name = cli.path;

    let x = ascii_img::convert(name, cli.vertical, cli.horizontal, cli.invert, false)?;

    match cli.savepath {
        Some(path) => fs::write(path, x)?,
        None => println!("{}", x),
    }

    Ok(())
}

//  '.,-~:;=!o*#%@$