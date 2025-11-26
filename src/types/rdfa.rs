//! Types for RDFa (Resource Description Framework in Attributes)
//!
//! RDFa is a W3C standard for embedding structured data in HTML using attributes.
//! It provides semantic markup for web content with 62% desktop adoption.

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::{PyDict, PyList};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// RDFa item representing a resource with properties
///
/// Corresponds to an element with `typeof` or `vocab` attribute.
/// Can contain nested items and multiple property values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RdfaItem {
    /// The resource type(s) from typeof attribute
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_of: Option<Vec<String>>,

    /// The subject URI from about attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,

    /// The vocabulary namespace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocab: Option<String>,

    /// Properties extracted from property attributes
    /// Key: property name, Value: array of property values
    #[serde(flatten)]
    pub properties: HashMap<String, Vec<RdfaValue>>,
}

/// Value of an RDFa property
///
/// Can be a literal text, URI reference, nested item, or typed literal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RdfaValue {
    /// Nested RDFa item (must come first for untagged serde)
    Item(Box<RdfaItem>),

    /// Typed literal with datatype
    TypedLiteral { value: String, datatype: String },

    /// URI reference
    Resource(String),

    /// Literal text value
    Literal(String),
}

impl RdfaItem {
    /// Create a new empty RDFa item
    pub fn new() -> Self {
        Self { type_of: None, about: None, vocab: None, properties: HashMap::new() }
    }

    /// Set the item type(s)
    pub fn with_type(mut self, types: Vec<String>) -> Self {
        self.type_of = Some(types);
        self
    }

    /// Set the vocab namespace
    pub fn with_vocab(mut self, vocab: String) -> Self {
        self.vocab = Some(vocab);
        self
    }

    /// Set the about URI
    pub fn with_about(mut self, about: String) -> Self {
        self.about = Some(about);
        self
    }

    /// Add a property value
    pub fn add_property(&mut self, name: String, value: RdfaValue) {
        self.properties.entry(name).or_default().push(value);
    }
}

#[cfg(feature = "python")]
impl RdfaItem {
    /// Convert to Python dictionary
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        // Add type(s) - always as a list for consistency
        if let Some(ref types) = self.type_of {
            dict.set_item("type", types.clone()).unwrap();
        }

        // Add vocab
        if let Some(ref vocab) = self.vocab {
            dict.set_item("vocab", vocab).unwrap();
        }

        // Add about
        if let Some(ref about) = self.about {
            dict.set_item("about", about).unwrap();
        }

        // Add properties
        for (key, values) in &self.properties {
            if values.len() == 1 {
                // Single value - add directly
                dict.set_item(key, values[0].to_py_value(py)).unwrap();
            } else {
                // Multiple values - add as list
                let list = PyList::empty_bound(py);
                for value in values {
                    list.append(value.to_py_value(py)).unwrap();
                }
                dict.set_item(key, list).unwrap();
            }
        }

        dict.unbind()
    }
}

