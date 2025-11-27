//! Phase 2: Social Media Tags (60%+45% adoption)
//!
//! Extracts Open Graph and Twitter Card metadata that control
//! how links appear when shared on social media platforms.
//!
//! - **Open Graph Protocol**: 60%+ adoption (Facebook, LinkedIn, WhatsApp, Slack, Discord)
//! - **Twitter Cards**: 45% adoption (Twitter/X)

pub mod opengraph;
pub mod twitter;

#[cfg(test)]
mod opengraph_tests;
#[cfg(test)]
mod twitter_tests;

pub use opengraph::extract as extract_opengraph;
pub use twitter::{
    extract as extract_twitter, extract_with_fallback as extract_twitter_with_fallback,
};
