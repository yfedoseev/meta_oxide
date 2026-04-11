//! Canonical, cross-format entity types.
//!
//! Format-native types in [`crate::types`] (`types::jsonld::Product`,
//! `types::microdata::MicrodataItem`, …) describe what one extractor found.
//! That's the right shape when you care about which format said what — but
//! agents and scrapers usually want the *merged* answer: "give me the product
//! on this page, sourced from whichever format had the field, and tell me
//! where each value came from".
//!
//! This module is that merged layer. Each canonical type provides a
//! [`CanonicalMerge::merge_from`] constructor that walks a [`crate::MetaGraph`]
//! in priority order — JSON-LD → microdata → RDFa → Open Graph → meta tags
//! → heuristics — and returns a single struct with [`crate::FieldValue`]
//! per field so each value carries its [`crate::FieldSource`].
//!
//! ```no_run
//! use meta_oxide::MetaParser;
//!
//! let graph = MetaParser::new().parse("<html>...</html>").unwrap();
//! if let Some(product) = graph.canonical_product() {
//!     if let Some(name) = product.name {
//!         println!("{} (from {:?})", name.value, name.source);
//!     }
//! }
//! ```

use crate::parser_facade::MetaGraph;

pub mod article;
pub mod breadcrumb;
pub mod event;
pub mod faq;
pub mod knowledge_graph;
pub mod money;
pub mod organization;
pub mod person;
pub mod product;
pub mod recipe;
pub mod review;
pub mod video;

pub use article::Article;
pub use breadcrumb::{Breadcrumb, BreadcrumbList};
pub use event::Event;
pub use faq::{FAQPage, QAPair};
pub use knowledge_graph::KnowledgeGraph;
pub use money::{Availability, Money};
pub use organization::Organization;
pub use person::Person;
pub use product::Product;
pub use recipe::Recipe;
pub use review::Review;
pub use video::VideoObject;

/// Trait implemented by every canonical entity type.
///
/// `merge_from` walks a [`MetaGraph`] across all formats and returns a
/// canonical view. Returns `None` if the graph contains nothing recognisable
/// as the requested entity.
pub trait CanonicalMerge: Sized {
    /// Merge sources from `graph` into a canonical instance, or return `None`
    /// if no source supplied enough information.
    fn merge_from(graph: &MetaGraph) -> Option<Self>;
}

/// Trait for canonical types that may appear multiple times on a page
/// (e.g. reviews, breadcrumbs).
pub trait CanonicalMergeAll: Sized {
    /// Merge every instance of this entity from `graph`.
    fn merge_all_from(graph: &MetaGraph) -> Vec<Self>;
}

// ---- shared internal helpers used by every canonical type ------------------

pub(crate) mod helpers {
    use crate::types::jsonld::JsonLdObject;
    use crate::types::microdata::{MicrodataItem, PropertyValue};
    use serde_json::Value;

    /// Return the first `Some(_)` from an array of `Option<T>` candidates.
    /// Replaces the ad-hoc `first_some` helper duplicated across canonical
    /// modules.
    pub fn first_some<T, const N: usize>(candidates: [Option<T>; N]) -> Option<T> {
        candidates.into_iter().flatten().next()
    }

    /// Coerce a `serde_json::Value` into an `f64`, accepting both numeric
    /// and string-encoded numbers (JSON-LD price/rating fields arrive as
    /// either shape).
    pub fn parse_number(value: &Value) -> Option<f64> {
        match value {
            Value::Number(n) => n.as_f64(),
            Value::String(s) => s.parse::<f64>().ok(),
            _ => None,
        }
    }

