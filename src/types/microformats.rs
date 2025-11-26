#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
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

#[cfg(feature = "python")]
impl MicroformatItem {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        dict.set_item("type", self.type_.clone()).unwrap();

        // Convert properties
        let props = PyDict::new_bound(py);
        for (key, values) in &self.properties {
            let py_values: Vec<PyObject> = values.iter().map(|v| v.to_python(py)).collect();
            props.set_item(key, py_values).unwrap();
        }
        dict.set_item("properties", props).unwrap();

        // Convert children if present
        if let Some(children) = &self.children {
            let py_children: Vec<PyObject> =
                children.iter().map(|child| child.to_py_dict(py).into()).collect();
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

#[cfg(feature = "python")]
impl PropertyValue {
    pub fn to_python(&self, py: Python) -> PyObject {
        match self {
            PropertyValue::Text(s) | PropertyValue::Url(s) => s.to_object(py),
            PropertyValue::Nested(item) => item.to_py_dict(py).into(),
        }
    }
}

/// h-card microformat representation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[cfg(feature = "python")]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[cfg(feature = "python")]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[cfg(feature = "python")]
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

/// h-review microformat representation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HReview {
    // Modern microformats2 properties
    pub name: Option<String>,
    pub content: Option<String>,
    pub published: Option<String>,

    // Legacy properties (backward compatibility)
    pub summary: Option<String>,
    pub dtreviewed: Option<String>,
    pub description: Option<String>,

    // Rating properties
    pub rating: Option<f32>,
    pub best: Option<f32>,
    pub worst: Option<f32>,

    // Item being reviewed (simple text or nested h-product/h-event)
    pub item: Option<String>,
    pub item_product: Option<Box<HProduct>>,

    // Reviewer (simple text or nested h-card)
    pub reviewer: Option<String>,
    pub reviewer_card: Option<Box<HCard>>,

    pub url: Option<String>,
    pub additional_properties: HashMap<String, Vec<String>>,
}

#[cfg(feature = "python")]
impl HReview {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        // Modern properties
        if let Some(name) = &self.name {
            dict.set_item("name", name).unwrap();
        }
        if let Some(content) = &self.content {
            dict.set_item("content", content).unwrap();
        }
        if let Some(published) = &self.published {
            dict.set_item("published", published).unwrap();
        }

        // Legacy properties (backward compatibility)
        if let Some(summary) = &self.summary {
            dict.set_item("summary", summary).unwrap();
        }
        if let Some(dtreviewed) = &self.dtreviewed {
            dict.set_item("dtreviewed", dtreviewed).unwrap();
        }
        if let Some(description) = &self.description {
            dict.set_item("description", description).unwrap();
        }

        // Rating properties
        if let Some(rating) = self.rating {
            dict.set_item("rating", rating).unwrap();
        }
        if let Some(best) = self.best {
            dict.set_item("best", best).unwrap();
        }
        if let Some(worst) = self.worst {
            dict.set_item("worst", worst).unwrap();
        }

        // Item properties
        if let Some(item) = &self.item {
            dict.set_item("item", item).unwrap();
        }
        if let Some(item_product) = &self.item_product {
            dict.set_item("item_product", item_product.to_py_dict(py)).unwrap();
        }

        // Reviewer properties
        if let Some(reviewer) = &self.reviewer {
            dict.set_item("reviewer", reviewer).unwrap();
        }
        if let Some(reviewer_card) = &self.reviewer_card {
            dict.set_item("reviewer_card", reviewer_card.to_py_dict(py)).unwrap();
        }

        if let Some(url) = &self.url {
            dict.set_item("url", url).unwrap();
        }

        for (key, values) in &self.additional_properties {
            dict.set_item(key, values.clone()).unwrap();
        }

        dict.into()
    }
}

/// h-recipe microformat representation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HRecipe {
    pub name: Option<String>,
    pub summary: Option<String>,
    pub ingredient: Vec<String>,
    pub instructions: Option<String>,
    pub duration: Option<String>,
    pub yield_: Option<String>,
    pub nutrition: Option<String>,
    pub photo: Option<String>,
    pub author: Option<String>,
    pub published: Option<String>,
    pub category: Vec<String>,
    pub additional_properties: HashMap<String, Vec<String>>,
}

