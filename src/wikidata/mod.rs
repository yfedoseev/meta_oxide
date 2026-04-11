//! Wikidata entity JSON parser → [`crate::canonical::KnowledgeGraph`].
//!
//! Wikidata's REST API returns entities as JSON documents shaped like:
//!
//! ```json
//! {
//!   "id": "Q937",
//!   "labels": { "en": { "language": "en", "value": "Albert Einstein" } },
//!   "descriptions": { "en": { "language": "en", "value": "..." } },
//!   "claims": {
//!     "P31": [...],   // instance of
//!     "P18": [...],   // image
//!     ...
//!   }
//! }
//! ```
//!
//! [`parse_entity`] converts the most useful fields into a
//! [`KnowledgeGraph`]. It does not enumerate every Wikidata claim — that
//! would explode the type — but it captures the bits an agent typically
//! needs (label, description, type label, image, Wikipedia link).
//!
//! ```no_run
//! let json = "...";
//! let kg = meta_oxide::wikidata::parse_entity(json).unwrap();
//! println!("{}: {}", kg.name, kg.description.unwrap_or_default());
//! ```

use crate::canonical::KnowledgeGraph;
use serde_json::Value;

/// Parse a Wikidata entity JSON document. Returns `Err` only when the input
/// isn't valid JSON; otherwise returns a [`KnowledgeGraph`] with whatever
/// fields could be extracted.
pub fn parse_entity(json: &str) -> Result<KnowledgeGraph, serde_json::Error> {
    let value: Value = serde_json::from_str(json)?;
    Ok(parse_entity_value(&value))
}

/// Same as [`parse_entity`] but takes an already-parsed [`Value`].
pub fn parse_entity_value(value: &Value) -> KnowledgeGraph {
    let mut kg = KnowledgeGraph::default();

    // Wikidata API can wrap entities in `entities: { Qxxx: { ... } }`.
    let entity = value
        .get("entities")
        .and_then(|e| e.as_object())
        .and_then(|map| map.values().next())
        .unwrap_or(value);

    if let Some(qid) = entity.get("id").and_then(|v| v.as_str()) {
        kg.wikidata_id = Some(qid.to_string());
    }

    if let Some(label) = first_lang_value(entity.get("labels"), "en") {
        kg.name = label;
    }

    if let Some(desc) = first_lang_value(entity.get("descriptions"), "en") {
        kg.description = Some(desc);
    }

    let claims = entity.get("claims").and_then(|c| c.as_object());

    // P31 = "instance of" → entity type label
    if let Some(claims) = claims {
        if let Some(type_label) = entity_type_from_claim(claims.get("P31")) {
            kg.entity_type = Some(type_label);
        }
        // P18 = "image" (Commons filename)
        if let Some(image) = first_claim_string(claims.get("P18")) {
            kg.image_url = Some(format!(
                "https://commons.wikimedia.org/wiki/Special:FilePath/{}",
                image.replace(' ', "_")
            ));
        }
    }

    // sitelinks → Wikipedia URL
    if let Some(sitelinks) = entity.get("sitelinks").and_then(|s| s.as_object()) {
        if let Some(wiki) = sitelinks.get("enwiki") {
            if let Some(url) = wiki.get("url").and_then(|u| u.as_str()) {
                kg.wikipedia_url = Some(url.to_string());
            } else if let Some(title) = wiki.get("title").and_then(|t| t.as_str()) {
                kg.wikipedia_url =
                    Some(format!("https://en.wikipedia.org/wiki/{}", title.replace(' ', "_")));
            }
        }
    }

    kg
}

fn first_lang_value(node: Option<&Value>, lang: &str) -> Option<String> {
    let map = node?.as_object()?;
    map.get(lang).and_then(|v| v.get("value")).and_then(|v| v.as_str()).map(str::to_string)
}

fn first_claim_string(claim: Option<&Value>) -> Option<String> {
    let arr = claim?.as_array()?;
    arr.iter().find_map(|c| {
        c.get("mainsnak")
            .and_then(|m| m.get("datavalue"))
            .and_then(|d| d.get("value"))
            .and_then(|v| v.as_str().map(str::to_string))
    })
}

