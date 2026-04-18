//! Schema.org `SpeakableSpecification` surface.
//!
//! Publishers (notably Google News) mark parts of an article as
//! "speakable" — suitable for voice-assistant readback. The marker is
//! a `speakable` property on the JSON-LD `Article` / `NewsArticle`
//! object, whose value is a `SpeakableSpecification` with CSS
//! selectors, XPath expressions, or URL fragments pointing at the
//! speakable content.
//!
//! This type is a consumer-friendly projection — the raw JSON-LD is
//! still available via [`crate::types::jsonld::JsonLdObject`] if you
//! need fidelity.

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};

/// A single SpeakableSpecification target.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Speakable {
    /// CSS selectors pointing at the speakable fragments.
    pub css_selectors: Vec<String>,
    /// XPath expressions pointing at the speakable fragments.
    pub xpaths: Vec<String>,
    /// URLs (usually fragment identifiers) pointing at the speakable fragments.
    pub urls: Vec<String>,
}

impl Speakable {
    /// True when the specification has no selectors.
    pub fn is_empty(&self) -> bool {
        self.css_selectors.is_empty() && self.xpaths.is_empty() && self.urls.is_empty()
    }
}

#[cfg(feature = "python")]
impl Speakable {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        if !self.css_selectors.is_empty() {
            dict.set_item("cssSelector", self.css_selectors.clone()).ok();
        }
        if !self.xpaths.is_empty() {
            dict.set_item("xpath", self.xpaths.clone()).ok();
        }
        if !self.urls.is_empty() {
            dict.set_item("url", self.urls.clone()).ok();
        }
        dict.unbind()
    }
}
