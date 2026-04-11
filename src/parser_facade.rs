//! Unified [`MetaParser`] facade and [`MetaGraph`] aggregate result.
//!
//! `meta_oxide`'s individual extractors (`extractors::meta`, `extractors::jsonld`,
//! …) each take an HTML string, parse it, and return one format's worth of
//! results. Calling all of them on the same document means parsing the HTML
//! N times — once per extractor — and stitching the results together by hand.
//!
//! [`MetaParser`] does the parse once and dispatches the resulting DOM to
//! every selected extractor, returning a [`MetaGraph`] that holds all of
//! their outputs side by side. Consumers (`reader_oxide`, `scraper_oxide`,
//! agent recipes) can then read whichever format they care about, or query
//! [`MetaGraph::get`] for a specific canonical field across all sources.
//!
//! ```no_run
//! use meta_oxide::MetaParser;
//!
//! let html = "<html>...</html>";
//! let graph = MetaParser::new()
//!     .with_base_url("https://example.com/page")
//!     .parse(html)
//!     .unwrap();
//!
//! // Format-native access
//! let og_title = graph.open_graph.title.as_deref();
//!
//! // Cross-format query (priority: JSON-LD → OG → Twitter → meta → heuristic)
//! if let Some(title) = graph.get("title") {
//!     println!("{} (from {:?})", title.value, title.source);
//! }
//! ```
//!
//! Note: `with_base_url` only resolves *relative* links inside the HTML
//! (turning `<link rel="canonical" href="/foo">` into an absolute URL).
//! `meta_oxide` never fetches anything over the network.

use crate::canonical::helpers::{first_jsonld_field, first_microdata_text};
use crate::canonical::{
    Article, BreadcrumbList, CanonicalMerge, CanonicalMergeAll, Event, FAQPage, Organization,
    Person, Product, Recipe, Review, VideoObject,
};
use crate::errors::Result;
use crate::extractors::common::html_utils;
use crate::provenance::{FieldSource, FieldValue};
use crate::types::dublin_core::DublinCore;
use crate::types::jsonld::JsonLdObject;
use crate::types::manifest::ManifestDiscovery;
use crate::types::meta::MetaTags;
use crate::types::microdata::MicrodataItem;
use crate::types::microformats::{
    HAdr, HCard, HEntry, HEvent, HFeed, HGeo, HProduct, HRecipe, HReview,
};
use crate::types::oembed::OEmbedDiscovery;
use crate::types::rdfa::{RdfaItem, RdfaValue};
use crate::types::social::{OpenGraph, TwitterCard};
use scraper::Html;
use std::collections::HashMap;

/// Bitmask selecting which extractors [`MetaParser`] should run.
///
/// Defaults to [`FormatMask::ALL`]. Use [`MetaParser::with_formats`] to opt out
/// of formats you don't need (e.g. skip microformats on a SERP page where
/// you only want the meta tags).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatMask(u32);

impl FormatMask {
    /// Standard HTML `<title>`, `<meta>`, and `<link>` tags.
    pub const META: Self = Self(1 << 0);
    /// Open Graph (`og:*`).
    pub const OPEN_GRAPH: Self = Self(1 << 1);
    /// Twitter Cards (`twitter:*`).
    pub const TWITTER: Self = Self(1 << 2);
    /// JSON-LD (`<script type="application/ld+json">`).
    pub const JSON_LD: Self = Self(1 << 3);
    /// HTML5 microdata (`itemscope`/`itemprop`).
    pub const MICRODATA: Self = Self(1 << 4);
    /// RDFa (`vocab`/`typeof`/`property`).
    pub const RDFA: Self = Self(1 << 5);
    /// Microformats2 (`h-card`, `h-entry`, …).
    pub const MICROFORMATS: Self = Self(1 << 6);
    /// Dublin Core (`dc.*` meta tags).
    pub const DUBLIN_CORE: Self = Self(1 << 7);
    /// oEmbed endpoint discovery.
    pub const OEMBED: Self = Self(1 << 8);
    /// Web App Manifest discovery (`<link rel="manifest">`).
    pub const MANIFEST: Self = Self(1 << 9);
    /// `rel-*` link relationships.
    pub const REL_LINKS: Self = Self(1 << 10);

    /// Empty selection (no extractors run).
    pub const NONE: Self = Self(0);
    /// Every supported format.
    pub const ALL: Self = Self(
        Self::META.0
            | Self::OPEN_GRAPH.0
            | Self::TWITTER.0
            | Self::JSON_LD.0
            | Self::MICRODATA.0
            | Self::RDFA.0
            | Self::MICROFORMATS.0
            | Self::DUBLIN_CORE.0
            | Self::OEMBED.0
            | Self::MANIFEST.0
            | Self::REL_LINKS.0,
    );

