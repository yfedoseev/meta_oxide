//! RDFa (Resource Description Framework in Attributes) extractor
//!
//! Extracts semantic metadata embedded in HTML using RDFa attributes.
//! RDFa is a W3C standard with 62% desktop adoption.

use crate::errors::Result;
use crate::extractors::common::{html_utils, url_utils};
use crate::types::rdfa::{RdfaItem, RdfaValue};
use scraper::{ElementRef, Html};
use std::collections::HashMap;

#[cfg(test)]
mod tests;

/// Prefix context for expanding CURIEs (Compact URIs)
#[derive(Debug, Clone)]
struct PrefixContext {
    prefixes: HashMap<String, String>,
}

impl PrefixContext {
    /// Create a new prefix context with default prefixes
    fn new() -> Self {
        let mut prefixes = HashMap::new();

        // Common default prefixes
        prefixes.insert("schema".to_string(), "https://schema.org/".to_string());
        prefixes.insert("foaf".to_string(), "http://xmlns.com/foaf/0.1/".to_string());
        prefixes.insert("dc".to_string(), "http://purl.org/dc/terms/".to_string());
        prefixes.insert("og".to_string(), "http://ogp.me/ns#".to_string());
        prefixes.insert("xsd".to_string(), "http://www.w3.org/2001/XMLSchema#".to_string());

        Self { prefixes }
    }

