//! Phase 5: oEmbed Endpoint Discovery
//!
//! Extracts oEmbed endpoint URLs from HTML link tags.
//! oEmbed is used by platforms like YouTube, Vimeo, Twitter, etc.

use crate::errors::Result;
use crate::extractors::common::{html_utils, url_utils};
use crate::types::oembed::{OEmbedDiscovery, OEmbedEndpoint, OEmbedFormat};

#[cfg(test)]
mod tests;

/// Discover oEmbed endpoints from HTML
///
/// # Arguments
/// * `html` - The HTML content
/// * `base_url` - Optional base URL for resolving relative URLs
///
/// # Returns
/// * `Result<OEmbedDiscovery>` - Discovered oEmbed endpoints or error
pub fn extract(html: &str, base_url: Option<&str>) -> Result<OEmbedDiscovery> {
    let document = html_utils::parse_html(html);
    let mut discovery = OEmbedDiscovery::default();

    // Look for link tags with rel="alternate" and type containing "oembed"
    if let Ok(selector) = html_utils::create_selector("link[rel~=\"alternate\"][type][href]") {
        for element in document.select(&selector) {
            if let (Some(link_type), Some(href)) =
                (html_utils::get_attr(&element, "type"), html_utils::get_attr(&element, "href"))
            {
                // Skip empty href attributes
                if href.trim().is_empty() {
                    continue;
                }

                let resolved_href = url_utils::resolve_url(base_url, &href).unwrap_or(href.clone());
                let title = html_utils::get_attr(&element, "title");

                // Check for oEmbed types
                let link_type_lower = link_type.to_lowercase();
                if link_type_lower.contains("oembed") {
                    let endpoint = OEmbedEndpoint {
                        href: resolved_href,
                        format: if link_type_lower.contains("json") {
                            OEmbedFormat::Json
                        } else if link_type_lower.contains("xml") {
                            OEmbedFormat::Xml
                        } else {
                            // Default to JSON if ambiguous
                            OEmbedFormat::Json
                        },
                        title,
                    };

                    match endpoint.format {
                        OEmbedFormat::Json => discovery.json_endpoints.push(endpoint),
                        OEmbedFormat::Xml => discovery.xml_endpoints.push(endpoint),
                    }
                }
            }
        }
    }

    Ok(discovery)
}
