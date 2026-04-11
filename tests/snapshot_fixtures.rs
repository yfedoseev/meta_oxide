//! Snapshot regression tests against curated real-world-shaped fixtures.
//!
//! Every test parses a fixture through [`meta_oxide::MetaParser`] (or one of
//! the direct parsers in `serp`/`wikipedia`/`wikidata`) and asserts that the
//! canonical output matches a committed `insta` snapshot. When extractor
//! behaviour changes, `cargo insta review` shows the diff and you either
//! accept it (intentional change) or reject (regression).
//!
//! Fixtures live inline as `&str` constants to keep the tests self-contained
//! and diff-able. For richer real-world HTML corpora, a `tests/corpus/` dir
//! with on-disk files is the next step — left for a follow-up.

use insta::{assert_json_snapshot, with_settings};
use meta_oxide::{serp, wikidata, wikipedia, MetaParser};
use serde_json::json;

/// E-commerce product page — JSON-LD Product with Offer + embedded Review.
const PRODUCT_JSONLD: &str = r#"
<!DOCTYPE html>
<html>
<head>
<title>Mechanical Keyboard — Acme Store</title>
<meta property="og:title" content="Mechanical Keyboard">
<meta property="og:site_name" content="Acme Store">
<link rel="canonical" href="https://acme.example.com/kb-001">
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Product",
  "name": "Mechanical Keyboard",
  "description": "RGB mechanical keyboard with Cherry MX Red switches.",
  "image": "https://acme.example.com/kb-001.jpg",
  "brand": "Acme",
  "sku": "KB-001",
  "gtin": "0123456789012",
  "url": "https://acme.example.com/kb-001",
  "offers": {
    "@type": "Offer",
    "price": "129.99",
    "priceCurrency": "USD",
    "availability": "https://schema.org/InStock"
  }
}
</script>
</head>
<body><h1>Mechanical Keyboard</h1></body>
</html>
"#;

/// News article — NewsArticle JSON-LD.
const NEWS_ARTICLE: &str = r#"
<!DOCTYPE html>
<html>
<head>
<title>Breaking: Example Occurs</title>
<meta property="og:image" content="https://example.com/hero.jpg">
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "NewsArticle",
  "headline": "Breaking: Example Occurs",
  "description": "A thorough examination of an example scenario.",
  "datePublished": "2026-04-10T08:30:00Z",
  "dateModified": "2026-04-10T09:00:00Z",
  "author": "Jane Reporter",
  "publisher": "The Daily Example",
  "articleSection": "Technology",
  "url": "https://example.com/story",
  "wordCount": 1500
}
</script>
</head>
<body></body>
</html>
"#;

/// Recipe page — schema.org Recipe with structured ingredients/instructions.
const RECIPE: &str = r#"
<script type="application/ld+json">
{
  "@type": "Recipe",
  "name": "Simple Pancakes",
  "description": "Fluffy breakfast pancakes in 15 minutes.",
  "author": "Chef Bob",
  "prepTime": "PT5M",
  "cookTime": "PT10M",
  "totalTime": "PT15M",
  "recipeYield": "4 servings",
  "recipeIngredient": ["1 cup flour", "1 cup milk", "1 egg", "pinch of salt"],
  "recipeInstructions": [
    {"@type": "HowToStep", "text": "Mix dry ingredients."},
    {"@type": "HowToStep", "text": "Add wet ingredients and whisk."},
    {"@type": "HowToStep", "text": "Cook on hot skillet."}
  ]
}
</script>
"#;

/// Event listing.
const EVENT: &str = r#"
<script type="application/ld+json">
{
  "@type": "MusicEvent",
  "name": "Example Festival 2026",
  "description": "Annual music festival.",
  "startDate": "2026-07-15T18:00:00Z",
  "endDate": "2026-07-15T23:00:00Z",
  "location": "Central Park, New York",
  "url": "https://example.com/festival-2026",
  "image": "https://example.com/festival.jpg"
}
</script>
"#;

/// Breadcrumb trail — out of order to verify sorting.
const BREADCRUMBS: &str = r#"
<script type="application/ld+json">
{
  "@type": "BreadcrumbList",
  "itemListElement": [
    {"@type": "ListItem", "position": 3, "name": "Keyboards", "item": "https://example.com/electronics/keyboards"},
    {"@type": "ListItem", "position": 1, "name": "Home", "item": "https://example.com/"},
    {"@type": "ListItem", "position": 2, "name": "Electronics", "item": "https://example.com/electronics"}
  ]
}
</script>
"#;

