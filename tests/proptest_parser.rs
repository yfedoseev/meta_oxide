//! Property-based invariant tests for [`meta_oxide::MetaParser`].
//!
//! The point of proptest here isn't to duplicate the unit tests that verify
//! specific HTML fragments map to specific outputs — it's to encode the
//! *structural* properties the parser must hold against any input the type
//! system allows. Concretely:
//!
//! - **Never panics**. The parser must tolerate garbage bytes, truncated HTML,
//!   adversarial nesting, unclosed tags, oversized documents — anything a
//!   scraper might feed it after a network hiccup. The worst acceptable
//!   outcome is `Err(MicroformatError)`; panicking is always a bug.
//! - **Idempotent**. Parsing the same input twice yields structurally
//!   equivalent graphs (same counts, same canonical accessors). This guards
//!   against non-determinism creeping in through iteration order on hash maps.
//! - **Parse/parse_dom equivalence**. `MetaParser::parse(html)` and
//!   `MetaParser::parse_dom(&Html::parse_document(html))` must produce the
//!   same result for the same input — otherwise the shared-DOM fast path has
//!   silently diverged from the re-parse path used by the legacy extractors.
//! - **Format mask monotonicity**. Enabling more formats in the mask can only
//!   add output, never remove it: `parse(mask).populated ⊆ parse(mask|f).populated`.
//!
//! Each property runs against ~256 random inputs per test run by default;
//! use `PROPTEST_CASES=1024` to crank it up locally or in a nightly CI job.

use meta_oxide::{FormatMask, MetaParser};
use proptest::prelude::*;
use scraper::Html;

/// A byte-oriented HTML strategy: arbitrary bytes filtered down to printable
/// ASCII plus the characters html5ever's tokenizer actually cares about.
/// This catches the panic/OOM class of bugs without spending all the proptest
/// budget on "string doesn't contain `<`" cases.
fn arbitrary_html() -> impl Strategy<Value = String> {
    prop_oneof![
        // 40%: arbitrary strings up to 2 KiB — byte-level robustness
        ".{0,2048}",
        // 20%: tag-soup mixing random text with real HTML tokens
        r#"(<[a-z]+>|</[a-z]+>|<meta [a-z]+="[a-z]*">|<script>|[a-zA-Z0-9 ]+){0,64}"#,
        // 20%: near-valid meta/OG snippets
        r#"(<meta (name|property)="[a-z:]+" content="[^"]{0,40}">){0,16}"#,
        // 20%: near-valid JSON-LD shapes (often malformed JSON)
        r#"<script type="application/ld\+json">\{"@type":"[A-Z][a-z]+"(,"name":"[^"]{0,40}")?\}</script>"#,
    ]
}

