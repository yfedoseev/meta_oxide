//! DuckDuckGo SERP parser stub. Returns an empty [`SerpGraph`] until a real
//! implementation is needed by a downstream caller. Tracked as a P1 follow-up.

use crate::serp::types::SerpGraph;

/// Parse a DuckDuckGo SERP page. Currently returns an empty graph.
pub fn parse_duckduckgo(_html: &str) -> SerpGraph {
    SerpGraph::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_returns_empty() {
        let serp = parse_duckduckgo("<html></html>");
        assert!(serp.organic.is_empty());
        assert!(serp.featured_snippet.is_none());
        assert!(serp.people_also_ask.is_empty());
    }
}
