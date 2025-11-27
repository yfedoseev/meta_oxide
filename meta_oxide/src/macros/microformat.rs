//! Declarative macro for generating microformat extractors
//!
//! This macro eliminates ~1,500 lines of duplicated extraction code by providing
//! a declarative syntax for defining microformat extractors.
//!
//! # Example
//!
//! ```ignore
//! microformat_extractor! {
//!     HCard, ".h-card" {
//!         name: text(".p-name"),
//!         email: url(".u-email"),
//!         photo: url(".u-photo"),
//!         tel: text(".p-tel"),
//!         org: text(".p-org"),
//!         note: text(".p-note"),
//!     }
//! }
//! ```
//!
//! This expands to a complete `extract()` function with proper error handling,
//! HTML parsing, and property extraction.

/// Generate a microformat extractor function
///
/// # Syntax
///
/// ```ignore
/// microformat_extractor! {
///     TypeName, "root-selector" {
///         field_name: property_type("css-selector"),
///         ...
///     }
/// }
/// ```
///
/// # Property Types
///
/// - `text(selector)` - Extract text content → `Option<String>`
/// - `url(selector)` - Extract URL from href/src attribute → `Option<String>`
/// - `html(selector)` - Extract inner HTML content → `Option<String>`
/// - `date(selector)` - Extract datetime attribute or text → `Option<String>`
/// - `multi_text(selector)` - Extract multiple text values → `Vec<String>`
/// - `multi_url(selector)` - Extract multiple URLs → `Vec<String>`
/// - `number(selector)` - Parse as f32 → `Option<f32>`
/// - `f64_number(selector)` - Parse as f64 → `Option<f64>`
///
/// # Generated Code
///
/// The macro generates a function with this signature:
/// ```ignore
/// pub fn extract(html: &str, base_url: Option<&str>) -> Result<Vec<TypeName>>
/// ```
#[macro_export]
macro_rules! microformat_extractor {
    // Main entry point: TypeName, root_selector { field: type(selector), ... }
    (
        $type_name:ty, $root_selector:literal {
            $(
                $field:ident : $prop_type:ident($selector:literal)
            ),* $(,)?
        }
    ) => {
        #[allow(unused_variables)]
        pub fn extract(html: &str, base_url: Option<&str>) -> $crate::Result<Vec<$type_name>> {
            use $crate::html_utils;

            let document = html_utils::parse_html(html);
            let mut items = Vec::new();

            let root_selector = html_utils::create_selector($root_selector)?;

            for element in document.select(&root_selector) {
                let mut item = <$type_name>::default();

                $(
                    microformat_extractor!(@extract_property
                        element,
                        item,
                        $field,
                        $prop_type,
                        $selector,
                        base_url
                    );
                )*

                items.push(item);
            }

            Ok(items)
        }
    };

    // Entry point with dual-field support for complex extractors
    (
        $type_name:ty, $root_selector:literal {
            $(
                $field:ident : $prop_type:ident($($selector:literal),+)
            ),* $(,)?
            ;
            $(
                ($text_field:ident, $nested_field:ident) : $dual_prop_type:ident($nested_sel:literal, $text_sel:literal)
            ),* $(,)?
        }
    ) => {
        #[allow(unused_variables)]
        pub fn extract(html: &str, base_url: Option<&str>) -> $crate::Result<Vec<$type_name>> {
            use $crate::html_utils;

            let document = html_utils::parse_html(html);
            let mut items = Vec::new();

            let root_selector = html_utils::create_selector($root_selector)?;

            for element in document.select(&root_selector) {
                let mut item = <$type_name>::default();

                // Extract regular properties
                $(
                    microformat_extractor!(@extract_property
                        element,
                        item,
                        $field,
                        $prop_type,
                        $($selector),+,
                        base_url
                    );
                )*

                // Extract dual-field properties
                $(
                    microformat_extractor!(@extract_dual_property
                        element,
                        item,
                        $text_field,
                        $nested_field,
                        $dual_prop_type,
                        $nested_sel,
                        $text_sel,
                        base_url
                    );
                )*

                items.push(item);
            }

            Ok(items)
        }
    };

    // Extract a single text property
    (@extract_property $element:ident, $item:ident, $field:ident, text, $selector:expr, $base_url:ident) => {
        if let Ok(sel) = $crate::html_utils::create_selector($selector) {
            if let Some(elem) = $element.select(&sel).next() {
                $item.$field = $crate::html_utils::extract_text(&elem);
            }
        }
    };

    // Extract a URL property (from href or src attribute)
    (@extract_property $element:ident, $item:ident, $field:ident, url, $selector:expr, $base_url:ident) => {
        if let Ok(sel) = $crate::html_utils::create_selector($selector) {
            if let Some(elem) = $element.select(&sel).next() {
                let url = $crate::html_utils::get_attr(&elem, "href")
                    .or_else(|| $crate::html_utils::get_attr(&elem, "src"));

                // Resolve relative URLs if base_url is provided
                if let Some(url_str) = url {
                    if let Some(base) = $base_url {
                        // Try to resolve relative URL
                        if let Ok(resolved) = $crate::url_utils::resolve_url(Some(base), &url_str) {
                            $item.$field = Some(resolved);
                        } else {
                            // If resolution fails, use original URL
                            $item.$field = Some(url_str);
                        }
                    } else {
                        $item.$field = Some(url_str);
                    }
                }
            }
        }
    };

    // Extract HTML content (inner HTML)
    (@extract_property $element:ident, $item:ident, $field:ident, html, $selector:expr, $base_url:ident) => {
        if let Ok(sel) = $crate::html_utils::create_selector($selector) {
            if let Some(elem) = $element.select(&sel).next() {
                let html_content = elem.inner_html().trim().to_string();
                if !html_content.is_empty() {
                    $item.$field = Some(html_content);
                }
            }
        }
    };

    // Extract datetime (from datetime attribute or text)
    (@extract_property $element:ident, $item:ident, $field:ident, date, $selector:expr, $base_url:ident) => {
        if let Ok(sel) = $crate::html_utils::create_selector($selector) {
            if let Some(elem) = $element.select(&sel).next() {
                $item.$field = $crate::html_utils::get_attr(&elem, "datetime")
                    .or_else(|| $crate::html_utils::extract_text(&elem));
            }
        }
    };

    // Extract multiple text values (Vec<String>)
    (@extract_property $element:ident, $item:ident, $field:ident, multi_text, $selector:expr, $base_url:ident) => {
        if let Ok(sel) = $crate::html_utils::create_selector($selector) {
            for elem in $element.select(&sel) {
                if let Some(text) = $crate::html_utils::extract_text(&elem) {
                    $item.$field.push(text);
                }
            }
        }
    };

    // Extract multiple URLs (Vec<String>)
    (@extract_property $element:ident, $item:ident, $field:ident, multi_url, $selector:expr, $base_url:ident) => {
        if let Ok(sel) = $crate::html_utils::create_selector($selector) {
            for elem in $element.select(&sel) {
                if let Some(url) = $crate::html_utils::get_attr(&elem, "href")
                    .or_else(|| $crate::html_utils::get_attr(&elem, "src")) {

                    // Resolve relative URLs if base_url is provided
                    if let Some(base) = $base_url {
                        if let Ok(resolved) = $crate::url_utils::resolve_url(Some(base), &url) {
                            $item.$field.push(resolved);
                        } else {
                            $item.$field.push(url);
                        }
                    } else {
                        $item.$field.push(url);
                    }
                }
            }
        }
    };

    // Extract numeric value (f32)
    (@extract_property $element:ident, $item:ident, $field:ident, number, $selector:expr, $base_url:ident) => {
        if let Ok(sel) = $crate::html_utils::create_selector($selector) {
            if let Some(elem) = $element.select(&sel).next() {
                if let Some(text) = $crate::html_utils::extract_text(&elem) {
                    // Try to parse as f32
                    if let Ok(num) = text.parse::<f32>() {
                        $item.$field = Some(num);
                    }
                }
            }
        }
    };

    // Extract numeric value (f64)
    (@extract_property $element:ident, $item:ident, $field:ident, f64_number, $selector:expr, $base_url:ident) => {
        if let Ok(sel) = $crate::html_utils::create_selector($selector) {
            if let Some(elem) = $element.select(&sel).next() {
                if let Some(text) = $crate::html_utils::extract_text(&elem) {
                    // Try to parse as f64
                    if let Ok(num) = text.parse::<f64>() {
                        $item.$field = Some(num);
                    }
                }
            }
        }
    };

    // Extract email (special handling for mailto: links)
    (@extract_property $element:ident, $item:ident, $field:ident, email, $selector:expr, $base_url:ident) => {
        if let Ok(sel) = $crate::html_utils::create_selector($selector) {
            if let Some(elem) = $element.select(&sel).next() {
                $item.$field = $crate::html_utils::get_attr(&elem, "href")
                    .map(|s| s.trim_start_matches("mailto:").to_string())
                    .or_else(|| $crate::html_utils::extract_text(&elem));
            }
        }
    };

    // Extract nested h-card microformat (Option<Box<HCard>>)
    (@extract_property $element:ident, $item:ident, $field:ident, nested_hcard, $selector:expr, $base_url:ident) => {
        if let Ok(sel) = $crate::html_utils::create_selector($selector) {
            if let Some(elem) = $element.select(&sel).next() {
                let nested_html = elem.html();
                if let Ok(items) = $crate::extractors::microformats::hcard::extract(&nested_html, $base_url) {
                    if let Some(item) = items.first() {
                        $item.$field = Some(Box::new(item.clone()));
                    }
                }
            }
        }
    };

    // Extract nested h-product microformat (Option<Box<HProduct>>)
    (@extract_property $element:ident, $item:ident, $field:ident, nested_hproduct, $selector:expr, $base_url:ident) => {
        if let Ok(sel) = $crate::html_utils::create_selector($selector) {
            if let Some(elem) = $element.select(&sel).next() {
                let nested_html = elem.html();
                if let Ok(items) = $crate::extractors::microformats::hproduct::extract(&nested_html, $base_url) {
                    if let Some(item) = items.first() {
                        $item.$field = Some(Box::new(item.clone()));
                    }
                }
            }
        }
    };

    // Extract nested h-card with text fallback (for dual-field patterns)
    // Tries nested h-card first, if not found falls back to text extraction
    (@extract_dual_property $element:ident, $item:ident, $text_field:ident, $nested_field:ident,
     nested_hcard_or_text, $nested_sel:expr, $text_sel:expr, $base_url:ident) => {
        let mut found_nested = false;
        if let Ok(sel) = $crate::html_utils::create_selector($nested_sel) {
            if let Some(elem) = $element.select(&sel).next() {
                let nested_html = elem.html();
                if let Ok(items) = $crate::extractors::microformats::hcard::extract(&nested_html, $base_url) {
                    if let Some(item) = items.first() {
                        $item.$nested_field = Some(Box::new(item.clone()));
                        found_nested = true;
                    }
                }
            }
        }
        if !found_nested {
            if let Ok(sel) = $crate::html_utils::create_selector($text_sel) {
                if let Some(elem) = $element.select(&sel).next() {
                    $item.$text_field = $crate::html_utils::extract_text(&elem);
                }
            }
        }
    };

    // Extract nested h-product with text fallback (for dual-field patterns)
    // Tries nested h-product first, if not found falls back to text extraction
    (@extract_dual_property $element:ident, $item:ident, $text_field:ident, $nested_field:ident,
     nested_hproduct_or_text, $nested_sel:expr, $text_sel:expr, $base_url:ident) => {
        let mut found_nested = false;
        if let Ok(sel) = $crate::html_utils::create_selector($nested_sel) {
            if let Some(elem) = $element.select(&sel).next() {
                let nested_html = elem.html();
                if let Ok(items) = $crate::extractors::microformats::hproduct::extract(&nested_html, $base_url) {
                    if let Some(item) = items.first() {
                        $item.$nested_field = Some(Box::new(item.clone()));
                        found_nested = true;
                    }
                }
            }
        }
        if !found_nested {
            if let Ok(sel) = $crate::html_utils::create_selector($text_sel) {
                if let Some(elem) = $element.select(&sel).next() {
                    $item.$text_field = $crate::html_utils::extract_text(&elem);
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test struct for macro verification
    #[allow(dead_code)]
    #[derive(Debug, Default, PartialEq)]
    struct TestCard {
        name: Option<String>,
        url: Option<String>,
        email: Option<String>,
        tags: Vec<String>,
    }

    #[test]
    fn test_macro_expands_correctly() {
        // This test verifies the macro compiles correctly
        // Actual functionality is tested in the extractor modules

        // Create a simple extractor using the macro
        mod test_extractor {
            use crate::errors::Result;

            #[derive(Debug, Default, PartialEq)]
            pub struct SimpleCard {
                pub name: Option<String>,
                pub url: Option<String>,
            }

            microformat_extractor! {
                SimpleCard, ".h-card" {
                    name: text(".p-name"),
                    url: url(".u-url"),
                }
            }
        }

        let html = r#"
            <div class="h-card">
                <span class="p-name">Test Name</span>
                <a class="u-url" href="https://example.com">Link</a>
            </div>
        "#;

        let result = test_extractor::extract(html, None);
        assert!(result.is_ok());

        let cards = result.unwrap();
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].name, Some("Test Name".to_string()));
        assert_eq!(cards[0].url, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_macro_with_multiple_items() {
        mod test_extractor {
            use crate::errors::Result;

            #[derive(Debug, Default, PartialEq)]
            pub struct TestItem {
                pub title: Option<String>,
            }

            microformat_extractor! {
                TestItem, ".test-item" {
                    title: text(".p-title"),
                }
            }
        }

        let html = r#"
            <div class="test-item">
                <span class="p-title">Item 1</span>
            </div>
            <div class="test-item">
                <span class="p-title">Item 2</span>
            </div>
        "#;

        let result = test_extractor::extract(html, None);
        assert!(result.is_ok());

        let items = result.unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].title, Some("Item 1".to_string()));
        assert_eq!(items[1].title, Some("Item 2".to_string()));
    }

    #[test]
    fn test_macro_with_multi_text() {
        mod test_extractor {
            use crate::errors::Result;

            #[derive(Debug, Default, PartialEq)]
            pub struct TaggedItem {
                pub name: Option<String>,
                pub tags: Vec<String>,
            }

            microformat_extractor! {
                TaggedItem, ".tagged-item" {
                    name: text(".p-name"),
                    tags: multi_text(".p-tag"),
                }
            }
        }

        let html = r#"
            <div class="tagged-item">
                <span class="p-name">Item</span>
                <span class="p-tag">tag1</span>
                <span class="p-tag">tag2</span>
                <span class="p-tag">tag3</span>
            </div>
        "#;

        let result = test_extractor::extract(html, None);
        assert!(result.is_ok());

        let items = result.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].tags, vec!["tag1", "tag2", "tag3"]);
    }

    #[test]
    fn test_macro_with_url_resolution() {
        mod test_extractor {
            use crate::errors::Result;

            #[derive(Debug, Default, PartialEq)]
            pub struct LinkedItem {
                pub name: Option<String>,
                pub url: Option<String>,
            }

            microformat_extractor! {
                LinkedItem, ".linked-item" {
                    name: text(".p-name"),
                    url: url(".u-url"),
                }
            }
        }

        let html = r#"
            <div class="linked-item">
                <span class="p-name">Item</span>
                <a class="u-url" href="/relative/path">Link</a>
            </div>
        "#;

        let result = test_extractor::extract(html, Some("https://example.com"));
        assert!(result.is_ok());

        let items = result.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].url, Some("https://example.com/relative/path".to_string()));
    }

    #[test]
    fn test_macro_with_html_content() {
        mod test_extractor {
            use crate::errors::Result;

            #[derive(Debug, Default, PartialEq)]
            pub struct ContentItem {
                pub title: Option<String>,
                pub content: Option<String>,
            }

            microformat_extractor! {
                ContentItem, ".content-item" {
                    title: text(".p-title"),
                    content: html(".e-content"),
                }
            }
        }

        let html = r#"
            <div class="content-item">
                <h1 class="p-title">Title</h1>
                <div class="e-content">
                    <p>This is <strong>HTML</strong> content.</p>
                </div>
            </div>
        "#;

        let result = test_extractor::extract(html, None);
        assert!(result.is_ok());

        let items = result.unwrap();
        assert_eq!(items.len(), 1);
        assert!(items[0].content.as_ref().unwrap().contains("<strong>HTML</strong>"));
    }

    #[test]
    fn test_macro_with_date() {
        mod test_extractor {
            use crate::errors::Result;

            #[derive(Debug, Default, PartialEq)]
            pub struct DateItem {
                pub title: Option<String>,
                pub published: Option<String>,
            }

            microformat_extractor! {
                DateItem, ".date-item" {
                    title: text(".p-title"),
                    published: date(".dt-published"),
                }
            }
        }

        let html = r#"
            <div class="date-item">
                <h1 class="p-title">Article</h1>
                <time class="dt-published" datetime="2024-01-15">January 15, 2024</time>
            </div>
        "#;

        let result = test_extractor::extract(html, None);
        assert!(result.is_ok());

        let items = result.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].published, Some("2024-01-15".to_string()));
    }

    #[test]
    fn test_macro_with_number() {
        mod test_extractor {
            use crate::errors::Result;

            #[derive(Debug, Default, PartialEq)]
            pub struct RatedItem {
                pub title: Option<String>,
                pub rating: Option<f32>,
            }

            microformat_extractor! {
                RatedItem, ".rated-item" {
                    title: text(".p-title"),
                    rating: number(".p-rating"),
                }
            }
        }

        let html = r#"
            <div class="rated-item">
                <h1 class="p-title">Product</h1>
                <span class="p-rating">4.5</span>
            </div>
        "#;

        let result = test_extractor::extract(html, None);
        assert!(result.is_ok());

        let items = result.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].rating, Some(4.5));
    }
}