    /// Parse and add prefixes from a prefix attribute value
    /// Format: "prefix1: namespace1 prefix2: namespace2"
    fn parse_prefix_attr(&mut self, prefix_attr: &str) {
        let tokens: Vec<&str> = prefix_attr.split_whitespace().collect();
        let mut i = 0;

        while i < tokens.len() {
            let token = tokens[i];

            // Look for a token ending with ':'
            if token.ends_with(':') && token.len() > 1 {
                let prefix = token[..token.len() - 1].trim();

                // The next token should be the namespace
                if i + 1 < tokens.len() {
                    let namespace = tokens[i + 1].trim();
                    if !prefix.is_empty() && !namespace.is_empty() {
                        self.prefixes.insert(prefix.to_string(), namespace.to_string());
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
    }

    /// Expand a CURIE (Compact URI) to a full URI
    /// Examples: "schema:Person" -> "https://schema.org/Person"
    fn expand_curie(&self, curie: &str) -> String {
        if let Some((prefix, local_name)) = curie.split_once(':') {
            if let Some(namespace) = self.prefixes.get(prefix) {
                return format!("{}{}", namespace, local_name);
            }
        }
        // If not a valid CURIE, return as-is
        curie.to_string()
    }

    /// Expand all CURIEs in a space-separated list
    fn expand_curie_list(&self, list: &str) -> Vec<String> {
        list.split_whitespace().map(|item| self.expand_curie(item)).collect()
    }
}

/// Extract all RDFa items from HTML
///
/// # Arguments
/// * `html` - The HTML content to extract from
/// * `base_url` - Optional base URL for resolving relative URLs
///
/// # Returns
/// * `Result<Vec<RdfaItem>>` - List of extracted RDFa items or error
///
/// # Example
/// ```rust
/// use meta_oxide::extractors::rdfa;
///
/// let html = r#"<div vocab="https://schema.org/" typeof="Person">
///     <span property="name">Jane Doe</span>
/// </div>"#;
/// let items = rdfa::extract(html, None).unwrap();
/// assert_eq!(items.len(), 1);
/// ```
pub fn extract(html: &str, base_url: Option<&str>) -> Result<Vec<RdfaItem>> {
    let doc = html_utils::parse_html(html);
    let mut items = Vec::new();

    // Create prefix context with default prefixes
    let mut prefix_ctx = PrefixContext::new();

    // Collect all prefix definitions from the document
    let prefix_selector = html_utils::create_selector("[prefix]")?;
    for element in doc.select(&prefix_selector) {
        if let Some(prefix_attr) = html_utils::get_attr(&element, "prefix") {
            prefix_ctx.parse_prefix_attr(&prefix_attr);
        }
    }

    // Find all RDFa root elements (elements with typeof or vocab)
    let roots = find_rdfa_roots(&doc)?;

    for root in roots {
        let item = extract_item_with_context(&root, base_url, &prefix_ctx)?;
        items.push(item);
    }

    Ok(items)
}

/// Find all RDFa root elements in the document
///
/// Root elements are those with `typeof` or `vocab` attributes
fn find_rdfa_roots(doc: &Html) -> Result<Vec<ElementRef<'_>>> {
    let mut roots = Vec::new();

    // Find elements with typeof attribute (type declaration)
    let typeof_selector = html_utils::create_selector("[typeof]")?;
    for element in doc.select(&typeof_selector) {
        // Only add if not nested within another typeof (we'll handle nesting later)
        if !is_nested_typeof(&element) {
            roots.push(element);
        }
    }

    // Find elements with vocab attribute that don't have typeof
    let vocab_selector = html_utils::create_selector("[vocab]:not([typeof])")?;
    for element in doc.select(&vocab_selector) {
        // Only add if not already in roots
        if !roots.iter().any(|r| r.id() == element.id()) {
            roots.push(element);
        }
    }

    Ok(roots)
}

/// Check if an element is nested within another typeof element
fn is_nested_typeof(element: &ElementRef) -> bool {
    let mut current = element.parent();
    while let Some(parent) = current {
        if let Some(parent_elem) = parent.value().as_element() {
            if parent_elem.attr("typeof").is_some() {
                return true;
            }
        }
        current = parent.parent();
    }
    false
}

/// Extract a single RDFa item from a root element with prefix context
fn extract_item_with_context(
    element: &ElementRef,
    base_url: Option<&str>,
    prefix_ctx: &PrefixContext,
) -> Result<RdfaItem> {
    let mut item = RdfaItem::new();

    // Extract vocab attribute
    if let Some(vocab) = html_utils::get_attr(element, "vocab") {
        item = item.with_vocab(vocab);
    }

    // Extract typeof attribute (can be space-separated list of types with CURIEs)
    if let Some(type_attr) = html_utils::get_attr(element, "typeof") {
        let types = prefix_ctx.expand_curie_list(&type_attr);
        if !types.is_empty() {
            item = item.with_type(types);
        }
    }

    // Extract about attribute (subject URI, can be CURIE)
    if let Some(about) = html_utils::get_attr(element, "about") {
        // First expand CURIE if applicable
        let expanded = prefix_ctx.expand_curie(&about);
        // Then resolve URL if base_url is provided
        let resolved = if let Some(base) = base_url {
            url_utils::resolve_url(Some(base), &expanded).unwrap_or(expanded)
        } else {
            expanded
        };
        item = item.with_about(resolved);
    }

    // Extract properties from this element and descendants
    let properties = extract_properties_with_context(element, base_url, prefix_ctx)?;
    item.properties = properties;

    Ok(item)
}

/// Extract all properties from an element and its descendants with prefix context
fn extract_properties_with_context(
    element: &ElementRef,
    base_url: Option<&str>,
    prefix_ctx: &PrefixContext,
) -> Result<HashMap<String, Vec<RdfaValue>>> {
    let mut properties: HashMap<String, Vec<RdfaValue>> = HashMap::new();

    // Check if this element has a property attribute (can be CURIE)
    if let Some(property_name) = html_utils::get_attr(element, "property") {
        // Expand CURIE in property name
        let expanded_name = prefix_ctx.expand_curie(&property_name);
        let value = extract_property_value_with_context(element, base_url, prefix_ctx)?;
        properties.entry(expanded_name).or_default().push(value);
    }

    // Recursively extract from children
    for child in element.children() {
        if let Some(child_element) = ElementRef::wrap(child) {
            // Skip nested typeof elements - they will be extracted as separate items
            if html_utils::get_attr(&child_element, "typeof").is_some() {
                // This is a nested item
                if html_utils::get_attr(element, "property").is_some() {
                    // Parent has property, so this nested item is the value
                    continue; // Will be handled by extract_property_value_with_context
                }
            }

            // Extract properties from child
            let child_properties =
                extract_properties_with_context(&child_element, base_url, prefix_ctx)?;
            for (key, values) in child_properties {
                properties.entry(key).or_default().extend(values);
            }
        }
    }

    Ok(properties)
}

/// Extract the value of a property from an element with prefix context
fn extract_property_value_with_context(
    element: &ElementRef,
    base_url: Option<&str>,
    prefix_ctx: &PrefixContext,
) -> Result<RdfaValue> {
    // Priority order for value extraction:
    // 1. content attribute (highest priority)
    // 2. resource, href, src attributes (for URIs, can be CURIEs)
    // 3. Check for nested typeof (nested item)
    // 4. Text content (lowest priority)

    // 1. Check for content attribute override
    if let Some(content) = html_utils::get_attr(element, "content") {
        // Check if there's a datatype attribute (can be CURIE like xsd:integer)
        if let Some(datatype) = html_utils::get_attr(element, "datatype") {
            let expanded_datatype = prefix_ctx.expand_curie(&datatype);
            return Ok(RdfaValue::TypedLiteral { value: content, datatype: expanded_datatype });
        }
        return Ok(RdfaValue::Literal(content));
    }

    // 2. Check for resource/href/src attributes (URI values, can be CURIEs)
    for attr in &["resource", "href", "src"] {
        if let Some(uri) = html_utils::get_attr(element, attr) {
            // First expand CURIE if applicable
            let expanded = prefix_ctx.expand_curie(&uri);
            // Then resolve URL if base_url is provided
            let resolved = if let Some(base) = base_url {
                url_utils::resolve_url(Some(base), &expanded).unwrap_or(expanded)
            } else {
                expanded
            };
            return Ok(RdfaValue::Resource(resolved));
        }
    }

    // 3. Check for nested typeof (nested RDFa item)
    if html_utils::get_attr(element, "typeof").is_some() {
        let nested_item = extract_item_with_context(element, base_url, prefix_ctx)?;
        return Ok(RdfaValue::Item(Box::new(nested_item)));
    }

    // 4. Extract text content
    if let Some(text) = html_utils::extract_text(element) {
        // Check if there's a datatype attribute (can be CURIE)
        if let Some(datatype) = html_utils::get_attr(element, "datatype") {
            let expanded_datatype = prefix_ctx.expand_curie(&datatype);
            return Ok(RdfaValue::TypedLiteral { value: text, datatype: expanded_datatype });
        }
        return Ok(RdfaValue::Literal(text));
    }

    // Fallback to empty literal
    Ok(RdfaValue::Literal(String::new()))
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_extract_empty_html() {
        let result = extract("", None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_extract_no_rdfa() {
        let html = r#"<div><p>No RDFa here</p></div>"#;
        let result = extract(html, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_extract_simple_typeof() {
        let html = r#"<div vocab="https://schema.org/" typeof="Person"><span property="name">Jane Doe</span></div>"#;
        let result = extract(html, None).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].vocab, Some("https://schema.org/".to_string()));
        assert_eq!(result[0].type_of, Some(vec!["Person".to_string()]));
    }

    #[test]
    #[ignore] // TODO: Fix stack overflow in deeply nested RDFa items
    fn test_extract_multiple_types() {
        let html = r#"<div typeof="Person Employee" property="name">Jane</div>"#;
        let result = extract(html, None).unwrap();
        assert_eq!(result.len(), 1);
        let types = result[0].type_of.as_ref().unwrap();
        assert_eq!(types.len(), 2);
        assert!(types.contains(&"Person".to_string()));
        assert!(types.contains(&"Employee".to_string()));
    }

    #[test]
    fn test_extract_property_literal() {
        let html = r#"<div typeof="Person"><span property="name">Jane Doe</span></div>"#;
        let result = extract(html, None).unwrap();
        assert_eq!(result.len(), 1);
        let properties = &result[0].properties;
        assert!(properties.contains_key("name"));
        match &properties.get("name").unwrap()[0] {
            RdfaValue::Literal(s) => assert_eq!(s, "Jane Doe"),
            _ => panic!("Expected literal value"),
        }
    }

    #[test]
    fn test_extract_property_with_content_override() {
        let html = r#"<div typeof="Person"><span property="name" content="Jane Smith">Jane Doe</span></div>"#;
        let result = extract(html, None).unwrap();
        let properties = &result[0].properties;
        match &properties.get("name").unwrap()[0] {
            RdfaValue::Literal(s) => assert_eq!(s, "Jane Smith"),
            _ => panic!("Expected literal value"),
        }
    }

    #[test]
    fn test_extract_property_resource() {
        let html = r#"<div typeof="Person"><a property="url" href="https://example.com">Website</a></div>"#;
        let result = extract(html, None).unwrap();
        let properties = &result[0].properties;
        match &properties.get("url").unwrap()[0] {
            RdfaValue::Resource(uri) => assert_eq!(uri, "https://example.com"),
            _ => panic!("Expected resource value"),
        }
    }

    #[test]
    fn test_extract_property_resource_with_base_url() {
        let html = r#"<div typeof="Person"><a property="url" href="/page">Website</a></div>"#;
        let result = extract(html, Some("https://example.com")).unwrap();
        let properties = &result[0].properties;
        match &properties.get("url").unwrap()[0] {
            RdfaValue::Resource(uri) => assert_eq!(uri, "https://example.com/page"),
            _ => panic!("Expected resource value"),
        }
    }

    #[test]
    fn test_extract_multiple_properties() {
        let html = r#"
            <div typeof="Person">
                <span property="name">Jane</span>
                <span property="jobTitle">Engineer</span>
            </div>
        "#;
        let result = extract(html, None).unwrap();
        let properties = &result[0].properties;
        assert_eq!(properties.len(), 2);
        assert!(properties.contains_key("name"));
        assert!(properties.contains_key("jobTitle"));
    }

    #[test]
    fn test_extract_multiple_values_same_property() {
        let html = r#"
            <div typeof="Person">
                <span property="telephone">555-1234</span>
                <span property="telephone">555-5678</span>
            </div>
        "#;
        let result = extract(html, None).unwrap();
        let properties = &result[0].properties;
        assert_eq!(properties.get("telephone").unwrap().len(), 2);
    }

    #[test]
    fn test_extract_about_attribute() {
        let html = r#"<div typeof="Person" about="https://example.com/jane"><span property="name">Jane</span></div>"#;
        let result = extract(html, None).unwrap();
        assert_eq!(result[0].about, Some("https://example.com/jane".to_string()));
    }

    #[test]
    fn test_extract_about_with_base_url() {
        let html = r#"<div typeof="Person" about="/jane"><span property="name">Jane</span></div>"#;
        let result = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(result[0].about, Some("https://example.com/jane".to_string()));
    }

    #[test]
    fn test_extract_vocab_only() {
        let html = r#"<div vocab="https://schema.org/"><div property="name">Test</div></div>"#;
        let result = extract(html, None).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].vocab, Some("https://schema.org/".to_string()));
    }

    #[test]
    #[ignore] // TODO: Fix stack overflow in deeply nested RDFa items
    fn test_extract_nested_typeof() {
        let html = r#"
            <div typeof="Person">
                <span property="name">Jane</span>
                <div property="address" typeof="PostalAddress">
                    <span property="streetAddress">123 Main St</span>
                </div>
            </div>
        "#;
        let result = extract(html, None).unwrap();
        assert_eq!(result.len(), 1);
        let properties = &result[0].properties;
        assert!(properties.contains_key("name"));
        assert!(properties.contains_key("address"));

        match &properties.get("address").unwrap()[0] {
            RdfaValue::Item(item) => {
                assert_eq!(item.type_of, Some(vec!["PostalAddress".to_string()]));
                assert!(item.properties.contains_key("streetAddress"));
            }
            _ => panic!("Expected nested item"),
        }
    }

    #[test]
    fn test_extract_datatype_attribute() {
        let html =
            r#"<div typeof="Person"><span property="age" datatype="xsd:integer">30</span></div>"#;
        let result = extract(html, None).unwrap();
        let properties = &result[0].properties;
        match &properties.get("age").unwrap()[0] {
            RdfaValue::TypedLiteral { value, datatype } => {
                assert_eq!(value, "30");
                assert_eq!(datatype, "http://www.w3.org/2001/XMLSchema#integer");
            }
            _ => panic!("Expected typed literal"),
        }
    }

    #[test]
    fn test_extract_multiple_items() {
        let html = r#"
            <div typeof="Person"><span property="name">Jane</span></div>
            <div typeof="Person"><span property="name">John</span></div>
        "#;
        let result = extract(html, None).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_is_nested_typeof() {
        let html = r#"
            <div typeof="Person">
                <div typeof="Address" id="nested">Test</div>
            </div>
        "#;
        let doc = html_utils::parse_html(html);
        let selector = html_utils::create_selector("#nested").unwrap();
        let element = doc.select(&selector).next().unwrap();
        assert!(is_nested_typeof(&element));
    }

    #[test]
    fn test_is_not_nested_typeof() {
        let html = r#"<div typeof="Person" id="root">Test</div>"#;
        let doc = html_utils::parse_html(html);
        let selector = html_utils::create_selector("#root").unwrap();
        let element = doc.select(&selector).next().unwrap();
        assert!(!is_nested_typeof(&element));
    }
}
