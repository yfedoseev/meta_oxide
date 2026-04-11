//! Canonical article — schema.org `Article`, `NewsArticle`, `BlogPosting`
//! merged with Open Graph `article:*` and standard meta tags.

use crate::canonical::helpers::{
    find_jsonld_of_type, find_microdata_of_type, first_some, jsonld_string, microdata_text,
    value_to_string,
};
use crate::canonical::CanonicalMerge;
use crate::parser_facade::MetaGraph;
use crate::provenance::{FieldSource, FieldValue};
use crate::types::jsonld::JsonLdObject;
use serde::{Deserialize, Serialize};

const ARTICLE_TYPES: &[&str] =
    &["Article", "NewsArticle", "BlogPosting", "TechArticle", "Report", "ScholarlyArticle"];

/// A schema.org Article (or any subtype) merged across formats.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Article {
    /// Article headline / title.
    pub headline: Option<FieldValue<String>>,
    /// Article description / standfirst.
    pub description: Option<FieldValue<String>>,
    /// Author name (single string — for multi-author pages, take the first).
    pub author: Option<FieldValue<String>>,
    /// Publisher / outlet name.
    pub publisher: Option<FieldValue<String>>,
    /// Lead image URL.
    pub image: Option<FieldValue<String>>,
    /// ISO 8601 publication timestamp.
    pub date_published: Option<FieldValue<String>>,
    /// ISO 8601 last-modified timestamp.
    pub date_modified: Option<FieldValue<String>>,
    /// Canonical URL for the article.
    pub url: Option<FieldValue<String>>,
    /// Section / category (e.g. `"Technology"`).
    pub section: Option<FieldValue<String>>,
    /// Word count if available.
    pub word_count: Option<FieldValue<u32>>,
    /// Full body text if the source exposed it (rare outside JSON-LD).
    pub body: Option<FieldValue<String>>,
}

