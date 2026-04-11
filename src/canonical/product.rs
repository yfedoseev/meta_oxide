//! Canonical product type — merges JSON-LD `Product`, microdata
//! `https://schema.org/Product`, and Open Graph `product:*` namespace into one
//! struct with field-level source attribution.

use crate::canonical::helpers::{
    find_jsonld_of_type, find_microdata_of_type, first_some, jsonld_string, microdata_nested,
    microdata_text, parse_number, value_to_string,
};
use crate::canonical::{Availability, CanonicalMerge, Money};
use crate::parser_facade::MetaGraph;
use crate::provenance::{FieldSource, FieldValue};
use crate::types::jsonld::JsonLdObject;
use crate::types::microdata::MicrodataItem;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A schema.org Product, merged from whichever formats supplied each field.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Product {
    /// Product name (e.g. `"Mechanical Keyboard"`).
    pub name: Option<FieldValue<String>>,
    /// Long-form description.
    pub description: Option<FieldValue<String>>,
    /// Primary image URL.
    pub image: Option<FieldValue<String>>,
    /// Brand or manufacturer name.
    pub brand: Option<FieldValue<String>>,
    /// Stock keeping unit.
    pub sku: Option<FieldValue<String>>,
    /// Global Trade Item Number (any GTIN-8/12/13/14 variant).
    pub gtin: Option<FieldValue<String>>,
    /// Canonical URL for the product page.
    pub url: Option<FieldValue<String>>,
    /// Price + currency.
    pub price: Option<FieldValue<Money>>,
    /// Stock status.
    pub availability: Option<FieldValue<Availability>>,
}

