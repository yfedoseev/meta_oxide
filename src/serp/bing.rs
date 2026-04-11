//! Bing SERP parser stub. Returns an empty [`SerpGraph`] until a real
//! implementation is needed by a downstream caller. Tracked as a P1 follow-up.

use crate::serp::types::SerpGraph;

/// Parse a Bing SERP page. Currently returns an empty graph.
pub fn parse_bing(_html: &str) -> SerpGraph {
    SerpGraph::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_returns_empty() {
        let serp = parse_bing("<html></html>");
        assert!(serp.organic.is_empty());
    }
}
