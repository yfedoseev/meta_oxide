//! Integration tests for the unified `MetaParser` facade.
//!
//! These exercise the full pipeline — MetaParser → shared-DOM extractors →
//! canonical types → heuristic layer — against fixtures that combine several
//! metadata formats on one page, the way real-world HTML does. Unit tests in
//! `src/canonical/*.rs` already cover each canonical merge in isolation;
//! this file verifies they still do the right thing when the page is messy.

use meta_oxide::{FieldSource, FormatMask, MetaParser};

const RICH_PRODUCT_PAGE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
<title>Acme Mechanical Keyboard — Buy Online</title>
<meta name="description" content="Plain-tag description.">
<meta name="author" content="Plain Author">
<meta property="og:title" content="Acme Keyboard">
<meta property="og:description" content="OG description of the keyboard.">
<meta property="og:image" content="https://example.com/og-kb.jpg">
<meta property="og:site_name" content="Acme Store">
<meta property="og:url" content="https://acme.example.com/kb">
<meta name="twitter:card" content="summary_large_image">
<meta name="twitter:title" content="Acme Keyboard (Twitter)">
<meta name="dc.creator" content="DC Author">
<meta name="dc.language" content="en">
<link rel="canonical" href="https://acme.example.com/kb">
<link rel="alternate" href="/fr/kb" hreflang="fr">
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Product",
  "name": "Mechanical Keyboard",
  "description": "RGB mechanical keyboard with Cherry MX switches.",
  "image": "https://example.com/kb.jpg",
  "brand": "Acme",
  "sku": "KB-001",
  "gtin": "0123456789012",
  "url": "https://acme.example.com/kb",
  "offers": {
    "@type": "Offer",
    "price": "129.99",
    "priceCurrency": "USD",
    "availability": "https://schema.org/InStock"
  },
  "review": [
    {"@type": "Review", "author": "Alice", "reviewBody": "Crisp keys.",
     "reviewRating": {"@type": "Rating", "ratingValue": "5", "bestRating": "5"}}
  ]
}
</script>
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "BreadcrumbList",
  "itemListElement": [
    {"@type": "ListItem", "position": 1, "name": "Home", "item": "https://acme.example.com/"},
    {"@type": "ListItem", "position": 2, "name": "Keyboards", "item": "https://acme.example.com/kb/"}
  ]
}
</script>
</head>
<body>
<article>
  <h1>Acme Mechanical Keyboard — Product Page</h1>
  <p>This is the first long paragraph that should populate the description heuristic if structured data were missing.</p>
  <time datetime="2026-03-15">March 15, 2026</time>
</article>
</body>
</html>
"#;

#[test]
fn rich_product_page_populates_all_formats() {
    let graph = MetaParser::new()
        .with_base_url("https://acme.example.com/kb")
        .parse(RICH_PRODUCT_PAGE)
        .unwrap();

    // Format-native fields all populated.
    assert!(!graph.json_ld.is_empty(), "json-ld empty");
    assert_eq!(graph.meta.title.as_deref(), Some("Acme Mechanical Keyboard — Buy Online"));
    assert_eq!(graph.open_graph.title.as_deref(), Some("Acme Keyboard"));
    assert_eq!(graph.twitter.title.as_deref(), Some("Acme Keyboard (Twitter)"));
    assert_eq!(graph.dublin_core.creator.as_deref(), Some("DC Author"));
    assert_eq!(graph.meta.canonical.as_deref(), Some("https://acme.example.com/kb"));
    assert_eq!(graph.meta.alternate.len(), 1);
}

#[test]
fn rich_product_page_title_priority_is_jsonld() {
    let graph = MetaParser::new().parse(RICH_PRODUCT_PAGE).unwrap();
    let title = graph.get("title").expect("title");
    assert_eq!(title.value, "Mechanical Keyboard");
    assert!(
        matches!(title.source, FieldSource::JsonLd(_)),
        "expected JSON-LD source, got {:?}",
        title.source
    );
}

#[test]
fn rich_product_page_description_priority_is_jsonld() {
    let graph = MetaParser::new().parse(RICH_PRODUCT_PAGE).unwrap();
    let desc = graph.get("description").expect("description");
    assert_eq!(desc.value, "RGB mechanical keyboard with Cherry MX switches.");
    assert!(matches!(desc.source, FieldSource::JsonLd(_)));
}

#[test]
fn rich_product_page_canonical_product_merge() {
    let graph = MetaParser::new().parse(RICH_PRODUCT_PAGE).unwrap();
    let product = graph.canonical_product().expect("canonical product");

    assert_eq!(product.name.as_ref().unwrap().value, "Mechanical Keyboard");
    assert_eq!(product.brand.as_ref().unwrap().value, "Acme");
    assert_eq!(product.sku.as_ref().unwrap().value, "KB-001");
    assert_eq!(product.gtin.as_ref().unwrap().value, "0123456789012");

    let price = product.price.as_ref().expect("price");
    assert_eq!(price.value.amount, 129.99);
    assert_eq!(price.value.currency, "USD");
    // Provenance: price should trace back to JSON-LD offers path.
    match &price.source {
        FieldSource::JsonLd(path) => assert!(path.contains("offers.price")),
        other => panic!("expected JSON-LD price source, got {:?}", other),
    }

    let availability = product.availability.as_ref().unwrap();
    assert_eq!(availability.value, meta_oxide::canonical::Availability::InStock);
}

