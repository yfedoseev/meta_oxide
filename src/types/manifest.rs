//! Types for Web App Manifest
//!
//! Web App Manifest is a JSON file providing metadata for Progressive Web Apps (PWAs).
//! It enables web applications to be installed on devices and provides app-like experiences.

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::{PyDict, PyList};
use serde::{Deserialize, Serialize};

/// Web App Manifest metadata
///
/// Complete manifest specification from W3C Web App Manifest spec
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppManifest {
    /// Application name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Short name for home screen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,

    /// Application description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Start URL when launched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_url: Option<String>,

    /// Display mode (fullscreen, standalone, minimal-ui, browser)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    /// Screen orientation preference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<String>,

    /// Theme color for browser chrome
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_color: Option<String>,

    /// Background color for splash screen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,

    /// Application scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    /// Language/direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,

    /// Text direction (ltr, rtl, auto)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<String>,

    /// Application icons
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub icons: Vec<ManifestIcon>,

    /// Related applications
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_applications: Vec<RelatedApplication>,

    /// Prefer related applications over web app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_related_applications: Option<bool>,

    /// Application categories
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,

    /// Screenshots for app stores
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub screenshots: Vec<ManifestImage>,

    /// Shortcuts for quick actions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shortcuts: Vec<ManifestShortcut>,

    /// Unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Manifest icon definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManifestIcon {
    /// Icon source URL
    pub src: String,

    /// Icon sizes (e.g., "192x192", "512x512")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizes: Option<String>,

    /// MIME type of the icon
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// Purpose of the icon (any, maskable, monochrome)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
}

