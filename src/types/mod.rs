//! Type definitions for metadata extraction

pub mod activitypub;
pub mod ai_directives;
pub mod app_links;
pub mod c2pa;
pub mod dublin_core;
pub mod feeds;
pub mod frames;
pub mod jsonld;
pub mod manifest;
pub mod meta;
pub mod microdata;
pub mod microformats;
pub mod oembed;
pub mod rdfa;
pub mod social;
pub mod speakable;
pub mod verification;

// Re-export microformat types for backward compatibility
pub use microformats::*;
