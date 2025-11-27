//! rel-* link relationships extractor
//!
//! Extracts HTML link relationships using the `rel` attribute.
//! Supports both `<link>` and `<a>` tags.
//!
//! Common rel types:
//! - rel-author: Link to author page/profile
//! - rel-me: Identity consolidation (IndieWeb)
//! - rel-webmention: Webmention endpoint (IndieWeb)
//! - rel-pingback: Pingback endpoint
//! - rel-license: Content license
//! - rel-payment: Payment/donation links
//! - rel-canonical: Canonical URL
//! - rel-alternate: Alternate versions
//! - rel-search: OpenSearch description
//! - rel-nofollow: No follow links
//! - rel-noopener: Security for external links

use crate::errors::Result;
use crate::extractors::common::{html_utils, url_utils};
use std::collections::HashMap;

/// Extract rel-* link relationships from HTML
///
/// Returns a HashMap mapping rel type to array of URLs
///
/// # Arguments
/// * `html` - HTML content to extract from
/// * `base_url` - Optional base URL for resolving relative URLs
///
/// # Returns
/// * `Result<HashMap<String, Vec<String>>>` - Map of rel type to URLs
pub fn extract(html: &str, base_url: Option<&str>) -> Result<HashMap<String, Vec<String>>> {
    let document = html_utils::parse_html(html);
    let mut rel_links: HashMap<String, Vec<String>> = HashMap::new();

    // Find all elements with rel and href attributes (link and a tags)
    let selector = html_utils::create_selector("[rel][href]")?;

    for element in document.select(&selector) {
        if let (Some(rel), Some(href)) =
            (html_utils::get_attr(&element, "rel"), html_utils::get_attr(&element, "href"))
        {
            // Skip empty rel or href
            if rel.trim().is_empty() || href.trim().is_empty() {
                continue;
            }

            // Resolve URL if base_url is provided
            let url = if let Some(base) = base_url {
                match url_utils::resolve_url(Some(base), &href) {
                    Ok(resolved) => resolved,
                    Err(_) => href.clone(), // Fall back to original if resolution fails
                }
            } else {
                href.clone()
            };

            // Handle multiple space-separated rel values
            for rel_value in rel.split_whitespace() {
                if rel_value.is_empty() {
                    continue;
                }
                let rel_type = rel_value.to_lowercase();
                rel_links.entry(rel_type).or_default().push(url.clone());
            }
        }
    }

    Ok(rel_links)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_rel_link() {
        let html = r#"<link rel="author" href="/about">"#;
        let links = extract(html, None).unwrap();
        assert_eq!(links.get("author"), Some(&vec!["/about".to_string()]));
    }

    #[test]
    fn test_multiple_rel_types() {
        let html = r#"
        <link rel="author" href="/about">
        <link rel="license" href="https://creativecommons.org/licenses/by/4.0/">
        "#;
        let links = extract(html, None).unwrap();
        assert!(links.contains_key("author"));
        assert!(links.contains_key("license"));
    }

    #[test]
    fn test_space_separated_rel() {
        let html = r#"<a rel="me noopener" href="https://twitter.com/user">Twitter</a>"#;
        let links = extract(html, None).unwrap();
        assert_eq!(links.get("me"), Some(&vec!["https://twitter.com/user".to_string()]));
        assert_eq!(links.get("noopener"), Some(&vec!["https://twitter.com/user".to_string()]));
    }

    #[test]
    fn test_url_resolution() {
        let html = r#"<link rel="author" href="/about">"#;
        let links = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(links.get("author"), Some(&vec!["https://example.com/about".to_string()]));
    }

    #[test]
    fn test_case_normalization() {
        let html = r#"<link rel="Author" href="/about">"#;
        let links = extract(html, None).unwrap();
        assert!(links.contains_key("author"));
        assert!(!links.contains_key("Author"));
    }

    #[test]
    fn test_missing_href() {
        let html = r#"<link rel="author">"#;
        let links = extract(html, None).unwrap();
        assert!(!links.contains_key("author"));
    }

    #[test]
    fn test_empty_rel() {
        let html = r#"<link rel="" href="/page">"#;
        let links = extract(html, None).unwrap();
        assert!(links.is_empty());
    }

    #[test]
    fn test_whitespace_only_rel() {
        let html = r#"<link rel="   " href="/page">"#;
        let links = extract(html, None).unwrap();
        assert!(links.is_empty());
    }

    #[test]
    fn test_multiple_same_rel() {
        let html = r#"
        <link rel="me" href="https://twitter.com/user">
        <link rel="me" href="https://github.com/user">
        "#;
        let links = extract(html, None).unwrap();
        assert_eq!(links.get("me").unwrap().len(), 2);
        assert!(links.get("me").unwrap().contains(&"https://twitter.com/user".to_string()));
        assert!(links.get("me").unwrap().contains(&"https://github.com/user".to_string()));
    }
}