/// Related native application
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelatedApplication {
    /// Platform identifier (play, itunes, windows, etc.)
    pub platform: String,

    /// URL to the application
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Application ID on the platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Generic manifest image (for screenshots)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManifestImage {
    /// Image source URL
    pub src: String,

    /// Image sizes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizes: Option<String>,

    /// MIME type of the image
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// Image label/description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Shortcut definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManifestShortcut {
    /// Shortcut name
    pub name: String,

    /// Shortcut URL
    pub url: String,

    /// Short name for the shortcut
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,

    /// Shortcut description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Icons for the shortcut
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub icons: Vec<ManifestIcon>,
}

/// Manifest link discovery result
///
/// Contains either just the href or both href and parsed manifest
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ManifestDiscovery {
    /// The manifest link URL (resolved)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,

    /// Parsed manifest content (if provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest: Option<WebAppManifest>,
}

#[cfg(feature = "python")]
impl WebAppManifest {
    /// Convert to Python dictionary
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        if let Some(ref name) = self.name {
            dict.set_item("name", name).unwrap();
        }
        if let Some(ref short_name) = self.short_name {
            dict.set_item("short_name", short_name).unwrap();
        }
        if let Some(ref description) = self.description {
            dict.set_item("description", description).unwrap();
        }
        if let Some(ref start_url) = self.start_url {
            dict.set_item("start_url", start_url).unwrap();
        }
        if let Some(ref display) = self.display {
            dict.set_item("display", display).unwrap();
        }
        if let Some(ref orientation) = self.orientation {
            dict.set_item("orientation", orientation).unwrap();
        }
        if let Some(ref theme_color) = self.theme_color {
            dict.set_item("theme_color", theme_color).unwrap();
        }
        if let Some(ref background_color) = self.background_color {
            dict.set_item("background_color", background_color).unwrap();
        }
        if let Some(ref scope) = self.scope {
            dict.set_item("scope", scope).unwrap();
        }
        if let Some(ref lang) = self.lang {
            dict.set_item("lang", lang).unwrap();
        }
        if let Some(ref dir) = self.dir {
            dict.set_item("dir", dir).unwrap();
        }
        if let Some(ref id) = self.id {
            dict.set_item("id", id).unwrap();
        }
        if let Some(prefer) = self.prefer_related_applications {
            dict.set_item("prefer_related_applications", prefer).unwrap();
        }

        // Icons array
        if !self.icons.is_empty() {
            let icons_list = PyList::empty_bound(py);
            for icon in &self.icons {
                icons_list.append(icon.to_py_dict(py)).unwrap();
            }
            dict.set_item("icons", icons_list).unwrap();
        }

        // Related applications array
        if !self.related_applications.is_empty() {
            let apps_list = PyList::empty_bound(py);
            for app in &self.related_applications {
                apps_list.append(app.to_py_dict(py)).unwrap();
            }
            dict.set_item("related_applications", apps_list).unwrap();
        }

        // Categories array
        if !self.categories.is_empty() {
            dict.set_item("categories", self.categories.clone()).unwrap();
        }

        // Screenshots array
        if !self.screenshots.is_empty() {
            let screenshots_list = PyList::empty_bound(py);
            for screenshot in &self.screenshots {
                screenshots_list.append(screenshot.to_py_dict(py)).unwrap();
            }
            dict.set_item("screenshots", screenshots_list).unwrap();
        }

        // Shortcuts array
        if !self.shortcuts.is_empty() {
            let shortcuts_list = PyList::empty_bound(py);
            for shortcut in &self.shortcuts {
                shortcuts_list.append(shortcut.to_py_dict(py)).unwrap();
            }
            dict.set_item("shortcuts", shortcuts_list).unwrap();
        }

        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl ManifestIcon {
    /// Convert to Python dictionary
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        dict.set_item("src", &self.src).unwrap();
        if let Some(ref sizes) = self.sizes {
            dict.set_item("sizes", sizes).unwrap();
        }
        if let Some(ref mime_type) = self.mime_type {
            dict.set_item("type", mime_type).unwrap();
        }
        if let Some(ref purpose) = self.purpose {
            dict.set_item("purpose", purpose).unwrap();
        }
        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl RelatedApplication {
    /// Convert to Python dictionary
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        dict.set_item("platform", &self.platform).unwrap();
        if let Some(ref url) = self.url {
            dict.set_item("url", url).unwrap();
        }
        if let Some(ref id) = self.id {
            dict.set_item("id", id).unwrap();
        }
        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl ManifestImage {
    /// Convert to Python dictionary
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        dict.set_item("src", &self.src).unwrap();
        if let Some(ref sizes) = self.sizes {
            dict.set_item("sizes", sizes).unwrap();
        }
        if let Some(ref mime_type) = self.mime_type {
            dict.set_item("type", mime_type).unwrap();
        }
        if let Some(ref label) = self.label {
            dict.set_item("label", label).unwrap();
        }
        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl ManifestShortcut {
    /// Convert to Python dictionary
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        dict.set_item("name", &self.name).unwrap();
        dict.set_item("url", &self.url).unwrap();
        if let Some(ref short_name) = self.short_name {
            dict.set_item("short_name", short_name).unwrap();
        }
        if let Some(ref description) = self.description {
            dict.set_item("description", description).unwrap();
        }
        if !self.icons.is_empty() {
            let icons_list = PyList::empty_bound(py);
            for icon in &self.icons {
                icons_list.append(icon.to_py_dict(py)).unwrap();
            }
            dict.set_item("icons", icons_list).unwrap();
        }
        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl ManifestDiscovery {
    /// Convert to Python dictionary
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        if let Some(ref href) = self.href {
            dict.set_item("href", href).unwrap();
        }
        if let Some(ref manifest) = self.manifest {
            dict.set_item("manifest", manifest.to_py_dict(py)).unwrap();
        }
        dict.unbind()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_app_manifest_default() {
        let manifest = WebAppManifest::default();
        assert!(manifest.name.is_none());
        assert!(manifest.icons.is_empty());
    }

    #[test]
    fn test_manifest_icon_serde() {
        let icon = ManifestIcon {
            src: "/icon-192.png".to_string(),
            sizes: Some("192x192".to_string()),
            mime_type: Some("image/png".to_string()),
            purpose: Some("any".to_string()),
        };

        let json = serde_json::to_string(&icon).unwrap();
        let deserialized: ManifestIcon = serde_json::from_str(&json).unwrap();
        assert_eq!(icon, deserialized);
    }

    #[test]
    fn test_manifest_serde_skip_none() {
        let manifest = WebAppManifest { name: Some("Test App".to_string()), ..Default::default() };

        let json = serde_json::to_value(&manifest).unwrap();
        let obj = json.as_object().unwrap();

        // None fields should be skipped
        assert!(obj.contains_key("name"));
        assert!(!obj.contains_key("description"));
        assert!(!obj.contains_key("start_url"));
    }

    #[test]
    fn test_manifest_full_parse() {
        let json = r##"
{
    "name": "Test PWA",
    "short_name": "Test",
    "start_url": "/",
    "display": "standalone",
    "theme_color": "#000000",
    "icons": [{
        "src": "/icon.png",
        "sizes": "192x192",
        "type": "image/png"
    }]
}
"##;

        let manifest: WebAppManifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.name, Some("Test PWA".to_string()));
        assert_eq!(manifest.icons.len(), 1);
    }

    #[test]
    fn test_manifest_to_py_dict() {
        Python::with_gil(|py| {
            let manifest = WebAppManifest {
                name: Some("Test App".to_string()),
                short_name: Some("Test".to_string()),
                ..Default::default()
            };

            let py_dict = manifest.to_py_dict(py);
            let dict = py_dict.bind(py);

            assert!(dict.contains("name").unwrap());
            assert!(dict.contains("short_name").unwrap());
        });
    }

    #[test]
    fn test_manifest_icon_to_py_dict() {
        Python::with_gil(|py| {
            let icon = ManifestIcon {
                src: "/icon.png".to_string(),
                sizes: Some("192x192".to_string()),
                mime_type: Some("image/png".to_string()),
                purpose: None,
            };

            let py_dict = icon.to_py_dict(py);
            let dict = py_dict.bind(py);

            assert!(dict.contains("src").unwrap());
            assert!(dict.contains("sizes").unwrap());
            assert!(dict.contains("type").unwrap());
        });
    }

    #[test]
    fn test_related_application() {
        let app = RelatedApplication {
            platform: "play".to_string(),
            url: Some("https://play.google.com/store/apps/details?id=com.example".to_string()),
            id: Some("com.example".to_string()),
        };

        let json = serde_json::to_string(&app).unwrap();
        let deserialized: RelatedApplication = serde_json::from_str(&json).unwrap();
        assert_eq!(app, deserialized);
    }

    #[test]
    fn test_manifest_shortcut() {
        let shortcut = ManifestShortcut {
            name: "New Item".to_string(),
            url: "/new".to_string(),
            short_name: None,
            description: Some("Create new item".to_string()),
            icons: vec![],
        };

        let json = serde_json::to_string(&shortcut).unwrap();
        let deserialized: ManifestShortcut = serde_json::from_str(&json).unwrap();
        assert_eq!(shortcut, deserialized);
    }

    #[test]
    fn test_manifest_discovery_default() {
        let discovery = ManifestDiscovery::default();
        assert!(discovery.href.is_none());
        assert!(discovery.manifest.is_none());
    }

    #[test]
    fn test_manifest_discovery_to_py_dict() {
        Python::with_gil(|py| {
            let discovery =
                ManifestDiscovery { href: Some("/manifest.json".to_string()), manifest: None };

            let py_dict = discovery.to_py_dict(py);
            let dict = py_dict.bind(py);

            assert!(dict.contains("href").unwrap());
        });
    }
}
