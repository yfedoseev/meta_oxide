# `scraper_oxide` → `meta_oxide` Requirements

Gaps `scraper_oxide` needs from `meta_oxide` to delete its duplicate
metadata extraction layer (`src/pipeline/schema.rs` + parts of
`src/search/google.rs`) and consolidate everything in one place.

Written by Claude on 2026-04-10 after reviewing meta_oxide's current
module tree. This is a wishlist, not a demand — some items are
"nice-to-have", a few are blockers for the consolidation.

---

## Current state of meta_oxide (as I see it)

**What exists and is great:**
- `extractors::meta` — standard `<meta>` tags (Phase 1)
- `extractors::social` — Open Graph, Twitter Cards (Phase 2)
- `extractors::jsonld` — JSON-LD / schema.org (Phase 3)
- `extractors::microdata` — HTML microdata (Phase 4)
- `extractors::rdfa` — RDFa (W3C)
- `extractors::microformats` — h-card, h-entry, etc. (Phase 7)
- `extractors::dublin_core` — Dublin Core (Phase 9)
- `extractors::oembed`, `extractors::manifest`, `extractors::rel_links`

13 formats covered. This is much further along than what `scraper_oxide`
reimplements locally.

**What's missing for scraper_oxide's use cases:**
A unified parser facade, entity canonicalization, SERP-specific
structures, and a heuristic fallback layer. Details below.

---

## Requirement 1: Unified `MetaParser` with one call → everything

### Motivation

Today, `scraper_oxide::pipeline::schema::SchemaExtractor` exists because
callers don't want to make N calls to N extractors and manually merge
results with priority ordering. It tries JSON-LD → Open Graph → Microdata
→ CSS → heuristics in order and returns whichever filled the field first,
with source attribution.

This layer should not live in `scraper_oxide`. Every caller — reader_oxide,
scraper_oxide, agentique — wants the same thing: give me HTML, tell me
everything you found, and tell me *where* you found it.

### Proposed API

```rust
use meta_oxide::{MetaParser, MetaGraph, FieldSource};

// One call, all formats.
let graph: MetaGraph = MetaParser::new()
    .with_base_url("https://example.com/product/42")
    .parse(html)?;

// Query a specific canonical field across all formats.
let title: Option<FieldValue> = graph.get("title");
if let Some(field) = title {
    println!("{} (from {})", field.value, field.source);
    // => "Mechanical Keyboard (from json_ld:name)"
}

// Raw access per format.
let og: &OpenGraph = graph.open_graph();
let jsonld: &[JsonLdItem] = graph.json_ld_items();

// Canonical schema.org entity, merged from all sources.
let product: Option<Product> = graph.canonical_product();
```

### Why the facade matters

- **One pass over HTML.** Currently each extractor re-parses the
  document. A unified parser parses once and dispatches.
