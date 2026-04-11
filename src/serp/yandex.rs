//! Yandex SERP parser stub. Returns an empty [`SerpGraph`] until a real
//! implementation is needed by a downstream caller. Tracked as a P2 follow-up.

use crate::serp::types::SerpGraph;

/// Parse a Yandex SERP page. Currently returns an empty graph.
pub fn parse_yandex(_html: &str) -> SerpGraph {
    SerpGraph::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_returns_empty() {
        let serp = parse_yandex("<html></html>");
        assert!(serp.organic.is_empty());
    }
}