    /// Walk a list of JSON-LD objects looking for the first text value
    /// matching any name in `field_names`. Used by the cross-format
    /// [`crate::MetaGraph::get`] lookup. Returns the matched value plus
    /// `(type_label, field_name)` so callers can stamp a FieldSource.
    pub fn first_jsonld_field<'a>(
        items: &[JsonLdObject],
        field_names: &'a [&'a str],
    ) -> Option<(String, String, &'a str)> {
        for item in items {
            let type_label = jsonld_type_label(item);
            for field in field_names {
                if let Some(value) = item.properties.get(*field) {
                    if let Some(s) = value_to_string(value) {
                        return Some((s, type_label, field));
                    }
                }
            }
        }
        None
    }

    /// Return the type label of a JSON-LD object as a string (first entry
    /// when `@type` is an array, `"Thing"` as a fallback).
    pub fn jsonld_type_label(item: &JsonLdObject) -> String {
        match &item.type_ {
            Some(Value::String(s)) => s.clone(),
            Some(Value::Array(arr)) => arr
                .first()
                .and_then(|v| v.as_str())
                .map(str::to_string)
                .unwrap_or_else(|| "Thing".to_string()),
            _ => "Thing".to_string(),
        }
    }

    /// Walk every microdata item looking for the first text-valued property
    /// matching any name in `field_names`.
    pub fn first_microdata_text<'a>(
        items: &[MicrodataItem],
        field_names: &'a [&'a str],
    ) -> Option<(String, &'a str)> {
        for item in items {
            for field in field_names {
                if let Some(values) = item.properties.get(*field) {
                    for value in values {
                        if let PropertyValue::Text(s) = value {
                            if !s.is_empty() {
                                return Some((s.clone(), field));
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Find the first JSON-LD object whose `@type` matches any of `wanted`.
    /// Recurses into `@graph` containers.
    pub fn find_jsonld_of_type<'a>(
        items: &'a [JsonLdObject],
        wanted: &[&str],
    ) -> Option<&'a JsonLdObject> {
        for item in items {
            if jsonld_type_matches(item, wanted) {
                return Some(item);
            }
            if let Some(ref graph) = item.graph {
                if let Some(found) = find_jsonld_of_type(graph, wanted) {
                    return Some(found);
                }
            }
        }
        None
    }

    /// Find every JSON-LD object whose `@type` matches any of `wanted`.
    /// Recurses into `@graph` containers.
    pub fn find_all_jsonld_of_type<'a>(
        items: &'a [JsonLdObject],
        wanted: &[&str],
    ) -> Vec<&'a JsonLdObject> {
        let mut out = Vec::new();
        collect_jsonld_of_type(items, wanted, &mut out);
        out
    }

    fn collect_jsonld_of_type<'a>(
        items: &'a [JsonLdObject],
        wanted: &[&str],
        out: &mut Vec<&'a JsonLdObject>,
    ) {
        for item in items {
            if jsonld_type_matches(item, wanted) {
                out.push(item);
            }
            if let Some(ref graph) = item.graph {
                collect_jsonld_of_type(graph, wanted, out);
            }
        }
    }

    /// Check whether a JSON-LD object's `@type` matches any wanted type name.
    pub fn jsonld_type_matches(item: &JsonLdObject, wanted: &[&str]) -> bool {
        match &item.type_ {
            Some(Value::String(s)) => wanted.iter().any(|w| s.eq_ignore_ascii_case(w)),
            Some(Value::Array(arr)) => arr.iter().any(|v| {
                v.as_str().is_some_and(|s| wanted.iter().any(|w| s.eq_ignore_ascii_case(w)))
            }),
            _ => false,
        }
    }

    /// Read a string-valued property from a JSON-LD object. Handles raw
    /// strings, numbers, booleans, and `{ "@value": "..." }` / `{ "name": "..." }`
    /// shapes.
    pub fn jsonld_string(item: &JsonLdObject, key: &str) -> Option<String> {
        item.properties.get(key).and_then(value_to_string)
    }

    /// Best-effort string coercion for arbitrary `serde_json::Value`s found
    /// inside JSON-LD payloads.
    pub fn value_to_string(value: &Value) -> Option<String> {
        match value {
            Value::String(s) if !s.is_empty() => Some(s.clone()),
            Value::Number(n) => Some(n.to_string()),
            Value::Bool(b) => Some(b.to_string()),
            Value::Array(arr) => arr.iter().find_map(value_to_string),
            Value::Object(map) => {
                for key in &["@value", "name", "url", "@id"] {
                    if let Some(v) = map.get(*key) {
                        if let Some(s) = value_to_string(v) {
                            return Some(s);
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Find the first microdata item whose `@type` (full schema.org URI or
    /// bare type name) matches any of `wanted`.
    pub fn find_microdata_of_type<'a>(
        items: &'a [MicrodataItem],
        wanted: &[&str],
    ) -> Option<&'a MicrodataItem> {
        items.iter().find(|item| microdata_type_matches(item, wanted))
    }

    /// Find every microdata item matching one of `wanted` types.
    pub fn find_all_microdata_of_type<'a>(
        items: &'a [MicrodataItem],
        wanted: &[&str],
    ) -> Vec<&'a MicrodataItem> {
        items.iter().filter(|item| microdata_type_matches(item, wanted)).collect()
    }

    /// Check whether a microdata item's `itemtype` matches any wanted name.
    /// Accepts both full `https://schema.org/Product` URLs and bare
    /// `Product` names.
    pub fn microdata_type_matches(item: &MicrodataItem, wanted: &[&str]) -> bool {
        let Some(types) = &item.item_type else { return false };
        for t in types {
            let bare = t.rsplit('/').next().unwrap_or(t.as_str());
            if wanted.iter().any(|w| bare.eq_ignore_ascii_case(w)) {
                return true;
            }
        }
        false
    }

    /// Read the first text-valued property from a microdata item.
    pub fn microdata_text(item: &MicrodataItem, key: &str) -> Option<String> {
        item.properties.get(key).and_then(|values| {
            values.iter().find_map(|v| match v {
                PropertyValue::Text(s) if !s.is_empty() => Some(s.clone()),
                _ => None,
            })
        })
    }

    /// Read the first nested item from a microdata property (e.g. `offers`
    /// inside a Product).
    pub fn microdata_nested<'a>(item: &'a MicrodataItem, key: &str) -> Option<&'a MicrodataItem> {
        item.properties.get(key).and_then(|values| {
            values.iter().find_map(|v| match v {
                PropertyValue::Item(boxed) => Some(boxed.as_ref()),
                _ => None,
            })
        })
    }
}