proptest! {
    // Bound the work per property so the suite runs in seconds on every
    // `cargo test`. Crank up with PROPTEST_CASES=N for deeper soak tests.
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// The parser must never panic on any input, no matter how adversarial.
    #[test]
    fn parse_never_panics_on_arbitrary_bytes(html in arbitrary_html()) {
        let _ = MetaParser::new().parse(&html);
    }

    /// Same but with heuristics disabled — different code path that also
    /// must never panic.
    #[test]
    fn parse_no_heuristics_never_panics(html in arbitrary_html()) {
        let _ = MetaParser::new().with_heuristics(false).parse(&html);
    }

    /// The `get()` cross-format query is called on the parsed graph from
    /// untrusted input; it must not panic either.
    #[test]
    fn get_never_panics_across_canonical_fields(html in arbitrary_html()) {
        let graph = MetaParser::new().parse(&html).unwrap();
        for field in [
            "title", "description", "image", "url",
            "site_name", "author", "language", "published_time",
            "nonexistent",
        ] {
            let _ = graph.get(field);
        }
    }

    /// Every canonical_* accessor must also be panic-safe on garbage input.
    #[test]
    fn canonical_accessors_never_panic(html in arbitrary_html()) {
        let graph = MetaParser::new().parse(&html).unwrap();
        let _ = graph.canonical_product();
        let _ = graph.canonical_article();
        let _ = graph.canonical_person();
        let _ = graph.canonical_organization();
        let _ = graph.canonical_event();
        let _ = graph.canonical_recipe();
        let _ = graph.canonical_video();
        let _ = graph.canonical_breadcrumbs();
        let _ = graph.canonical_reviews();
        let _ = graph.canonical_faq();
    }

    /// Parsing the same HTML twice must yield graphs that agree on the
    /// populated format counts. Guards against non-determinism leaking in
    /// through hashmap iteration order or mutable global state.
    #[test]
    fn parse_is_idempotent(html in arbitrary_html()) {
        let parser = MetaParser::new();
        let a = parser.parse(&html).unwrap();
        let b = parser.parse(&html).unwrap();
        prop_assert_eq!(a.json_ld.len(), b.json_ld.len());
        prop_assert_eq!(a.microdata.len(), b.microdata.len());
        prop_assert_eq!(a.rdfa.len(), b.rdfa.len());
        prop_assert_eq!(&a.meta.title, &b.meta.title);
        prop_assert_eq!(&a.open_graph.title, &b.open_graph.title);
        prop_assert_eq!(a.heuristic_fills.len(), b.heuristic_fills.len());
        prop_assert_eq!(a.canonical_product().is_some(), b.canonical_product().is_some());
        prop_assert_eq!(a.canonical_article().is_some(), b.canonical_article().is_some());
    }

    /// `parse(html)` and `parse_dom(&parse_html(html))` must produce
    /// identical graphs — the shared-DOM fast path can't silently drift
    /// from the re-parse-every-time path.
    #[test]
    fn parse_dom_matches_parse(html in arbitrary_html()) {
        let parser = MetaParser::new();
        let via_string = parser.parse(&html).unwrap();
        let dom = Html::parse_document(&html);
        let via_dom = parser.parse_dom(&dom).unwrap();
        prop_assert_eq!(via_string.json_ld.len(), via_dom.json_ld.len());
        prop_assert_eq!(via_string.microdata.len(), via_dom.microdata.len());
        prop_assert_eq!(&via_string.meta.title, &via_dom.meta.title);
        prop_assert_eq!(&via_string.open_graph.title, &via_dom.open_graph.title);
        prop_assert_eq!(via_string.heuristic_fills.len(), via_dom.heuristic_fills.len());
    }

    /// Format mask monotonicity: enabling more formats can only populate
    /// more fields, never fewer. Formally, for any `mask` and any extra
    /// format `f`, if `parse(mask)` populates a given field then
    /// `parse(mask | f)` populates it too.
    ///
    /// We test the strongest case: `parse(META)` populates a subset of what
    /// `parse(ALL)` populates.
    #[test]
    fn format_mask_is_monotonic(html in arbitrary_html()) {
        let parser = MetaParser::new().with_heuristics(false);
        let meta_only = parser.clone().with_formats(FormatMask::META).parse(&html).unwrap();
        let all_formats = parser.clone().with_formats(FormatMask::ALL).parse(&html).unwrap();

        if meta_only.meta.title.is_some() {
            prop_assert_eq!(&meta_only.meta.title, &all_formats.meta.title);
        }
        if meta_only.meta.description.is_some() {
            prop_assert_eq!(&meta_only.meta.description, &all_formats.meta.description);
        }
        // META alone can never populate JSON-LD, microdata, etc.
        prop_assert!(meta_only.json_ld.is_empty());
        prop_assert!(meta_only.microdata.is_empty());
        prop_assert!(meta_only.open_graph.title.is_none());
    }

    /// FormatMask::NONE runs no extractors — every format-specific field
    /// must stay at its default.
    #[test]
    fn format_mask_none_populates_nothing(html in arbitrary_html()) {
        let graph = MetaParser::new()
            .with_heuristics(false)
            .with_formats(FormatMask::NONE)
            .parse(&html)
            .unwrap();
        prop_assert!(graph.meta.title.is_none());
        prop_assert!(graph.json_ld.is_empty());
        prop_assert!(graph.microdata.is_empty());
        prop_assert!(graph.rdfa.is_empty());
        prop_assert!(graph.open_graph.title.is_none());
        prop_assert!(graph.twitter.title.is_none());
        prop_assert!(graph.h_card.is_empty());
        prop_assert!(graph.heuristic_fills.is_empty());
    }

    /// `parse_reader` must match `parse` on the same bytes — they're two
    /// entry points to the same pipeline.
    #[test]
    fn parse_reader_matches_parse(html in arbitrary_html()) {
        let parser = MetaParser::new();
        let from_str = parser.parse(&html).unwrap();
        let from_reader = parser.parse_reader(html.as_bytes()).unwrap();
        prop_assert_eq!(from_str.json_ld.len(), from_reader.json_ld.len());
        prop_assert_eq!(&from_str.meta.title, &from_reader.meta.title);
        prop_assert_eq!(from_str.heuristic_fills.len(), from_reader.heuristic_fills.len());
    }

    /// SERP parser also must tolerate garbage.
    #[test]
    fn serp_parse_google_never_panics(html in arbitrary_html()) {
        let _ = meta_oxide::serp::parse_google(&html);
    }

    /// Wikipedia infobox parser must tolerate garbage.
    #[test]
    fn wikipedia_parse_infobox_never_panics(html in arbitrary_html()) {
        let _ = meta_oxide::wikipedia::parse_infobox(&html);
    }

    /// Wikidata parser must tolerate arbitrary non-JSON — it should return
    /// an Err, never panic.
    #[test]
    fn wikidata_parse_entity_never_panics(input in ".{0,2048}") {
        let _ = meta_oxide::wikidata::parse_entity(&input);
    }
}
