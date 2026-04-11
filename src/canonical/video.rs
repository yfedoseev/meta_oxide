//! Canonical schema.org `VideoObject` merged across formats.

use crate::canonical::helpers::{find_jsonld_of_type, jsonld_string, value_to_string};
use crate::canonical::CanonicalMerge;
use crate::parser_facade::MetaGraph;
use crate::provenance::{FieldSource, FieldValue};
use serde::{Deserialize, Serialize};

/// A schema.org VideoObject merged across formats.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VideoObject {
    /// Video title.
    pub name: Option<FieldValue<String>>,
    /// Description / synopsis.
    pub description: Option<FieldValue<String>>,
    /// Direct video URL (`og:video`, JSON-LD `contentUrl`).
    pub content_url: Option<FieldValue<String>>,
    /// Embed URL.
    pub embed_url: Option<FieldValue<String>>,
    /// Thumbnail still.
    pub thumbnail_url: Option<FieldValue<String>>,
    /// ISO 8601 duration string (`"PT4M33S"`).
    pub duration: Option<FieldValue<String>>,
    /// Upload timestamp.
    pub upload_date: Option<FieldValue<String>>,
}

impl CanonicalMerge for VideoObject {
    fn merge_from(graph: &MetaGraph) -> Option<Self> {
        let jsonld = find_jsonld_of_type(&graph.json_ld, &["VideoObject", "Movie"]);

        let mut video = VideoObject::default();

        video.name = jsonld
            .and_then(|o| jsonld_string(o, "name"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("VideoObject.name".into())))
            .or_else(|| {
                graph
                    .open_graph
                    .title
                    .clone()
                    .map(|v| FieldValue::new(v, FieldSource::OpenGraph("og:title".into())))
            });

        video.description =
            jsonld
                .and_then(|o| jsonld_string(o, "description"))
                .map(|v| FieldValue::new(v, FieldSource::JsonLd("VideoObject.description".into())))
                .or_else(|| {
                    graph.open_graph.description.clone().map(|v| {
                        FieldValue::new(v, FieldSource::OpenGraph("og:description".into()))
                    })
                });

        video.content_url = jsonld
            .and_then(|o| jsonld_string(o, "contentUrl"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("VideoObject.contentUrl".into())))
            .or_else(|| {
                graph.open_graph.videos.first().map(|v| {
                    FieldValue::new(v.url.clone(), FieldSource::OpenGraph("og:video".into()))
                })
            });

        video.embed_url = jsonld
            .and_then(|o| jsonld_string(o, "embedUrl"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("VideoObject.embedUrl".into())));

        video.thumbnail_url = jsonld
            .and_then(|o| {
                o.properties
                    .get("thumbnailUrl")
                    .and_then(value_to_string)
                    .or_else(|| o.properties.get("thumbnail").and_then(value_to_string))
            })
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("VideoObject.thumbnailUrl".into())))
            .or_else(|| {
                graph
                    .open_graph
                    .image
                    .clone()
                    .map(|v| FieldValue::new(v, FieldSource::OpenGraph("og:image".into())))
            });

        video.duration = jsonld
            .and_then(|o| jsonld_string(o, "duration"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("VideoObject.duration".into())));

        video.upload_date = jsonld
            .and_then(|o| jsonld_string(o, "uploadDate"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("VideoObject.uploadDate".into())));

        if video.name.is_none() && video.content_url.is_none() {
            None
        } else {
            Some(video)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MetaParser;

    #[test]
    fn og_video_backfills_content_url() {
        let html = r#"
<meta property="og:video" content="https://example.com/clip.mp4">
<meta property="og:video:type" content="video/mp4">
<script type="application/ld+json">
{"@type": "VideoObject", "name": "OG-backed clip"}
</script>
"#;
        let graph = crate::MetaParser::new().parse(html).unwrap();
        let video = graph.canonical_video().expect("video");
        assert_eq!(video.name.as_ref().unwrap().value, "OG-backed clip");
        // JSON-LD didn't set contentUrl; og:video should fill it.
        assert_eq!(video.content_url.as_ref().unwrap().value, "https://example.com/clip.mp4");
    }

    #[test]
    fn og_video_only_returns_video() {
        let html = r#"
<meta property="og:title" content="Video Title">
<meta property="og:video" content="https://example.com/v.mp4">
<script type="application/ld+json">
{"@type": "VideoObject"}
</script>
"#;
        let graph = crate::MetaParser::new().parse(html).unwrap();
        let video = graph.canonical_video().expect("video from og");
        assert_eq!(video.name.as_ref().unwrap().value, "Video Title");
        assert_eq!(video.content_url.as_ref().unwrap().value, "https://example.com/v.mp4");
    }

    #[test]
    fn empty_video_rejected() {
        let html = r#"<script type="application/ld+json">{"@type": "VideoObject"}</script>"#;
        let graph = crate::MetaParser::new().parse(html).unwrap();
        assert!(graph.canonical_video().is_none());
    }

    #[test]
    fn merges_jsonld_video_object() {
        let html = r#"
<script type="application/ld+json">
{
  "@type": "VideoObject",
  "name": "Demo Video",
  "contentUrl": "https://example.com/v.mp4",
  "duration": "PT4M33S",
  "uploadDate": "2026-01-01"
}
</script>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let video = graph.canonical_video().expect("video");
        assert_eq!(video.name.unwrap().value, "Demo Video");
        assert_eq!(video.content_url.unwrap().value, "https://example.com/v.mp4");
        assert_eq!(video.duration.unwrap().value, "PT4M33S");
    }
}