- **Source attribution.** Every value carries where it came from —
  critical for agent debugging ("did this price come from JSON-LD or a
  guess from the `.price` class?").
- **Priority policy is library-owned.** JSON-LD wins over OG wins over
  microdata is well-established; callers shouldn't have to restate it.

### What `scraper_oxide` will delete once this lands

- `src/pipeline/schema.rs` (entire file, ~900 lines + 19 tests)
- `pipeline::SchemaExtractor`, `FieldSpec`, `FieldSource`, `ExtractionResult`,
  `extract_schema`, `Heuristic`
- The re-exports in `pipeline/mod.rs`

---

## Requirement 2: Canonical entity types (cross-format)

### Motivation

Today, a product's price can appear as:

- `json_ld: offers.price` — `"29.99"` or `29.99` or `{"@value": "29.99"}`
- `microdata: [itemprop=price]` — text content of a `<span>`
- `og: product:price:amount` — meta tag content
- `heuristic: .price` — CSS-matched text node

Each extractor returns its own shape, so callers have to normalize. That
normalization belongs in meta_oxide once, not in every consumer.

### Proposed API

```rust
use meta_oxide::canonical::{Product, Article, Person, Organization, Event, Recipe};

// Canonical, merged, deduplicated.
let product: Product = graph.canonical_product()?;
assert_eq!(product.name, "Mechanical Keyboard");
assert_eq!(product.price, Some(Money { amount: 29.99, currency: "USD" }));
assert_eq!(product.availability, Some(Availability::InStock));
assert_eq!(product.brand, Some("Acme"));

// Each field carries provenance for debugging.
assert_eq!(product.name_source(), FieldSource::JsonLd("name"));
assert_eq!(product.price_source(), FieldSource::JsonLd("offers.price"));
```

### Canonical types needed

| Type            | Schema.org mapping  | Used by                                   |
| --------------- | ------------------- | ----------------------------------------- |
| `Product`       | schema:Product      | e-commerce scraping, price monitoring     |
| `Article`       | schema:Article      | news, blogs, content sites                |
| `Person`        | schema:Person       | author extraction, about pages            |
| `Organization`  | schema:Organization | company pages, about pages                |
| `Event`         | schema:Event        | event listings, calendars                 |
| `Recipe`        | schema:Recipe       | food sites                                |
| `VideoObject`   | schema:VideoObject  | media extraction                          |
| `BreadcrumbList`| schema:BreadcrumbList | site structure inference               |
| `Review`        | schema:Review       | product reviews, ratings                  |
| `FAQPage`       | schema:FAQPage      | Q&A extraction, agents                    |

Most of these probably map 1:1 to types that already exist in
`types::jsonld`. The request is to lift them to a format-independent
canonical layer so consumers don't care whether a Product came from
JSON-LD or microdata or OG.

---

## Requirement 3: SERP-specific types and parser

### Motivation

`scraper_oxide` has to parse Google SERPs for the Serper competitor.
Featured snippets, People Also Ask, and Knowledge Graph panels are all
structured entities — they just live inside Google's DOM instead of
`<script type="application/ld+json">`.

Every one of these is metadata extraction. It should live in meta_oxide
with all the other metaformat parsers.

### Proposed API

```rust
use meta_oxide::serp::{SerpGraph, FeaturedSnippet, PeopleAlsoAsk, KnowledgeGraph, SiteLinks};

let serp: SerpGraph = meta_oxide::serp::parse_google(html)?;

if let Some(fs) = serp.featured_snippet {
    println!("{}: {}", fs.title, fs.text);
    println!("from {}", fs.source_url);
}

for q in &serp.people_also_ask {
    println!("Q: {}", q.question);
    if let Some(a) = &q.answer { println!("A: {}", a); }
}

if let Some(kg) = serp.knowledge_graph {
    println!("Entity: {} ({})", kg.name, kg.entity_type);
    for (k, v) in &kg.attributes { println!("  {}: {}", k, v); }
}
```

### Types

```rust
pub struct SerpGraph {
    pub featured_snippet: Option<FeaturedSnippet>,
    pub knowledge_graph: Option<KnowledgeGraph>,
    pub people_also_ask: Vec<PeopleAlsoAsk>,
    pub related_searches: Vec<String>,
    pub site_links: Vec<SiteLink>,
}

pub struct FeaturedSnippet {
    pub text: String,
    pub title: String,
    pub source_url: String,
    pub snippet_type: FeaturedSnippetType, // Paragraph, List, Table, Video
}

pub struct KnowledgeGraph {
    pub name: String,
    pub entity_type: String, // "Person", "Organization", "Place", "Movie", ...
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub wikipedia_url: Option<String>,
    pub wikidata_id: Option<String>,
    pub attributes: HashMap<String, String>,
    pub related_entities: Vec<String>,
}

pub struct PeopleAlsoAsk {
    pub question: String,
    pub answer: Option<String>,
    pub source_url: Option<String>,
    pub source_title: Option<String>,
}
```

### Parsers needed

| Engine      | Priority | Notes                                                    |
| ----------- | -------- | -------------------------------------------------------- |
| Google      | P0       | The one we need first; most churn-prone                  |
| Bing        | P1       | Has its own KG + featured snippets                       |
| DuckDuckGo  | P1       | Abstracts / Instant Answers                              |
| Yandex      | P2       | Limited SERP features                                    |

### Why this belongs in meta_oxide

- It's metadata extraction from HTML — same domain as every other
  extractor in the crate.
- The output types (`KnowledgeGraph`, `FeaturedSnippet`) are reusable
  outside SERP contexts (Wikidata, Wikipedia infoboxes, rich snippets on
  regular pages).
- Google's DOM changes constantly. Centralizing the parsers makes
  drift-fixes a one-line change instead of a multi-repo scramble.

### What `scraper_oxide` will delete once this lands

- `src/search/google.rs::parse_google_serp` (the whole parser)
- `src/search/google.rs::parse_organic`, `parse_featured_snippet`, `parse_paa`
- `FeaturedSnippet`, `PeopleAlsoAsk` types in `src/search/mod.rs`
- The SERP-specific test fixtures move too: `tests/fixtures/google_serp.html`

`scraper_oxide` keeps: fetching the SERP with the stealth client, then
one call: `meta_oxide::serp::parse_google(html)`.

---

## Requirement 4: Heuristic / CSS fallback layer

### Motivation

Real pages have no structured data. For a scraper aimed at the open web,
heuristic fallback matters more than schema.org purity.

`scraper_oxide::pipeline::schema::SchemaExtractor` has a `Heuristic` enum
and CSS-attribute fallback — when JSON-LD / OG / microdata all return
nothing, it tries:

- `<title>` → title
- `<meta name="description">` → description
- First `<h1>` → title fallback
- `<article>` or largest `<main>` child → body
- `<img>` near the top → primary image
- `[itemprop=price]`, `.price`, `[data-price]` → price
- First `<time>` → published date

These rules are well-understood and should live in one place. `reader_oxide`
probably has some of this already; consolidating in meta_oxide makes
the heuristic layer a peer of the format extractors instead of scattered.

### Proposed API

```rust
let graph = MetaParser::new()
    .with_heuristics(true)  // default on
    .parse(html)?;

// Values pulled by heuristics are still in the graph, just tagged
// with a different FieldSource.
if let Some(title) = graph.get("title") {
    match title.source {
        FieldSource::JsonLd(_) => println!("from JSON-LD"),
        FieldSource::OpenGraph(_) => println!("from Open Graph"),
        FieldSource::Heuristic(rule) => println!("from heuristic: {}", rule),
        _ => {}
    }
}
```

---

## Requirement 5: Wikidata / Wikipedia direct parsers (nice-to-have)

### Motivation

When an agent wants "the canonical representation of an entity", the
ideal source is Wikidata, not Google's rendering of it. A future path:

1. Agent scrapes Google SERP → gets Knowledge Graph panel with a
   `wikidata_id`
2. Agent fetches the Wikidata entity JSON directly
3. meta_oxide parses it into the same `KnowledgeGraph` type

This is P2 — not needed for the first consolidation — but worth keeping
the types flexible enough to support it.

### Proposed API

```rust
// Wikipedia infobox HTML → KnowledgeGraph
let kg = meta_oxide::wikipedia::parse_infobox(html)?;

// Wikidata entity JSON → KnowledgeGraph
let kg = meta_oxide::wikidata::parse_entity(json)?;
```

---

## Requirement 6: Streaming-friendly API for large documents

### Motivation

Not a blocker today but worth noting: extracting from a 500 KB HTML
document shouldn't require holding the whole thing + the parsed DOM +
every extractor's intermediate state in memory simultaneously. Agents
running under token/memory budgets care.

### Shape

```rust
// Option A: give the parser a &str, get back a graph (current model)
let graph = MetaParser::new().parse(html)?;

// Option B: accept a Read, stream internally, return graph
let graph = MetaParser::new().parse_reader(reader)?;
```

Not urgent. Mentioning it so the API design doesn't paint itself into a
corner.

---

## Non-requirements (explicit)

To keep scope tight, `scraper_oxide` is **not** asking meta_oxide to:

- Fetch HTML. meta_oxide gets strings; fetching is scraper_oxide's job.
- Run LLMs / heuristic re-writing. All of meta_oxide's logic is
  deterministic pattern matching. No AI inside.
- Understand site-specific schemas. Domain extractors (Amazon-specific,
  Walmart-specific, etc.) belong in the caller, not the metaformat layer.
- Cache results. Caching is a separate concern and lives in the caller.
- Track change diffs between page versions. That's a consumer concern.

---

## Priority summary

| # | Requirement                            | Priority | scraper_oxide impact                     |
| - | -------------------------------------- | -------- | ---------------------------------------- |
| 1 | Unified `MetaParser` + `MetaGraph`     | **P0**   | Deletes `pipeline::schema` (900 LOC)     |
| 2 | Canonical entity types                 | **P0**   | Deletes `pipeline::schema::FieldSpec`    |
| 3 | SERP parsers + types                   | **P0**   | Deletes `search/google.rs` SERP logic    |
| 4 | Heuristic / CSS fallback               | P1       | Lets us drop the `Heuristic` enum        |
| 5 | Wikidata / Wikipedia direct parsers    | P2       | Future agent recipes                     |
| 6 | Streaming-friendly API                 | P2       | Large-page memory wins                   |

Items 1–3 together are what blocks the `scraper_oxide` consolidation.
Items 4–6 are follow-ups.

---

## How `scraper_oxide` plans to use this after the refactor

```rust
// Before (today):
let schema = pipeline::schema::SchemaExtractor::product();
let result = pipeline::extract_schema(html, &schema)?;
// + manual merge with reader_oxide's structured_data field
// + separate google.rs SERP parser

// After:
let graph = meta_oxide::MetaParser::new()
    .with_base_url(&url)
    .parse(html)?;
let product = graph.canonical_product();
// scraper_oxide does zero metadata parsing itself.

// Serper server:
let serp_html = stealth_client.fetch(&google_url).await?;
let serp = meta_oxide::serp::parse_google(&serp_html.raw_html)?;
// scraper_oxide does zero SERP parsing itself.
```

`scraper_oxide` becomes a pure fetch-and-stealth library. `meta_oxide`
becomes the universal metadata parser. Clean SRP split.

---

## Open questions for the meta_oxide author

1. **Facade or composition?** Would you prefer a single `MetaParser`
   struct that orchestrates the existing extractors, or a top-level
   `extract_all(html)` function that returns a `MetaGraph` by calling
   each extractor internally? Both work; I don't care which.

2. **Canonical types vs. format-native types.** Do you see the canonical
   types (Requirement 2) as a new top-level module
   (`meta_oxide::canonical::Product`), or as enriched versions of the
   existing `types::jsonld::Product`-equivalent? My instinct is
   new module, because "canonical" is a cross-format concern.

3. **SERP as its own module vs. inside extractors/.** I wrote
   `meta_oxide::serp` above but `meta_oxide::extractors::serp::google`
   would also fit the existing layout. Your call.

4. **Breaking changes OK?** scraper_oxide is fine with meta_oxide
   shipping a 0.2 with the new facade even if it breaks existing
   consumers. reader_oxide is the other consumer — worth checking with
   it before cutting the release.

---

## What happens next (suggested)

1. You read this, push back on whatever's wrong.
2. I draft the `MetaParser` + `MetaGraph` API in meta_oxide (PR).
3. I move the SERP parsers and types (PR).
4. scraper_oxide deletes `pipeline::schema` and `google.rs` parser, points
   at meta_oxide. This is a single-session migration once (1)–(3) are in.
5. reader_oxide audited for its own structured-data duplication and
   brought into the same pattern.

Total effort: ~2–3 focused days across all three crates if priorities
stay stable.
