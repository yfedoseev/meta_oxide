//! Types for Dublin Core metadata (Phase 9)
//!
//! Dublin Core is a metadata standard with 15 core elements
//! commonly used in digital libraries and archives.

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};

/// Dublin Core metadata elements
///
/// Dublin Core Metadata Element Set (DCMES) provides a simple
/// and standardized set of conventions for describing resources.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DublinCore {
    /// A name given to the resource
    pub title: Option<String>,

    /// An entity primarily responsible for making the resource
    pub creator: Option<String>,

    /// The topic of the resource
    pub subject: Option<Vec<String>>,

    /// An account of the resource
    pub description: Option<String>,

    /// An entity responsible for making the resource available
    pub publisher: Option<String>,

    /// An entity responsible for making contributions to the resource
    pub contributor: Option<Vec<String>>,

    /// A point or period of time associated with an event in the lifecycle of the resource
    pub date: Option<String>,

    /// The nature or genre of the resource
    pub type_: Option<String>,

    /// The file format, physical medium, or dimensions of the resource
    pub format: Option<String>,

    /// An unambiguous reference to the resource within a given context
    pub identifier: Option<String>,

    /// A related resource from which the described resource is derived
    pub source: Option<String>,

    /// A language of the resource
    pub language: Option<String>,

    /// A related resource
    pub relation: Option<String>,

    /// The spatial or temporal topic of the resource
    pub coverage: Option<String>,

    /// Information about rights held in and over the resource
    pub rights: Option<String>,
}

// Python conversion implementations
#[cfg(feature = "python")]
impl DublinCore {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        if let Some(ref v) = self.title {
            dict.set_item("title", v).unwrap();
        }
        if let Some(ref v) = self.creator {
            dict.set_item("creator", v).unwrap();
        }
        if let Some(ref v) = self.subject {
            dict.set_item("subject", v.clone()).unwrap();
        }
        if let Some(ref v) = self.description {
            dict.set_item("description", v).unwrap();
        }
        if let Some(ref v) = self.publisher {
            dict.set_item("publisher", v).unwrap();
        }
        if let Some(ref v) = self.contributor {
            dict.set_item("contributor", v.clone()).unwrap();
        }
        if let Some(ref v) = self.date {
            dict.set_item("date", v).unwrap();
        }
        if let Some(ref v) = self.type_ {
            dict.set_item("type", v).unwrap();
        }
        if let Some(ref v) = self.format {
            dict.set_item("format", v).unwrap();
        }
        if let Some(ref v) = self.identifier {
            dict.set_item("identifier", v).unwrap();
        }
        if let Some(ref v) = self.source {
            dict.set_item("source", v).unwrap();
        }
        if let Some(ref v) = self.language {
            dict.set_item("language", v).unwrap();
        }
        if let Some(ref v) = self.relation {
            dict.set_item("relation", v).unwrap();
        }
        if let Some(ref v) = self.coverage {
            dict.set_item("coverage", v).unwrap();
        }
        if let Some(ref v) = self.rights {
            dict.set_item("rights", v).unwrap();
        }

        dict.unbind()
    }
}
