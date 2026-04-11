//! Canonical schema.org `FAQPage` — extracts question/answer pairs.

use crate::canonical::helpers::{find_jsonld_of_type, value_to_string};
use crate::canonical::CanonicalMerge;
use crate::parser_facade::MetaGraph;
use crate::provenance::FieldSource;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// One question/answer pair from an FAQPage.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QAPair {
    /// The question text.
    pub question: String,
    /// The accepted answer text, if present.
    pub answer: Option<String>,
}

/// A schema.org FAQPage with all question/answer pairs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FAQPage {
    /// All Q&A pairs in document order.
    pub questions: Vec<QAPair>,
    /// Where the page came from (always JSON-LD for now).
    pub source: Option<FieldSource>,
}

impl CanonicalMerge for FAQPage {
    fn merge_from(graph: &MetaGraph) -> Option<Self> {
        let jsonld = find_jsonld_of_type(&graph.json_ld, &["FAQPage"])?;
        let main_entity = jsonld.properties.get("mainEntity")?;
        let entities = match main_entity {
            Value::Array(arr) => arr.clone(),
            single => vec![single.clone()],
        };

        let mut questions = Vec::new();
        for entity in &entities {
            let Value::Object(map) = entity else { continue };
            let Some(question) = map.get("name").and_then(value_to_string) else { continue };
            let answer = map.get("acceptedAnswer").and_then(|aa| match aa {
                Value::Object(answer_map) => answer_map.get("text").and_then(value_to_string),
                _ => value_to_string(aa),
            });
            questions.push(QAPair { question, answer });
        }

        if questions.is_empty() {
            return None;
        }
        Some(FAQPage { questions, source: Some(FieldSource::JsonLd("FAQPage".into())) })
    }
}

#[cfg(test)]
mod tests {
    use crate::MetaParser;

    #[test]
    fn single_main_entity_as_object() {
        // mainEntity can be a single object, not an array.
        let html = r#"
<script type="application/ld+json">
{
  "@type": "FAQPage",
  "mainEntity": {
    "@type": "Question",
    "name": "Only question?",
    "acceptedAnswer": {"@type": "Answer", "text": "Yes."}
  }
}
</script>
"#;
        let graph = crate::MetaParser::new().parse(html).unwrap();
        let faq = graph.canonical_faq().expect("faq");
        assert_eq!(faq.questions.len(), 1);
        assert_eq!(faq.questions[0].question, "Only question?");
        assert_eq!(faq.questions[0].answer.as_deref(), Some("Yes."));
    }

    #[test]
    fn empty_main_entity_rejected() {
        let html = r#"
<script type="application/ld+json">
{"@type": "FAQPage", "mainEntity": []}
</script>
"#;
        let graph = crate::MetaParser::new().parse(html).unwrap();
        assert!(graph.canonical_faq().is_none());
    }

    #[test]
    fn parses_faq_page() {
        let html = r#"
<script type="application/ld+json">
{
  "@type": "FAQPage",
  "mainEntity": [
    {
      "@type": "Question",
      "name": "What is meta_oxide?",
      "acceptedAnswer": {"@type": "Answer", "text": "A metadata extraction library."}
    },
    {
      "@type": "Question",
      "name": "Is it fast?",
      "acceptedAnswer": {"@type": "Answer", "text": "Yes, very."}
    }
  ]
}
</script>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let faq = graph.canonical_faq().expect("faq");
        assert_eq!(faq.questions.len(), 2);
        assert_eq!(faq.questions[0].question, "What is meta_oxide?");
        assert_eq!(faq.questions[0].answer.as_deref(), Some("A metadata extraction library."));
    }
}