/// FAQ page — multiple Q/A pairs.
const FAQ: &str = r#"
<script type="application/ld+json">
{
  "@type": "FAQPage",
  "mainEntity": [
    {
      "@type": "Question",
      "name": "What is meta_oxide?",
      "acceptedAnswer": {"@type": "Answer", "text": "A universal HTML metadata extraction library."}
    },
    {
      "@type": "Question",
      "name": "Which formats does it support?",
      "acceptedAnswer": {"@type": "Answer", "text": "Thirteen, including JSON-LD, Open Graph, microdata, and RDFa."}
    }
  ]
}
</script>
"#;

/// Microdata-only product — tests the degradation fallback when JSON-LD absent.
const PRODUCT_MICRODATA: &str = r#"
<div itemscope itemtype="https://schema.org/Product">
  <span itemprop="name">Mouse Pad</span>
  <meta itemprop="description" content="Large gaming mouse pad.">
  <meta itemprop="brand" content="Acme">
  <div itemprop="offers" itemscope itemtype="https://schema.org/Offer">
    <meta itemprop="price" content="19.99">
    <meta itemprop="priceCurrency" content="USD">
    <link itemprop="availability" href="https://schema.org/InStock">
  </div>
</div>
"#;

/// Page with only Open Graph tags — tests OG fallback through `MetaGraph::get`.
const OG_ONLY: &str = r#"
<meta property="og:title" content="OG Only Page">
<meta property="og:description" content="Description from OG alone.">
<meta property="og:image" content="https://example.com/og.jpg">
<meta property="og:url" content="https://example.com/og-only">
<meta property="og:site_name" content="OG Example">
"#;

/// Heuristic-only page — no structured data at all.
const HEURISTIC_ONLY: &str = r#"
<html>
<body>
<h1>Unstructured Heading</h1>
<p>A sufficiently long first paragraph that clears the twenty-character heuristic threshold.</p>
<article>The article body here is long enough to be considered a real body element for heuristic purposes.</article>
<time datetime="2026-04-10"></time>
<img src="/hero.jpg">
</body>
</html>
"#;

/// Wikipedia article infobox.
const WIKIPEDIA_INFOBOX: &str = r#"
<html>
<head><link rel="canonical" href="https://en.wikipedia.org/wiki/Test_Subject"></head>
<body>
<h1 id="firstHeading">Test Subject</h1>
<table class="infobox vcard">
<tr><th>Born</th><td>1 January 1900</td></tr>
<tr><th>Died</th><td>31 December 1999</td></tr>
<tr><th>Nationality</th><td>Example</td></tr>
<tr><th>Known for</th><td>Being a test fixture</td></tr>
<tr><td><img src="//upload.wikimedia.org/portrait.jpg"></td></tr>
</table>
</body>
</html>
"#;

/// Wikidata Special:EntityData JSON blob.
const WIKIDATA_ENTITY: &str = r#"
{
  "id": "Q42",
  "labels": {
    "en": {"language": "en", "value": "Example Entity"},
    "de": {"language": "de", "value": "Beispiel-Entität"}
  },
  "descriptions": {
    "en": {"language": "en", "value": "A fictional test entity."}
  },
  "claims": {
    "P31": [{"mainsnak": {"datavalue": {"value": {"id": "Q5"}}}}],
    "P18": [{"mainsnak": {"datavalue": {"value": "Example image.jpg"}}}]
  },
  "sitelinks": {
    "enwiki": {"site": "enwiki", "title": "Example Entity"}
  }
}
"#;

/// Google SERP fragment — minimal shape matching the real DOM selectors.
const GOOGLE_SERP: &str = r#"
<html>
<body>
<div class="xpdopen">
Asynchronous programming in Rust is a first-class language feature supporting
non-blocking concurrent execution through the futures abstraction.
<a href="https://rust-lang.github.io/async-book/"><h3>Async Book</h3></a>
</div>
<a href="https://doc.rust-lang.org/book/ch16-00-concurrency.html"><h3>Rust Book — Concurrency</h3></a>
<p>Concurrency support is one of Rust's headline features in the standard library.</p>
<a href="https://tokio.rs/tokio/tutorial"><h3>Tokio Tutorial</h3></a>
<p>Tokio is an asynchronous runtime for the Rust programming language.</p>
<div class="related-question-pair">
<div role="heading">What is async in Rust?</div>
<div>A language feature for writing futures-based concurrent code.</div>
<a href="https://rust-lang.github.io/async-book/">async-book</a>
</div>
</body>
</html>
"#;

