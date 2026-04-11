//! Canonical schema.org `Review` — supports multiple per page.

use crate::canonical::helpers::{
    find_all_jsonld_of_type, jsonld_string, parse_number, value_to_string,
};
use crate::canonical::CanonicalMergeAll;
use crate::parser_facade::MetaGraph;
use crate::provenance::{FieldSource, FieldValue};
use crate::types::jsonld::JsonLdObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A schema.org Review merged across formats.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Review {
    /// Author display name.
    pub author: Option<FieldValue<String>>,
    /// Free-text body of the review.
    pub body: Option<FieldValue<String>>,
    /// Rating value (1-5 scale typically).
    pub rating: Option<FieldValue<f64>>,
    /// Rating ceiling (`5` for the common 1-5 scale).
    pub best_rating: Option<FieldValue<f64>>,
    /// ISO 8601 publication date.
    pub date_published: Option<FieldValue<String>>,
    /// What the review is about (book title, product name, …).
    pub item_reviewed: Option<FieldValue<String>>,
}

impl CanonicalMergeAll for Review {
    fn merge_all_from(graph: &MetaGraph) -> Vec<Self> {
        let mut out = Vec::new();
        for item in find_all_jsonld_of_type(&graph.json_ld, &["Review"]) {
            out.push(build_review(item));
        }
        out
    }
}

fn build_review(item: &JsonLdObject) -> Review {
    let mut review = Review::default();

    review.author = item
        .properties
        .get("author")
        .and_then(value_to_string)
        .map(|v| FieldValue::new(v, FieldSource::JsonLd("Review.author".into())));

    review.body = jsonld_string(item, "reviewBody")
        .or_else(|| jsonld_string(item, "description"))
        .map(|v| FieldValue::new(v, FieldSource::JsonLd("Review.reviewBody".into())));

    if let Some(rating) = item.properties.get("reviewRating") {
        let (value, best) = parse_rating(rating);
        review.rating = value.map(|v| {
            FieldValue::new(v, FieldSource::JsonLd("Review.reviewRating.ratingValue".into()))
        });
        review.best_rating = best.map(|v| {
            FieldValue::new(v, FieldSource::JsonLd("Review.reviewRating.bestRating".into()))
        });
    }

    review.date_published = jsonld_string(item, "datePublished")
        .map(|v| FieldValue::new(v, FieldSource::JsonLd("Review.datePublished".into())));

    review.item_reviewed = item
        .properties
        .get("itemReviewed")
        .and_then(value_to_string)
        .map(|v| FieldValue::new(v, FieldSource::JsonLd("Review.itemReviewed".into())));

    review
}

fn parse_rating(value: &Value) -> (Option<f64>, Option<f64>) {
    match value {
        Value::Object(map) => {
            let rating = map.get("ratingValue").and_then(parse_number);
            let best = map.get("bestRating").and_then(parse_number);
            (rating, best)
        }
        Value::Number(n) => (n.as_f64(), None),
        Value::String(s) => (s.parse::<f64>().ok(), None),
        _ => (None, None),
    }
}

#[cfg(test)]
mod tests {
    use crate::MetaParser;

    #[test]
    fn rating_as_scalar_number() {
        let html = r#"
<script type="application/ld+json">
{"@type": "Review", "author": "C", "reviewRating": 4}
</script>
"#;
        let graph = crate::MetaParser::new().parse(html).unwrap();
        let reviews = graph.canonical_reviews();
        assert_eq!(reviews.len(), 1);
        assert_eq!(reviews[0].rating.as_ref().unwrap().value, 4.0);
    }

    #[test]
    fn review_with_no_rating_still_captured() {
        let html = r#"
<script type="application/ld+json">
{"@type": "Review", "author": "D", "reviewBody": "Text only."}
</script>
"#;
        let graph = crate::MetaParser::new().parse(html).unwrap();
        let reviews = graph.canonical_reviews();
        assert_eq!(reviews.len(), 1);
        assert!(reviews[0].rating.is_none());
        assert_eq!(reviews[0].body.as_ref().unwrap().value, "Text only.");
    }

    #[test]
    fn no_reviews_on_plain_page() {
        let graph = crate::MetaParser::new().parse("<html></html>").unwrap();
        assert!(graph.canonical_reviews().is_empty());
    }

    #[test]
    fn collects_all_reviews_from_page() {
        let html = r#"
<script type="application/ld+json">
{
  "@type": "Review",
  "author": "Alice",
  "reviewBody": "Loved it.",
  "reviewRating": {"@type": "Rating", "ratingValue": "5", "bestRating": "5"}
}
</script>
<script type="application/ld+json">
{
  "@type": "Review",
  "author": "Bob",
  "reviewBody": "Meh.",
  "reviewRating": {"@type": "Rating", "ratingValue": 3}
}
</script>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let reviews = graph.canonical_reviews();
        assert_eq!(reviews.len(), 2);
        assert_eq!(reviews[0].author.as_ref().unwrap().value, "Alice");
        assert_eq!(reviews[0].rating.as_ref().unwrap().value, 5.0);
        assert_eq!(reviews[1].rating.as_ref().unwrap().value, 3.0);
    }
}