    /// Whether `self` includes all bits in `other`.
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Bitwise union of two masks.
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Bitwise difference (`self` minus `other`).
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }
}

impl Default for FormatMask {
    fn default() -> Self {
        Self::ALL
    }
}

impl std::ops::BitOr for FormatMask {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        self.union(rhs)
    }
}

impl std::ops::BitOrAssign for FormatMask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Builder for [`MetaParser`] / [`MetaGraph`].
///
/// Construct with [`MetaParser::new`], chain configuration with
/// [`MetaParser::with_base_url`] / [`MetaParser::with_heuristics`] /
/// [`MetaParser::with_formats`], and call [`MetaParser::parse`] (or
/// [`MetaParser::parse_dom`] if you already have a parsed `Html`).
#[derive(Debug, Clone)]
pub struct MetaParser {
    base_url: Option<String>,
    heuristics: bool,
    formats: FormatMask,
}

impl Default for MetaParser {
    fn default() -> Self {
        Self::new()
    }
}

impl MetaParser {
    /// Create a parser with default settings: every format enabled, heuristics
    /// on, no base URL.
    pub fn new() -> Self {
        Self { base_url: None, heuristics: true, formats: FormatMask::ALL }
    }

    /// Set the base URL used to resolve *relative* links inside the HTML
    /// (e.g. turning `<link rel="canonical" href="/foo">` into an absolute
    /// URL). `meta_oxide` does not fetch anything — this is purely for
    /// link resolution.
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Enable or disable the heuristic / CSS-fallback layer (default: enabled).
    ///
    /// The heuristic layer is filled in by Phase 5 of the refactor; this flag
    /// is wired up now so callers can opt in or out without an API change later.
    pub fn with_heuristics(mut self, enabled: bool) -> Self {
        self.heuristics = enabled;
        self
    }

    /// Restrict which formats are extracted. Default is [`FormatMask::ALL`].
    pub fn with_formats(mut self, mask: FormatMask) -> Self {
        self.formats = mask;
        self
    }

    /// Parse an HTML string and run every selected extractor against the
    /// resulting DOM exactly once.
    pub fn parse(&self, html: &str) -> Result<MetaGraph> {
        self.parse_dom(&html_utils::parse_html(html))
    }

    /// Parse HTML from a [`std::io::Read`] source.
    ///
    /// The current implementation reads the reader to a `String` and then
    /// delegates to [`MetaParser::parse`]. This exists so consumers that
    /// already work with `Read` (file handles, network sockets) don't have
    /// to materialise a `String` themselves at the call site, and so the
    /// API surface stays stable when a true streaming
    /// (`html5ever::tokenizer`) implementation lands later.
    pub fn parse_reader<R: std::io::Read>(&self, mut reader: R) -> Result<MetaGraph> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf).map_err(|e| {
            crate::errors::MicroformatError::ParseError(format!("read failed: {}", e))
        })?;
        self.parse(&buf)
    }

    /// Run every selected extractor against an already-parsed DOM. Useful when
    /// you've parsed the document yourself and want to avoid a redundant parse.
    pub fn parse_dom(&self, dom: &Html) -> Result<MetaGraph> {
        let base = self.base_url.as_deref();
        let mut graph = MetaGraph::default();

        if self.formats.contains(FormatMask::META) {
            graph.meta = crate::extractors::meta::extract_from_dom(dom, base)?;
        }
        if self.formats.contains(FormatMask::OPEN_GRAPH) {
            graph.open_graph = crate::extractors::social::opengraph::extract_from_dom(dom, base)?;
        }
        if self.formats.contains(FormatMask::TWITTER) {
            graph.twitter = crate::extractors::social::twitter::extract_from_dom(dom, base)?;
        }
        if self.formats.contains(FormatMask::JSON_LD) {
            graph.json_ld = crate::extractors::jsonld::extract_from_dom(dom, base)?;
        }
        if self.formats.contains(FormatMask::MICRODATA) {
            graph.microdata = crate::extractors::microdata::extract_from_dom(dom, base)?;
        }
        if self.formats.contains(FormatMask::RDFA) {
            graph.rdfa = crate::extractors::rdfa::extract_from_dom(dom, base)?;
        }
        if self.formats.contains(FormatMask::MICROFORMATS) {
            graph.h_card = crate::extractors::microformats::hcard::extract_from_dom(dom, base)?;
            graph.h_entry = crate::extractors::microformats::hentry::extract_from_dom(dom, base)?;
            graph.h_event = crate::extractors::microformats::hevent::extract_from_dom(dom, base)?;
            graph.h_review = crate::extractors::microformats::hreview::extract_from_dom(dom, base)?;
            graph.h_recipe = crate::extractors::microformats::hrecipe::extract_from_dom(dom, base)?;
            graph.h_product =
                crate::extractors::microformats::hproduct::extract_from_dom(dom, base)?;
            graph.h_feed = crate::extractors::microformats::hfeed::extract_from_dom(dom, base)?;
            graph.h_adr = crate::extractors::microformats::hadr::extract_from_dom(dom, base)?;
            graph.h_geo = crate::extractors::microformats::hgeo::extract_from_dom(dom, base)?;
        }
        if self.formats.contains(FormatMask::DUBLIN_CORE) {
            graph.dublin_core = crate::extractors::dublin_core::extract_from_dom(dom)?;
        }
        if self.formats.contains(FormatMask::OEMBED) {
            graph.oembed = crate::extractors::oembed::extract_from_dom(dom, base)?;
        }
        if self.formats.contains(FormatMask::MANIFEST) {
            graph.manifest = crate::extractors::manifest::extract_from_dom(dom, base)?;
        }
        if self.formats.contains(FormatMask::REL_LINKS) {
            graph.rel_links = crate::extractors::rel_links::extract_from_dom(dom, base)?;
        }

        if self.heuristics {
            crate::heuristics::apply(dom, &mut graph, base);
        }

        Ok(graph)
    }
}

