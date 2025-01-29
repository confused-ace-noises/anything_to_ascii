use anything_to_ascii::{
    core::cli::*,
    prelude::{AsciiAudio, AsciiImg, AsciiVid}, read::read::{read_dir_no_parallel, read_dir_parallel},
}; //read::read_video::{read_dir_no_parallel, read_dir_parallel}};
use clap::Parser;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{error::Error, ffi::OsStr, fs, io::Write, path::Path, thread, time};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        // Commands::Api { no_parallel } => {}

        Commands::Image {
            path,
            width,
            height,
            invert,
            savepath,
            colored,
            uniform_char,
            no_parallel,
        } => {
            let name = path;

            let x = if !no_parallel {
                AsciiImg::new_paralleled(name, height, width, invert, !colored, uniform_char)?
            } else {
                AsciiImg::new_sequential(name, height, width, invert, !colored, uniform_char)?
            };

            match savepath {
                Some(path) => fs::write(path, x.to_string())?,
                None => println!("{}", x),
            }
        }

        Commands::Video {
            path,
            width,
            height,
            invert,
            savepath,
            colored,
            uniform_char,
            no_parallel,
            delay_frames,
            n_frames,
        } => {
            let video = if !no_parallel {
                AsciiVid::new_paralleled(
                    path,
                    n_frames,
                    height,
                    width,
                    invert,
                    !colored,
                    uniform_char,
                )?
            } else {
                AsciiVid::new_sequential(
                    path,
                    n_frames,
                    height,
                    width,
                    invert,
                    !colored,
                    uniform_char,
                )?
            };

            match savepath {
                Some(sv_path) => {
                    let name = Path::new(&sv_path)
                        .file_stem()
                        .unwrap_or(OsStr::new("video"))
                        .to_string_lossy()
                        .to_string();
                    let save_path = Path::new(&sv_path);

                    let len = video.0.len();

                    fs::create_dir_all(save_path).expect("failed to write folders");

                    (0..len).into_par_iter().for_each(|index| {
                        let frame_file_name = format!("{}_frame{}.txt", name, index);

                        // Combine save folder path with the frame file name
                        let frame_file_path = save_path.join(frame_file_name);

                        let text = &video.0[index];

                        fs::write(frame_file_path, text.to_string()).expect("failed to write");
                    });
                }

                None => match delay_frames {
                    Some(delay) => play_ascii_frames(video.0.into_iter().map(|img| img.to_string()).collect(), delay as usize),
                    None => play_ascii_frames(video.0.into_iter().map(|img| img.to_string()).collect(), 100),
                },
            }
        }
        Commands::Audio {
            path,
            height,
            invert,
            savepath,
            uniform_char,
            no_parallel,
            media_type,
        } => {
            let waveform = if !no_parallel {
                AsciiAudio::new_parallel(
                    path,
                    media_type,
                    height.unwrap_or(255),
                    uniform_char,
                    invert,
                )?
            } else {
                AsciiAudio::new_sequential(
                    path,
                    media_type,
                    height.unwrap_or(255),
                    uniform_char,
                    invert,
                )?
            };

            let contents = waveform
                .0
                .into_iter_vecs()
                .map(|x| {
                    x.into_iter()
                        .map(|y| y.to_string())
                        .collect::<Vec<_>>()
                        .join("")
                })
                .collect::<Vec<_>>()
                .join("\n");
            match savepath {
                Some(savepath) => fs::write(savepath, contents)?,
                None => println!("{}", contents),
            }
        }

        Commands::Read {
            path,
            no_parallel,
            frame_delay,
        } => {
            let frames = {
                if !no_parallel {
                    read_dir_parallel(path)
                } else {
                    read_dir_no_parallel(path)
                }
            }?;

            play_ascii_frames(frames, frame_delay.unwrap_or(100));
        }
    }

    Ok(())
}

fn play_ascii_frames(frames: Vec<String>, frame_delay: usize) {
    // Clear the terminal
    print!("\x1B[2J");

    for frame in frames {
        // Move cursor to the top-left corner
        print!("\x1B[H");

        // Print the current frame
        println!("{}", frame);

        // Flush stdout to ensure the frame is displayed immediately
        std::io::stdout().flush().unwrap();

        // Wait for the specified delay
        thread::sleep(time::Duration::from_millis(frame_delay as u64));
    }
}
