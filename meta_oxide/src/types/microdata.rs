//! Types for HTML5 Microdata (Phase 4 - 26% adoption)
//!
//! Microdata is an HTML specification for embedding structured data using
//! itemscope, itemtype, and itemprop attributes with Schema.org vocabulary.

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::{PyDict, PyList};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A microdata item with properties
///
/// Corresponds to an element with `itemscope` attribute.
/// Can contain nested items and multiple property values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MicrodataItem {
    /// Schema.org type(s) from itemtype attribute
    /// Can be multiple types separated by spaces
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<Vec<String>>,

    /// Unique identifier from itemid attribute (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Properties extracted from itemprop attributes
    /// Key: property name, Value: array of property values
    #[serde(flatten)]
    pub properties: HashMap<String, Vec<PropertyValue>>,
}

/// Value of a microdata property
///
/// Can be text, URL, or a nested item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    /// Text content or simple value
    Text(String),

    /// Nested microdata item
    Item(Box<MicrodataItem>),
}

impl MicrodataItem {
    /// Create a new empty microdata item
    pub fn new() -> Self {
        Self { item_type: None, id: None, properties: HashMap::new() }
    }

    /// Set the item type(s)
    pub fn with_type(mut self, types: Vec<String>) -> Self {
        self.item_type = Some(types);
        self
    }

    /// Set the item ID
    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Add a text property
    pub fn add_text_property(&mut self, name: String, value: String) {
        self.properties.entry(name).or_default().push(PropertyValue::Text(value));
    }

    /// Add a nested item property
    pub fn add_item_property(&mut self, name: String, item: MicrodataItem) {
        self.properties.entry(name).or_default().push(PropertyValue::Item(Box::new(item)));
    }
}

#[cfg(feature = "python")]
impl MicrodataItem {
    /// Convert to Python dictionary
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        // Add type(s) - always as a list for consistency
        if let Some(ref types) = self.item_type {
            dict.set_item("type", types.clone()).unwrap();
        }

        // Add id
        if let Some(ref id) = self.id {
            dict.set_item("id", id).unwrap();
        }

        // Add properties
        for (key, values) in &self.properties {
            if values.len() == 1 {
                // Single value - add directly
                match &values[0] {
                    PropertyValue::Text(s) => {
                        dict.set_item(key, s).unwrap();
                    }
                    PropertyValue::Item(item) => {
                        dict.set_item(key, item.to_py_dict(py)).unwrap();
                    }
                }
            } else {
                // Multiple values - add as list
                let list = PyList::empty_bound(py);
                for value in values {
                    match value {
                        PropertyValue::Text(s) => {
                            list.append(s).unwrap();
                        }
                        PropertyValue::Item(item) => {
                            list.append(item.to_py_dict(py)).unwrap();
                        }
                    }
                }
                dict.set_item(key, list).unwrap();
            }
        }

        dict.unbind()
    }
}