/// Aggregate result of running every selected extractor against one document.
///
/// Each field corresponds to one format's output. Format-native consumers can
/// reach for the field they want directly (`graph.open_graph.title`,
/// `graph.json_ld[0]`, …); cross-format consumers can use [`MetaGraph::get`] to
/// query a logical field across sources with priority ordering.
#[derive(Debug, Clone, Default)]
pub struct MetaGraph {
    /// Standard `<title>` / `<meta>` / `<link>` tags.
    pub meta: MetaTags,
    /// Open Graph (`og:*`).
    pub open_graph: OpenGraph,
    /// Twitter Cards (`twitter:*`).
    pub twitter: TwitterCard,
    /// All JSON-LD objects found in `<script type="application/ld+json">`.
    pub json_ld: Vec<JsonLdObject>,
    /// All top-level microdata items.
    pub microdata: Vec<MicrodataItem>,
    /// All RDFa items.
    pub rdfa: Vec<RdfaItem>,
    /// Dublin Core meta tags.
    pub dublin_core: DublinCore,
    /// Discovered oEmbed endpoints.
    pub oembed: OEmbedDiscovery,
    /// Discovered Web App Manifest link.
    pub manifest: ManifestDiscovery,
    /// `rel`-attribute link relationships, keyed by rel type.
    pub rel_links: HashMap<String, Vec<String>>,

    // Microformats2 — one Vec per h-* type.
    /// `h-card` items.
    pub h_card: Vec<HCard>,
    /// `h-entry` items.
    pub h_entry: Vec<HEntry>,
    /// `h-event` items.
    pub h_event: Vec<HEvent>,
    /// `h-review` items.
    pub h_review: Vec<HReview>,
    /// `h-recipe` items.
    pub h_recipe: Vec<HRecipe>,
    /// `h-product` items.
    pub h_product: Vec<HProduct>,
    /// `h-feed` items.
    pub h_feed: Vec<HFeed>,
    /// `h-adr` items.
    pub h_adr: Vec<HAdr>,
    /// `h-geo` items.
    pub h_geo: Vec<HGeo>,

    /// Heuristic / CSS-fallback fills, keyed by canonical field name. Populated
    /// only when [`MetaParser::with_heuristics`] is enabled and the heuristic
    /// layer (Phase 5) is implemented. Empty for now.
    pub heuristic_fills: HashMap<String, FieldValue<String>>,
}

impl MetaGraph {
    /// Canonical schema.org `Product` merged from every available source.
    pub fn canonical_product(&self) -> Option<Product> {
        Product::merge_from(self)
    }

    /// Canonical schema.org `Article` (or `NewsArticle`/`BlogPosting`) merged
    /// from every available source.
    pub fn canonical_article(&self) -> Option<Article> {
        Article::merge_from(self)
    }

    /// Canonical schema.org `Person` merged from every available source.
    pub fn canonical_person(&self) -> Option<Person> {
        Person::merge_from(self)
    }

    /// Canonical schema.org `Organization` (or `LocalBusiness`) merged across
    /// formats.
    pub fn canonical_organization(&self) -> Option<Organization> {
        Organization::merge_from(self)
    }

    /// Canonical schema.org `Event` merged across formats.
    pub fn canonical_event(&self) -> Option<Event> {
        Event::merge_from(self)
    }

    /// Canonical schema.org `Recipe` merged across formats.
    pub fn canonical_recipe(&self) -> Option<Recipe> {
        Recipe::merge_from(self)
    }

