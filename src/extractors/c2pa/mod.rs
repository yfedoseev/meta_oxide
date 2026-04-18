//! C2PA Content Credentials discovery extractor.
//!
//! See [`crate::types::c2pa`] for the output shape.

use crate::errors::Result;
use crate::extractors::common::{html_utils, url_utils};
use crate::types::c2pa::C2paSurface;
use scraper::Html;

#[cfg(test)]
mod tests;

/// Extract C2PA surface metadata from an HTML string.
pub fn extract(html: &str, base_url: Option<&str>) -> Result<C2paSurface> {
    extract_from_dom(&html_utils::parse_html(html), base_url)
}

/// Extract C2PA surface metadata from an already-parsed DOM.
pub fn extract_from_dom(document: &Html, base_url: Option<&str>) -> Result<C2paSurface> {
    let mut out = C2paSurface::default();

    // 1. meta[name|property="c2pa:*"]
    if let Ok(selector) =
        html_utils::create_selector("meta[name][content], meta[property][content]")
    {
        for element in document.select(&selector) {
            let name = html_utils::get_attr(&element, "name")
                .or_else(|| html_utils::get_attr(&element, "property"));
            let Some(name) = name else { continue };
            let Some(content) = html_utils::get_attr(&element, "content") else { continue };
            let content = content.trim().to_string();
            if content.is_empty() {
                continue;
            }
            let lname = name.to_ascii_lowercase();
            let Some(key) = lname.strip_prefix("c2pa:") else {
                continue;
            };
            match key {
                "claim_generator" | "claim-generator" => out.claim_generator = Some(content),
                "producer" => out.producer = Some(content),
                "signer" | "issuer" => out.signer = Some(content),
                "title" => out.title = Some(content),
                "format" => out.format = Some(content),
                "ai_action" | "ai-action" | "action" => out.ai_action = Some(content),
                "manifest" | "manifest_url" | "manifest-url" => {
                    out.manifest_url = Some(resolve(base_url, &content));
                }
                "thumbnail" | "thumbnail_url" => {
                    out.thumbnail_url = Some(resolve(base_url, &content));
                }
                _ => {
                    out.other.insert(key.to_string(), content);
                }
            }
        }
    }

    // 2. link[rel="c2pa-manifest"] — overrides meta if present.
    if let Ok(selector) = html_utils::create_selector("link[rel][href]") {
        for element in document.select(&selector) {
            let Some(rel) = html_utils::get_attr(&element, "rel") else { continue };
            let Some(href) = html_utils::get_attr(&element, "href") else { continue };
            if rel.split_ascii_whitespace().any(|r| {
                let r = r.to_ascii_lowercase();
                r == "c2pa-manifest" || r == "contentcredentials" || r == "content-credentials"
            }) {
                out.manifest_url = Some(resolve(base_url, &href));
            }
        }
    }

    Ok(out)
}

fn resolve(base_url: Option<&str>, url: &str) -> String {
    url_utils::resolve_url(base_url, url).unwrap_or_else(|_| url.to_string())
}