#[cfg(feature = "python")]
impl RdfaValue {
    /// Convert to Python value
    pub fn to_py_value(&self, py: Python) -> PyObject {
        match self {
            RdfaValue::Literal(s) => s.to_object(py),
            RdfaValue::Resource(uri) => uri.to_object(py),
            RdfaValue::Item(item) => item.to_py_dict(py).to_object(py),
            RdfaValue::TypedLiteral { value, datatype } => {
                let dict = PyDict::new_bound(py);
                dict.set_item("value", value).unwrap();
                dict.set_item("datatype", datatype).unwrap();
                dict.to_object(py)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rdfa_item_new() {
        let item = RdfaItem::new();
        assert!(item.type_of.is_none());
        assert!(item.about.is_none());
        assert!(item.vocab.is_none());
        assert!(item.properties.is_empty());
    }

    #[test]
    fn test_rdfa_item_default() {
        let item = RdfaItem::default();
        assert!(item.type_of.is_none());
        assert!(item.properties.is_empty());
    }

    #[test]
    fn test_rdfa_item_with_type() {
        let item = RdfaItem::new().with_type(vec!["https://schema.org/Person".to_string()]);
        assert_eq!(item.type_of, Some(vec!["https://schema.org/Person".to_string()]));
    }

    #[test]
    fn test_rdfa_item_with_vocab() {
        let item = RdfaItem::new().with_vocab("https://schema.org/".to_string());
        assert_eq!(item.vocab, Some("https://schema.org/".to_string()));
    }

    #[test]
    fn test_rdfa_item_with_about() {
        let item = RdfaItem::new().with_about("https://example.com/person/123".to_string());
        assert_eq!(item.about, Some("https://example.com/person/123".to_string()));
    }

    #[test]
    fn test_rdfa_item_add_property() {
        let mut item = RdfaItem::new();
        item.add_property("name".to_string(), RdfaValue::Literal("Jane Doe".to_string()));

        assert_eq!(item.properties.get("name").unwrap().len(), 1);
        match &item.properties.get("name").unwrap()[0] {
            RdfaValue::Literal(s) => assert_eq!(s, "Jane Doe"),
            _ => panic!("Expected literal value"),
        }
    }

    #[test]
    fn test_rdfa_item_multiple_properties() {
        let mut item = RdfaItem::new();
        item.add_property("telephone".to_string(), RdfaValue::Literal("555-1234".to_string()));
        item.add_property("telephone".to_string(), RdfaValue::Literal("555-5678".to_string()));

        assert_eq!(item.properties.get("telephone").unwrap().len(), 2);
    }

    #[test]
    fn test_rdfa_value_literal() {
        let value = RdfaValue::Literal("test".to_string());
        match value {
            RdfaValue::Literal(s) => assert_eq!(s, "test"),
            _ => panic!("Expected literal value"),
        }
    }

    #[test]
    fn test_rdfa_value_resource() {
        let value = RdfaValue::Resource("https://example.com".to_string());
        match value {
            RdfaValue::Resource(uri) => assert_eq!(uri, "https://example.com"),
            _ => panic!("Expected resource value"),
        }
    }

    #[test]
    fn test_rdfa_value_item() {
        let nested = RdfaItem::new();
        let value = RdfaValue::Item(Box::new(nested));
        match value {
            RdfaValue::Item(_) => {}
            _ => panic!("Expected item value"),
        }
    }

    #[test]
    fn test_rdfa_value_typed_literal() {
        let value = RdfaValue::TypedLiteral {
            value: "42".to_string(),
            datatype: "http://www.w3.org/2001/XMLSchema#integer".to_string(),
        };
        match value {
            RdfaValue::TypedLiteral { value, datatype } => {
                assert_eq!(value, "42");
                assert_eq!(datatype, "http://www.w3.org/2001/XMLSchema#integer");
            }
            _ => panic!("Expected typed literal"),
        }
    }

    #[test]
    fn test_rdfa_item_clone() {
        let mut item = RdfaItem::new().with_type(vec!["Person".to_string()]);
        item.add_property("name".to_string(), RdfaValue::Literal("Jane".to_string()));

        let cloned = item.clone();
        assert_eq!(item, cloned);
    }

    #[test]
    fn test_rdfa_item_partial_eq() {
        let mut item1 = RdfaItem::new().with_type(vec!["Person".to_string()]);
        item1.add_property("name".to_string(), RdfaValue::Literal("Jane".to_string()));

        let mut item2 = RdfaItem::new().with_type(vec!["Person".to_string()]);
        item2.add_property("name".to_string(), RdfaValue::Literal("Jane".to_string()));

        assert_eq!(item1, item2);
    }

    #[test]
    fn test_serde_serialize_deserialize() {
        let mut item = RdfaItem::new()
            .with_type(vec!["https://schema.org/Person".to_string()])
            .with_vocab("https://schema.org/".to_string())
            .with_about("https://example.com/jane".to_string());
        item.add_property("name".to_string(), RdfaValue::Literal("Jane Doe".to_string()));

        let json = serde_json::to_string(&item).unwrap();
        let deserialized: RdfaItem = serde_json::from_str(&json).unwrap();

        assert_eq!(item, deserialized);
    }

    #[test]
    fn test_serde_skip_none_fields() {
        let item = RdfaItem::new();
        let json = serde_json::to_value(&item).unwrap();

        // None fields should be skipped
        assert!(!json.as_object().unwrap().contains_key("type"));
        assert!(!json.as_object().unwrap().contains_key("vocab"));
        assert!(!json.as_object().unwrap().contains_key("about"));
    }

    #[test]
    fn test_serde_flatten_properties() {
        let mut item = RdfaItem::new();
        item.add_property("name".to_string(), RdfaValue::Literal("Jane".to_string()));

        let json = serde_json::to_value(&item).unwrap();
        let obj = json.as_object().unwrap();

        // Properties should be flattened into the root object
        assert!(obj.contains_key("name"));
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_to_py_dict_basic() {
        Python::with_gil(|py| {
            let mut item = RdfaItem::new().with_type(vec!["https://schema.org/Person".to_string()]);
            item.add_property("name".to_string(), RdfaValue::Literal("Jane Doe".to_string()));

            let py_dict = item.to_py_dict(py);
            let dict = py_dict.bind(py);

            assert!(dict.contains("type").unwrap());
            assert!(dict.contains("name").unwrap());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_to_py_dict_with_vocab() {
        Python::with_gil(|py| {
            let item = RdfaItem::new().with_vocab("https://schema.org/".to_string());

            let py_dict = item.to_py_dict(py);
            let dict = py_dict.bind(py);

            assert!(dict.contains("vocab").unwrap());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_to_py_dict_with_about() {
        Python::with_gil(|py| {
            let item = RdfaItem::new().with_about("https://example.com/jane".to_string());

            let py_dict = item.to_py_dict(py);
            let dict = py_dict.bind(py);

            assert!(dict.contains("about").unwrap());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_to_py_dict_multiple_values() {
        Python::with_gil(|py| {
            let mut item = RdfaItem::new();
            item.add_property("telephone".to_string(), RdfaValue::Literal("555-1234".to_string()));
            item.add_property("telephone".to_string(), RdfaValue::Literal("555-5678".to_string()));

            let py_dict = item.to_py_dict(py);
            let dict = py_dict.bind(py);

            assert!(dict.contains("telephone").unwrap());
            // Should be a list since there are multiple values
            let tel = dict.get_item("telephone").unwrap().unwrap();
            assert!(tel.is_instance_of::<PyList>());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_to_py_dict_nested_item() {
        Python::with_gil(|py| {
            let mut item = RdfaItem::new();
            let mut address =
                RdfaItem::new().with_type(vec!["https://schema.org/PostalAddress".to_string()]);
            address.add_property(
                "streetAddress".to_string(),
                RdfaValue::Literal("123 Main".to_string()),
            );

            item.add_property("address".to_string(), RdfaValue::Item(Box::new(address)));

            let py_dict = item.to_py_dict(py);
            let dict = py_dict.bind(py);

            assert!(dict.contains("address").unwrap());
            let address_val = dict.get_item("address").unwrap().unwrap();
            assert!(address_val.is_instance_of::<PyDict>());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_rdfa_value_to_py_literal() {
        Python::with_gil(|py| {
            let value = RdfaValue::Literal("test".to_string());
            let py_value = value.to_py_value(py);
            let py_str: String = py_value.extract(py).unwrap();
            assert_eq!(py_str, "test");
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_rdfa_value_to_py_resource() {
        Python::with_gil(|py| {
            let value = RdfaValue::Resource("https://example.com".to_string());
            let py_value = value.to_py_value(py);
            let py_str: String = py_value.extract(py).unwrap();
            assert_eq!(py_str, "https://example.com");
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_rdfa_value_to_py_typed_literal() {
        Python::with_gil(|py| {
            let value = RdfaValue::TypedLiteral {
                value: "42".to_string(),
                datatype: "xsd:integer".to_string(),
            };
            let py_value = value.to_py_value(py);
            let py_dict: Bound<PyDict> = py_value.extract(py).unwrap();
            assert!(py_dict.contains("value").unwrap());
            assert!(py_dict.contains("datatype").unwrap());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_rdfa_value_to_py_nested_item() {
        Python::with_gil(|py| {
            let item = RdfaItem::new().with_type(vec!["Person".to_string()]);
            let value = RdfaValue::Item(Box::new(item));
            let py_value = value.to_py_value(py);
            let py_dict: Bound<PyDict> = py_value.extract(py).unwrap();
            assert!(py_dict.contains("type").unwrap());
        });
    }
}