    /// Canonical schema.org `VideoObject` merged across formats.
    pub fn canonical_video(&self) -> Option<VideoObject> {
        VideoObject::merge_from(self)
    }

    /// Canonical schema.org `BreadcrumbList`.
    pub fn canonical_breadcrumbs(&self) -> Option<BreadcrumbList> {
        BreadcrumbList::merge_from(self)
    }

    /// Every schema.org `Review` found on the page.
    pub fn canonical_reviews(&self) -> Vec<Review> {
        Review::merge_all_from(self)
    }

    /// Canonical schema.org `FAQPage`.
    pub fn canonical_faq(&self) -> Option<FAQPage> {
        FAQPage::merge_from(self)
    }

    /// Look up a logical field across every format with a priority ordering.
    ///
    /// Priority (highest first): JSON-LD → microdata → RDFa → Open Graph →
    /// Twitter Cards → Dublin Core → standard meta tags → heuristics.
    ///
    /// Recognised field names: `"title"`, `"description"`, `"image"`, `"url"`,
    /// `"author"`, `"language"`, `"site_name"`, `"published_time"`. Returns
    /// `None` if no source supplied a value.
    pub fn get(&self, canonical_field: &str) -> Option<FieldValue<String>> {
        match canonical_field {
            "title" => self.lookup_title(),
            "description" => self.lookup_description(),
            "image" => self.lookup_image(),
            "url" => self.lookup_url(),
            "site_name" => self.lookup_site_name(),
            "author" => self.lookup_author(),
            "language" => self.lookup_language(),
            "published_time" => self.lookup_published_time(),
            _ => None,
        }
    }

    fn lookup_title(&self) -> Option<FieldValue<String>> {
        if let Some((value, type_label, field)) =
            first_jsonld_field(&self.json_ld, &["headline", "name"])
        {
            return Some(FieldValue::new(
                value,
                FieldSource::JsonLd(format!("{}.{}", type_label, field)),
            ));
        }
        if let Some((name, field)) = first_microdata_text(&self.microdata, &["name", "headline"]) {
            return Some(FieldValue::new(name, FieldSource::Microdata(field.to_string())));
        }
        if let Some((value, predicate)) =
            first_rdfa_text(&self.rdfa, &["name", "title", "headline"])
        {
            return Some(FieldValue::new(value, FieldSource::Rdfa(predicate)));
        }
        if let Some(t) = self.open_graph.title.clone() {
            return Some(FieldValue::new(t, FieldSource::OpenGraph("og:title".into())));
        }
        if let Some(t) = self.twitter.title.clone() {
            return Some(FieldValue::new(t, FieldSource::Twitter("twitter:title".into())));
        }
        if let Some(t) = self.dublin_core.title.clone() {
            return Some(FieldValue::new(t, FieldSource::DublinCore("dc.title".into())));
        }
        if let Some(t) = self.meta.title.clone() {
            return Some(FieldValue::new(t, FieldSource::Meta("title".into())));
        }
        self.heuristic_fills.get("title").cloned()
    }

    fn lookup_description(&self) -> Option<FieldValue<String>> {
        if let Some((value, type_label, field)) =
            first_jsonld_field(&self.json_ld, &["description"])
        {
            return Some(FieldValue::new(
                value,
                FieldSource::JsonLd(format!("{}.{}", type_label, field)),
            ));
        }
        if let Some((d, field)) = first_microdata_text(&self.microdata, &["description"]) {
            return Some(FieldValue::new(d, FieldSource::Microdata(field.to_string())));
        }
        if let Some((value, predicate)) = first_rdfa_text(&self.rdfa, &["description"]) {
            return Some(FieldValue::new(value, FieldSource::Rdfa(predicate)));
        }
        if let Some(d) = self.open_graph.description.clone() {
            return Some(FieldValue::new(d, FieldSource::OpenGraph("og:description".into())));
        }
        if let Some(d) = self.twitter.description.clone() {
            return Some(FieldValue::new(d, FieldSource::Twitter("twitter:description".into())));
        }
        if let Some(d) = self.dublin_core.description.clone() {
            return Some(FieldValue::new(d, FieldSource::DublinCore("dc.description".into())));
        }
        if let Some(d) = self.meta.description.clone() {
            return Some(FieldValue::new(d, FieldSource::Meta("description".into())));
        }
        self.heuristic_fills.get("description").cloned()
    }

