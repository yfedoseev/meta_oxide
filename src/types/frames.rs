//! Farcaster Frames and Open Frames metadata
//!
//! Frames are an open spec for interactive mini-apps embedded in social
//! feeds via `<meta>` tags. Two conventions are in wide use in 2026:
//!
//! - **Farcaster Frames** — `fc:frame:*` (original Warpcast-led spec)
//! - **Open Frames** — `of:*` (vendor-neutral variant adopted by Lens,
//!   Bluesky extensions, and third-party clients)
//!
//! This extractor does not render or execute frames — it surfaces the
//! declared metadata so downstream tools (link-preview renderers,
//! moderation pipelines, agent workflows) can inspect them.

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};

/// A single Frame button declared via `fc:frame:button:N` / `of:button:N`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FrameButton {
    /// 1-based index (`button:1`, `button:2`, …).
    pub index: u8,
    /// Button label shown to the user.
    pub label: Option<String>,
    /// Action kind (`post`, `post_redirect`, `link`, `mint`, `tx`). Spec-dependent.
    pub action: Option<String>,
    /// Target URL for `post` / `post_redirect` / `link`.
    pub target: Option<String>,
    /// Post-URL override (for `tx` flows).
    pub post_url: Option<String>,
}

/// Farcaster Frame / Open Frame metadata discovered in a document.
///
/// All fields are optional — a page may declare only a subset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Frame {
    /// Declared spec version (`vNext`, `2024-02-09`, …) from `fc:frame` or `of:version`.
    pub version: Option<String>,
    /// Frame image URL (`fc:frame:image` / `of:image`).
    pub image: Option<String>,
    /// Image aspect ratio hint (`1.91:1` or `1:1`).
    pub image_aspect_ratio: Option<String>,
    /// Post URL invoked when a button is pressed.
    pub post_url: Option<String>,
    /// Input box prompt text (`fc:frame:input:text` / `of:input:text`).
    pub input_text: Option<String>,
    /// Opaque state token round-tripped across frame interactions.
    pub state: Option<String>,
    /// Refresh period hint in seconds (`of:refresh_period`).
    pub refresh_period: Option<u32>,
    /// Accepts list (`of:accepts:*`) — client capability negotiation.
    pub accepts: Vec<String>,
    /// Buttons in declaration order.
    pub buttons: Vec<FrameButton>,
    /// Whether the page declared any `fc:frame:*` tags.
    pub has_farcaster: bool,
    /// Whether the page declared any `of:*` tags.
    pub has_open_frames: bool,
}

impl Frame {
    /// True when neither Farcaster nor Open Frames tags were present.
    pub fn is_empty(&self) -> bool {
        !self.has_farcaster
            && !self.has_open_frames
            && self.image.is_none()
            && self.buttons.is_empty()
    }
}

#[cfg(feature = "python")]
impl Frame {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        if let Some(v) = &self.version {
            dict.set_item("version", v).ok();
        }
        if let Some(v) = &self.image {
            dict.set_item("image", v).ok();
        }
        if let Some(v) = &self.image_aspect_ratio {
            dict.set_item("image_aspect_ratio", v).ok();
        }
        if let Some(v) = &self.post_url {
            dict.set_item("post_url", v).ok();
        }
        if let Some(v) = &self.input_text {
            dict.set_item("input_text", v).ok();
        }
        if let Some(v) = &self.state {
            dict.set_item("state", v).ok();
        }
        if let Some(v) = self.refresh_period {
            dict.set_item("refresh_period", v).ok();
        }
        if !self.accepts.is_empty() {
            dict.set_item("accepts", self.accepts.clone()).ok();
        }
        if !self.buttons.is_empty() {
            let buttons: Vec<Py<PyDict>> = self.buttons.iter().map(|b| b.to_py_dict(py)).collect();
            dict.set_item("buttons", buttons).ok();
        }
        dict.set_item("has_farcaster", self.has_farcaster).ok();
        dict.set_item("has_open_frames", self.has_open_frames).ok();
        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl FrameButton {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        dict.set_item("index", self.index).ok();
        if let Some(v) = &self.label {
            dict.set_item("label", v).ok();
        }
        if let Some(v) = &self.action {
            dict.set_item("action", v).ok();
        }
        if let Some(v) = &self.target {
            dict.set_item("target", v).ok();
        }
        if let Some(v) = &self.post_url {
            dict.set_item("post_url", v).ok();
        }
        dict.unbind()
    }
}
