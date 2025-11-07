use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a microformat item with properties and type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroformatItem {
    pub type_: Vec<String>,
    pub properties: HashMap<String, Vec<PropertyValue>>,
    pub children: Option<Vec<MicroformatItem>>,
}

impl MicroformatItem {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        dict.set_item("type", self.type_.clone()).unwrap();

        // Convert properties
        let props = PyDict::new_bound(py);
        for (key, values) in &self.properties {
            let py_values: Vec<PyObject> = values
                .iter()
                .map(|v| v.to_python(py))
                .collect();
            props.set_item(key, py_values).unwrap();
        }
        dict.set_item("properties", props).unwrap();

        // Convert children if present
        if let Some(children) = &self.children {
            let py_children: Vec<PyObject> = children
                .iter()
                .map(|child| child.to_py_dict(py).into())
                .collect();
            dict.set_item("children", py_children).unwrap();
        }

        dict.into()
    }
}

/// Represents different types of property values
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    Text(String),
    Url(String),
    Nested(Box<MicroformatItem>),
}

impl PropertyValue {
    pub fn to_python(&self, py: Python) -> PyObject {
        match self {
            PropertyValue::Text(s) | PropertyValue::Url(s) => s.to_object(py),
            PropertyValue::Nested(item) => item.to_py_dict(py).into(),
        }
    }
}

/// h-card microformat representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HCard {
    pub name: Option<String>,
    pub url: Option<String>,
    pub photo: Option<String>,
    pub email: Option<String>,
    pub tel: Option<String>,
    pub note: Option<String>,
    pub org: Option<String>,
    pub additional_properties: HashMap<String, Vec<String>>,
}

impl HCard {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        if let Some(name) = &self.name {
            dict.set_item("name", name).unwrap();
        }
        if let Some(url) = &self.url {
            dict.set_item("url", url).unwrap();
        }
        if let Some(photo) = &self.photo {
            dict.set_item("photo", photo).unwrap();
        }
        if let Some(email) = &self.email {
            dict.set_item("email", email).unwrap();
        }
        if let Some(tel) = &self.tel {
            dict.set_item("tel", tel).unwrap();
        }
        if let Some(note) = &self.note {
            dict.set_item("note", note).unwrap();
        }
        if let Some(org) = &self.org {
            dict.set_item("org", org).unwrap();
        }

        for (key, values) in &self.additional_properties {
            dict.set_item(key, values.clone()).unwrap();
        }

        dict.into()
    }
}

/// h-entry microformat representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HEntry {
    pub name: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub published: Option<String>,
    pub updated: Option<String>,
    pub author: Option<Box<HCard>>,
    pub url: Option<String>,
    pub category: Vec<String>,
    pub additional_properties: HashMap<String, Vec<String>>,
}

impl HEntry {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        if let Some(name) = &self.name {
            dict.set_item("name", name).unwrap();
        }
        if let Some(summary) = &self.summary {
            dict.set_item("summary", summary).unwrap();
        }
        if let Some(content) = &self.content {
            dict.set_item("content", content).unwrap();
        }
        if let Some(published) = &self.published {
            dict.set_item("published", published).unwrap();
        }
        if let Some(updated) = &self.updated {
            dict.set_item("updated", updated).unwrap();
        }
        if let Some(author) = &self.author {
            dict.set_item("author", author.to_py_dict(py)).unwrap();
        }
        if let Some(url) = &self.url {
            dict.set_item("url", url).unwrap();
        }
        if !self.category.is_empty() {
            dict.set_item("category", self.category.clone()).unwrap();
        }

        for (key, values) in &self.additional_properties {
            dict.set_item(key, values.clone()).unwrap();
        }

        dict.into()
    }
}

/// h-event microformat representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HEvent {
    pub name: Option<String>,
    pub summary: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub additional_properties: HashMap<String, Vec<String>>,
}

impl HEvent {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        if let Some(name) = &self.name {
            dict.set_item("name", name).unwrap();
        }
        if let Some(summary) = &self.summary {
            dict.set_item("summary", summary).unwrap();
        }
        if let Some(start) = &self.start {
            dict.set_item("start", start).unwrap();
        }
        if let Some(end) = &self.end {
            dict.set_item("end", end).unwrap();
        }
        if let Some(location) = &self.location {
            dict.set_item("location", location).unwrap();
        }
        if let Some(url) = &self.url {
            dict.set_item("url", url).unwrap();
        }
        if let Some(description) = &self.description {
            dict.set_item("description", description).unwrap();
        }

        for (key, values) in &self.additional_properties {
            dict.set_item(key, values.clone()).unwrap();
        }

        dict.into()
    }
}