fn bind_snapshot_settings<F: FnOnce()>(f: F) {
    with_settings!({
        sort_maps => true,
        omit_expression => true,
    }, {
        f();
    });
}

#[test]
fn snapshot_canonical_product_from_jsonld() {
    bind_snapshot_settings(|| {
        let graph = MetaParser::new().parse(PRODUCT_JSONLD).unwrap();
        let product = graph.canonical_product().expect("product");
        assert_json_snapshot!("product_jsonld", product);
    });
}

#[test]
fn snapshot_canonical_product_from_microdata() {
    bind_snapshot_settings(|| {
        let graph = MetaParser::new().parse(PRODUCT_MICRODATA).unwrap();
        let product = graph.canonical_product().expect("product");
        assert_json_snapshot!("product_microdata", product);
    });
}

#[test]
fn snapshot_canonical_article() {
    bind_snapshot_settings(|| {
        let graph = MetaParser::new().parse(NEWS_ARTICLE).unwrap();
        let article = graph.canonical_article().expect("article");
        assert_json_snapshot!("article_news", article);
    });
}

#[test]
fn snapshot_canonical_recipe() {
    bind_snapshot_settings(|| {
        let graph = MetaParser::new().parse(RECIPE).unwrap();
        let recipe = graph.canonical_recipe().expect("recipe");
        assert_json_snapshot!("recipe_pancakes", recipe);
    });
}

#[test]
fn snapshot_canonical_event() {
    bind_snapshot_settings(|| {
        let graph = MetaParser::new().parse(EVENT).unwrap();
        let event = graph.canonical_event().expect("event");
        assert_json_snapshot!("event_music_festival", event);
    });
}

#[test]
fn snapshot_canonical_breadcrumbs() {
    bind_snapshot_settings(|| {
        let graph = MetaParser::new().parse(BREADCRUMBS).unwrap();
        let crumbs = graph.canonical_breadcrumbs().expect("breadcrumbs");
        assert_json_snapshot!("breadcrumbs_sorted", crumbs);
    });
}

#[test]
fn snapshot_canonical_faq() {
    bind_snapshot_settings(|| {
        let graph = MetaParser::new().parse(FAQ).unwrap();
        let faq = graph.canonical_faq().expect("faq");
        assert_json_snapshot!("faq_page", faq);
    });
}

#[test]
fn snapshot_og_only_via_get_queries() {
    bind_snapshot_settings(|| {
        let graph = MetaParser::new().parse(OG_ONLY).unwrap();
        let queries = json!({
            "title": graph.get("title"),
            "description": graph.get("description"),
            "image": graph.get("image"),
            "url": graph.get("url"),
            "site_name": graph.get("site_name"),
        });
        assert_json_snapshot!("og_only_queries", queries);
    });
}

#[test]
fn snapshot_heuristic_only_fills() {
    bind_snapshot_settings(|| {
        let graph = MetaParser::new().parse(HEURISTIC_ONLY).unwrap();
        assert_json_snapshot!("heuristic_only_fills", graph.heuristic_fills);
    });
}

#[test]
fn snapshot_wikipedia_infobox() {
    bind_snapshot_settings(|| {
        let kg = wikipedia::parse_infobox(WIKIPEDIA_INFOBOX);
        assert_json_snapshot!("wikipedia_infobox", kg);
    });
}

#[test]
fn snapshot_wikidata_entity() {
    bind_snapshot_settings(|| {
        let kg = wikidata::parse_entity(WIKIDATA_ENTITY).unwrap();
        assert_json_snapshot!("wikidata_entity", kg);
    });
}

#[test]
fn snapshot_google_serp_parse() {
    bind_snapshot_settings(|| {
        let serp = serp::parse_google(GOOGLE_SERP);
        assert_json_snapshot!("google_serp", serp);
    });
}
