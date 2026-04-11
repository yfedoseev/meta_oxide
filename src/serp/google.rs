//! Google SERP parser. Lifted from `scraper_oxide::search::google` so the
//! parser logic lives next to every other HTML metadata extractor in
//! `meta_oxide` rather than scattered across crates.
//!
//! Selectors here are deliberately conservative — Google rotates DOM classes
//! constantly, but `a:has(h3)` for organic results, `div.xpdopen`/`div.ULSxyf`
//! for featured snippets, and `div.related-question-pair` for People Also Ask
//! have stayed stable for years.

use crate::serp::types::{FeaturedSnippet, OrganicResult, PeopleAlsoAsk, SerpGraph};
use scraper::{ElementRef, Html, Selector};

/// Default cap on organic results when callers don't specify one. Google's
/// default first page is 10 results.
pub const DEFAULT_MAX_ORGANIC: usize = 10;

/// Parse a Google SERP page from raw HTML.
///
/// Returns a [`SerpGraph`] populated with the organic results, featured
/// snippet, and People Also Ask block. Knowledge graph, related searches,
/// and site links are not yet extracted (the right-rail panels live in DOM
/// containers Google rewrites monthly; we'll add them when a downstream
/// caller actually needs them).
pub fn parse_google(html: &str) -> SerpGraph {
    parse_google_with_limit(html, DEFAULT_MAX_ORGANIC)
}

/// Parse a Google SERP page with an explicit cap on organic result count.
pub fn parse_google_with_limit(html: &str, max_organic: usize) -> SerpGraph {
    parse_google_dom(&Html::parse_document(html), max_organic)
}

/// Parse a Google SERP from an already-parsed DOM. Useful when the same
/// document needs to feed both [`crate::MetaParser`] and the SERP parser.
pub fn parse_google_dom(doc: &Html, max_organic: usize) -> SerpGraph {
    SerpGraph {
        organic: parse_organic(doc, max_organic),
        featured_snippet: parse_featured_snippet(doc),
        people_also_ask: parse_paa(doc),
        ..SerpGraph::default()
    }
}

/// Walk the DOM for anchor tags with an `<h3>` descendant whose href is
/// an organic (off-Google) URL. For each, collect a nearby text block as
/// the snippet. Dedup on URL and cap at `max_results`.
fn parse_organic(doc: &Html, max_results: usize) -> Vec<OrganicResult> {
    let anchor_sel = Selector::parse("a:has(h3)").expect("valid selector");
    let h3_sel = Selector::parse("h3").expect("valid selector");

    let mut results = Vec::new();
    let mut seen_urls = std::collections::HashSet::new();

    for anchor in doc.select(&anchor_sel) {
        if results.len() >= max_results {
            break;
        }

        let href = match anchor.value().attr("href") {
            Some(h) if is_organic_result_url(h) => h.to_string(),
            _ => continue,
        };

        if !seen_urls.insert(href.clone()) {
            continue;
        }

        let title = anchor.select(&h3_sel).next().map(element_text).unwrap_or_default();

        if title.is_empty() {
            continue;
        }

        let snippet = find_snippet_near(anchor);

        results.push(OrganicResult {
            position: (results.len() + 1) as u32,
            title,
            url: href,
            snippet,
            date: None,
        });
    }

    results
}

/// Find a snippet by walking up from an anchor and looking for the nearest
/// ancestor with substantial text (>= 40 chars).
fn find_snippet_near(anchor: ElementRef) -> String {
    let mut current = anchor.parent();
    for _ in 0..5 {
        let Some(node) = current else {
            break;
        };
        if let Some(el) = ElementRef::wrap(node) {
            let text = element_text(el);
            if text.len() >= 40 {
                return text;
            }
            current = el.parent();
        } else {
            break;
        }
    }
    String::new()
}

fn parse_featured_snippet(doc: &Html) -> Option<FeaturedSnippet> {
    let candidates = ["div.xpdopen", "div.ULSxyf", "div.kp-wholepage", "div.hgKElc"];

    for sel_str in candidates {
        let Ok(sel) = Selector::parse(sel_str) else { continue };
        let Some(container) = doc.select(&sel).next() else { continue };
        let text = element_text(container);
        if text.len() < 20 {
            continue;
        }

        let Ok(link_sel) = Selector::parse("a[href]") else { continue };
        let link = container
            .select(&link_sel)
            .find(|a| a.value().attr("href").is_some_and(is_organic_result_url));
        let Some(a) = link else { continue };

        let url = a.value().attr("href").unwrap_or("").to_string();
        let h3_sel = Selector::parse("h3").ok();
        let title = h3_sel
            .and_then(|s| a.select(&s).next())
            .map(element_text)
            .unwrap_or_else(|| element_text(a));

        if url.is_empty() {
            continue;
        }

        return Some(FeaturedSnippet { text, url, title, snippet_type: None });
    }
    None
}

