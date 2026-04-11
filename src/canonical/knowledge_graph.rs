//! Canonical knowledge-graph entity — used by [`crate::serp`] (Google
//! Knowledge Graph panels), [`crate::wikipedia`] (Wikipedia infoboxes), and
//! [`crate::wikidata`] (Wikidata entity JSON), so they can all return the same
//! shape.
//!
//! Lives in `canonical/` rather than `serp/` so that the Wikipedia and
//! Wikidata parsers don't have to depend on the SERP module just to use the
//! type. The SERP module re-exports it as [`crate::serp::KnowledgeGraph`].

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A normalized knowledge-graph entity (Person, Organization, Place, Movie,
/// …) merged from whichever source produced it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    /// Display name of the entity.
    pub name: String,
    /// Schema.org-style type label (`"Person"`, `"Organization"`, `"Movie"`).
    pub entity_type: Option<String>,
    /// Short description / standfirst.
    pub description: Option<String>,
    /// Hero image URL.
    pub image_url: Option<String>,
    /// Wikipedia page URL if linked.
    pub wikipedia_url: Option<String>,
    /// Wikidata `Q`-id if known.
    pub wikidata_id: Option<String>,
    /// Free-form attribute map (`"Born"` → `"1879-03-14"`, `"Spouse"` → …).
    pub attributes: HashMap<String, String>,
    /// Names of related entities surfaced by the source.
    pub related_entities: Vec<String>,
}