impl Default for MicrodataItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_microdata_item_new() {
        let item = MicrodataItem::new();
        assert!(item.item_type.is_none());
        assert!(item.id.is_none());
        assert!(item.properties.is_empty());
    }

    #[test]
    fn test_microdata_item_with_type() {
        let item = MicrodataItem::new().with_type(vec!["https://schema.org/Person".to_string()]);

        assert_eq!(item.item_type, Some(vec!["https://schema.org/Person".to_string()]));
    }

    #[test]
    fn test_microdata_item_with_multiple_types() {
        let item = MicrodataItem::new().with_type(vec![
            "https://schema.org/Person".to_string(),
            "https://schema.org/Employee".to_string(),
        ]);

        assert_eq!(item.item_type.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_microdata_item_with_id() {
        let item = MicrodataItem::new().with_id("person-123".to_string());

        assert_eq!(item.id, Some("person-123".to_string()));
    }

    #[test]
    fn test_add_text_property() {
        let mut item = MicrodataItem::new();
        item.add_text_property("name".to_string(), "Jane Doe".to_string());

        assert_eq!(item.properties.get("name").unwrap().len(), 1);
        match &item.properties.get("name").unwrap()[0] {
            PropertyValue::Text(s) => assert_eq!(s, "Jane Doe"),
            _ => panic!("Expected text value"),
        }
    }

    #[test]
    fn test_add_multiple_text_properties() {
        let mut item = MicrodataItem::new();
        item.add_text_property("telephone".to_string(), "555-1234".to_string());
        item.add_text_property("telephone".to_string(), "555-5678".to_string());

        assert_eq!(item.properties.get("telephone").unwrap().len(), 2);
    }

    #[test]
    fn test_add_nested_item() {
        let mut item = MicrodataItem::new();
        let mut address =
            MicrodataItem::new().with_type(vec!["https://schema.org/PostalAddress".to_string()]);
        address.add_text_property("streetAddress".to_string(), "123 Main St".to_string());

        item.add_item_property("address".to_string(), address);

        assert_eq!(item.properties.get("address").unwrap().len(), 1);
        match &item.properties.get("address").unwrap()[0] {
            PropertyValue::Item(nested) => {
                assert_eq!(
                    nested.item_type.as_ref().unwrap()[0],
                    "https://schema.org/PostalAddress"
                );
            }
            _ => panic!("Expected nested item"),
        }
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_to_py_dict_basic() {
        Python::with_gil(|py| {
            let mut item =
                MicrodataItem::new().with_type(vec!["https://schema.org/Person".to_string()]);
            item.add_text_property("name".to_string(), "Jane Doe".to_string());

            let py_dict = item.to_py_dict(py);
            let dict = py_dict.bind(py);

            assert!(dict.contains("type").unwrap());
            assert!(dict.contains("name").unwrap());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_to_py_dict_with_nested_item() {
        Python::with_gil(|py| {
            let mut item =
                MicrodataItem::new().with_type(vec!["https://schema.org/Person".to_string()]);
            item.add_text_property("name".to_string(), "Jane Doe".to_string());

            let mut address = MicrodataItem::new()
                .with_type(vec!["https://schema.org/PostalAddress".to_string()]);
            address.add_text_property("streetAddress".to_string(), "123 Main St".to_string());

            item.add_item_property("address".to_string(), address);

            let py_dict = item.to_py_dict(py);
            let dict = py_dict.bind(py);

            assert!(dict.contains("name").unwrap());
            assert!(dict.contains("address").unwrap());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_to_py_dict_multiple_values() {
        Python::with_gil(|py| {
            let mut item = MicrodataItem::new();
            item.add_text_property("telephone".to_string(), "555-1234".to_string());
            item.add_text_property("telephone".to_string(), "555-5678".to_string());

            let py_dict = item.to_py_dict(py);
            let dict = py_dict.bind(py);

            assert!(dict.contains("telephone").unwrap());
            // Should be a list since there are multiple values
            let tel = dict.get_item("telephone").unwrap().unwrap();
            assert!(tel.is_instance_of::<PyList>());
        });
    }

    #[test]
    fn test_property_value_text() {
        let value = PropertyValue::Text("test".to_string());
        match value {
            PropertyValue::Text(s) => assert_eq!(s, "test"),
            _ => panic!("Expected text value"),
        }
    }

    #[test]
    fn test_property_value_item() {
        let item = MicrodataItem::new();
        let value = PropertyValue::Item(Box::new(item));
        match value {
            PropertyValue::Item(_) => {}
            _ => panic!("Expected item value"),
        }
    }

    #[test]
    fn test_serde_serialize_deserialize() {
        let mut item = MicrodataItem::new()
            .with_type(vec!["https://schema.org/Person".to_string()])
            .with_id("person-123".to_string());
        item.add_text_property("name".to_string(), "Jane Doe".to_string());

        let json = serde_json::to_string(&item).unwrap();
        let deserialized: MicrodataItem = serde_json::from_str(&json).unwrap();

        assert_eq!(item, deserialized);
    }
}
