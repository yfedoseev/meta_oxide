use thiserror::Error;

#[derive(Error, Debug)]
pub enum MicroformatError {
    #[error("Failed to parse HTML: {0}")]
    ParseError(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Missing required property: {0}")]
    MissingProperty(String),

    #[error("Invalid microformat structure: {0}")]
    InvalidStructure(String),

    #[error("Extraction failed: {0}")]
    ExtractionFailed(String),
}

pub type Result<T> = std::result::Result<T, MicroformatError>;