fn entity_type_from_claim(claim: Option<&Value>) -> Option<String> {
    let arr = claim?.as_array()?;
    arr.iter().find_map(|c| {
        c.get("mainsnak")
            .and_then(|m| m.get("datavalue"))
            .and_then(|d| d.get("value"))
            .and_then(|v| v.get("id"))
            .and_then(|id| id.as_str().map(str::to_string))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
{
  "id": "Q937",
  "labels": {
    "en": {"language": "en", "value": "Albert Einstein"},
    "de": {"language": "de", "value": "Albert Einstein"}
  },
  "descriptions": {
    "en": {"language": "en", "value": "German-born theoretical physicist"}
  },
  "claims": {
    "P31": [{"mainsnak": {"datavalue": {"value": {"id": "Q5"}}}}],
    "P18": [{"mainsnak": {"datavalue": {"value": "Einstein 1921 by F Schmutzer.jpg"}}}]
  },
  "sitelinks": {
    "enwiki": {"site": "enwiki", "title": "Albert Einstein"}
  }
}
"#;

    #[test]
    fn parses_basic_entity() {
        let kg = parse_entity(SAMPLE).unwrap();
        assert_eq!(kg.name, "Albert Einstein");
        assert_eq!(kg.wikidata_id.as_deref(), Some("Q937"));
        assert_eq!(kg.description.as_deref(), Some("German-born theoretical physicist"));
        assert_eq!(kg.entity_type.as_deref(), Some("Q5"));
        assert!(kg.image_url.as_deref().unwrap().contains("Special:FilePath"));
        assert_eq!(
            kg.wikipedia_url.as_deref(),
            Some("https://en.wikipedia.org/wiki/Albert_Einstein")
        );
    }

    #[test]
    fn handles_entities_wrapper() {
        let wrapped = format!(r#"{{"entities": {{"Q937": {} }} }}"#, SAMPLE);
        let kg = parse_entity(&wrapped).unwrap();
        assert_eq!(kg.name, "Albert Einstein");
    }

    #[test]
    fn invalid_json_errors() {
        assert!(parse_entity("not json").is_err());
    }

    #[test]
    fn missing_claims_doesnt_panic() {
        let json = r#"{"id": "Q42", "labels": {"en": {"language":"en","value":"Douglas Adams"}}}"#;
        let kg = parse_entity(json).unwrap();
        assert_eq!(kg.name, "Douglas Adams");
        assert_eq!(kg.wikidata_id.as_deref(), Some("Q42"));
        assert!(kg.entity_type.is_none());
        assert!(kg.image_url.is_none());
    }

    #[test]
    fn missing_label_yields_empty_name() {
        let json = r#"{"id": "Q1"}"#;
        let kg = parse_entity(json).unwrap();
        assert!(kg.name.is_empty());
        assert_eq!(kg.wikidata_id.as_deref(), Some("Q1"));
    }

    #[test]
    fn sitelink_with_url_field() {
        let json = r#"
{
  "id": "Q42",
  "labels": {"en": {"language": "en", "value": "x"}},
  "sitelinks": {
    "enwiki": {"site": "enwiki", "title": "x", "url": "https://en.wikipedia.org/custom"}
  }
}
"#;
        let kg = parse_entity(json).unwrap();
        assert_eq!(kg.wikipedia_url.as_deref(), Some("https://en.wikipedia.org/custom"));
    }

    #[test]
    fn image_filename_normalised_to_commons_url() {
        let json = r#"
{
  "id": "Q1",
  "labels": {"en": {"language": "en", "value": "x"}},
  "claims": {
    "P18": [{"mainsnak": {"datavalue": {"value": "Some image.jpg"}}}]
  }
}
"#;
        let kg = parse_entity(json).unwrap();
        assert!(kg.image_url.as_ref().unwrap().contains("Some_image.jpg"));
        assert!(kg.image_url.as_ref().unwrap().contains("Special:FilePath"));
    }
}
