pub mod core;
pub mod utils;
pub mod image;
pub mod video;
pub mod audio;
pub mod read;

pub use utils::error::Error;

pub mod prelude {
    pub use crate::image::image_to_ascii::AsciiImg;
    pub use crate::video::video_to_ascii::AsciiVid;
    pub use crate::audio::audio_to_ascii::AsciiAudio;
}