use std::{env, fs};
use image::GenericImageView;
use rocket;
use rocket::data::ByteUnit;
use rocket::{post, Data};
use rocket::tokio::io::AsyncReadExt;
use crate::prelude::{AsciiAudio, AsciiImg, AsciiVid};
use crate::utils::utils::Verbosity;
use xxhash_rust::xxh3::xxh3_128;


// pub fn new_parallel(path: String, target_height: Option<u32>, target_width: Option<u32>, invert: bool, grayscale: bool, uniform: bool)

#[post("/api/img_to_ascii?<height>&<width>&<invert>&<colored>&<uniform>", data = "<data>")]
pub async fn api_img_to_ascii_parallel(
    data: Data<'_>,
    height: Option<usize>,
    width: Option<usize>,
    invert: bool,
    colored: bool,
    uniform: bool,
) -> String {
    let mut buffer = Vec::new();

    // Read the raw bytes from the body of the request
    data.open(ByteUnit::Megabyte(10)).read_to_end(&mut buffer).await.unwrap();

    // Log the size of the buffer to check that image data is received
    println!("Received image data: {} bytes", buffer.len());

    // Ensure the buffer contains data
    if buffer.is_empty() {
        return "No image data received.".to_string();
    }

    // Try to load the image from the raw bytes
    let image = match image::load_from_memory(&buffer) {
        Ok(img) => img,
        Err(e) => return format!("error: {}", e),
    };

    // Log the image dimensions
    println!("Image loaded with dimensions: {:?}", image.dimensions());

    // Process the image and return its ASCII representation
    let ascii_image = AsciiImg::new_parallel(image, height, width, invert, !colored, uniform, Verbosity::Normal)
        .unwrap();

    ascii_image.to_string()
}

#[post("/api/img_to_ascii?<height>&<width>&<invert>&<colored>&<uniform>", data = "<data>")]
pub async fn api_img_to_ascii_sequential(
    data: Data<'_>,
    height: Option<usize>,
    width: Option<usize>,
    invert: bool,
    colored: bool,
    uniform: bool,
) -> String {
    let mut buffer = Vec::new();

    // Read the raw bytes from the body of the request
    data.open(ByteUnit::Megabyte(10)).read_to_end(&mut buffer).await.unwrap();

    // Log the size of the buffer to check that image data is received
    println!("Received image data: {} bytes", buffer.len());

    // Ensure the buffer contains data
    if buffer.is_empty() {
        return "No image data received.".to_string();
    }

    // Try to load the image from the raw bytes
    let image = match image::load_from_memory(&buffer) {
        Ok(img) => img,
        Err(e) => return format!("error: {}", e),
    };

    // Log the image dimensions
    println!("Image loaded with dimensions: {:?}", image.dimensions());

    // Process the image and return its ASCII representation
    let ascii_image = AsciiImg::new_sequential(image, height, width, invert, !colored, uniform, Verbosity::Normal)
        .unwrap();

    ascii_image.to_string()
}

#[post("/api/video_to_ascii?<height>&<width>&<nframes>&<invert>&<colored>&<uniform>", data = "<data>")]
pub async fn api_video_to_ascii_parallel(
    data: Data<'_>,
    height: Option<usize>,
    width: Option<usize>,
    nframes: Option<usize>,
    invert: bool,
    colored: bool,
    uniform: bool,
) -> String {
    // Buffer to store video data
    let mut buffer = Vec::new();
    data.open(ByteUnit::Megabyte(100))
        .read_to_end(&mut buffer)
        .await
        .unwrap();

    

    let hash = xxh3_128(&buffer);
    
    // Save the received video data to a temporary file
    let video_path = format!("/tmp/received_video:{}.mp4", hash);
    std::fs::write(&video_path, buffer).unwrap();
    
    
    let vid_ascii = AsciiVid::new_paralleled(&video_path, nframes, height, width, invert, !colored, uniform, Verbosity::Normal);
    
    #[allow(unused_must_use)]
    fs::remove_file(video_path);
    
    let ascii_frames = match vid_ascii {
        Ok(ascii_video) => ascii_video.0.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
        Err(e) => return format!("error: {}", e)
    };

    // Join all frames with a separator
    ascii_frames.join("\n---\n")
}

