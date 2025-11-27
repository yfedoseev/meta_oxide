//! Phase 9: Dublin Core Metadata Extractor
//!
//! Extracts Dublin Core metadata elements from HTML meta tags.

use crate::errors::Result;
use crate::extractors::common::html_utils;
use crate::types::dublin_core::DublinCore;

#[cfg(test)]
mod tests;

/// Extract Dublin Core metadata from HTML
///
/// # Arguments
/// * `html` - The HTML content
///
/// # Returns
/// * `Result<DublinCore>` - Extracted Dublin Core metadata or error
pub fn extract(html: &str) -> Result<DublinCore> {
    let document = html_utils::parse_html(html);
    let mut dc = DublinCore::default();

    // Extract Dublin Core meta tags (both DC. and dc. prefixes)
    if let Ok(selector) = html_utils::create_selector("meta[name][content]") {
        for element in document.select(&selector) {
            if let (Some(name), Some(content)) =
                (html_utils::get_attr(&element, "name"), html_utils::get_attr(&element, "content"))
            {
                let content = content.trim().to_string();
                if content.is_empty() {
                    continue;
                }

                // Handle both DC. and dc. prefixes (case-insensitive)
                let name_lower = name.to_lowercase();
                let dc_name = if let Some(stripped) = name_lower.strip_prefix("dc.") {
                    stripped
                } else if let Some(stripped) = name_lower.strip_prefix("dcterms.") {
                    stripped
                } else {
                    continue;
                };

                match dc_name {
                    "title" => dc.title = Some(content),
                    "creator" => dc.creator = Some(content),
                    "subject" => {
                        // Split by comma or semicolon
                        let subjects: Vec<String> = content
                            .split(&[',', ';'][..])
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        dc.subject = Some(subjects);
                    }
                    "description" => dc.description = Some(content),
                    "publisher" => dc.publisher = Some(content),
                    "contributor" => {
                        // Split by comma or semicolon
                        let contributors: Vec<String> = content
                            .split(&[',', ';'][..])
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        dc.contributor = Some(contributors);
                    }
                    "date" => dc.date = Some(content),
                    "type" => dc.type_ = Some(content),
                    "format" => dc.format = Some(content),
                    "identifier" => dc.identifier = Some(content),
                    "source" => dc.source = Some(content),
                    "language" => dc.language = Some(content),
                    "relation" => dc.relation = Some(content),
                    "coverage" => dc.coverage = Some(content),
                    "rights" => dc.rights = Some(content),
                    _ => {}
                }
            }
        }
    }

    Ok(dc)
}
