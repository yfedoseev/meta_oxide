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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_creation() {
        let err = MicroformatError::ParseError("test error".to_string());
        let err_string = err.to_string();
        assert!(err_string.contains("Failed to parse HTML"));
        assert!(err_string.contains("test error"));
    }

    #[test]
    fn test_parse_error_display() {
        let err = MicroformatError::ParseError("invalid selector".to_string());
        assert_eq!(err.to_string(), "Failed to parse HTML: invalid selector");
    }

    #[test]
    fn test_invalid_url_from_parse_error() {
        let url_err = url::Url::parse("not a url").unwrap_err();
        let mf_err: MicroformatError = url_err.into();
        assert!(matches!(mf_err, MicroformatError::InvalidUrl(_)));
        assert!(mf_err.to_string().contains("Invalid URL"));
    }

    #[test]
    fn test_invalid_url_display() {
        use url::ParseError;
        let err = MicroformatError::InvalidUrl(ParseError::EmptyHost);
        assert!(err.to_string().contains("Invalid URL"));
    }

    #[test]
    fn test_missing_property_error() {
        let err = MicroformatError::MissingProperty("name".to_string());
        let err_string = err.to_string();
        assert!(err_string.contains("Missing required property"));
        assert!(err_string.contains("name"));
    }

    #[test]
    fn test_missing_property_display() {
        let err = MicroformatError::MissingProperty("url".to_string());
        assert_eq!(err.to_string(), "Missing required property: url");
    }

    #[test]
    fn test_invalid_structure_error() {
        let err = MicroformatError::InvalidStructure("bad format".to_string());
        let err_string = err.to_string();
        assert!(err_string.contains("Invalid microformat structure"));
        assert!(err_string.contains("bad format"));
    }

    #[test]
    fn test_extraction_failed_error() {
        let err = MicroformatError::ExtractionFailed("timeout".to_string());
        let err_string = err.to_string();
        assert!(err_string.contains("Extraction failed"));
        assert!(err_string.contains("timeout"));
    }

    #[test]
    fn test_error_is_send_sync() {
        // Verify error can be sent across threads
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        assert_send::<MicroformatError>();
        assert_sync::<MicroformatError>();
    }

    #[test]
    fn test_error_source() {
        let url_err = url::Url::parse("invalid").unwrap_err();
        let mf_err: MicroformatError = url_err.into();

        // Test that the error can be used with error handling libraries
        use std::error::Error;
        let _source: Option<&(dyn Error + 'static)> = mf_err.source();
    }
}
