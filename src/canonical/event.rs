//! Canonical schema.org `Event` merged across formats.

use crate::canonical::helpers::{
    find_jsonld_of_type, find_microdata_of_type, jsonld_string, microdata_text, value_to_string,
};
use crate::canonical::CanonicalMerge;
use crate::parser_facade::MetaGraph;
use crate::provenance::{FieldSource, FieldValue};
use serde::{Deserialize, Serialize};

const EVENT_TYPES: &[&str] = &[
    "Event",
    "BusinessEvent",
    "ChildrensEvent",
    "ComedyEvent",
    "DanceEvent",
    "DeliveryEvent",
    "EducationEvent",
    "ExhibitionEvent",
    "Festival",
    "FoodEvent",
    "LiteraryEvent",
    "MusicEvent",
    "PublicationEvent",
    "SaleEvent",
    "ScreeningEvent",
    "SocialEvent",
    "SportsEvent",
    "TheaterEvent",
    "VisualArtsEvent",
];

/// A schema.org Event merged across formats.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Event {
    /// Event name / title.
    pub name: Option<FieldValue<String>>,
    /// Description.
    pub description: Option<FieldValue<String>>,
    /// ISO 8601 start time.
    pub start_date: Option<FieldValue<String>>,
    /// ISO 8601 end time.
    pub end_date: Option<FieldValue<String>>,
    /// Venue / location string.
    pub location: Option<FieldValue<String>>,
    /// URL for the event page.
    pub url: Option<FieldValue<String>>,
    /// Promotional image.
    pub image: Option<FieldValue<String>>,
}

impl CanonicalMerge for Event {
    fn merge_from(graph: &MetaGraph) -> Option<Self> {
        let jsonld = find_jsonld_of_type(&graph.json_ld, EVENT_TYPES);
        let microdata = find_microdata_of_type(&graph.microdata, EVENT_TYPES);

        let mut event = Event::default();

        event.name = jsonld
            .and_then(|o| jsonld_string(o, "name"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Event.name".into())))
            .or_else(|| {
                microdata
                    .and_then(|m| microdata_text(m, "name"))
                    .map(|v| FieldValue::new(v, FieldSource::Microdata("Event.name".into())))
            });

        event.description = jsonld
            .and_then(|o| jsonld_string(o, "description"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Event.description".into())))
            .or_else(|| {
                microdata
                    .and_then(|m| microdata_text(m, "description"))
                    .map(|v| FieldValue::new(v, FieldSource::Microdata("Event.description".into())))
            });

        event.start_date = jsonld
            .and_then(|o| jsonld_string(o, "startDate"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Event.startDate".into())))
            .or_else(|| {
                microdata
                    .and_then(|m| microdata_text(m, "startDate"))
                    .map(|v| FieldValue::new(v, FieldSource::Microdata("Event.startDate".into())))
            });

        event.end_date = jsonld
            .and_then(|o| jsonld_string(o, "endDate"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Event.endDate".into())));

        event.location = jsonld
            .and_then(|o| o.properties.get("location").and_then(value_to_string))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Event.location".into())))
            .or_else(|| {
                microdata
                    .and_then(|m| microdata_text(m, "location"))
                    .map(|v| FieldValue::new(v, FieldSource::Microdata("Event.location".into())))
            });

        event.url = jsonld
            .and_then(|o| jsonld_string(o, "url"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Event.url".into())));

        event.image = jsonld
            .and_then(|o| o.properties.get("image").and_then(value_to_string))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Event.image".into())));

        event.name.as_ref()?;
        Some(event)
    }
}

#[cfg(test)]
mod tests {
    use crate::{FieldSource, MetaParser};

    #[test]
    fn microdata_event_discovered() {
        let html = r#"
<div itemscope itemtype="https://schema.org/Event">
  <span itemprop="name">Microdata Fest</span>
  <meta itemprop="startDate" content="2026-08-01">
  <span itemprop="location">Venue</span>
</div>
"#;
        let graph = crate::MetaParser::new().parse(html).unwrap();
        let event = graph.canonical_event().expect("event");
        assert_eq!(event.name.as_ref().unwrap().value, "Microdata Fest");
        assert!(matches!(event.name.as_ref().unwrap().source, FieldSource::Microdata(_)));
    }

    #[test]
    fn event_without_name_rejected() {
        let html = r#"
<script type="application/ld+json">
{"@type": "Event", "startDate": "2026-08-01"}
</script>
"#;
        let graph = crate::MetaParser::new().parse(html).unwrap();
        assert!(graph.canonical_event().is_none());
    }

    #[test]
    fn merges_jsonld_event() {
        let html = r#"
<script type="application/ld+json">
{
  "@type": "MusicEvent",
  "name": "Big Concert",
  "startDate": "2026-05-01T20:00:00Z",
  "location": "Madison Square Garden"
}
</script>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let event = graph.canonical_event().expect("event");
        assert_eq!(event.name.unwrap().value, "Big Concert");
        assert_eq!(event.start_date.unwrap().value, "2026-05-01T20:00:00Z");
        assert_eq!(event.location.unwrap().value, "Madison Square Garden");
    }
}
