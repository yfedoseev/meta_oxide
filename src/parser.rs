use crate::errors::{MicroformatError, Result};
use crate::types::{MicroformatItem, PropertyValue};
use scraper::{Html, Selector};
use std::collections::HashMap;

/// Parse HTML and extract all microformats
pub fn parse_html(
    html: &str,
    base_url: Option<&str>,
) -> Result<HashMap<String, Vec<MicroformatItem>>> {
    let document = Html::parse_document(html);
    let mut results: HashMap<String, Vec<MicroformatItem>> = HashMap::new();

    // Find all elements with microformat classes (h-*, p-*, u-*, dt-*, e-*)
    let mf_selector = Selector::parse("[class*='h-']")
        .map_err(|e| MicroformatError::ParseError(e.to_string()))?;

    for element in document.select(&mf_selector) {
        if let Some(classes) = element.value().attr("class") {
            // Check for root microformat classes (h-*)
            let h_classes: Vec<&str> = classes
                .split_whitespace()
                .filter(|c| c.starts_with("h-"))
                .collect();

            if !h_classes.is_empty() {
                let item = parse_microformat_item(&element, base_url)?;

                for h_class in h_classes {
                    results
                        .entry(h_class.to_string())
                        .or_insert_with(Vec::new)
                        .push(item.clone());
                }
            }
        }
    }

    Ok(results)
}

/// Parse a single microformat item
fn parse_microformat_item(
    element: &scraper::ElementRef,
    base_url: Option<&str>,
) -> Result<MicroformatItem> {
    let mut properties: HashMap<String, Vec<PropertyValue>> = HashMap::new();
    let mut type_classes = Vec::new();

    // Extract type classes (h-*)
    if let Some(classes) = element.value().attr("class") {
        type_classes = classes
            .split_whitespace()
            .filter(|c| c.starts_with("h-"))
            .map(String::from)
            .collect();
    }

    // Extract properties
    extract_properties(element, &mut properties, base_url)?;

    Ok(MicroformatItem {
        type_: type_classes,
        properties,
        children: None,
    })
}

/// Extract properties from a microformat element
fn extract_properties(
    element: &scraper::ElementRef,
    properties: &mut HashMap<String, Vec<PropertyValue>>,
    base_url: Option<&str>,
) -> Result<()> {
    // Find all property elements (p-*, u-*, dt-*, e-*)
    for child in element.descendants() {
        if let Some(child_element) = scraper::ElementRef::wrap(child) {
            if let Some(classes) = child_element.value().attr("class") {
                for class in classes.split_whitespace() {
                    let (prefix, name) = if let Some(name) = class.strip_prefix("p-") {
                        ("p", name)
                    } else if let Some(name) = class.strip_prefix("u-") {
                        ("u", name)
                    } else if let Some(name) = class.strip_prefix("dt-") {
                        ("dt", name)
                    } else if let Some(name) = class.strip_prefix("e-") {
                        ("e", name)
                    } else {
                        continue;
                    };

                    let value = extract_property_value(&child_element, prefix, base_url)?;
                    properties
                        .entry(name.to_string())
                        .or_insert_with(Vec::new)
                        .push(value);
                }
            }
        }
    }

    Ok(())
}

/// Extract a property value based on its type
fn extract_property_value(
    element: &scraper::ElementRef,
    prefix: &str,
    base_url: Option<&str>,
) -> Result<PropertyValue> {
    match prefix {
        "p" => {
            // Plain text
            let text = element.text().collect::<String>().trim().to_string();
            Ok(PropertyValue::Text(text))
        }
        "u" => {
            // URL
            let url = element
                .value()
                .attr("href")
                .or_else(|| element.value().attr("src"))
                .unwrap_or_else(|| element.text().collect::<String>().trim());

            let absolute_url = if let Some(base) = base_url {
                resolve_url(base, url)?
            } else {
                url.to_string()
            };

            Ok(PropertyValue::Url(absolute_url))
        }
        "dt" => {
            // DateTime
            let datetime = element
                .value()
                .attr("datetime")
                .or_else(|| element.value().attr("value"))
                .unwrap_or_else(|| element.text().collect::<String>().trim());

            Ok(PropertyValue::Text(datetime.to_string()))
        }
        "e" => {
            // Embedded HTML
            let html = element.inner_html();
            Ok(PropertyValue::Text(html))
        }
        _ => Ok(PropertyValue::Text(String::new())),
    }
}

/// Resolve a relative URL against a base URL
fn resolve_url(base: &str, relative: &str) -> Result<String> {
    let base_url = url::Url::parse(base)?;
    let resolved = base_url.join(relative)?;
    Ok(resolved.to_string())
}
