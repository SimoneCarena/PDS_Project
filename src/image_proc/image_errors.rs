use image::ImageError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageManipulationError {
    #[error("IO Error {0}")]
    IOError(#[from] std::io::Error),

    #[error("Image Error {0}")]
    ImageError(#[from] ImageError),

    #[error("Could not copy to clipboard")]
    ClipboardError(#[from] arboard::Error)
}