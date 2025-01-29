use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("image error: {0}")]
    ImageError(#[from] image::error::ImageError),

    #[error("video error: {0}")]
    VideoError(#[from] video_rs::Error),

    #[error("audio error: {0}")]
    AudioError(#[from] symphonia::core::errors::Error),

    #[error("Network error (rocket): {0}")]
    RocketError(#[from] rocket::Error),

    #[error("io error: {0}")]
    IO(#[from] std::io::Error),

    #[error("error: {0}")]
    LibError(&'static str)
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Self::LibError(value)
    }
}