impl CanonicalMerge for Article {
    fn merge_from(graph: &MetaGraph) -> Option<Self> {
        let jsonld = find_jsonld_of_type(&graph.json_ld, ARTICLE_TYPES);
        let microdata = find_microdata_of_type(&graph.microdata, ARTICLE_TYPES);

        // Only synthesize an Article when at least one structured-data source
        // declared an Article type. OG/meta tags fill in extra fields but
        // they don't, on their own, prove the page is an article.
        if jsonld.is_none() && microdata.is_none() {
            return None;
        }

        let mut article = Article::default();

        article.headline = first_some([
            jsonld
                .and_then(|o| jsonld_string(o, "headline").or_else(|| jsonld_string(o, "name")))
                .map(|v| FieldValue::new(v, FieldSource::JsonLd("Article.headline".into()))),
            microdata
                .and_then(|m| microdata_text(m, "headline").or_else(|| microdata_text(m, "name")))
                .map(|v| FieldValue::new(v, FieldSource::Microdata("Article.headline".into()))),
            graph
                .open_graph
                .title
                .clone()
                .map(|v| FieldValue::new(v, FieldSource::OpenGraph("og:title".into()))),
            graph.meta.title.clone().map(|v| FieldValue::new(v, FieldSource::Meta("title".into()))),
        ]);

        article.description = first_some([
            jsonld
                .and_then(|o| jsonld_string(o, "description"))
                .map(|v| FieldValue::new(v, FieldSource::JsonLd("Article.description".into()))),
            microdata
                .and_then(|m| microdata_text(m, "description"))
                .map(|v| FieldValue::new(v, FieldSource::Microdata("Article.description".into()))),
            graph
                .open_graph
                .description
                .clone()
                .map(|v| FieldValue::new(v, FieldSource::OpenGraph("og:description".into()))),
            graph
                .meta
                .description
                .clone()
                .map(|v| FieldValue::new(v, FieldSource::Meta("description".into()))),
        ]);

        article.author = first_some([
            jsonld
                .and_then(jsonld_author)
                .map(|v| FieldValue::new(v, FieldSource::JsonLd("Article.author".into()))),
            microdata
                .and_then(|m| microdata_text(m, "author"))
                .map(|v| FieldValue::new(v, FieldSource::Microdata("Article.author".into()))),
            graph
                .dublin_core
                .creator
                .clone()
                .map(|v| FieldValue::new(v, FieldSource::DublinCore("dc.creator".into()))),
            graph
                .meta
                .author
                .clone()
                .map(|v| FieldValue::new(v, FieldSource::Meta("author".into()))),
        ]);

        article.publisher = first_some([
            jsonld
                .and_then(jsonld_publisher)
                .map(|v| FieldValue::new(v, FieldSource::JsonLd("Article.publisher".into()))),
            graph
                .open_graph
                .site_name
                .clone()
                .map(|v| FieldValue::new(v, FieldSource::OpenGraph("og:site_name".into()))),
        ]);

        article.image = first_some([
            jsonld
                .and_then(|o| o.properties.get("image").and_then(value_to_string))
                .map(|v| FieldValue::new(v, FieldSource::JsonLd("Article.image".into()))),
            microdata
                .and_then(|m| microdata_text(m, "image"))
                .map(|v| FieldValue::new(v, FieldSource::Microdata("Article.image".into()))),
            graph
                .open_graph
                .image
                .clone()
                .map(|v| FieldValue::new(v, FieldSource::OpenGraph("og:image".into()))),
        ]);

        article.date_published = first_some([
            jsonld
                .and_then(|o| jsonld_string(o, "datePublished"))
                .map(|v| FieldValue::new(v, FieldSource::JsonLd("Article.datePublished".into()))),
            microdata.and_then(|m| microdata_text(m, "datePublished")).map(|v| {
                FieldValue::new(v, FieldSource::Microdata("Article.datePublished".into()))
            }),
            graph
                .dublin_core
                .date
                .clone()
                .map(|v| FieldValue::new(v, FieldSource::DublinCore("dc.date".into()))),
        ]);

        article.date_modified = jsonld
            .and_then(|o| jsonld_string(o, "dateModified"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Article.dateModified".into())));

        article.url = first_some([
            jsonld
                .and_then(|o| jsonld_string(o, "url"))
                .map(|v| FieldValue::new(v, FieldSource::JsonLd("Article.url".into()))),
            graph
                .open_graph
                .url
                .clone()
                .map(|v| FieldValue::new(v, FieldSource::OpenGraph("og:url".into()))),
            graph
                .meta
                .canonical
                .clone()
                .map(|v| FieldValue::new(v, FieldSource::Meta("canonical".into()))),
        ]);

        article.section = jsonld
            .and_then(|o| jsonld_string(o, "articleSection"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Article.articleSection".into())));

        article.word_count = jsonld
            .and_then(|o| o.properties.get("wordCount").and_then(|v| v.as_u64()))
            .map(|n| FieldValue::new(n as u32, FieldSource::JsonLd("Article.wordCount".into())));

        article.body = jsonld
            .and_then(|o| jsonld_string(o, "articleBody"))
            .map(|v| FieldValue::new(v, FieldSource::JsonLd("Article.articleBody".into())));

        // Require at least a headline for the article to be useful.
        article.headline.as_ref()?;
        Some(article)
    }
}

fn jsonld_author(item: &JsonLdObject) -> Option<String> {
    item.properties.get("author").and_then(value_to_string)
}

fn jsonld_publisher(item: &JsonLdObject) -> Option<String> {
    item.properties.get("publisher").and_then(value_to_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MetaParser;

    const NEWS_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
<title>Plain Headline</title>
<meta property="og:title" content="OG Headline">
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "NewsArticle",
  "headline": "JSON-LD Headline",
  "description": "An interesting story.",
  "author": "Jane Reporter",
  "publisher": "The Daily Example",
  "datePublished": "2026-04-10T08:30:00Z",
  "dateModified": "2026-04-10T09:00:00Z",
  "image": "https://example.com/lead.jpg",
  "url": "https://example.com/story"
}
</script>
</head>
<body></body>
</html>
"#;

    #[test]
    fn jsonld_news_article_wins_over_meta() {
        let graph = MetaParser::new().parse(NEWS_HTML).unwrap();
        let article = graph.canonical_article().expect("article");

        assert_eq!(article.headline.as_ref().unwrap().value, "JSON-LD Headline");
        assert!(matches!(article.headline.as_ref().unwrap().source, FieldSource::JsonLd(_)));
        assert_eq!(article.author.as_ref().unwrap().value, "Jane Reporter");
        assert_eq!(article.publisher.as_ref().unwrap().value, "The Daily Example");
        assert_eq!(article.date_published.as_ref().unwrap().value, "2026-04-10T08:30:00Z");
    }

    #[test]
    fn returns_none_when_no_article_type_present() {
        let html = r#"
            <title>Plain Headline</title>
            <meta property="og:title" content="OG Headline">
        "#;
        let graph = MetaParser::new().parse(html).unwrap();
        // No JSON-LD or microdata Article-typed object exists, so even
        // though OG/meta carry headline-shaped data, we don't fabricate
        // a `canonical_article()`. The Article merger only fires when at
        // least one source declared the page as Article/NewsArticle/etc.
        assert!(graph.canonical_article().is_none());
    }

    #[test]
    fn returns_none_for_non_article_page() {
        let html = r#"<html><body><h1>Hi</h1></body></html>"#;
        let graph = MetaParser::new().parse(html).unwrap();
        assert!(graph.canonical_article().is_none());
    }

    #[test]
    fn microdata_article_is_discovered() {
        let html = r#"
<article itemscope itemtype="https://schema.org/NewsArticle">
  <h1 itemprop="headline">Microdata Story</h1>
  <meta itemprop="description" content="From microdata.">
  <meta itemprop="author" content="Microdata Author">
  <meta itemprop="datePublished" content="2026-03-01T00:00:00Z">
</article>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let article = graph.canonical_article().expect("microdata article");
        assert_eq!(article.headline.as_ref().unwrap().value, "Microdata Story");
        assert!(matches!(article.headline.as_ref().unwrap().source, FieldSource::Microdata(_)));
        assert_eq!(article.author.as_ref().unwrap().value, "Microdata Author");
    }

    #[test]
    fn jsonld_article_without_headline_rejected() {
        // Phase 3 contract: we require a headline — an article with nothing
        // but a publisher field isn't worth returning.
        let html = r#"
<script type="application/ld+json">
{"@type": "Article", "publisher": "Empty Gazette"}
</script>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        assert!(graph.canonical_article().is_none());
    }

    #[test]
    fn jsonld_article_word_count_parsed() {
        let html = r#"
<script type="application/ld+json">
{"@type": "Article", "headline": "Long Read", "wordCount": 2500}
</script>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let article = graph.canonical_article().unwrap();
        assert_eq!(article.word_count.as_ref().unwrap().value, 2500);
    }
}
