//! Common types and utilities for all extractors

use crate::errors::Result;
use std::fmt::Debug;

/// Base trait for all HTML extractors
///
/// This trait defines the common interface that all extractors (microformats,
/// Open Graph, JSON-LD, etc.) must implement.
#[allow(dead_code)]
pub trait Extractor: Debug {
    /// The output type this extractor produces
    type Output: Debug;

    /// Extract data from HTML
    ///
    /// # Arguments
    /// * `html` - The HTML content to extract from
    /// * `base_url` - Optional base URL for resolving relative URLs
    ///
    /// # Returns
    /// * `Result<Self::Output>` - Extracted data or error
    fn extract(html: &str, base_url: Option<&str>) -> Result<Self::Output>;
}

/// Utility functions for URL resolution
pub mod url_utils {
    use url::{ParseError, Url};

    /// Resolve a URL (possibly relative) against a base URL
    pub fn resolve_url(base_url: Option<&str>, url: &str) -> Result<String, ParseError> {
        if let Some(base) = base_url {
            let base_parsed = Url::parse(base)?;
            let resolved = base_parsed.join(url)?;
            Ok(resolved.to_string())
        } else {
            // If no base URL, try parsing as absolute
            let parsed = Url::parse(url)?;
            Ok(parsed.to_string())
        }
    }

    /// Check if a URL is valid
    #[allow(dead_code)]
    pub fn is_valid_url(url: &str) -> bool {
        Url::parse(url).is_ok()
    }
}

/// Utility functions for HTML parsing
pub mod html_utils {
    use crate::errors::{MicroformatError, Result};
    use scraper::{Html, Selector};

    /// Parse HTML and return a document
    pub fn parse_html(html: &str) -> Html {
        Html::parse_document(html)
    }

    /// Create a CSS selector, returning error if invalid
    pub fn create_selector(selector: &str) -> Result<Selector> {
        Selector::parse(selector).map_err(|e| {
            MicroformatError::ParseError(format!("Invalid selector '{}': {:?}", selector, e))
        })
    }

    /// Extract text content from an element, trimming whitespace
    pub fn extract_text(element: &scraper::ElementRef) -> Option<String> {
        let text = element.text().collect::<String>();
        let trimmed = text.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    }

    /// Get attribute value from an element
    pub fn get_attr(element: &scraper::ElementRef, attr: &str) -> Option<String> {
        element.value().attr(attr).map(|s| s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_url_relative() {
        let result = url_utils::resolve_url(Some("https://example.com/page"), "../other");
        assert_eq!(result.unwrap(), "https://example.com/other");
    }

    #[test]
    fn test_resolve_url_absolute() {
        let result = url_utils::resolve_url(Some("https://example.com/"), "https://other.com/");
        assert_eq!(result.unwrap(), "https://other.com/");
    }

    #[test]
    fn test_resolve_url_no_base() {
        let result = url_utils::resolve_url(None, "https://example.com/");
        assert_eq!(result.unwrap(), "https://example.com/");
    }

    #[test]
    fn test_is_valid_url() {
        assert!(url_utils::is_valid_url("https://example.com"));
        assert!(url_utils::is_valid_url("http://example.com/path"));
        assert!(!url_utils::is_valid_url("not a url"));
        assert!(!url_utils::is_valid_url(""));
    }

    #[test]
    fn test_extract_text_with_whitespace() {
        let html = html_utils::parse_html("<p>  Hello World  </p>");
        let selector = html_utils::create_selector("p").unwrap();
        let element = html.select(&selector).next().unwrap();
        assert_eq!(html_utils::extract_text(&element), Some("Hello World".to_string()));
    }

    #[test]
    fn test_extract_text_empty() {
        let html = html_utils::parse_html("<p>   </p>");
        let selector = html_utils::create_selector("p").unwrap();
        let element = html.select(&selector).next().unwrap();
        assert_eq!(html_utils::extract_text(&element), None);
    }

    #[test]
    fn test_get_attr_exists() {
        let html = html_utils::parse_html(r#"<a href="https://example.com">Link</a>"#);
        let selector = html_utils::create_selector("a").unwrap();
        let element = html.select(&selector).next().unwrap();
        assert_eq!(html_utils::get_attr(&element, "href"), Some("https://example.com".to_string()));
    }

    #[test]
    fn test_get_attr_missing() {
        let html = html_utils::parse_html(r#"<a>Link</a>"#);
        let selector = html_utils::create_selector("a").unwrap();
        let element = html.select(&selector).next().unwrap();
        assert_eq!(html_utils::get_attr(&element, "href"), None);
    }

    #[test]
    fn test_resolve_url_invalid_base() {
        let result = url_utils::resolve_url(Some("not-a-url"), "/path");
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_url_relative_without_base() {
        let result = url_utils::resolve_url(None, "/relative/path");
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_url_empty_string() {
        // Test with empty relative URL
        let result = url_utils::resolve_url(Some("https://example.com/"), "");
        // Empty string resolves to base URL
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://example.com/");
    }

    #[test]
    fn test_resolve_url_with_special_characters() {
        let result =
            url_utils::resolve_url(Some("https://example.com"), "/path?query=hello world&foo=bar");
        assert!(result.is_ok());
        let url = result.unwrap();
        assert!(url.contains("example.com"));
    }

    #[test]
    fn test_resolve_url_with_fragment() {
        let result = url_utils::resolve_url(Some("https://example.com"), "/page#section").unwrap();
        assert_eq!(result, "https://example.com/page#section");
    }

    #[test]
    fn test_resolve_url_parent_directory() {
        let result =
            url_utils::resolve_url(Some("https://example.com/foo/bar/"), "../baz").unwrap();
        assert_eq!(result, "https://example.com/foo/baz");
    }

    #[test]
    fn test_is_valid_url_with_whitespace() {
        // URL parser trims whitespace, so test with embedded whitespace
        assert!(!url_utils::is_valid_url("https://example .com"));
        assert!(!url_utils::is_valid_url("ht tps://example.com"));
    }

    #[test]
    fn test_is_valid_url_empty() {
        assert!(!url_utils::is_valid_url(""));
    }

    #[test]
    fn test_create_selector_invalid_syntax() {
        let result = html_utils::create_selector("div[[[invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_create_selector_empty() {
        let result = html_utils::create_selector("");
        assert!(result.is_err());
    }
}
