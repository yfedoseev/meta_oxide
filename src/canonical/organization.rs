//! Canonical schema.org `Organization` (`Organization`, `LocalBusiness`,
//! `Corporation`, …) merged across formats.

use crate::canonical::helpers::{
    find_jsonld_of_type, find_microdata_of_type, jsonld_string, microdata_text,
};
use crate::canonical::CanonicalMerge;
use crate::parser_facade::MetaGraph;
use crate::provenance::{FieldSource, FieldValue};
use serde::{Deserialize, Serialize};

const ORG_TYPES: &[&str] =
    &["Organization", "LocalBusiness", "Corporation", "EducationalOrganization", "NGO"];

/// A schema.org Organization (or any subtype) merged from multiple sources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Organization {
    /// Legal / display name.
    pub name: Option<FieldValue<String>>,
    /// Tagline / short description.
    pub description: Option<FieldValue<String>>,
    /// Homepage URL.
    pub url: Option<FieldValue<String>>,
    /// Logo image URL.
    pub logo: Option<FieldValue<String>>,
    /// Telephone number.
    pub telephone: Option<FieldValue<String>>,
    /// Contact email.
    pub email: Option<FieldValue<String>>,
}

impl CanonicalMerge for Organization {
    fn merge_from(graph: &MetaGraph) -> Option<Self> {
        let jsonld = find_jsonld_of_type(&graph.json_ld, ORG_TYPES);
        let microdata = find_microdata_of_type(&graph.microdata, ORG_TYPES);

        let mut org = Organization::default();

        org.name = jsonld
            .and_then(|o| jsonld_string(o, "name"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Organization.name".into())))
            .or_else(|| {
                microdata
                    .and_then(|m| microdata_text(m, "name"))
                    .map(|v| FieldValue::new(v, FieldSource::Microdata("Organization.name".into())))
            })
            .or_else(|| {
                graph
                    .open_graph
                    .site_name
                    .clone()
                    .map(|v| FieldValue::new(v, FieldSource::OpenGraph("og:site_name".into())))
            });

        org.description = jsonld
            .and_then(|o| jsonld_string(o, "description"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Organization.description".into())))
            .or_else(|| {
                microdata.and_then(|m| microdata_text(m, "description")).map(|v| {
                    FieldValue::new(v, FieldSource::Microdata("Organization.description".into()))
                })
            });

        org.url = jsonld
            .and_then(|o| jsonld_string(o, "url"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Organization.url".into())))
            .or_else(|| {
                microdata
                    .and_then(|m| microdata_text(m, "url"))
                    .map(|v| FieldValue::new(v, FieldSource::Microdata("Organization.url".into())))
            });

        org.logo = jsonld
            .and_then(|o| jsonld_string(o, "logo"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Organization.logo".into())))
            .or_else(|| {
                microdata
                    .and_then(|m| microdata_text(m, "logo"))
                    .map(|v| FieldValue::new(v, FieldSource::Microdata("Organization.logo".into())))
            });

        org.telephone = jsonld
            .and_then(|o| jsonld_string(o, "telephone"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Organization.telephone".into())));

        org.email = jsonld
            .and_then(|o| jsonld_string(o, "email"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Organization.email".into())));

        org.name.as_ref()?;
        Some(org)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MetaParser;

    #[test]
    fn microdata_organization() {
        let html = r#"
<div itemscope itemtype="https://schema.org/Organization">
  <span itemprop="name">Microdata Org</span>
  <meta itemprop="description" content="A company.">
  <link itemprop="url" href="https://org.example.com">
  <link itemprop="logo" href="https://org.example.com/logo.png">
</div>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let org = graph.canonical_organization().expect("microdata org");
        assert_eq!(org.name.as_ref().unwrap().value, "Microdata Org");
        assert!(matches!(org.name.as_ref().unwrap().source, FieldSource::Microdata(_)));
        assert_eq!(org.description.as_ref().unwrap().value, "A company.");
        assert_eq!(org.logo.as_ref().unwrap().value, "https://org.example.com/logo.png");
    }

    #[test]
    fn og_site_name_backfills_organization_name() {
        let html = r#"
<meta property="og:site_name" content="OG Site">
<script type="application/ld+json">
{"@type": "Organization", "description": "Our company"}
</script>
"#;
        // No name in JSON-LD — OG site_name fills it
        let graph = crate::MetaParser::new().parse(html).unwrap();
        let org = graph.canonical_organization().expect("org via og");
        assert_eq!(org.name.as_ref().unwrap().value, "OG Site");
        assert!(matches!(org.name.as_ref().unwrap().source, FieldSource::OpenGraph(_)));
    }

    #[test]
    fn merges_jsonld_local_business() {
        let html = r#"
<script type="application/ld+json">
{
  "@type": "LocalBusiness",
  "name": "Acme Coffee",
  "url": "https://acme.example.com",
  "telephone": "+1-555-0100"
}
</script>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let org = graph.canonical_organization().expect("org");
        assert_eq!(org.name.unwrap().value, "Acme Coffee");
        assert_eq!(org.telephone.unwrap().value, "+1-555-0100");
    }
}