impl CanonicalMerge for Product {
    fn merge_from(graph: &MetaGraph) -> Option<Self> {
        let jsonld = find_jsonld_of_type(&graph.json_ld, &["Product"]);
        let microdata = find_microdata_of_type(&graph.microdata, &["Product"]);

        let mut product = Product::default();

        // name
        product.name = first_some([
            jsonld
                .and_then(|o| jsonld_string(o, "name"))
                .map(|v| FieldValue::new(v, FieldSource::JsonLd("Product.name".into()))),
            microdata
                .and_then(|m| microdata_text(m, "name"))
                .map(|v| FieldValue::new(v, FieldSource::Microdata("Product.name".into()))),
            graph
                .open_graph
                .title
                .clone()
                .map(|v| FieldValue::new(v, FieldSource::OpenGraph("og:title".into()))),
        ]);

        // description
        product.description = first_some([
            jsonld
                .and_then(|o| jsonld_string(o, "description"))
                .map(|v| FieldValue::new(v, FieldSource::JsonLd("Product.description".into()))),
            microdata
                .and_then(|m| microdata_text(m, "description"))
                .map(|v| FieldValue::new(v, FieldSource::Microdata("Product.description".into()))),
            graph
                .open_graph
                .description
                .clone()
                .map(|v| FieldValue::new(v, FieldSource::OpenGraph("og:description".into()))),
        ]);

        // image
        product.image = first_some([
            jsonld
                .and_then(jsonld_image)
                .map(|v| FieldValue::new(v, FieldSource::JsonLd("Product.image".into()))),
            microdata
                .and_then(|m| microdata_text(m, "image"))
                .map(|v| FieldValue::new(v, FieldSource::Microdata("Product.image".into()))),
            graph
                .open_graph
                .image
                .clone()
                .map(|v| FieldValue::new(v, FieldSource::OpenGraph("og:image".into()))),
        ]);

        // brand
        product.brand = first_some([
            jsonld
                .and_then(jsonld_brand)
                .map(|v| FieldValue::new(v, FieldSource::JsonLd("Product.brand".into()))),
            microdata
                .and_then(|m| microdata_text(m, "brand"))
                .map(|v| FieldValue::new(v, FieldSource::Microdata("Product.brand".into()))),
        ]);

        // sku
        product.sku = first_some([
            jsonld
                .and_then(|o| jsonld_string(o, "sku"))
                .map(|v| FieldValue::new(v, FieldSource::JsonLd("Product.sku".into()))),
            microdata
                .and_then(|m| microdata_text(m, "sku"))
                .map(|v| FieldValue::new(v, FieldSource::Microdata("Product.sku".into()))),
        ]);

        // gtin
        product.gtin = first_some([
            jsonld
                .and_then(|o| {
                    jsonld_string(o, "gtin")
                        .or_else(|| jsonld_string(o, "gtin13"))
                        .or_else(|| jsonld_string(o, "gtin12"))
                        .or_else(|| jsonld_string(o, "gtin8"))
                        .or_else(|| jsonld_string(o, "gtin14"))
                })
                .map(|v| FieldValue::new(v, FieldSource::JsonLd("Product.gtin".into()))),
            microdata
                .and_then(|m| microdata_text(m, "gtin"))
                .map(|v| FieldValue::new(v, FieldSource::Microdata("Product.gtin".into()))),
        ]);

        // url
        product.url = first_some([
            jsonld
                .and_then(|o| jsonld_string(o, "url"))
                .map(|v| FieldValue::new(v, FieldSource::JsonLd("Product.url".into()))),
            microdata
                .and_then(|m| microdata_text(m, "url"))
                .map(|v| FieldValue::new(v, FieldSource::Microdata("Product.url".into()))),
            graph
                .open_graph
                .url
                .clone()
                .map(|v| FieldValue::new(v, FieldSource::OpenGraph("og:url".into()))),
            graph
                .meta
                .canonical
                .clone()
                .map(|v| FieldValue::new(v, FieldSource::Meta("canonical".into()))),
        ]);

        // price
        product.price = first_some([
            jsonld
                .and_then(jsonld_price)
                .map(|m| FieldValue::new(m, FieldSource::JsonLd("Product.offers.price".into()))),
            microdata
                .and_then(microdata_price)
                .map(|m| FieldValue::new(m, FieldSource::Microdata("Product.offers.price".into()))),
        ]);

        // availability
        product.availability = first_some([
            jsonld.and_then(jsonld_availability).map(|a| {
                FieldValue::new(a, FieldSource::JsonLd("Product.offers.availability".into()))
            }),
            microdata.and_then(microdata_availability).map(|a| {
                FieldValue::new(a, FieldSource::Microdata("Product.offers.availability".into()))
            }),
        ]);

        // A Product with literally nothing in it is uninteresting; require at
        // least a name OR a price for the result to be useful.
        if product.name.is_none() && product.price.is_none() {
            None
        } else {
            Some(product)
        }
    }
}

// ---- JSON-LD helpers -------------------------------------------------------

fn jsonld_image(item: &JsonLdObject) -> Option<String> {
    item.properties.get("image").and_then(value_to_string)
}

fn jsonld_brand(item: &JsonLdObject) -> Option<String> {
    item.properties.get("brand").and_then(value_to_string)
}

fn jsonld_price(item: &JsonLdObject) -> Option<Money> {
    let offers = item.properties.get("offers")?;
    extract_offer_price(offers)
}

fn extract_offer_price(value: &Value) -> Option<Money> {
    match value {
        Value::Object(map) => {
            let price = map.get("price").and_then(parse_number)?;
            let currency = map.get("priceCurrency").and_then(value_to_string).unwrap_or_default();
            Some(Money::new(price, currency))
        }
        Value::Array(arr) => arr.iter().find_map(extract_offer_price),
        _ => None,
    }
}

fn jsonld_availability(item: &JsonLdObject) -> Option<Availability> {
    let offers = item.properties.get("offers")?;
    extract_offer_availability(offers)
}