fn parse_paa(doc: &Html) -> Vec<PeopleAlsoAsk> {
    let Ok(sel) = Selector::parse("div.related-question-pair") else {
        return Vec::new();
    };
    let heading_sel = Selector::parse("[role=\"heading\"]").ok();
    let link_sel = Selector::parse("a[href]").ok();

    doc.select(&sel)
        .filter_map(|pair| {
            let question = heading_sel
                .as_ref()
                .and_then(|s| pair.select(s).next())
                .map(element_text)
                .filter(|s| !s.is_empty())?;

            let link = link_sel.as_ref().and_then(|s| pair.select(s).next());

            let url = link.and_then(|a| a.value().attr("href")).map(str::to_string);

            // Answer = all text under the pair minus the question itself.
            let all = element_text(pair);
            let answer = all.strip_prefix(&question).unwrap_or(&all).trim().to_string();
            let answer = if answer.is_empty() { None } else { Some(answer) };

            Some(PeopleAlsoAsk { question, answer, source_url: url, source_title: None })
        })
        .collect()
}

fn element_text(el: ElementRef) -> String {
    let raw: String = el.text().collect::<Vec<_>>().join(" ");
    let mut out = String::with_capacity(raw.len());
    let mut last_ws = false;
    for c in raw.chars() {
        if c.is_whitespace() {
            if !last_ws {
                out.push(' ');
            }
            last_ws = true;
        } else {
            out.push(c);
            last_ws = false;
        }
    }
    out.trim().to_string()
}

/// Check if a URL looks like an organic search result (not Google internal).
fn is_organic_result_url(url: &str) -> bool {
    url.starts_with("http")
        && !url.contains("google.com")
        && !url.contains("googleapis.com")
        && !url.contains("gstatic.com")
        && !url.contains("youtube.com/results")
        && !url.contains("accounts.google")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
<!DOCTYPE html>
<html>
<body>
    <div class="xpdopen">
        Asynchronous programming in Rust uses async/await to write
        non-blocking code that runs on a futures executor.
        <a href="https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html">
            <h3>Getting Started — Async Book</h3>
        </a>
    </div>
    <div>
        <a href="https://doc.rust-lang.org/book/ch16-00-concurrency.html">
            <h3>The Rust Programming Language Book — Concurrency</h3>
        </a>
        <span>Concurrency support is one of Rust's headline features and the standard library ships several primitives.</span>
    </div>
    <div>
        <a href="https://tokio.rs/tokio/tutorial">
            <h3>Tokio Tutorial</h3>
        </a>
        <span>Tokio is an asynchronous runtime for the Rust programming language. It provides the building blocks needed for writing networking applications.</span>
    </div>
    <div class="related-question-pair">
        <div role="heading">What is async in Rust?</div>
        <div>It's a language feature for writing futures-based concurrent code.</div>
        <a href="https://rust-lang.github.io/async-book/">async-book</a>
    </div>
    <div class="related-question-pair">
        <div role="heading">Is Rust good for async?</div>
        <div>Yes, with Tokio it competes with Go and Node for async server workloads.</div>
        <a href="https://tokio.rs/">tokio.rs</a>
    </div>
</body>
</html>
"#;

    #[test]
    fn parses_organic_results() {
        let serp = parse_google(SAMPLE);
        assert!(
            serp.organic.len() >= 2,
            "expected at least 2 organic results, got {}",
            serp.organic.len()
        );
        let urls: Vec<&str> = serp.organic.iter().map(|r| r.url.as_str()).collect();
        assert!(urls.iter().any(|u| u.contains("doc.rust-lang.org/book")));
        assert!(urls.iter().any(|u| u.contains("tokio.rs/tokio/tutorial")));
    }

    #[test]
    fn organic_positions_are_sequential() {
        let serp = parse_google(SAMPLE);
        for (i, r) in serp.organic.iter().enumerate() {
            assert_eq!(r.position as usize, i + 1);
        }
    }

    #[test]
    fn parses_featured_snippet() {
        let serp = parse_google(SAMPLE);
        let fs = serp.featured_snippet.expect("expected featured snippet");
        assert!(fs.text.contains("Asynchronous programming"));
        assert!(fs.url.contains("async-book"));
    }

    #[test]
    fn parses_people_also_ask() {
        let serp = parse_google(SAMPLE);
        assert_eq!(serp.people_also_ask.len(), 2);
        assert_eq!(serp.people_also_ask[0].question, "What is async in Rust?");
        assert_eq!(serp.people_also_ask[1].question, "Is Rust good for async?");
        assert!(serp.people_also_ask[0].answer.is_some());
    }

    #[test]
    fn empty_html_yields_empty_serp() {
        let serp = parse_google("<html></html>");
        assert!(serp.organic.is_empty());
        assert!(serp.featured_snippet.is_none());
        assert!(serp.people_also_ask.is_empty());
    }

    #[test]
    fn max_results_limit_respected() {
        let serp = parse_google_with_limit(SAMPLE, 1);
        assert!(serp.organic.len() <= 1);
    }

    #[test]
    fn google_internal_urls_filtered() {
        let html = r#"
            <a href="https://www.google.com/search?q=rust"><h3>Google search</h3></a>
            <a href="https://example.com/page"><h3>Real result</h3></a>
        "#;
        let serp = parse_google(html);
        assert!(serp.organic.iter().all(|r| !r.url.contains("google.com")));
        assert_eq!(serp.organic.len(), 1);
    }

    #[test]
    fn organic_results_dedup_on_url() {
        let html = r#"
            <a href="https://example.com/x"><h3>First</h3></a>
            <a href="https://example.com/x"><h3>Duplicate</h3></a>
        "#;
        let serp = parse_google(html);
        assert_eq!(serp.organic.len(), 1);
    }
}
