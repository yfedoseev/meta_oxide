//! Canonical schema.org `BreadcrumbList`.

use crate::canonical::helpers::{find_jsonld_of_type, value_to_string};
use crate::canonical::CanonicalMerge;
use crate::parser_facade::MetaGraph;
use crate::provenance::FieldSource;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// One entry in a breadcrumb trail.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Breadcrumb {
    /// Position in the trail (1-indexed).
    pub position: u32,
    /// Display name (`"Electronics > Keyboards > Mechanical"` → `"Mechanical"`).
    pub name: String,
    /// Optional URL of the breadcrumb target.
    pub url: Option<String>,
}

/// A schema.org BreadcrumbList. Position-ordered.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BreadcrumbList {
    /// All breadcrumbs in order.
    pub items: Vec<Breadcrumb>,
    /// Where the trail came from.
    pub source: Option<FieldSource>,
}

impl CanonicalMerge for BreadcrumbList {
    fn merge_from(graph: &MetaGraph) -> Option<Self> {
        let jsonld = find_jsonld_of_type(&graph.json_ld, &["BreadcrumbList"])?;
        let element_list = jsonld.properties.get("itemListElement")?;
        let elements = match element_list {
            Value::Array(arr) => arr,
            _ => return None,
        };

        let mut items = Vec::new();
        for elem in elements {
            let Value::Object(map) = elem else { continue };
            let position =
                map.get("position").and_then(|v| v.as_u64()).unwrap_or((items.len() + 1) as u64)
                    as u32;
            // The top-level `name` field always wins for the display label,
            // because schema.org allows `item` to be either a nested object
            // or a bare URL. When `item` is a string we treat it as the URL.
            let display_name = map.get("name").and_then(value_to_string);
            let (item_name, item_url) = match map.get("item") {
                Some(Value::Object(item_map)) => (
                    item_map.get("name").and_then(value_to_string),
                    item_map
                        .get("@id")
                        .and_then(value_to_string)
                        .or_else(|| item_map.get("url").and_then(value_to_string)),
                ),
                Some(Value::String(s)) => (None, Some(s.clone())),
                _ => (None, None),
            };
            let name = display_name.or(item_name);
            let url = item_url.or_else(|| map.get("url").and_then(value_to_string));
            if let Some(name) = name {
                items.push(Breadcrumb { position, name, url });
            }
        }

        if items.is_empty() {
            return None;
        }
        items.sort_by_key(|b| b.position);
        Some(BreadcrumbList { items, source: Some(FieldSource::JsonLd("BreadcrumbList".into())) })
    }
}

#[cfg(test)]
mod tests {
    use crate::MetaParser;

    #[test]
    fn empty_breadcrumb_list_rejected() {
        let html = r#"<script type="application/ld+json">{"@type":"BreadcrumbList","itemListElement":[]}</script>"#;
        let graph = crate::MetaParser::new().parse(html).unwrap();
        assert!(graph.canonical_breadcrumbs().is_none());
    }

    #[test]
    fn breadcrumb_positions_sorted() {
        // Deliberately out-of-order in the source.
        let html = r#"
<script type="application/ld+json">
{
  "@type": "BreadcrumbList",
  "itemListElement": [
    {"@type": "ListItem", "position": 3, "name": "Third"},
    {"@type": "ListItem", "position": 1, "name": "First"},
    {"@type": "ListItem", "position": 2, "name": "Second"}
  ]
}
</script>
"#;
        let graph = crate::MetaParser::new().parse(html).unwrap();
        let crumbs = graph.canonical_breadcrumbs().unwrap();
        assert_eq!(crumbs.items[0].name, "First");
        assert_eq!(crumbs.items[1].name, "Second");
        assert_eq!(crumbs.items[2].name, "Third");
    }

    #[test]
    fn breadcrumb_item_as_object() {
        let html = r#"
<script type="application/ld+json">
{
  "@type": "BreadcrumbList",
  "itemListElement": [
    {"@type": "ListItem", "position": 1,
     "item": {"@id": "https://example.com/home", "name": "Home"}}
  ]
}
</script>
"#;
        let graph = crate::MetaParser::new().parse(html).unwrap();
        let crumbs = graph.canonical_breadcrumbs().unwrap();
        assert_eq!(crumbs.items[0].name, "Home");
        assert_eq!(crumbs.items[0].url.as_deref(), Some("https://example.com/home"));
    }

    #[test]
    fn parses_breadcrumb_list() {
        let html = r#"
<script type="application/ld+json">
{
  "@type": "BreadcrumbList",
  "itemListElement": [
    {"@type": "ListItem", "position": 1, "name": "Home", "item": "https://example.com/"},
    {"@type": "ListItem", "position": 2, "name": "Electronics", "item": "https://example.com/electronics"},
    {"@type": "ListItem", "position": 3, "name": "Keyboards"}
  ]
}
</script>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let crumbs = graph.canonical_breadcrumbs().expect("breadcrumbs");
        assert_eq!(crumbs.items.len(), 3);
        assert_eq!(crumbs.items[0].name, "Home");
        assert_eq!(crumbs.items[2].name, "Keyboards");
    }
}