fn extract_offer_availability(value: &Value) -> Option<Availability> {
    match value {
        Value::Object(map) => {
            map.get("availability").and_then(value_to_string).map(|s| Availability::parse(&s))
        }
        Value::Array(arr) => arr.iter().find_map(extract_offer_availability),
        _ => None,
    }
}

// ---- Microdata helpers -----------------------------------------------------

fn microdata_price(item: &MicrodataItem) -> Option<Money> {
    let offer = microdata_nested(item, "offers")?;
    let price_str = microdata_text(offer, "price")?;
    let amount = price_str.parse::<f64>().ok()?;
    let currency = microdata_text(offer, "priceCurrency").unwrap_or_default();
    Some(Money::new(amount, currency))
}

fn microdata_availability(item: &MicrodataItem) -> Option<Availability> {
    let offer = microdata_nested(item, "offers")?;
    microdata_text(offer, "availability").map(|s| Availability::parse(&s))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MetaParser;

    const PRODUCT_HTML_JSONLD: &str = r#"
<!DOCTYPE html>
<html>
<head>
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Product",
  "name": "Mechanical Keyboard",
  "description": "RGB mechanical keyboard.",
  "image": "https://example.com/kb.jpg",
  "brand": "Acme",
  "sku": "KB-001",
  "gtin": "0123456789012",
  "url": "https://example.com/kb",
  "offers": {
    "@type": "Offer",
    "price": "29.99",
    "priceCurrency": "USD",
    "availability": "https://schema.org/InStock"
  }
}
</script>
</head>
<body></body>
</html>
"#;

    const PRODUCT_HTML_MICRODATA: &str = r#"
<div itemscope itemtype="https://schema.org/Product">
  <span itemprop="name">Microdata Keyboard</span>
  <span itemprop="description">A different keyboard.</span>
  <span itemprop="brand">MicroBrand</span>
  <div itemprop="offers" itemscope itemtype="https://schema.org/Offer">
    <span itemprop="price">19.50</span>
    <meta itemprop="priceCurrency" content="EUR">
    <link itemprop="availability" href="https://schema.org/OutOfStock">
  </div>
</div>
"#;

    #[test]
    fn merges_jsonld_product() {
        let graph = MetaParser::new().parse(PRODUCT_HTML_JSONLD).unwrap();
        let product = graph.canonical_product().expect("should find product");

        assert_eq!(product.name.as_ref().unwrap().value, "Mechanical Keyboard");
        assert!(matches!(product.name.as_ref().unwrap().source, FieldSource::JsonLd(_)));
        assert_eq!(product.brand.as_ref().unwrap().value, "Acme");
        assert_eq!(product.sku.as_ref().unwrap().value, "KB-001");
        assert_eq!(product.gtin.as_ref().unwrap().value, "0123456789012");

        let price = product.price.as_ref().unwrap();
        assert_eq!(price.value.amount, 29.99);
        assert_eq!(price.value.currency, "USD");

        let availability = product.availability.as_ref().unwrap();
        assert_eq!(availability.value, Availability::InStock);
    }

    #[test]
    fn falls_back_to_microdata_when_no_jsonld() {
        let graph = MetaParser::new().parse(PRODUCT_HTML_MICRODATA).unwrap();
        let product = graph.canonical_product().expect("should find product");

        assert_eq!(product.name.as_ref().unwrap().value, "Microdata Keyboard");
        assert!(matches!(product.name.as_ref().unwrap().source, FieldSource::Microdata(_)));

        let price = product.price.as_ref().unwrap();
        assert_eq!(price.value.amount, 19.50);
        assert_eq!(price.value.currency, "EUR");
        assert_eq!(product.availability.as_ref().unwrap().value, Availability::OutOfStock);
    }

    #[test]
    fn returns_none_when_no_product_data() {
        let html = r#"<html><body><h1>Just a heading</h1></body></html>"#;
        let graph = MetaParser::new().parse(html).unwrap();
        assert!(graph.canonical_product().is_none());
    }
}