    fn lookup_image(&self) -> Option<FieldValue<String>> {
        if let Some((value, type_label, field)) = first_jsonld_field(&self.json_ld, &["image"]) {
            return Some(FieldValue::new(
                value,
                FieldSource::JsonLd(format!("{}.{}", type_label, field)),
            ));
        }
        if let Some((i, field)) = first_microdata_text(&self.microdata, &["image"]) {
            return Some(FieldValue::new(i, FieldSource::Microdata(field.to_string())));
        }
        if let Some(i) = self.open_graph.image.clone() {
            return Some(FieldValue::new(i, FieldSource::OpenGraph("og:image".into())));
        }
        if let Some(i) = self.twitter.image.clone() {
            return Some(FieldValue::new(i, FieldSource::Twitter("twitter:image".into())));
        }
        self.heuristic_fills.get("image").cloned()
    }

    fn lookup_url(&self) -> Option<FieldValue<String>> {
        if let Some((value, type_label, field)) = first_jsonld_field(&self.json_ld, &["url"]) {
            return Some(FieldValue::new(
                value,
                FieldSource::JsonLd(format!("{}.{}", type_label, field)),
            ));
        }
        if let Some((u, field)) = first_microdata_text(&self.microdata, &["url"]) {
            return Some(FieldValue::new(u, FieldSource::Microdata(field.to_string())));
        }
        if let Some(u) = self.open_graph.url.clone() {
            return Some(FieldValue::new(u, FieldSource::OpenGraph("og:url".into())));
        }
        if let Some(u) = self.meta.canonical.clone() {
            return Some(FieldValue::new(u, FieldSource::Meta("canonical".into())));
        }
        None
    }

    fn lookup_site_name(&self) -> Option<FieldValue<String>> {
        if let Some(s) = self.open_graph.site_name.clone() {
            return Some(FieldValue::new(s, FieldSource::OpenGraph("og:site_name".into())));
        }
        if let Some(s) = self.meta.application_name.clone() {
            return Some(FieldValue::new(s, FieldSource::Meta("application-name".into())));
        }
        None
    }

    fn lookup_author(&self) -> Option<FieldValue<String>> {
        if let Some((value, type_label, field)) = first_jsonld_field(&self.json_ld, &["author"]) {
            return Some(FieldValue::new(
                value,
                FieldSource::JsonLd(format!("{}.{}", type_label, field)),
            ));
        }
        if let Some(a) = self.dublin_core.creator.clone() {
            return Some(FieldValue::new(a, FieldSource::DublinCore("dc.creator".into())));
        }
        if let Some(a) = self.meta.author.clone() {
            return Some(FieldValue::new(a, FieldSource::Meta("author".into())));
        }
        None
    }

    fn lookup_language(&self) -> Option<FieldValue<String>> {
        if let Some(l) = self.open_graph.locale.clone() {
            return Some(FieldValue::new(l, FieldSource::OpenGraph("og:locale".into())));
        }
        if let Some(l) = self.dublin_core.language.clone() {
            return Some(FieldValue::new(l, FieldSource::DublinCore("dc.language".into())));
        }
        if let Some(l) = self.meta.language.clone() {
            return Some(FieldValue::new(l, FieldSource::Meta("html[lang]".into())));
        }
        None
    }

    fn lookup_published_time(&self) -> Option<FieldValue<String>> {
        if let Some((value, type_label, field)) =
            first_jsonld_field(&self.json_ld, &["datePublished"])
        {
            return Some(FieldValue::new(
                value,
                FieldSource::JsonLd(format!("{}.{}", type_label, field)),
            ));
        }
        if let Some(t) = self.dublin_core.date.clone() {
            return Some(FieldValue::new(t, FieldSource::DublinCore("dc.date".into())));
        }
        None
    }
}

