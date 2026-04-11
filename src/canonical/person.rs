//! Canonical schema.org `Person` merged across formats.

use crate::canonical::helpers::{
    find_jsonld_of_type, find_microdata_of_type, jsonld_string, microdata_text,
};
use crate::canonical::CanonicalMerge;
use crate::parser_facade::MetaGraph;
use crate::provenance::{FieldSource, FieldValue};
use serde::{Deserialize, Serialize};

/// A schema.org Person merged from JSON-LD / microdata.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Person {
    /// Full name.
    pub name: Option<FieldValue<String>>,
    /// Job title (e.g. `"Software Engineer"`).
    pub job_title: Option<FieldValue<String>>,
    /// Email address.
    pub email: Option<FieldValue<String>>,
    /// Personal homepage / profile URL.
    pub url: Option<FieldValue<String>>,
    /// Avatar / portrait image.
    pub image: Option<FieldValue<String>>,
    /// Affiliated organisation name.
    pub affiliation: Option<FieldValue<String>>,
}

impl CanonicalMerge for Person {
    fn merge_from(graph: &MetaGraph) -> Option<Self> {
        let jsonld = find_jsonld_of_type(&graph.json_ld, &["Person"]);
        let microdata = find_microdata_of_type(&graph.microdata, &["Person"]);

        let mut person = Person::default();

        person.name = jsonld
            .and_then(|o| jsonld_string(o, "name"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Person.name".into())))
            .or_else(|| {
                microdata
                    .and_then(|m| microdata_text(m, "name"))
                    .map(|v| FieldValue::new(v, FieldSource::Microdata("Person.name".into())))
            });

        person.job_title = jsonld
            .and_then(|o| jsonld_string(o, "jobTitle"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Person.jobTitle".into())))
            .or_else(|| {
                microdata
                    .and_then(|m| microdata_text(m, "jobTitle"))
                    .map(|v| FieldValue::new(v, FieldSource::Microdata("Person.jobTitle".into())))
            });

        person.email = jsonld
            .and_then(|o| jsonld_string(o, "email"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Person.email".into())))
            .or_else(|| {
                microdata
                    .and_then(|m| microdata_text(m, "email"))
                    .map(|v| FieldValue::new(v, FieldSource::Microdata("Person.email".into())))
            });

        person.url = jsonld
            .and_then(|o| jsonld_string(o, "url"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Person.url".into())))
            .or_else(|| {
                microdata
                    .and_then(|m| microdata_text(m, "url"))
                    .map(|v| FieldValue::new(v, FieldSource::Microdata("Person.url".into())))
            });

        person.image = jsonld
            .and_then(|o| jsonld_string(o, "image"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Person.image".into())))
            .or_else(|| {
                microdata
                    .and_then(|m| microdata_text(m, "image"))
                    .map(|v| FieldValue::new(v, FieldSource::Microdata("Person.image".into())))
            });

        person.affiliation = jsonld
            .and_then(|o| jsonld_string(o, "affiliation"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Person.affiliation".into())));

        person.name.as_ref()?;
        Some(person)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MetaParser;

    #[test]
    fn merges_jsonld_person() {
        let html = r#"
<script type="application/ld+json">
{
  "@type": "Person",
  "name": "Jane Doe",
  "jobTitle": "Software Engineer",
  "email": "jane@example.com",
  "url": "https://example.com/jane",
  "image": "https://example.com/jane.jpg",
  "affiliation": "Acme Corp"
}
</script>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let person = graph.canonical_person().expect("person");
        assert_eq!(person.name.as_ref().unwrap().value, "Jane Doe");
        assert_eq!(person.job_title.as_ref().unwrap().value, "Software Engineer");
        assert_eq!(person.email.as_ref().unwrap().value, "jane@example.com");
        assert_eq!(person.url.as_ref().unwrap().value, "https://example.com/jane");
        assert_eq!(person.image.as_ref().unwrap().value, "https://example.com/jane.jpg");
        assert_eq!(person.affiliation.as_ref().unwrap().value, "Acme Corp");
    }

    #[test]
    fn microdata_person_falls_through() {
        let html = r#"
<div itemscope itemtype="https://schema.org/Person">
  <span itemprop="name">Bob</span>
  <span itemprop="jobTitle">Engineer</span>
  <meta itemprop="email" content="bob@example.com">
  <link itemprop="url" href="https://example.com/bob">
  <link itemprop="image" href="https://example.com/bob.jpg">
</div>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let person = graph.canonical_person().expect("microdata person");
        assert_eq!(person.name.as_ref().unwrap().value, "Bob");
        assert!(matches!(person.name.as_ref().unwrap().source, FieldSource::Microdata(_)));
        assert_eq!(person.job_title.as_ref().unwrap().value, "Engineer");
    }

    #[test]
    fn unnamed_person_rejected() {
        let html = r#"
<script type="application/ld+json">
{"@type": "Person", "jobTitle": "Ghost"}
</script>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        assert!(graph.canonical_person().is_none());
    }
}
