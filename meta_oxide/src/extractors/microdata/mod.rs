//! Phase 4: HTML5 Microdata extraction (26% adoption)
//!
//! Extracts structured data using itemscope, itemtype, and itemprop attributes
//! with Schema.org vocabulary.

use crate::errors::Result;
use crate::extractors::common::{html_utils, url_utils};
use crate::types::microdata::MicrodataItem;
use scraper::{ElementRef, Selector};

#[cfg(test)]
mod tests;

/// Extract all microdata items from HTML
///
/// Finds all elements with `itemscope` attribute and extracts their properties.
///
/// # Arguments
/// * `html` - The HTML content
/// * `base_url` - Optional base URL for resolving relative URLs
///
/// # Returns
/// * `Result<Vec<MicrodataItem>>` - All microdata items found
pub fn extract(html: &str, base_url: Option<&str>) -> Result<Vec<MicrodataItem>> {
    let document = html_utils::parse_html(html);
    let mut items = Vec::new();

    // Find all top-level itemscope elements (not nested)
    let itemscope_selector = Selector::parse("[itemscope]").unwrap();

    for element in document.select(&itemscope_selector) {
        // Skip if this is a nested itemscope (will be handled as property)
        if !is_top_level_itemscope(&element) {
            continue;
        }

        if let Ok(item) = extract_item(&element, base_url) {
            items.push(item);
        }
    }

    Ok(items)
}

/// Check if an itemscope element is top-level (not nested in another itemscope)
fn is_top_level_itemscope(element: &ElementRef) -> bool {
    // Check if any ancestor has itemscope attribute and this element has itemprop
    // If so, this is a nested item and should not be extracted at top level
    if element.value().attr("itemprop").is_some() {
        // This element has itemprop, check if there's a parent itemscope
        let mut parent = element.parent();
        while let Some(node) = parent {
            if let Some(parent_element) = ElementRef::wrap(node) {
                if parent_element.value().attr("itemscope").is_some() {
                    // Found parent with itemscope, this is nested
                    return false;
                }
            }
            parent = node.parent();
        }
    }
    true
}

/// Extract a single microdata item from an element
fn extract_item(element: &ElementRef, base_url: Option<&str>) -> Result<MicrodataItem> {
    let mut item = MicrodataItem::new();

    // Extract itemtype
    if let Some(itemtype_str) = element.value().attr("itemtype") {
        let types: Vec<String> = itemtype_str.split_whitespace().map(|s| s.to_string()).collect();
        if !types.is_empty() {
            item.item_type = Some(types);
        }
    }

    // Extract itemid
    if let Some(itemid) = element.value().attr("itemid") {
        item.id = Some(itemid.to_string());
    }

    // Extract properties (itemprop children)
    extract_properties(element, &mut item, base_url)?;

    Ok(item)
}

