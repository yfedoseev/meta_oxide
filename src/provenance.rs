//! Field provenance tracking for the unified [`MetaParser`] facade.
//!
//! When several extractors all produce a value for the same conceptual field
//! (e.g. JSON-LD `Product.name`, microdata `[itemprop=name]`, and `og:title`
//! all carrying the page title), consumers need to know *where* a value came
//! from — for debugging, for confidence scoring, and for choosing whether to
//! trust a structured result over a heuristic guess.
//!
//! [`FieldSource`] tags a value with its origin and [`FieldValue`] pairs the
//! value with that tag.
//!
//! [`MetaParser`]: crate::MetaParser

use serde::{Deserialize, Serialize};

/// Where a piece of metadata was extracted from.
///
/// The string payloads carry the format-specific path or attribute name so
/// that source attribution survives merging — e.g. `JsonLd("Product.offers.price")`
/// vs `Microdata("price")` vs `Heuristic(".price")`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "format", content = "path", rename_all = "snake_case")]
pub enum FieldSource {
    /// A JSON-LD object property, e.g. `"Product.name"` or `"Article.author.name"`.
    JsonLd(String),
    /// An Open Graph property, e.g. `"og:title"`.
    OpenGraph(String),
    /// A Twitter Card property, e.g. `"twitter:card"`.
    Twitter(String),
    /// An HTML5 microdata `itemprop` value.
    Microdata(String),
    /// An RDFa property URI or CURIE.
    Rdfa(String),
    /// A microformats2 class, e.g. `"h-card.name"`.
    Microformats(String),
    /// A Dublin Core meta tag, e.g. `"dc.title"`.
    DublinCore(String),
    /// A standard HTML tag or `<meta name="...">`, e.g. `"title"` or `"description"`.
    Meta(String),
    /// A web app manifest field.
    Manifest(String),
    /// A heuristic / CSS-fallback rule. The string is the rule name,
    /// e.g. `"first_h1"`, `"first_paragraph"`, or `".price"`.
    ///
    /// Stored as `String` (rather than `&'static str`) so that the enclosing
    /// [`FieldValue`] can derive `Deserialize` cleanly. Construct with
    /// [`FieldSource::heuristic`] for the common static-name case.
    Heuristic(String),
}

impl FieldSource {
    /// Construct a [`FieldSource::Heuristic`] from a static rule name without
    /// callers having to write `.into()` at every site.
    pub fn heuristic(rule: &'static str) -> Self {
        FieldSource::Heuristic(rule.to_string())
    }
}

/// A value paired with its source attribution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldValue<T> {
    /// The extracted value.
    pub value: T,
    /// Where this value originated.
    pub source: FieldSource,
}

impl<T> FieldValue<T> {
    /// Construct a new [`FieldValue`].
    pub fn new(value: T, source: FieldSource) -> Self {
        Self { value, source }
    }

    /// Map the inner value while preserving the source attribution.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> FieldValue<U> {
        FieldValue { value: f(self.value), source: self.source }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_value_carries_source() {
        let fv = FieldValue::new(
            "Mechanical Keyboard".to_string(),
            FieldSource::JsonLd("Product.name".into()),
        );
        assert_eq!(fv.value, "Mechanical Keyboard");
        assert_eq!(fv.source, FieldSource::JsonLd("Product.name".into()));
    }

    #[test]
    fn field_value_map_preserves_source() {
        let fv = FieldValue::new(42_i32, FieldSource::heuristic("count_h1"));
        let mapped = fv.map(|n| n * 2);
        assert_eq!(mapped.value, 84);
        assert_eq!(mapped.source, FieldSource::heuristic("count_h1"));
    }

    #[test]
    fn field_source_serializes_with_format_tag() {
        let s = serde_json::to_string(&FieldSource::JsonLd("Product.name".into())).unwrap();
        assert!(s.contains("\"format\":\"json_ld\""));
        assert!(s.contains("\"path\":\"Product.name\""));
    }

    #[test]
    fn field_source_roundtrips_through_serde() {
        let original = FieldSource::Microdata("price".into());
        let json = serde_json::to_string(&original).unwrap();
        let parsed: FieldSource = serde_json::from_str(&json).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn every_source_variant_roundtrips() {
        let variants = [
            FieldSource::JsonLd("a".into()),
            FieldSource::OpenGraph("b".into()),
            FieldSource::Twitter("c".into()),
            FieldSource::Microdata("d".into()),
            FieldSource::Rdfa("e".into()),
            FieldSource::Microformats("f".into()),
            FieldSource::DublinCore("g".into()),
            FieldSource::Meta("h".into()),
            FieldSource::Manifest("i".into()),
            FieldSource::heuristic("j"),
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let back: FieldSource = serde_json::from_str(&json).unwrap();
            assert_eq!(v, back);
        }
    }

    #[test]
    fn field_value_serializes_with_value_and_source() {
        let fv = FieldValue::new("hello".to_string(), FieldSource::Meta("title".into()));
        let json = serde_json::to_string(&fv).unwrap();
        assert!(json.contains("\"value\":\"hello\""));
        assert!(json.contains("\"source\""));
        let back: FieldValue<String> = serde_json::from_str(&json).unwrap();
        assert_eq!(back, fv);
    }

    #[test]
    fn heuristic_constructor_uses_static_name() {
        let s = FieldSource::heuristic("first_h1");
        assert_eq!(s, FieldSource::Heuristic("first_h1".to_string()));
    }
}
