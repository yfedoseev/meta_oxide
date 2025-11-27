//! Extractors for various structured data formats
//!
//! Organized by implementation phase (prioritized by real-world adoption):
//! - Phase 1: Standard HTML Meta Tags (100% adoption)
//! - Phase 2: Social Media - Open Graph, Twitter Cards (60%+45% adoption)
//! - Phase 3: JSON-LD / Schema.org (41% adoption, growing)
//! - Phase 4: Microdata (26% adoption)
//! - Phase 5: oEmbed (content embedding)
//! - Phase 7: Microformats (5-10% adoption)
//! - Phase 9: Dublin Core (archives and digital libraries)

pub mod common;

// Phase 1: Standard Meta Tags (100% adoption) - IMPLEMENTED
pub mod meta;

// Phase 2: Social Media (60%+45% adoption) - IMPLEMENTED
pub mod social;

// Phase 3: JSON-LD (41% adoption) - IMPLEMENTED
pub mod jsonld;

// Phase 4: Microdata (26% adoption) - IMPLEMENTED
pub mod microdata;

// Phase 5: oEmbed - IMPLEMENTED
pub mod oembed;

// Phase 7: Microformats (5-10% adoption) - IMPLEMENTED
pub mod microformats;

// Phase 9: Dublin Core - IMPLEMENTED
pub mod dublin_core;

// RDFa - W3C standard for semantic markup (62% adoption)
pub mod rdfa;

// Web App Manifest - PWA metadata
pub mod manifest;

// rel-* link relationships
pub mod rel_links;

// Re-export microformats extractors for backward compatibility
#[allow(unused_imports)]
pub use microformats::{extract_hcard, extract_hentry, extract_hevent};