/// Extract all properties from an itemscope element
fn extract_properties(
    scope: &ElementRef,
    item: &mut MicrodataItem,
    base_url: Option<&str>,
) -> Result<()> {
    // Find all descendants with itemprop attribute within this scope
    for descendant in scope.descendants() {
        if let Some(element) = ElementRef::wrap(descendant) {
            if let Some(prop_name) = element.value().attr("itemprop") {
                // Check if this property belongs to this scope or a nested scope
                if belongs_to_scope(scope, &element) {
                    // Extract property value
                    if element.value().attr("itemscope").is_some() {
                        // This is a nested item
                        if let Ok(nested_item) = extract_item(&element, base_url) {
                            item.add_item_property(prop_name.to_string(), nested_item);
                        }
                    } else {
                        // This is a text/URL property
                        if let Some(value) = extract_property_value(&element, base_url) {
                            item.add_text_property(prop_name.to_string(), value);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// Check if an itemprop element belongs to the given scope
fn belongs_to_scope(scope: &ElementRef, prop_element: &ElementRef) -> bool {
    // The property belongs to this scope if there's no intervening itemscope
    // between scope and prop_element

    let scope_id = scope.id();
    let mut current = prop_element.parent();

    while let Some(node) = current {
        if let Some(element) = ElementRef::wrap(node) {
            // If we reached the scope, property belongs to it
            if element.id() == scope_id {
                return true;
            }

            // If we hit another itemscope before reaching our scope,
            // this property belongs to that nested scope instead
            if element.value().attr("itemscope").is_some() && element.id() != scope_id {
                return false;
            }
        }
        current = node.parent();
    }

    false
}

/// Extract the value of a property element
fn extract_property_value(element: &ElementRef, base_url: Option<&str>) -> Option<String> {
    let tag_name = element.value().name();

    // Get value based on element type
    let value = match tag_name {
        // meta and link tags use specific attributes
        "meta" => element.value().attr("content").map(|s| s.to_string()),
        "link" => element.value().attr("href").map(|s| s.to_string()),
        // a, area, audio, embed, iframe, img, source, track, video use URL attributes
        "a" | "area" => element.value().attr("href").map(|s| s.to_string()),
        "audio" | "embed" | "iframe" | "img" | "source" | "track" | "video" => {
            element.value().attr("src").map(|s| s.to_string())
        }
        // object uses data attribute
        "object" => element.value().attr("data").map(|s| s.to_string()),
        // data element uses value attribute
        "data" => element.value().attr("value").map(|s| s.to_string()),
        // meter uses value attribute
        "meter" => element.value().attr("value").map(|s| s.to_string()),
        // time uses datetime attribute if present, otherwise text content
        "time" => {
            if let Some(datetime) = element.value().attr("datetime") {
                Some(datetime.to_string())
            } else {
                let text: String = element.text().collect();
                if !text.trim().is_empty() {
                    Some(text)
                } else {
                    None
                }
            }
        }
        // For all other elements, use text content
        _ => {
            let text: String = element.text().collect();
            // Return empty string instead of None to preserve empty properties
            Some(text)
        }
    }?;

    // Resolve relative URLs if needed
    if is_url_property(tag_name, element) {
        if let Ok(resolved) = url_utils::resolve_url(base_url, &value) {
            return Some(resolved);
        }
    }

    Some(value)
}

/// Check if a property should be treated as a URL
fn is_url_property(tag_name: &str, element: &ElementRef) -> bool {
    matches!(tag_name, "a" | "area" | "link")
        || (matches!(tag_name, "audio" | "embed" | "iframe" | "img" | "source" | "track" | "video")
            && element.value().attr("src").is_some())
        || (tag_name == "object" && element.value().attr("data").is_some())
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_is_url_property() {
        let html = html_utils::parse_html(r#"<a href="test">Link</a>"#);
        let selector = Selector::parse("a").unwrap();
        let element = html.select(&selector).next().unwrap();

        assert!(is_url_property("a", &element));
        assert!(!is_url_property("span", &element));
    }

    #[test]
    fn test_extract_property_value_from_text() {
        let html = html_utils::parse_html(r#"<span>Test Value</span>"#);
        let selector = Selector::parse("span").unwrap();
        let element = html.select(&selector).next().unwrap();

        let value = extract_property_value(&element, None);
        assert_eq!(value, Some("Test Value".to_string()));
    }

    #[test]
    fn test_extract_property_value_from_meta() {
        let html = html_utils::parse_html(r#"<meta content="test">"#);
        let selector = Selector::parse("meta").unwrap();
        let element = html.select(&selector).next().unwrap();

        let value = extract_property_value(&element, None);
        assert_eq!(value, Some("test".to_string()));
    }

    #[test]
    fn test_extract_property_value_from_link() {
        let html = html_utils::parse_html(r#"<link href="https://example.com">"#);
        let selector = Selector::parse("link").unwrap();
        let element = html.select(&selector).next().unwrap();

        let value = extract_property_value(&element, None);
        assert_eq!(value, Some("https://example.com/".to_string()));
    }
}
