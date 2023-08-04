use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadFontError{
    #[error("IO Error {0}")]
    IOError(#[from] std::io::Error),

    #[error("Unsupported OS")]
    OSError,

    #[error("Cannot find any font")]
    FontSourceError,

    #[error("Invalid File name")]
    InvalidFileNameError,

    #[error("Font Conversion Error")]
    FontConversionError
}

