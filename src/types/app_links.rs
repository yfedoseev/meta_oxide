//! Mobile / native-app deep-linking metadata.
//!
//! Covers three adjacent conventions that sites emit side by side:
//!
//! - **Facebook App Links** (`al:ios:*`, `al:android:*`, `al:web:*`,
//!   `al:windows:*`, `al:windows_phone:*`, `al:windows_universal:*`).
//! - **Apple Smart App Banner** (`apple-itunes-app`) — legacy but still
//!   ubiquitous on news and e-commerce sites.
//! - **Google Play intent hints** (`google-play-app`, `android-app-intent`).
//!
//! We normalise these into a single [`AppLinks`] view so consumers can
//! reason about "where does this page want to open natively" without
//! caring about which vendor invented which tag.

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};

/// Per-platform App Links target.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AppLinkPlatform {
    /// Custom URL scheme (`twitter://`, `fb://`).
    pub url: Option<String>,
    /// Universal / deep link URL (HTTPS, not a scheme).
    pub app_name: Option<String>,
    /// iOS App Store ID / Android package name.
    pub app_store_id: Option<String>,
    /// Google Play package name (alias of `app_store_id` on Android).
    pub package: Option<String>,
    /// Custom URL the app should open (`al:*:url`).
    pub class: Option<String>,
}

/// Apple Smart App Banner (`<meta name="apple-itunes-app">`).
///
/// Body is a comma-separated `app-id=NNN, app-argument=URL, affiliate-data=…`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AppleItunesApp {
    /// Numeric App Store ID.
    pub app_id: Option<String>,
    /// URL/deep link the app should open.
    pub app_argument: Option<String>,
    /// iTunes affiliate attribution data.
    pub affiliate_data: Option<String>,
}

/// Google Play / Android intent hint (`<meta name="google-play-app">`
/// or `<meta name="android-app-intent">`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GooglePlayApp {
    /// Android package name (`com.example.app`).
    pub app_id: Option<String>,
    /// Full `intent://` URI if declared.
    pub intent: Option<String>,
}

/// Unified deep-linking / App Links surface for a page.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AppLinks {
    /// iOS App Links (`al:ios:*`).
    pub ios: Option<AppLinkPlatform>,
    /// iPad-specific App Links (`al:ipad:*`).
    pub ipad: Option<AppLinkPlatform>,
    /// iPhone-specific App Links (`al:iphone:*`).
    pub iphone: Option<AppLinkPlatform>,
    /// Android App Links (`al:android:*`).
    pub android: Option<AppLinkPlatform>,
    /// Web fallback URL (`al:web:url`) and `should_fallback` hint.
    pub web_url: Option<String>,
    /// Whether clients should fall back to `web_url` (`al:web:should_fallback`).
    pub web_should_fallback: Option<bool>,
    /// Windows family (`al:windows*:*` merged together).
    pub windows: Option<AppLinkPlatform>,
    /// Apple Smart App Banner metadata.
    pub apple_itunes_app: Option<AppleItunesApp>,
    /// Google Play / Android intent hints.
    pub google_play_app: Option<GooglePlayApp>,
}

impl AppLinks {
    /// True when no platform hints were found.
    pub fn is_empty(&self) -> bool {
        self.ios.is_none()
            && self.ipad.is_none()
            && self.iphone.is_none()
            && self.android.is_none()
            && self.web_url.is_none()
            && self.windows.is_none()
            && self.apple_itunes_app.is_none()
            && self.google_play_app.is_none()
    }
}

#[cfg(feature = "python")]
impl AppLinks {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        if let Some(v) = &self.ios {
            dict.set_item("ios", v.to_py_dict(py)).ok();
        }
        if let Some(v) = &self.ipad {
            dict.set_item("ipad", v.to_py_dict(py)).ok();
        }
        if let Some(v) = &self.iphone {
            dict.set_item("iphone", v.to_py_dict(py)).ok();
        }
        if let Some(v) = &self.android {
            dict.set_item("android", v.to_py_dict(py)).ok();
        }
        if let Some(v) = &self.web_url {
            dict.set_item("web_url", v).ok();
        }
        if let Some(v) = self.web_should_fallback {
            dict.set_item("web_should_fallback", v).ok();
        }
        if let Some(v) = &self.windows {
            dict.set_item("windows", v.to_py_dict(py)).ok();
        }
        if let Some(v) = &self.apple_itunes_app {
            dict.set_item("apple_itunes_app", v.to_py_dict(py)).ok();
        }
        if let Some(v) = &self.google_play_app {
            dict.set_item("google_play_app", v.to_py_dict(py)).ok();
        }
        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl AppLinkPlatform {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        if let Some(v) = &self.url {
            dict.set_item("url", v).ok();
        }
        if let Some(v) = &self.app_name {
            dict.set_item("app_name", v).ok();
        }
        if let Some(v) = &self.app_store_id {
            dict.set_item("app_store_id", v).ok();
        }
        if let Some(v) = &self.package {
            dict.set_item("package", v).ok();
        }
        if let Some(v) = &self.class {
            dict.set_item("class", v).ok();
        }
        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl AppleItunesApp {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        if let Some(v) = &self.app_id {
            dict.set_item("app_id", v).ok();
        }
        if let Some(v) = &self.app_argument {
            dict.set_item("app_argument", v).ok();
        }
        if let Some(v) = &self.affiliate_data {
            dict.set_item("affiliate_data", v).ok();
        }
        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl GooglePlayApp {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        if let Some(v) = &self.app_id {
            dict.set_item("app_id", v).ok();
        }
        if let Some(v) = &self.intent {
            dict.set_item("intent", v).ok();
        }
        dict.unbind()
    }
}