#[test]
fn rich_product_page_breadcrumbs_extracted() {
    let graph = MetaParser::new().parse(RICH_PRODUCT_PAGE).unwrap();
    let crumbs = graph.canonical_breadcrumbs().expect("breadcrumbs");
    assert_eq!(crumbs.items.len(), 2);
    assert_eq!(crumbs.items[0].name, "Home");
    assert_eq!(crumbs.items[1].name, "Keyboards");
}

#[test]
fn structured_data_suppresses_heuristic_description() {
    // With og:description present, the first-paragraph heuristic should not fire.
    let graph = MetaParser::new().parse(RICH_PRODUCT_PAGE).unwrap();
    assert!(
        !graph.heuristic_fills.contains_key("description"),
        "heuristic description should not have fired; got {:?}",
        graph.heuristic_fills.get("description")
    );
}

#[test]
fn time_datetime_populates_heuristic_published_time() {
    // datePublished is not in the JSON-LD on this fixture, so the heuristic
    // should run.
    let graph = MetaParser::new().parse(RICH_PRODUCT_PAGE).unwrap();
    let t = graph.heuristic_fills.get("published_time").expect("published_time");
    assert_eq!(t.value, "2026-03-15");
}

#[test]
fn no_structured_data_page_degrades_to_heuristics_only() {
    let bare = r#"
<html>
<body>
<h1>Unstructured Heading</h1>
<p>A first paragraph long enough to trip the heuristic threshold.</p>
<article>Long article body.</article>
<time datetime="2026-04-10"></time>
</body>
</html>
"#;
    let graph = MetaParser::new().parse(bare).unwrap();

    // No canonical entities survive.
    assert!(graph.canonical_product().is_none());
    assert!(graph.canonical_article().is_none());

    // But heuristics fill the basic fields.
    let title = graph.get("title").expect("heuristic title");
    assert_eq!(title.value, "Unstructured Heading");
    assert!(matches!(title.source, FieldSource::Heuristic(_)));

    assert!(graph.heuristic_fills.contains_key("description"));
    assert!(graph.heuristic_fills.contains_key("body"));
}

#[test]
fn format_mask_selective_extraction() {
    let graph =
        MetaParser::new().with_formats(FormatMask::JSON_LD).parse(RICH_PRODUCT_PAGE).unwrap();

    // Only JSON-LD should be populated.
    assert!(!graph.json_ld.is_empty());
    assert!(graph.meta.title.is_none(), "meta skipped");
    assert!(graph.open_graph.title.is_none(), "og skipped");
    assert!(graph.twitter.title.is_none(), "twitter skipped");

    // canonical_product still works because it only needs JSON-LD.
    assert!(graph.canonical_product().is_some());
}

#[test]
fn parse_dom_reuses_already_parsed_html() {
    let dom = scraper::Html::parse_document(RICH_PRODUCT_PAGE);
    let parser = MetaParser::new().with_base_url("https://acme.example.com/kb");
    let graph = parser.parse_dom(&dom).unwrap();
    assert_eq!(graph.json_ld.len(), 2);
    assert!(graph.canonical_product().is_some());
}

#[test]
fn parse_is_behaviorally_identical_across_repeated_invocations() {
    // Idempotency — parsing the same HTML twice yields structurally equivalent graphs.
    let a = MetaParser::new().parse(RICH_PRODUCT_PAGE).unwrap();
    let b = MetaParser::new().parse(RICH_PRODUCT_PAGE).unwrap();
    assert_eq!(a.json_ld.len(), b.json_ld.len());
    assert_eq!(a.open_graph.title, b.open_graph.title);
    assert_eq!(a.canonical_product().is_some(), b.canonical_product().is_some());
    assert_eq!(a.heuristic_fills.len(), b.heuristic_fills.len());
}

#[test]
fn reviews_surfaced_from_embedded_product() {
    let graph = MetaParser::new().parse(RICH_PRODUCT_PAGE).unwrap();
    // Review is embedded inside the Product — not a top-level @type.
    // canonical_reviews walks the full json_ld list, so whether embedded reviews
    // are surfaced depends on whether the JSON-LD extractor unwraps them.
    // This test locks in current behavior: embedded reviews under Product are
    // NOT currently surfaced as top-level reviews (that's a known limitation).
    let reviews = graph.canonical_reviews();
    // Current behavior: 0 — review is inside Product.review, not top-level.
    // When @graph unwrapping or review promotion is added later, this becomes > 0.
    assert_eq!(reviews.len(), 0, "reviews inside Product.review are not yet promoted to top-level");
}
