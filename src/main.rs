use anything_to_ascii::api::api::*;
use anything_to_ascii::report;
use anything_to_ascii::utils::utils::Verbosity;
use anything_to_ascii::{
    core::cli::*,
    prelude::{AsciiAudio, AsciiImg, AsciiVid},
    read::read::*,
}; //read::read_video::{read_dir_no_parallel, read_dir_parallel}};
use clap::Parser;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rocket::{routes, tokio, Config};
use std::{error::Error, ffi::OsStr, fs, io::Write, path::Path, thread, time};
use anything_to_ascii::timestamp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    report!(Verbosity::Normal, @normal "The program started fine. Depending on the file size, you may need to wait quite a lot of time for it to load.");
    let cli = Cli::parse();
    let verbosity = match (cli.silent, cli.verbose) {
        (true, false) => Verbosity::Silent,
        (false, true) => Verbosity::Verbose, 
        (false, false) => Verbosity::Normal,
        (true, true) => panic!("...please message the creator, because something *very* weird just happened"),
    };
    match cli.command {
        Commands::Api { no_parallel, port } => {
            let _ = build_rocket(no_parallel, port).launch().await;
        }

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
                AsciiImg::new_parallel_file(name, height, width, invert, !colored, uniform_char, verbosity)?
            } else {
                AsciiImg::new_sequential_file(name, height, width, invert, !colored, uniform_char, verbosity)?
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
                    &path,
                    n_frames,
                    height,
                    width,
                    invert,
                    !colored,
                    uniform_char,
                    verbosity
                )?
            } else {
                AsciiVid::new_sequential(
                    &path,
                    n_frames,
                    height,
                    width,
                    invert,
                    !colored,
                    uniform_char,
                    verbosity
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
                    Some(delay) => play_ascii_frames(
                        video.0.into_iter().map(|img| img.to_string()).collect(),
                        delay as usize,
                    ),
                    None => play_ascii_frames(
                        video.0.into_iter().map(|img| img.to_string()).collect(),
                        100,
                    ),
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
        } => {
            let waveform = if !no_parallel {
                AsciiAudio::new_parallel(
                    &path,
                    height.unwrap_or(255),
                    uniform_char,
                    invert,
                    verbosity,
                )?
            } else {
                AsciiAudio::new_sequential(
                    &path,
                    height.unwrap_or(255),
                    uniform_char,
                    invert,
                    verbosity
                )?
            };

            let contents = waveform.to_string();
                
            match savepath {
                Some(savepath) => fs::write(savepath, contents)?,
                None => println!("{}", contents),
            }
        }

        Commands::Read {
            path,
            no_parallel,
            frame_delay,
            read_api_output,
            read_api_output_to_dir,
        } => {
            

            let frames = {
                if read_api_output {
                    read_video_from_api_file(&path)
                } else if let Some(dir_name) = read_api_output_to_dir {
                    let frames = read_video_from_api_file(&path)?;
                    
                    let general_name = Path::new(&path).file_stem().unwrap_or(OsStr::new("video")).to_string_lossy().to_string();
                    for frame in frames.into_iter().enumerate() {
                        let frame_name = format!("{}_frame{}.txt", general_name, frame.0);

                        let savepath = Path::new(&dir_name).join(frame_name);

                        fs::write(savepath, frame.1)?;
                    }

                    return Ok(());
                } else {if !no_parallel {
                    read_dir_parallel(path)
                } else {
                    read_dir_no_parallel(path)
                }}
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

fn build_rocket(no_parallel: bool, port: Option<u16>) -> rocket::Rocket<rocket::Build> {
    let config = Config {
        port: port.unwrap_or(8000),
        ..Default::default()
    };
    
    if !no_parallel {
        rocket::custom(config).mount("/", routes![api_img_to_ascii_parallel, api_video_to_ascii_parallel, api_audio_to_ascii_parallel])
    } else {
        rocket::custom(config).mount("/", routes![api_img_to_ascii_sequential, api_video_to_ascii_sequential, api_audio_to_ascii_sequential])
    }
}
