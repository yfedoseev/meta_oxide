//! Phase 7: Microformats2 extraction (5-10% adoption)
//!
//! Microformats are a simple way to mark up information in HTML using class names.
//! While they have lower adoption (5-10%), they're important for the IndieWeb community
//! and personal blogs.

pub mod hadr;
pub mod hcard;
pub mod hentry;
pub mod hevent;
pub mod hfeed;
pub mod hgeo;
pub mod hproduct;
pub mod hrecipe;
pub mod hreview;

pub use hcard::extract as extract_hcard;
pub use hentry::extract as extract_hentry;
pub use hevent::extract as extract_hevent;