#[cfg(feature = "python")]
impl HRecipe {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        if let Some(name) = &self.name {
            dict.set_item("name", name).unwrap();
        }
        if let Some(summary) = &self.summary {
            dict.set_item("summary", summary).unwrap();
        }
        if !self.ingredient.is_empty() {
            dict.set_item("ingredient", self.ingredient.clone()).unwrap();
        }
        if let Some(instructions) = &self.instructions {
            dict.set_item("instructions", instructions).unwrap();
        }
        if let Some(duration) = &self.duration {
            dict.set_item("duration", duration).unwrap();
        }
        if let Some(yield_) = &self.yield_ {
            dict.set_item("yield", yield_).unwrap();
        }
        if let Some(nutrition) = &self.nutrition {
            dict.set_item("nutrition", nutrition).unwrap();
        }
        if let Some(photo) = &self.photo {
            dict.set_item("photo", photo).unwrap();
        }
        if let Some(author) = &self.author {
            dict.set_item("author", author).unwrap();
        }
        if let Some(published) = &self.published {
            dict.set_item("published", published).unwrap();
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

/// h-product microformat representation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HProduct {
    pub name: Option<String>,
    pub description: Option<String>,
    pub photo: Option<String>,
    pub price: Option<String>,
    pub brand: Option<String>,
    pub category: Vec<String>,
    pub rating: Option<f32>,
    pub url: Option<String>,
    pub identifier: Option<String>,
    pub additional_properties: HashMap<String, Vec<String>>,
}

#[cfg(feature = "python")]
impl HProduct {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        if let Some(name) = &self.name {
            dict.set_item("name", name).unwrap();
        }
        if let Some(description) = &self.description {
            dict.set_item("description", description).unwrap();
        }
        if let Some(photo) = &self.photo {
            dict.set_item("photo", photo).unwrap();
        }
        if let Some(price) = &self.price {
            dict.set_item("price", price).unwrap();
        }
        if let Some(brand) = &self.brand {
            dict.set_item("brand", brand).unwrap();
        }
        if !self.category.is_empty() {
            dict.set_item("category", self.category.clone()).unwrap();
        }
        if let Some(rating) = self.rating {
            dict.set_item("rating", rating).unwrap();
        }
        if let Some(url) = &self.url {
            dict.set_item("url", url).unwrap();
        }
        if let Some(identifier) = &self.identifier {
            dict.set_item("identifier", identifier).unwrap();
        }

        for (key, values) in &self.additional_properties {
            dict.set_item(key, values.clone()).unwrap();
        }

        dict.into()
    }
}

/// h-feed microformat representation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HFeed {
    pub name: Option<String>,
    pub author: Option<String>,
    pub url: Option<String>,
    pub photo: Option<String>,
    pub additional_properties: HashMap<String, Vec<String>>,
}

#[cfg(feature = "python")]
impl HFeed {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        if let Some(name) = &self.name {
            dict.set_item("name", name).unwrap();
        }
        if let Some(author) = &self.author {
            dict.set_item("author", author).unwrap();
        }
        if let Some(url) = &self.url {
            dict.set_item("url", url).unwrap();
        }
        if let Some(photo) = &self.photo {
            dict.set_item("photo", photo).unwrap();
        }

        for (key, values) in &self.additional_properties {
            dict.set_item(key, values.clone()).unwrap();
        }

        dict.into()
    }
}

/// h-adr microformat representation (physical address)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HAdr {
    pub street_address: Option<String>,
    pub extended_address: Option<String>,
    pub post_office_box: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country_name: Option<String>,
    pub additional_properties: HashMap<String, Vec<String>>,
}

#[cfg(feature = "python")]
impl HAdr {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        if let Some(street_address) = &self.street_address {
            dict.set_item("street_address", street_address).unwrap();
        }
        if let Some(extended_address) = &self.extended_address {
            dict.set_item("extended_address", extended_address).unwrap();
        }
        if let Some(post_office_box) = &self.post_office_box {
            dict.set_item("post_office_box", post_office_box).unwrap();
        }
        if let Some(locality) = &self.locality {
            dict.set_item("locality", locality).unwrap();
        }
        if let Some(region) = &self.region {
            dict.set_item("region", region).unwrap();
        }
        if let Some(postal_code) = &self.postal_code {
            dict.set_item("postal_code", postal_code).unwrap();
        }
        if let Some(country_name) = &self.country_name {
            dict.set_item("country_name", country_name).unwrap();
        }

        for (key, values) in &self.additional_properties {
            dict.set_item(key, values.clone()).unwrap();
        }

        dict.into()
    }
}

/// h-geo microformat representation (geographic coordinates)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HGeo {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub additional_properties: HashMap<String, Vec<String>>,
}

#[cfg(feature = "python")]
impl HGeo {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        if let Some(latitude) = self.latitude {
            dict.set_item("latitude", latitude).unwrap();
        }
        if let Some(longitude) = self.longitude {
            dict.set_item("longitude", longitude).unwrap();
        }
        if let Some(altitude) = self.altitude {
            dict.set_item("altitude", altitude).unwrap();
        }

        for (key, values) in &self.additional_properties {
            dict.set_item(key, values.clone()).unwrap();
        }

        dict.into()
    }
}
