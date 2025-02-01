pub mod core;
pub mod utils;
pub mod image;
pub mod video;
pub mod audio;
pub mod read;
pub mod api;

use std::time::{SystemTime, UNIX_EPOCH};

pub use utils::error::Error;

pub mod prelude {
    pub use crate::image::image_to_ascii::AsciiImg;
    pub use crate::video::video_to_ascii::AsciiVid;
    pub use crate::audio::audio_to_ascii::AsciiAudio;
}

#[macro_export]
macro_rules! report {
    ($verbosity:expr, @normal $($message:tt),+) => {
        let time = timestamp();

        match $verbosity {
            Verbosity::Silent => (),
            _ => {
                print!("[NORMAL; {:02}:{:02}:{:02}.{:03}]\t", time.0, time.1, time.2, time.3);
                println!($($message),+)
            }
        }
    };

    ($verbosity:expr, @verbose $($message:tt),+ ) => {
        let time = timestamp();
        
        match $verbosity {
            Verbosity::Verbose => {
                print!("[VERBOSE; {:02}:{:02}:{:02}.{:03}]\t", time.0, time.1, time.2, time.3);
                println!($($message),+)
            },
            _ => ()
        }
    };
}

pub fn timestamp() -> (u64, u64, u64, u128) {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    let total_millis = time.as_millis(); // Get total milliseconds since UNIX_EPOCH
    let total_seconds = time.as_secs();  // Get total seconds since UNIX_EPOCH

    let hours = (total_seconds / 3600) % 24; // Extract hours (mod 24 for local time)
    let minutes = (total_seconds / 60) % 60; // Extract minutes
    let seconds = total_seconds % 60; // Extract seconds
    let milliseconds = total_millis % 1000; // Extract milliseconds

    (hours, minutes, seconds, milliseconds)
}