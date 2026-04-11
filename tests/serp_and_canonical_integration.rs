//! Integration tests for SERP parsing and canonical article/organization flows.

use meta_oxide::{serp, MetaParser};

#[test]
fn serp_parser_extracts_organic_featured_snippet_and_paa() {
    let html = r#"
<html>
<body>
<div class="xpdopen">
  A featured-snippet body that's comfortably longer than the twenty character threshold.
  <a href="https://rust-lang.github.io/async-book/">
    <h3>Async Book</h3>
  </a>
</div>
<a href="https://example.com/one"><h3>First result</h3></a>
<p>A substantial snippet for the first result that goes well past forty characters.</p>
<a href="https://example.com/two"><h3>Second result</h3></a>
<p>Another substantial snippet over forty characters long for matching purposes.</p>
<div class="related-question-pair">
  <div role="heading">How does X work?</div>
  <div>By doing Y.</div>
  <a href="https://example.com/x">source</a>
</div>
</body>
</html>
"#;

    let graph = serp::parse_google(html);
    assert!(graph.organic.len() >= 2);
    assert_eq!(graph.organic[0].position, 1);
    assert!(graph.organic.iter().any(|r| r.url.contains("example.com/one")));
    assert!(graph.organic.iter().any(|r| r.url.contains("example.com/two")));

    let fs = graph.featured_snippet.expect("featured snippet");
    assert!(fs.text.contains("featured-snippet body"));
    assert!(fs.url.contains("async-book"));

    assert_eq!(graph.people_also_ask.len(), 1);
    assert_eq!(graph.people_also_ask[0].question, "How does X work?");
}

#[test]
fn canonical_article_merges_jsonld_and_meta() {
    let html = r#"
<html>
<head>
<title>Plain Headline</title>
<meta name="description" content="Plain description.">
<meta name="author" content="Plain Author">
<meta property="og:image" content="https://example.com/lead.jpg">
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "NewsArticle",
  "headline": "JSON-LD Headline",
  "datePublished": "2026-04-10T08:30:00Z",
  "author": "JSON-LD Author",
  "publisher": "The Example Gazette",
  "articleSection": "Technology"
}
</script>
</head>
<body><h1>Heading</h1></body>
</html>
"#;

    let graph = MetaParser::new().parse(html).unwrap();
    let article = graph.canonical_article().expect("article");

    assert_eq!(article.headline.as_ref().unwrap().value, "JSON-LD Headline");
    assert_eq!(article.author.as_ref().unwrap().value, "JSON-LD Author");
    assert_eq!(article.publisher.as_ref().unwrap().value, "The Example Gazette");
    assert_eq!(article.section.as_ref().unwrap().value, "Technology");
    // Image comes from og:image since JSON-LD didn't set one.
    assert_eq!(article.image.as_ref().unwrap().value, "https://example.com/lead.jpg");
}

#[test]
fn canonical_organization_via_local_business_microdata() {
    let html = r#"
<div itemscope itemtype="https://schema.org/LocalBusiness">
  <span itemprop="name">Corner Coffee</span>
  <span itemprop="description">Neighborhood espresso bar.</span>
  <link itemprop="url" href="https://corner.example.com">
  <link itemprop="logo" href="https://corner.example.com/logo.png">
</div>
"#;
    let graph = MetaParser::new().parse(html).unwrap();
    let org = graph.canonical_organization().expect("org");
    assert_eq!(org.name.as_ref().unwrap().value, "Corner Coffee");
    assert_eq!(org.logo.as_ref().unwrap().value, "https://corner.example.com/logo.png");
}

#[test]
fn wikipedia_infobox_produces_usable_knowledge_graph() {
    let html = r#"
<h1 id="firstHeading">Curie</h1>
<link rel="canonical" href="https://en.wikipedia.org/wiki/Marie_Curie">
<table class="infobox vcard">
<tr><th>Born</th><td>7 November 1867</td></tr>
<tr><th>Died</th><td>4 July 1934</td></tr>
<tr><th>Nationality</th><td>Polish-French</td></tr>
</table>
"#;
    let kg = meta_oxide::wikipedia::parse_infobox(html);
    assert_eq!(kg.name, "Curie");
    assert_eq!(kg.entity_type.as_deref(), Some("Person"));
    assert_eq!(kg.attributes.get("Born").map(String::as_str), Some("7 November 1867"));
    assert_eq!(kg.attributes.get("Nationality").map(String::as_str), Some("Polish-French"));
}

#[test]
fn wikidata_and_wikipedia_share_knowledge_graph_type() {
    // Compile-time proof: both modules return the same KnowledgeGraph type,
    // so consumers can treat them interchangeably.
    let wiki_kg = meta_oxide::wikipedia::parse_infobox("<h1>Test</h1>");
    let wikidata_json = r#"{"id":"Q1","labels":{"en":{"language":"en","value":"Test"}}}"#;
    let data_kg = meta_oxide::wikidata::parse_entity(wikidata_json).unwrap();

    let kgs: Vec<meta_oxide::canonical::KnowledgeGraph> = vec![wiki_kg, data_kg];
    assert_eq!(kgs.len(), 2);
    for kg in &kgs {
        assert_eq!(kg.name, "Test");
    }
}
