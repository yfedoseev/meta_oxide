//! Types for oEmbed endpoint discovery (Phase 5)
//!
//! oEmbed is a format for allowing an embedded representation of a URL
//! on third party sites. Many platforms (YouTube, Vimeo, Twitter, etc.)
//! support oEmbed for easy content embedding.

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};

/// oEmbed endpoint discovered from HTML link tags
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OEmbedEndpoint {
    /// The oEmbed endpoint URL
    pub href: String,

    /// The format type (application/json+oembed or text/xml+oembed)
    pub format: OEmbedFormat,

    /// Optional title for the endpoint
    pub title: Option<String>,
}

/// oEmbed format type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OEmbedFormat {
    /// JSON format (application/json+oembed)
    Json,
    /// XML format (text/xml+oembed)
    Xml,
}

/// Collection of discovered oEmbed endpoints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OEmbedDiscovery {
    /// JSON endpoints
    pub json_endpoints: Vec<OEmbedEndpoint>,

    /// XML endpoints
    pub xml_endpoints: Vec<OEmbedEndpoint>,
}

impl OEmbedDiscovery {
    /// Check if any endpoints were discovered
    pub fn has_endpoints(&self) -> bool {
        !self.json_endpoints.is_empty() || !self.xml_endpoints.is_empty()
    }

    /// Get the preferred JSON endpoint if available
    pub fn preferred_json(&self) -> Option<&OEmbedEndpoint> {
        self.json_endpoints.first()
    }

    /// Get the preferred XML endpoint if available
    pub fn preferred_xml(&self) -> Option<&OEmbedEndpoint> {
        self.xml_endpoints.first()
    }
}

// Python conversion implementations
#[cfg(feature = "python")]
impl OEmbedEndpoint {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        dict.set_item("href", &self.href).unwrap();
        dict.set_item(
            "format",
            match self.format {
                OEmbedFormat::Json => "json",
                OEmbedFormat::Xml => "xml",
            },
        )
        .unwrap();

        if let Some(ref title) = self.title {
            dict.set_item("title", title).unwrap();
        }

        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl OEmbedDiscovery {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        if !self.json_endpoints.is_empty() {
            let json_eps: Vec<_> = self.json_endpoints.iter().map(|ep| ep.to_py_dict(py)).collect();
            dict.set_item("json_endpoints", json_eps).unwrap();
        }

        if !self.xml_endpoints.is_empty() {
            let xml_eps: Vec<_> = self.xml_endpoints.iter().map(|ep| ep.to_py_dict(py)).collect();
            dict.set_item("xml_endpoints", xml_eps).unwrap();
        }

        dict.unbind()
    }
}
