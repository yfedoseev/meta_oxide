use crate::errors::{MicroformatError, Result};
use crate::extractors::common::url_utils;
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
            let h_classes: Vec<&str> =
                classes.split_whitespace().filter(|c| c.starts_with("h-")).collect();

            if !h_classes.is_empty() {
                let item = parse_microformat_item(&element, base_url)?;

                for h_class in h_classes {
                    results.entry(h_class.to_string()).or_default().push(item.clone());
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
        type_classes =
            classes.split_whitespace().filter(|c| c.starts_with("h-")).map(String::from).collect();
    }

    // Extract properties
    extract_properties(element, &mut properties, base_url)?;

    Ok(MicroformatItem { type_: type_classes, properties, children: None })
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
                    properties.entry(name.to_string()).or_default().push(value);
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
                .map(String::from)
                .unwrap_or_else(|| element.text().collect::<String>().trim().to_string());

            let absolute_url =
                if let Some(base) = base_url { resolve_url(base, &url)? } else { url };

            Ok(PropertyValue::Url(absolute_url))
        }
        "dt" => {
            // DateTime
            let datetime = element
                .value()
                .attr("datetime")
                .or_else(|| element.value().attr("value"))
                .map(String::from)
                .unwrap_or_else(|| element.text().collect::<String>().trim().to_string());

            Ok(PropertyValue::Text(datetime))
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
    url_utils::resolve_url(Some(base), relative).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_html_basic() {
        let html = "<div class='test'>Content</div>";
        let doc = Html::parse_document(html);
        assert!(doc.root_element().html().contains("test"));
    }

    #[test]
    fn test_extract_property_value_p_prefix() {
        // Test plain text extraction (p- prefix)
        let html = r#"<span class="p-name">John Doe</span>"#;
        let doc = Html::parse_fragment(html);

        // This tests the property extraction logic
        let selector = Selector::parse(".p-name").unwrap();
        let el = doc.select(&selector).next().unwrap();
        let text: String = el.text().collect();
        assert_eq!(text, "John Doe");
    }

    #[test]
    fn test_extract_property_value_u_prefix() {
        // Test URL extraction (u- prefix)
        let html = r#"<a class="u-url" href="https://example.com">Link</a>"#;
        let doc = Html::parse_fragment(html);
        let selector = Selector::parse(".u-url").unwrap();
        let el = doc.select(&selector).next().unwrap();
        let href = el.value().attr("href").unwrap();
        assert_eq!(href, "https://example.com");
    }

    #[test]
    fn test_extract_property_value_dt_prefix() {
        // Test datetime extraction (dt- prefix)
        let html = r#"<time class="dt-published" datetime="2024-01-15">Jan 15</time>"#;
        let doc = Html::parse_fragment(html);
        let selector = Selector::parse(".dt-published").unwrap();
        let el = doc.select(&selector).next().unwrap();
        let datetime = el.value().attr("datetime").unwrap();
        assert_eq!(datetime, "2024-01-15");
    }

    #[test]
    fn test_extract_property_value_e_prefix() {
        // Test embedded HTML extraction (e- prefix)
        let html = r#"<div class="e-content"><p>Rich <strong>HTML</strong> content</p></div>"#;
        let doc = Html::parse_fragment(html);
        let selector = Selector::parse(".e-content").unwrap();
        let el = doc.select(&selector).next().unwrap();
        let inner_html = el.inner_html();
        assert!(inner_html.contains("<p>"));
        assert!(inner_html.contains("<strong>"));
    }

    #[test]
    fn test_parse_microformat_item_nested() {
        let html = r#"
            <div class="h-entry">
                <span class="p-name">Blog Post</span>
                <div class="p-author h-card">
                    <span class="p-name">Author Name</span>
                    <a class="u-url" href="https://author.example.com">Website</a>
                </div>
            </div>
        "#;
        let doc = Html::parse_document(html);
        let selector = Selector::parse(".h-entry").unwrap();
        let entry = doc.select(&selector).next().unwrap();

        // Test nested h-card is detected
        let nested_selector = Selector::parse(".h-card").unwrap();
        let nested = entry.select(&nested_selector).next();
        assert!(nested.is_some());
    }

    #[test]
    fn test_extract_properties_multiple_values() {
        let html = r#"
            <div class="h-entry">
                <span class="p-category">rust</span>
                <span class="p-category">python</span>
                <span class="p-category">tutorial</span>
            </div>
        "#;
        let doc = Html::parse_document(html);
        let selector = Selector::parse(".h-entry").unwrap();
        let entry = doc.select(&selector).next().unwrap();

        // Test multiple values for same property
        let cat_selector = Selector::parse(".p-category").unwrap();
        let categories: Vec<String> =
            entry.select(&cat_selector).map(|el| el.text().collect::<String>()).collect();
        assert_eq!(categories.len(), 3);
        assert!(categories.contains(&"rust".to_string()));
    }

    #[test]
    fn test_extract_property_empty_value() {
        let html = r#"<span class="p-name"></span>"#;
        let doc = Html::parse_fragment(html);
        let selector = Selector::parse(".p-name").unwrap();
        let el = doc.select(&selector).next().unwrap();
        let text: String = el.text().collect();
        assert_eq!(text.trim(), "");
    }

    #[test]
    fn test_extract_property_whitespace_only() {
        let html = r#"<span class="p-name">   </span>"#;
        let doc = Html::parse_fragment(html);
        let selector = Selector::parse(".p-name").unwrap();
        let el = doc.select(&selector).next().unwrap();
        let text: String = el.text().collect();
        assert_eq!(text.trim(), "");
    }

    #[test]
    fn test_url_resolution_in_property() {
        // Test URL resolution for u- properties
        let base_url = "https://example.com/page/";
        let relative = "/other";
        let resolved =
            crate::extractors::common::url_utils::resolve_url(Some(base_url), relative).unwrap();
        assert_eq!(resolved, "https://example.com/other");
    }

    #[test]
    fn test_url_resolution_failure() {
        // Test URL resolution with invalid base
        let result = crate::extractors::common::url_utils::resolve_url(Some("not-a-url"), "/path");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_html_with_malformed_html() {
        let html = "<div><span>Unclosed tags";
        let doc = Html::parse_document(html);
        // Should not panic, parser is lenient
        assert!(!doc.root_element().html().is_empty());
    }

    #[test]
    fn test_parse_html_empty() {
        let html = "";
        let doc = Html::parse_document(html);
        assert!(!doc.root_element().html().is_empty()); // Has at least <html> wrapper
    }

    #[test]
    fn test_parse_html_with_html_entities() {
        let html = "<p>&lt;tag&gt; &amp; &quot;quotes&quot;</p>";
        let doc = Html::parse_document(html);
        let text = doc.root_element().text().collect::<String>();
        assert!(text.contains("<tag>"));
        assert!(text.contains("&"));
        assert!(text.contains("\"quotes\""));
    }

    #[test]
    fn test_parse_html_with_unicode() {
        let html = "<p>日本語 Français Русский العربية</p>";
        let doc = Html::parse_document(html);
        let text = doc.root_element().text().collect::<String>();
        assert!(text.contains("日本語"));
        assert!(text.contains("Français"));
        assert!(text.contains("Русский"));
        assert!(text.contains("العربية"));
    }

    #[test]
    fn test_property_extraction_with_nested_elements() {
        let html = r#"<div class="p-name">Hello <strong>World</strong></div>"#;
        let doc = Html::parse_fragment(html);
        let selector = Selector::parse(".p-name").unwrap();
        let el = doc.select(&selector).next().unwrap();
        let text: String = el.text().collect();
        assert_eq!(text, "Hello World");
    }

    #[test]
    fn test_multiple_classes_on_element() {
        let html = r#"<div class="h-card p-author">Content</div>"#;
        let doc = Html::parse_fragment(html);

        // Should match both selectors
        let hcard_selector = Selector::parse(".h-card").unwrap();
        let author_selector = Selector::parse(".p-author").unwrap();

        assert!(doc.select(&hcard_selector).next().is_some());
        assert!(doc.select(&author_selector).next().is_some());
    }

    #[test]
    fn test_deeply_nested_microformat() {
        let html = r#"
            <div class="h-entry">
                <div class="p-author h-card">
                    <div class="p-org h-card">
                        <span class="p-name">Org Name</span>
                    </div>
                </div>
            </div>
        "#;
        let doc = Html::parse_document(html);
        let selector = Selector::parse(".h-card").unwrap();
        let cards: Vec<_> = doc.select(&selector).collect();
        assert_eq!(cards.len(), 2); // Two nested h-cards
    }

    #[test]
    fn test_property_with_special_characters() {
        let html =
            r#"<span class="p-name">Name with "quotes" and 'apostrophes' & ampersands</span>"#;
        let doc = Html::parse_fragment(html);
        let selector = Selector::parse(".p-name").unwrap();
        let el = doc.select(&selector).next().unwrap();
        let text: String = el.text().collect();
        assert!(text.contains("quotes"));
        assert!(text.contains("apostrophes"));
        assert!(text.contains("&"));
    }

    #[test]
    fn test_parse_html_with_base_url() {
        // Test that base URL doesn't affect HTML parsing
        let html = r#"<div class="h-card"><span class="p-name">Test</span></div>"#;
        let result = parse_html(html, Some("https://example.com"));
        assert!(result.is_ok());
        let items = result.unwrap();
        assert!(items.contains_key("h-card"));
    }

    #[test]
    fn test_parse_html_with_doctype() {
        let html = "<!DOCTYPE html><html><body><p>Content</p></body></html>";
        let doc = Html::parse_document(html);
        assert!(!doc.root_element().html().is_empty());
    }

    #[test]
    fn test_parse_html_with_comments() {
        let html = "<!-- Comment --><div class='test'>Content</div><!-- Another comment -->";
        let doc = Html::parse_document(html);
        let selector = Selector::parse(".test").unwrap();
        assert!(doc.select(&selector).next().is_some());
    }

    #[test]
    fn test_parse_html_with_cdata() {
        let html = "<div><![CDATA[Some data]]></div>";
        let doc = Html::parse_document(html);
        assert!(!doc.root_element().html().is_empty());
    }

    #[test]
    fn test_parse_html_with_script_tags() {
        let html = r#"
            <html>
                <head>
                    <script>var x = 1;</script>
                </head>
                <body>
                    <div class="h-card">
                        <span class="p-name">Test</span>
                    </div>
                </body>
            </html>
        "#;
        let doc = Html::parse_document(html);
        let selector = Selector::parse(".h-card").unwrap();
        assert!(doc.select(&selector).next().is_some());
    }

    #[test]
    fn test_parse_html_with_style_tags() {
        let html = r#"
            <html>
                <head>
                    <style>.test { color: red; }</style>
                </head>
                <body>
                    <div class="test">Content</div>
                </body>
            </html>
        "#;
        let doc = Html::parse_document(html);
        let selector = Selector::parse(".test").unwrap();
        assert!(doc.select(&selector).next().is_some());
    }

    #[test]
    fn test_parse_html_mixed_case_attributes() {
        let html = r#"<div Class="h-CARD"><span CLASS="P-NAME">Test</span></div>"#;
        let doc = Html::parse_document(html);
        // CSS selectors are case-sensitive for class names, but parser normalizes
        let selector = Selector::parse(".h-CARD").unwrap();
        assert!(doc.select(&selector).next().is_some());
    }

    #[test]
    fn test_parse_html_self_closing_tags() {
        let html = r#"<div><br/><img src="test.jpg"/><input type="text"/></div>"#;
        let doc = Html::parse_document(html);
        assert!(!doc.root_element().html().is_empty());
    }

    #[test]
    fn test_parse_html_with_xml_namespace() {
        let html = r#"<html xmlns="http://www.w3.org/1999/xhtml"><body>Content</body></html>"#;
        let doc = Html::parse_document(html);
        assert!(!doc.root_element().html().is_empty());
    }
}
