//! Type definitions for metadata extraction

pub mod dublin_core;
pub mod jsonld;
pub mod manifest;
pub mod meta;
pub mod microdata;
pub mod microformats;
pub mod oembed;
pub mod rdfa;
pub mod social;

// Re-export microformat types for backward compatibility
pub use microformats::*;
