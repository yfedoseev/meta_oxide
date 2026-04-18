//! C2PA / Content Credentials surface.
//!
//! The [C2PA specification](https://c2pa.org/specifications/) defines
//! cryptographically-signed content provenance manifests (claim generator,
//! signer, assertions, ingredients). Full verification happens in media
//! bytes — not in HTML. But in 2026, many publishers also declare the
//! manifest's existence in the HTML head:
//!
//! - `<meta name="c2pa:claim_generator" content="Adobe Photoshop 26.0 / OpenAI DALL·E 3">`
//! - `<meta property="c2pa:producer" content="AP News">`
//! - `<link rel="c2pa-manifest" href="/path/to/manifest.c2pa">`
//!
//! This extractor surfaces those declarations. It does **not** fetch or
//! verify the signed manifest — that requires a full C2PA validator
//! (see [`c2pa-rs`](https://github.com/contentauth/c2pa-rs)). Think of
//! this output as "a hint that Content Credentials exist on this page".

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// C2PA metadata surfaced from HTML.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct C2paSurface {
    /// Human-readable claim-generator string (tool that created/edited the asset).
    pub claim_generator: Option<String>,
    /// Asset producer (organization / person).
    pub producer: Option<String>,
    /// Signer / issuer name from the trust list.
    pub signer: Option<String>,
    /// Asset title declared in the manifest.
    pub title: Option<String>,
    /// Asset format (`image/jpeg`, `video/mp4`, …).
    pub format: Option<String>,
    /// High-level AI-action summary (`ai-generated`, `edited`, `trained-on`).
    pub ai_action: Option<String>,
    /// URL of the signed C2PA manifest (`<link rel="c2pa-manifest" href="…">`
    /// or `<meta name="c2pa:manifest">`).
    pub manifest_url: Option<String>,
    /// Thumbnail URL declared in the manifest.
    pub thumbnail_url: Option<String>,
    /// Free-form additional `c2pa:*` meta tags we didn't model explicitly.
    pub other: BTreeMap<String, String>,
}

impl C2paSurface {
    /// True when no C2PA-related declarations were found.
    pub fn is_empty(&self) -> bool {
        self.claim_generator.is_none()
            && self.producer.is_none()
            && self.signer.is_none()
            && self.title.is_none()
            && self.format.is_none()
            && self.ai_action.is_none()
            && self.manifest_url.is_none()
            && self.thumbnail_url.is_none()
            && self.other.is_empty()
    }
}

#[cfg(feature = "python")]
impl C2paSurface {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        macro_rules! opt {
            ($f:ident, $k:literal) => {
                if let Some(v) = &self.$f {
                    dict.set_item($k, v).ok();
                }
            };
        }
        opt!(claim_generator, "claim_generator");
        opt!(producer, "producer");
        opt!(signer, "signer");
        opt!(title, "title");
        opt!(format, "format");
        opt!(ai_action, "ai_action");
        opt!(manifest_url, "manifest_url");
        opt!(thumbnail_url, "thumbnail_url");
        if !self.other.is_empty() {
            let other = PyDict::new_bound(py);
            for (k, v) in &self.other {
                other.set_item(k, v).ok();
            }
            dict.set_item("other", other).ok();
        }
        dict.unbind()
    }
}
