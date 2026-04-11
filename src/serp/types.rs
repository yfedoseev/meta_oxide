//! SERP type hierarchy. See [`crate::serp`] for an overview.

use crate::canonical::KnowledgeGraph;
use serde::{Deserialize, Serialize};

/// Aggregate parse of a single SERP page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SerpGraph {
    /// Featured snippet box, if Google chose to show one.
    pub featured_snippet: Option<FeaturedSnippet>,
    /// Right-hand-side knowledge panel, if present.
    pub knowledge_graph: Option<KnowledgeGraph>,
    /// "People also ask" expandable questions.
    pub people_also_ask: Vec<PeopleAlsoAsk>,
    /// "Related searches" suggestions at the bottom of the page.
    pub related_searches: Vec<String>,
    /// Site links shown under a top organic result.
    pub site_links: Vec<SiteLink>,
    /// Organic search results in display order.
    pub organic: Vec<OrganicResult>,
}

/// One organic search result.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrganicResult {
    /// 1-indexed position on the page.
    pub position: u32,
    /// Result title (the `<h3>` text).
    pub title: String,
    /// Destination URL.
    pub url: String,
    /// Snippet / preview text under the title.
    pub snippet: String,
    /// Optional date pulled from the snippet, if Google added one.
    pub date: Option<String>,
}

/// Featured snippet (the answer box at the top of the SERP).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FeaturedSnippet {
    /// Snippet body text.
    pub text: String,
    /// Title of the source page.
    pub title: String,
    /// URL of the source page.
    pub url: String,
    /// Layout type when known.
    pub snippet_type: Option<FeaturedSnippetType>,
}

/// Featured snippet layout variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeaturedSnippetType {
    /// Single paragraph of text.
    Paragraph,
    /// Bulleted or numbered list.
    List,
    /// Tabular layout.
    Table,
    /// Video answer card.
    Video,
}

/// One question/answer pair from the "People also ask" widget.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PeopleAlsoAsk {
    /// Question text.
    pub question: String,
    /// Answer text if Google expanded it.
    pub answer: Option<String>,
    /// Source page URL if Google attributed one.
    pub source_url: Option<String>,
    /// Source page title if Google attributed one.
    pub source_title: Option<String>,
}

/// One site link displayed under a top organic result.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SiteLink {
    /// Title of the linked sub-page.
    pub title: String,
    /// URL of the linked sub-page.
    pub url: String,
    /// Snippet text shown alongside the link, if any.
    pub snippet: Option<String>,
}
