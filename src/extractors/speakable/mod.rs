//! Schema.org `SpeakableSpecification` extractor.
//!
//! Operates on a slice of [`JsonLdObject`]s (typically
//! [`MetaGraph::json_ld`](crate::MetaGraph::json_ld)). Walks each object,
//! including its `@graph` children, and surfaces every `speakable` value
//! found anywhere in the tree.

use crate::types::jsonld::JsonLdObject;
use crate::types::speakable::Speakable;
use serde_json::Value;

#[cfg(test)]
mod tests;

/// Collect every `SpeakableSpecification` declared in a JSON-LD list.
///
/// Returns an empty vector when no speakable markers are present.
pub fn collect(objects: &[JsonLdObject]) -> Vec<Speakable> {
    let mut out = Vec::new();
    for obj in objects {
        walk_object(obj, &mut out);
    }
    out
}

fn walk_object(obj: &JsonLdObject, out: &mut Vec<Speakable>) {
    if let Some(v) = obj.properties.get("speakable") {
        push_from_value(v, out);
    }
    // Also walk properties recursively — speakable can live on nested
    // objects (e.g. inside `mainEntityOfPage.speakable`).
    for v in obj.properties.values() {
        walk_value(v, out);
    }
    if let Some(graph) = &obj.graph {
        for child in graph {
            walk_object(child, out);
        }
    }
}

fn walk_value(value: &Value, out: &mut Vec<Speakable>) {
    match value {
        Value::Object(map) => {
            if let Some(sp) = map.get("speakable") {
                push_from_value(sp, out);
            }
            for v in map.values() {
                walk_value(v, out);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                walk_value(v, out);
            }
        }
        _ => {}
    }
}

fn push_from_value(value: &Value, out: &mut Vec<Speakable>) {
    match value {
        Value::Array(arr) => {
            for v in arr {
                if let Some(s) = build_from(v) {
                    if !s.is_empty() {
                        out.push(s);
                    }
                }
            }
        }
        _ => {
            if let Some(s) = build_from(value) {
                if !s.is_empty() {
                    out.push(s);
                }
            }
        }
    }
}

fn build_from(value: &Value) -> Option<Speakable> {
    match value {
        Value::String(url) => Some(Speakable { urls: vec![url.clone()], ..Speakable::default() }),
        Value::Object(map) => {
            let mut sp = Speakable::default();
            sp.css_selectors = collect_strings(map.get("cssSelector"));
            sp.xpaths = collect_strings(map.get("xpath"));
            sp.urls = collect_strings(map.get("url"));
            Some(sp)
        }
        _ => None,
    }
}

fn collect_strings(v: Option<&Value>) -> Vec<String> {
    let Some(v) = v else { return Vec::new() };
    match v {
        Value::String(s) => vec![s.clone()],
        Value::Array(arr) => arr.iter().filter_map(|x| x.as_str().map(str::to_string)).collect(),
        _ => Vec::new(),
    }
}