#[post("/api/video_to_ascii?<height>&<width>&<nframes>&<invert>&<colored>&<uniform>", data = "<data>")]
pub async fn api_video_to_ascii_sequential(
    data: Data<'_>,
    height: Option<usize>,
    width: Option<usize>,
    nframes: Option<usize>,
    invert: bool,
    colored: bool,
    uniform: bool,
) -> String {
    // Buffer to store video data
    let mut buffer = Vec::new();
    data.open(ByteUnit::Megabyte(100))
        .read_to_end(&mut buffer)
        .await
        .unwrap();

    

    let hash = xxh3_128(&buffer);
    
    // Save the received video data to a temporary file
    let temp_dir = env::temp_dir();
    let video_path = temp_dir.join(format!("received_video:{}.mp4", hash)).to_string_lossy().to_string();    
    std::fs::write(&video_path, buffer).unwrap();
    
    
    let vid_ascii = AsciiVid::new_sequential(&video_path, nframes, height, width, invert, !colored, uniform, Verbosity::Normal);

    #[allow(unused_must_use)]
    fs::remove_file(video_path);
    let ascii_frames = match vid_ascii {
        Ok(ascii_video) => ascii_video.0.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
        Err(e) => return format!("error: {}", e)
    };

    // Join all frames with a separator
    ascii_frames.join("\n###\n")
}

//media_type: String,
// max_height: usize,
// uniform: bool,
// invert: bool,

#[post("/api/audio_to_ascii?<height>&<invert>&<uniform>", data = "<data>")]
pub async fn api_audio_to_ascii_parallel(
    data: Data<'_>,
    height: Option<usize>,
    // mediatype: String,
    invert: bool,
    uniform: bool,
) -> String {
    // Buffer to store video data.
    let mut buffer = Vec::new();
    data.open(ByteUnit::Megabyte(100))
        .read_to_end(&mut buffer)
        .await
        .unwrap();

    

    let hash = xxh3_128(&buffer);
    
    // Save the received video data to a temporary file
    let temp_dir = env::temp_dir();
    let audio_path = temp_dir.join(format!("received_audio:{}.mp3", hash)).to_string_lossy().to_string();

    std::fs::write(&audio_path, buffer).unwrap();
    
    let ascii_wave = AsciiAudio::new_parallel(&audio_path, height.unwrap_or(255), uniform, invert, Verbosity::Normal);

    #[allow(unused_must_use)]
    fs::remove_file(audio_path);

    let ascii = match ascii_wave {
        Ok(ascii_wave) => ascii_wave.to_string(),
        Err(e) => format!("error: {}", e),
    };

    ascii
}

#[post("/api/audio_to_ascii?<height>&<invert>&<uniform>", data = "<data>")]
pub async fn api_audio_to_ascii_sequential(
    data: Data<'_>,
    height: Option<usize>,
    invert: bool,
    uniform: bool,
) -> String {
    // Buffer to store video data.
    let mut buffer = Vec::new();
    data.open(ByteUnit::Megabyte(100))
        .read_to_end(&mut buffer)
        .await
        .unwrap();

    

    let hash = xxh3_128(&buffer);
    
    // Save the received video data to a temporary file
    let temp_dir = env::temp_dir();
    let audio_path = temp_dir.join(format!("received_audio:.{}.mp3", hash)).to_string_lossy().to_string();

    std::fs::write(&audio_path, buffer).unwrap();
    
    let ascii_wave = AsciiAudio::new_sequential(&audio_path, height.unwrap_or(255), uniform, invert, Verbosity::Normal);


    #[allow(unused_must_use)]
    fs::remove_file(audio_path);

    let ascii = match ascii_wave {
        Ok(ascii_wave) => ascii_wave.to_string(),
        Err(e) => format!("error: {}", e),
    };

    ascii
}

// apk ...........................................