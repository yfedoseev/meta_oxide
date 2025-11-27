//! Phase 3: JSON-LD / Schema.org extraction (41% adoption)
//!
//! Extracts structured data from <script type="application/ld+json"> tags.
//! Enables Google Rich Results, AI/LLM training data, and rich metadata.

use crate::errors::Result;
use crate::extractors::common::html_utils;
use crate::types::jsonld::JsonLdObject;
use scraper::Selector;

#[cfg(test)]
mod tests;

/// Extract all JSON-LD objects from HTML
///
/// Finds all <script type="application/ld+json"> tags and parses their JSON content.
///
/// # Arguments
/// * `html` - The HTML content
/// * `_base_url` - Optional base URL (not used for JSON-LD)
///
/// # Returns
/// * `Result<Vec<JsonLdObject>>` - All JSON-LD objects found
pub fn extract(html: &str, _base_url: Option<&str>) -> Result<Vec<JsonLdObject>> {
    let document = html_utils::parse_html(html);
    let mut objects = Vec::new();

    // Find all <script type="application/ld+json"> tags
    let selector = match Selector::parse("script[type='application/ld+json']") {
        Ok(s) => s,
        Err(_) => return Ok(objects),
    };

    for script in document.select(&selector) {
        // Get the text content of the script tag
        let json_text: String = script.text().collect();
        let json_text = json_text.trim();

        if json_text.is_empty() {
            continue;
        }

        // Parse JSON
        match serde_json::from_str::<JsonLdObject>(json_text) {
            Ok(obj) => {
                // If object has @graph, extract all items from graph
                if let Some(ref graph) = obj.graph {
                    objects.extend(graph.clone());
                } else {
                    objects.push(obj);
                }
            }
            Err(e) => {
                // Log parse error but continue with other scripts
                eprintln!("JSON-LD parse error: {}", e);
                continue;
            }
        }
    }

    Ok(objects)
}

/// Extract JSON-LD objects of a specific type
///
/// # Arguments
/// * `html` - The HTML content
/// * `type_name` - The @type to filter for (e.g., "Article", "Product")
///
/// # Returns
/// * `Result<Vec<JsonLdObject>>` - Filtered JSON-LD objects
#[allow(dead_code)]
pub fn extract_by_type(html: &str, type_name: &str) -> Result<Vec<JsonLdObject>> {
    let all_objects = extract(html, None)?;

    let filtered: Vec<JsonLdObject> = all_objects
        .into_iter()
        .filter(|obj| {
            if let Some(ref type_) = obj.type_ {
                match type_ {
                    serde_json::Value::String(s) => s == type_name,
                    serde_json::Value::Array(arr) => {
                        arr.iter().any(|v| v.as_str() == Some(type_name))
                    }
                    _ => false,
                }
            } else {
                false
            }
        })
        .collect();

    Ok(filtered)
}
