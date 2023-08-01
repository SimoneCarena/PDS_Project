use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScreenshotError {
    #[error("Screen Capture Error")]
    ScreenCaptureError,

    #[error("Screen Retrival Error")]
    ScreenRetvError,

    #[error("Image Processing Error")]
    ImageProcessError,

    #[error("IO Error {0}")]
    IOError(#[from] std::io::Error)

}