/// Walk RDFa items looking for the first literal value whose predicate
/// matches any of `field_names`. RDFa property names are often CURIE-form
/// (e.g. `schema:name`, `dc:title`); we accept either the bare name or any
/// CURIE suffix matching a requested field.
fn first_rdfa_text(items: &[RdfaItem], field_names: &[&str]) -> Option<(String, String)> {
    for item in items {
        for (predicate, values) in &item.properties {
            let bare = predicate.rsplit([':', '/']).next().unwrap_or(predicate.as_str());
            if !field_names.iter().any(|f| bare.eq_ignore_ascii_case(f)) {
                continue;
            }
            for value in values {
                let literal = match value {
                    RdfaValue::Literal(s) => Some(s.clone()),
                    RdfaValue::TypedLiteral { value, .. } => Some(value.clone()),
                    _ => None,
                };
                if let Some(s) = literal {
                    if !s.is_empty() {
                        return Some((s, predicate.clone()));
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const RICH_HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <title>Plain Title</title>
    <meta name="description" content="Plain description.">
    <meta property="og:title" content="OG Title">
    <meta property="og:description" content="OG description.">
    <meta property="og:image" content="https://example.com/og.jpg">
    <meta property="og:site_name" content="Example Site">
    <meta name="twitter:card" content="summary">
    <meta name="twitter:title" content="Twitter Title">
    <link rel="canonical" href="https://example.com/page">
    <script type="application/ld+json">
    {
      "@context": "https://schema.org",
      "@type": "Product",
      "name": "Mechanical Keyboard",
      "description": "RGB mechanical keyboard.",
      "image": "https://example.com/kb.jpg",
      "url": "https://example.com/page"
    }
    </script>
</head>
<body><h1>Page Heading</h1></body>
</html>
"#;

    #[test]
    fn parse_runs_every_extractor_in_one_pass() {
        let graph =
            MetaParser::new().with_base_url("https://example.com/").parse(RICH_HTML).unwrap();

        // Format-native fields populated.
        assert_eq!(graph.meta.title.as_deref(), Some("Plain Title"));
        assert_eq!(graph.open_graph.title.as_deref(), Some("OG Title"));
        assert_eq!(graph.twitter.title.as_deref(), Some("Twitter Title"));
        assert_eq!(graph.json_ld.len(), 1);
        assert_eq!(graph.meta.canonical.as_deref(), Some("https://example.com/page"));
    }

    #[test]
    fn get_title_prefers_jsonld_over_og() {
        let graph = MetaParser::new().parse(RICH_HTML).unwrap();
        let title = graph.get("title").expect("title should resolve");
        assert_eq!(title.value, "Mechanical Keyboard");
        assert!(matches!(title.source, FieldSource::JsonLd(_)));
    }

    #[test]
    fn get_description_prefers_jsonld() {
        let graph = MetaParser::new().parse(RICH_HTML).unwrap();
        let desc = graph.get("description").expect("description should resolve");
        assert_eq!(desc.value, "RGB mechanical keyboard.");
        assert!(matches!(desc.source, FieldSource::JsonLd(_)));
    }

    #[test]
    fn get_falls_back_to_og_when_jsonld_missing() {
        let html = r#"
            <meta property="og:title" content="OG Only">
            <title>Plain</title>
        "#;
        let graph = MetaParser::new().parse(html).unwrap();
        let title = graph.get("title").unwrap();
        assert_eq!(title.value, "OG Only");
        assert!(matches!(title.source, FieldSource::OpenGraph(_)));
    }

    #[test]
    fn get_falls_back_to_meta_when_only_title_tag() {
        let html = r#"<title>Just A Title</title>"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let title = graph.get("title").unwrap();
        assert_eq!(title.value, "Just A Title");
        assert!(matches!(title.source, FieldSource::Meta(_)));
    }

    #[test]
    fn get_returns_none_for_unknown_field() {
        let graph = MetaParser::new().parse(RICH_HTML).unwrap();
        assert!(graph.get("nonexistent").is_none());
    }

    #[test]
    fn parse_dom_avoids_re_parse() {
        let dom = Html::parse_document(RICH_HTML);
        let graph = MetaParser::new().parse_dom(&dom).unwrap();
        assert_eq!(graph.json_ld.len(), 1);
        assert_eq!(graph.open_graph.site_name.as_deref(), Some("Example Site"));
    }

    #[test]
    fn format_mask_filters_extractors() {
        let graph = MetaParser::new()
            .with_formats(FormatMask::META | FormatMask::OPEN_GRAPH)
            .parse(RICH_HTML)
            .unwrap();
        // Selected formats run.
        assert_eq!(graph.meta.title.as_deref(), Some("Plain Title"));
        assert_eq!(graph.open_graph.title.as_deref(), Some("OG Title"));
        // Unselected formats stay empty.
        assert!(graph.json_ld.is_empty());
        assert!(graph.twitter.title.is_none());
    }

    #[test]
    fn format_mask_contains_works() {
        let mask = FormatMask::META | FormatMask::JSON_LD;
        assert!(mask.contains(FormatMask::META));
        assert!(mask.contains(FormatMask::JSON_LD));
        assert!(!mask.contains(FormatMask::OPEN_GRAPH));
    }

    #[test]
    fn empty_html_yields_empty_graph_without_error() {
        let graph = MetaParser::new().parse("").unwrap();
        assert!(graph.meta.title.is_none());
        assert!(graph.json_ld.is_empty());
        assert!(graph.get("title").is_none());
    }

    #[test]
    fn heuristic_layer_fills_h1_when_no_structured_title() {
        let html = r#"<html><body><h1>Heuristic Heading</h1><p>The first paragraph that is long enough to pass the threshold.</p></body></html>"#;
        let graph = MetaParser::new().parse(html).unwrap();
        // No <title>, og:title, or jsonld → heuristic should kick in.
        let title = graph.get("title").expect("heuristic should supply title");
        assert_eq!(title.value, "Heuristic Heading");
        assert!(matches!(title.source, FieldSource::Heuristic(_)));
        assert_eq!(graph.heuristic_fills.get("title").unwrap().value, "Heuristic Heading");
        // Description heuristic too.
        assert!(graph.heuristic_fills.contains_key("description"));
    }

    #[test]
    fn heuristics_disabled_leaves_no_fills() {
        let html = r#"<html><body><h1>Heading</h1></body></html>"#;
        let graph = MetaParser::new().with_heuristics(false).parse(html).unwrap();
        assert!(graph.heuristic_fills.is_empty());
        assert!(graph.get("title").is_none());
    }

    #[test]
    fn get_site_name_from_og_site_name() {
        let html = r#"<meta property="og:site_name" content="Example Site">"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let site = graph.get("site_name").unwrap();
        assert_eq!(site.value, "Example Site");
        assert!(matches!(site.source, FieldSource::OpenGraph(_)));
    }

    #[test]
    fn get_site_name_falls_back_to_application_name() {
        let html = r#"<meta name="application-name" content="My App">"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let site = graph.get("site_name").unwrap();
        assert_eq!(site.value, "My App");
        assert!(matches!(site.source, FieldSource::Meta(_)));
    }

    #[test]
    fn get_author_from_jsonld() {
        let html = r#"
<script type="application/ld+json">
{"@type": "Article", "headline": "Story", "author": "Jane Reporter"}
</script>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let author = graph.get("author").unwrap();
        assert_eq!(author.value, "Jane Reporter");
        assert!(matches!(author.source, FieldSource::JsonLd(_)));
    }

    #[test]
    fn get_author_falls_back_to_dublin_core_then_meta() {
        let html_dc = r#"<meta name="dc.creator" content="Dublin Author">"#;
        let graph = MetaParser::new().parse(html_dc).unwrap();
        let author = graph.get("author").unwrap();
        assert_eq!(author.value, "Dublin Author");
        assert!(matches!(author.source, FieldSource::DublinCore(_)));

        let html_meta = r#"<meta name="author" content="Meta Author">"#;
        let graph = MetaParser::new().parse(html_meta).unwrap();
        let author = graph.get("author").unwrap();
        assert_eq!(author.value, "Meta Author");
        assert!(matches!(author.source, FieldSource::Meta(_)));
    }

    #[test]
    fn get_language_priority_og_dc_html() {
        // og:locale wins
        let html = r#"
            <html lang="en">
            <meta property="og:locale" content="fr_FR">
            <meta name="dc.language" content="de">
        "#;
        let graph = MetaParser::new().parse(html).unwrap();
        let lang = graph.get("language").unwrap();
        assert_eq!(lang.value, "fr_FR");
        assert!(matches!(lang.source, FieldSource::OpenGraph(_)));

        // DC wins over html[lang]
        let html = r#"<html lang="en"><meta name="dc.language" content="de">"#;
        let graph = MetaParser::new().parse(html).unwrap();
        assert!(matches!(graph.get("language").unwrap().source, FieldSource::DublinCore(_)));

        // html[lang] is last resort
        let html = r#"<html lang="ja">"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let lang = graph.get("language").unwrap();
        assert_eq!(lang.value, "ja");
        assert!(matches!(lang.source, FieldSource::Meta(_)));
    }

    #[test]
    fn get_published_time_priority() {
        // JSON-LD wins
        let html = r#"
<script type="application/ld+json">
{"@type": "Article", "headline": "x", "datePublished": "2026-04-10T08:30:00Z"}
</script>
<meta name="dc.date" content="2020-01-01">
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let t = graph.get("published_time").unwrap();
        assert_eq!(t.value, "2026-04-10T08:30:00Z");
        assert!(matches!(t.source, FieldSource::JsonLd(_)));

        // Dublin Core fallback
        let html = r#"<meta name="dc.date" content="2020-01-01">"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let t = graph.get("published_time").unwrap();
        assert_eq!(t.value, "2020-01-01");
        assert!(matches!(t.source, FieldSource::DublinCore(_)));
    }

    #[test]
    fn get_url_priority_jsonld_microdata_og_canonical() {
        // JSON-LD wins
        let html = r#"
<link rel="canonical" href="https://example.com/canon">
<meta property="og:url" content="https://example.com/og">
<script type="application/ld+json">
{"@type": "Thing", "url": "https://example.com/jsonld"}
</script>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let url = graph.get("url").unwrap();
        assert_eq!(url.value, "https://example.com/jsonld");
        assert!(matches!(url.source, FieldSource::JsonLd(_)));

        // OG fallback
        let html = r#"
<link rel="canonical" href="https://example.com/canon">
<meta property="og:url" content="https://example.com/og">
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let url = graph.get("url").unwrap();
        assert_eq!(url.value, "https://example.com/og");
        assert!(matches!(url.source, FieldSource::OpenGraph(_)));

        // canonical fallback
        let html = r#"<link rel="canonical" href="https://example.com/canon">"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let url = graph.get("url").unwrap();
        assert_eq!(url.value, "https://example.com/canon");
        assert!(matches!(url.source, FieldSource::Meta(_)));
    }

    #[test]
    fn get_image_priority_chain() {
        // JSON-LD wins
        let html = r#"
<meta property="og:image" content="https://example.com/og.jpg">
<script type="application/ld+json">
{"@type": "Thing", "image": "https://example.com/jsonld.jpg"}
</script>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        assert_eq!(graph.get("image").unwrap().value, "https://example.com/jsonld.jpg");

        // OG fallback
        let html = r#"<meta property="og:image" content="https://example.com/og.jpg">"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let img = graph.get("image").unwrap();
        assert_eq!(img.value, "https://example.com/og.jpg");
        assert!(matches!(img.source, FieldSource::OpenGraph(_)));

        // Twitter fallback
        let html = r#"<meta name="twitter:image" content="https://example.com/tw.jpg">"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let img = graph.get("image").unwrap();
        assert_eq!(img.value, "https://example.com/tw.jpg");
        assert!(matches!(img.source, FieldSource::Twitter(_)));
    }

    #[test]
    fn rdfa_supplies_title_when_other_formats_missing() {
        let html = r#"
<div vocab="https://schema.org/" typeof="Person">
  <span property="name">RDFa Name</span>
</div>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let title = graph.get("title").expect("rdfa title");
        assert_eq!(title.value, "RDFa Name");
        assert!(matches!(title.source, FieldSource::Rdfa(_)));
    }

    #[test]
    fn jsonld_beats_rdfa_in_priority_chain() {
        let html = r#"
<div vocab="https://schema.org/" typeof="Thing">
  <span property="name">RDFa Name</span>
</div>
<script type="application/ld+json">
{"@type": "Thing", "name": "JSON-LD Name"}
</script>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let title = graph.get("title").expect("title");
        assert_eq!(title.value, "JSON-LD Name");
        assert!(matches!(title.source, FieldSource::JsonLd(_)));
    }

    #[test]
    fn microdata_first_item_supplies_title_via_get() {
        let html = r#"
<div itemscope itemtype="https://schema.org/Thing">
  <span itemprop="name">Microdata Name</span>
</div>
"#;
        let graph = MetaParser::new().parse(html).unwrap();
        let title = graph.get("title").unwrap();
        assert_eq!(title.value, "Microdata Name");
        assert!(matches!(title.source, FieldSource::Microdata(_)));
    }

    #[test]
    fn format_mask_none_runs_no_extractors() {
        let graph = MetaParser::new().with_formats(FormatMask::NONE).parse(RICH_HTML).unwrap();
        assert!(graph.meta.title.is_none());
        assert!(graph.json_ld.is_empty());
        assert!(graph.open_graph.title.is_none());
    }

    #[test]
    fn format_mask_bitor_assign_works() {
        let mut mask = FormatMask::META;
        mask |= FormatMask::JSON_LD;
        assert!(mask.contains(FormatMask::META));
        assert!(mask.contains(FormatMask::JSON_LD));
        assert!(!mask.contains(FormatMask::OPEN_GRAPH));
    }

    #[test]
    fn format_mask_difference_excludes_bits() {
        let all_without_microformats = FormatMask::ALL.difference(FormatMask::MICROFORMATS);
        assert!(!all_without_microformats.contains(FormatMask::MICROFORMATS));
        assert!(all_without_microformats.contains(FormatMask::META));
        assert!(all_without_microformats.contains(FormatMask::JSON_LD));
    }

    #[test]
    fn default_parser_equals_new() {
        let a = MetaParser::new().parse(RICH_HTML).unwrap();
        let b = MetaParser::default().parse(RICH_HTML).unwrap();
        assert_eq!(a.json_ld.len(), b.json_ld.len());
        assert_eq!(a.meta.title, b.meta.title);
    }

    #[test]
    fn parse_reader_reads_from_io_source() {
        let html = b"<title>From Reader</title>";
        let graph = MetaParser::new().parse_reader(&html[..]).unwrap();
        assert_eq!(graph.meta.title.as_deref(), Some("From Reader"));
    }

    #[test]
    fn structured_title_wins_over_heuristic() {
        let html = r#"<title>Real Title</title><body><h1>Heading</h1></body>"#;
        let graph = MetaParser::new().parse(html).unwrap();
        // Structured <title> tag wins; heuristic should not overwrite.
        assert!(!graph.heuristic_fills.contains_key("title"));
        let title = graph.get("title").unwrap();
        assert_eq!(title.value, "Real Title");
        assert!(matches!(title.source, FieldSource::Meta(_)));
    }
}
