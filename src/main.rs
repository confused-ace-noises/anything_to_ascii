use std::{error::Error, ffi::OsStr, fs, io::Write, path::Path, thread, time};

use clap::Parser;
use image_to_ascii::{ascii_img, core::cli::*, from_audio::ascii_waveform::convert_audio, from_video::ascii_video::convert_video, read::read_video::{read_dir_no_parallel, read_dir_parallel}};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
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

            let x = ascii_img::convert(
                name,
                height,
                width,
                invert,
                !colored,
                uniform_char,
                !no_parallel,
            )?;

            match savepath {
                Some(path) => fs::write(path, x)?,
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
            let video = convert_video(
                path,
                n_frames.and_then(|k| Some(k as u32)),
                height,
                width,
                invert,
                !colored,
                uniform_char,
                !no_parallel,
            )?;

            match savepath {
                Some(sv_path) => {
                    let name = Path::new(&sv_path)
                        .file_stem()
                        .unwrap_or(OsStr::new("video"))
                        .to_string_lossy()
                        .to_string();
                    let save_path = Path::new(&sv_path);

                    let len = video.len();

                    fs::create_dir_all(save_path).expect("failed to write folders");

                    (0..len).into_par_iter().for_each(|index| {
                        let frame_file_name = format!("{}_frame{}.txt", name, index);

                        // Combine save folder path with the frame file name
                        let frame_file_path = save_path.join(frame_file_name);

                        let text = &video[index];

                        fs::write(frame_file_path, text)
                            .expect("failed to write");
                    });
                }
                None => {
                    match delay_frames {
                        Some(delay) => play_ascii_frames(video, delay as usize),
                        None => play_ascii_frames(video, 100),
                    }
                },
            }
        }
        Commands::Audio { path, height, invert, savepath, uniform_char, no_parallel, media_type } => {
            let waveform = convert_audio(path, media_type, height.unwrap_or(255), uniform_char, invert, !no_parallel)?;
            match savepath {
                Some(savepath) => {
                    fs::write(savepath, waveform)?
                },
                None => println!("{}", waveform),
            }
        },

        Commands::Read { path, no_parallel, frame_delay} => {
            let frames = {if !no_parallel {
                read_dir_parallel(path)
            } else {
                read_dir_no_parallel(path)
            }}?;

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