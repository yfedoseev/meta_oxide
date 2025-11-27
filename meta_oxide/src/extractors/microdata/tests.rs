//! Tests for HTML5 Microdata extraction (Phase 4)
//!
//! Tests follow TDD approach - written before implementation

use super::*;
use crate::types::microdata::PropertyValue;

#[test]
fn test_extract_empty_html() {
    let html = "<html><body></body></html>";
    let items = extract(html, None).unwrap();
    assert_eq!(items.len(), 0);
}

#[test]
fn test_extract_basic_person() {
    let html = r#"
    <div itemscope itemtype="https://schema.org/Person">
        <span itemprop="name">Jane Doe</span>
        <span itemprop="jobTitle">Software Engineer</span>
    </div>
    "#;

    let items = extract(html, None).unwrap();
    assert_eq!(items.len(), 1);

    let item = &items[0];
    assert_eq!(item.item_type, Some(vec!["https://schema.org/Person".to_string()]));

    let name = &item.properties.get("name").unwrap()[0];
    match name {
        PropertyValue::Text(s) => assert_eq!(s, "Jane Doe"),
        _ => panic!("Expected text value"),
    }

    let job = &item.properties.get("jobTitle").unwrap()[0];
    match job {
        PropertyValue::Text(s) => assert_eq!(s, "Software Engineer"),
        _ => panic!("Expected text value"),
    }
}

#[test]
fn test_extract_with_url_properties() {
    let html = r#"
    <div itemscope itemtype="https://schema.org/Person">
        <a itemprop="url" href="https://example.com">Website</a>
        <img itemprop="image" src="https://example.com/photo.jpg" alt="Photo">
    </div>
    "#;

    let items = extract(html, None).unwrap();
    assert_eq!(items.len(), 1);

    let url = &items[0].properties.get("url").unwrap()[0];
    match url {
        PropertyValue::Text(s) => assert_eq!(s, "https://example.com/"),
        _ => panic!("Expected URL"),
    }

    let image = &items[0].properties.get("image").unwrap()[0];
    match image {
        PropertyValue::Text(s) => assert_eq!(s, "https://example.com/photo.jpg"),
        _ => panic!("Expected image URL"),
    }
}

#[test]
fn test_extract_nested_item() {
    let html = r#"
    <div itemscope itemtype="https://schema.org/Person">
        <span itemprop="name">Jane Doe</span>
        <div itemprop="address" itemscope itemtype="https://schema.org/PostalAddress">
            <span itemprop="streetAddress">123 Main St</span>
            <span itemprop="addressLocality">San Francisco</span>
            <span itemprop="addressRegion">CA</span>
        </div>
    </div>
    "#;

    let items = extract(html, None).unwrap();
    assert_eq!(items.len(), 1);

    let address_prop = &items[0].properties.get("address").unwrap()[0];
    match address_prop {
        PropertyValue::Item(address) => {
            assert_eq!(
                address.item_type,
                Some(vec!["https://schema.org/PostalAddress".to_string()])
            );

            let street = &address.properties.get("streetAddress").unwrap()[0];
            match street {
                PropertyValue::Text(s) => assert_eq!(s, "123 Main St"),
                _ => panic!("Expected text"),
            }

            let city = &address.properties.get("addressLocality").unwrap()[0];
            match city {
                PropertyValue::Text(s) => assert_eq!(s, "San Francisco"),
                _ => panic!("Expected text"),
            }
        }
        _ => panic!("Expected nested item"),
    }
}

#[test]
fn test_extract_multiple_items() {
    let html = r#"
    <div>
        <div itemscope itemtype="https://schema.org/Person">
            <span itemprop="name">Jane Doe</span>
        </div>
        <div itemscope itemtype="https://schema.org/Person">
            <span itemprop="name">John Smith</span>
        </div>
    </div>
    "#;

    let items = extract(html, None).unwrap();
    assert_eq!(items.len(), 2);
}

#[test]
fn test_extract_multiple_values_same_property() {
    let html = r#"
    <div itemscope itemtype="https://schema.org/Person">
        <span itemprop="name">Jane Doe</span>
        <span itemprop="telephone">555-1234</span>
        <span itemprop="telephone">555-5678</span>
    </div>
    "#;

    let items = extract(html, None).unwrap();
    assert_eq!(items.len(), 1);

    let telephones = items[0].properties.get("telephone").unwrap();
    assert_eq!(telephones.len(), 2);
}

#[test]
fn test_extract_with_itemid() {
    let html = r#"
    <div itemscope itemtype="https://schema.org/Person" itemid="person-123">
        <span itemprop="name">Jane Doe</span>
    </div>
    "#;

    let items = extract(html, None).unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, Some("person-123".to_string()));
}

#[test]
fn test_extract_multiple_types() {
    let html = r#"
    <div itemscope itemtype="https://schema.org/Person https://schema.org/Employee">
        <span itemprop="name">Jane Doe</span>
    </div>
    "#;

    let items = extract(html, None).unwrap();
    assert_eq!(items.len(), 1);

    let types = items[0].item_type.as_ref().unwrap();
    assert_eq!(types.len(), 2);
    assert!(types.contains(&"https://schema.org/Person".to_string()));
    assert!(types.contains(&"https://schema.org/Employee".to_string()));
}

#[test]
fn test_extract_article() {
    let html = r#"
    <article itemscope itemtype="https://schema.org/Article">
        <h1 itemprop="headline">Amazing Article</h1>
        <p itemprop="description">This is a great article</p>
        <span itemprop="author" itemscope itemtype="https://schema.org/Person">
            <span itemprop="name">Jane Doe</span>
        </span>
        <time itemprop="datePublished" datetime="2024-01-15">January 15, 2024</time>
    </article>
    "#;

    let items = extract(html, None).unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].item_type, Some(vec!["https://schema.org/Article".to_string()]));
}

#[test]
fn test_extract_product() {
    let html = r#"
    <div itemscope itemtype="https://schema.org/Product">
        <span itemprop="name">Wireless Headphones</span>
        <span itemprop="description">Noise-cancelling headphones</span>
        <span itemprop="brand">TechBrand</span>
        <div itemprop="offers" itemscope itemtype="https://schema.org/Offer">
            <span itemprop="price">299.99</span>
            <span itemprop="priceCurrency">USD</span>
        </div>
    </div>
    "#;

    let items = extract(html, None).unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].item_type, Some(vec!["https://schema.org/Product".to_string()]));

    let offers = &items[0].properties.get("offers").unwrap()[0];
    match offers {
        PropertyValue::Item(offer) => {
            assert_eq!(offer.item_type, Some(vec!["https://schema.org/Offer".to_string()]));
        }
        _ => panic!("Expected nested offer"),
    }
}

#[test]
fn test_extract_no_itemtype() {
    let html = r#"
    <div itemscope>
        <span itemprop="name">Jane Doe</span>
    </div>
    "#;

    let items = extract(html, None).unwrap();
    assert_eq!(items.len(), 1);
    assert!(items[0].item_type.is_none());
}

#[test]
fn test_extract_with_meta_tag() {
    let html = r#"
    <div itemscope itemtype="https://schema.org/Article">
        <h1 itemprop="headline">Article Title</h1>
        <meta itemprop="datePublished" content="2024-01-15">
    </div>
    "#;

    let items = extract(html, None).unwrap();
    assert_eq!(items.len(), 1);

    let date = &items[0].properties.get("datePublished").unwrap()[0];
    match date {
        PropertyValue::Text(s) => assert_eq!(s, "2024-01-15"),
        _ => panic!("Expected date from meta content"),
    }
}

#[test]
fn test_extract_with_link_tag() {
    let html = r#"
    <div itemscope itemtype="https://schema.org/Article">
        <link itemprop="image" href="https://example.com/image.jpg">
    </div>
    "#;

    let items = extract(html, None).unwrap();
    assert_eq!(items.len(), 1);

    let image = &items[0].properties.get("image").unwrap()[0];
    match image {
        PropertyValue::Text(s) => assert_eq!(s, "https://example.com/image.jpg"),
        _ => panic!("Expected URL from link href"),
    }
}

#[test]
fn test_extract_deeply_nested() {
    let html = r#"
    <div itemscope itemtype="https://schema.org/Organization">
        <span itemprop="name">Company</span>
        <div itemprop="address" itemscope itemtype="https://schema.org/PostalAddress">
            <span itemprop="streetAddress">123 Main St</span>
            <div itemprop="geo" itemscope itemtype="https://schema.org/GeoCoordinates">
                <meta itemprop="latitude" content="37.7749">
                <meta itemprop="longitude" content="-122.4194">
            </div>
        </div>
    </div>
    "#;

    let items = extract(html, None).unwrap();
    assert_eq!(items.len(), 1);

    let address = &items[0].properties.get("address").unwrap()[0];
    match address {
        PropertyValue::Item(addr) => {
            let geo = &addr.properties.get("geo").unwrap()[0];
            match geo {
                PropertyValue::Item(geo_item) => {
                    let lat = &geo_item.properties.get("latitude").unwrap()[0];
                    match lat {
                        PropertyValue::Text(s) => assert_eq!(s, "37.7749"),
                        _ => panic!("Expected latitude"),
                    }
                }
                _ => panic!("Expected geo item"),
            }
        }
        _ => panic!("Expected address item"),
    }
}

#[test]
fn test_extract_with_base_url() {
    let html = r#"
    <div itemscope itemtype="https://schema.org/Person">
        <a itemprop="url" href="/about">About</a>
    </div>
    "#;

    let items = extract(html, Some("https://example.com")).unwrap();
    assert_eq!(items.len(), 1);

    let url = &items[0].properties.get("url").unwrap()[0];
    match url {
        PropertyValue::Text(s) => assert_eq!(s, "https://example.com/about"),
        _ => panic!("Expected resolved URL"),
    }
}

#[test]
fn test_extract_empty_itemprop() {
    let html = r#"
    <div itemscope itemtype="https://schema.org/Person">
        <span itemprop="name"></span>
    </div>
    "#;

    let items = extract(html, None).unwrap();
    assert_eq!(items.len(), 1);
    // Empty values should still be extracted
    assert!(items[0].properties.contains_key("name"));
}

#[test]
fn test_extract_with_whitespace() {
    let html = r#"
    <div itemscope itemtype="https://schema.org/Person">
        <span itemprop="name">
            Jane Doe
        </span>
    </div>
    "#;

    let items = extract(html, None).unwrap();
    assert_eq!(items.len(), 1);

    let name = &items[0].properties.get("name").unwrap()[0];
    match name {
        PropertyValue::Text(s) => {
            // Should trim whitespace
            let trimmed = s.trim();
            assert_eq!(trimmed, "Jane Doe");
        }
        _ => panic!("Expected text"),
    }
}

#[test]
fn test_extract_unicode_content() {
    let html = r#"
    <div itemscope itemtype="https://schema.org/Person">
        <span itemprop="name">日本語 名前</span>
        <span itemprop="description">Описание на русском</span>
    </div>
    "#;

    let items = extract(html, None).unwrap();
    assert_eq!(items.len(), 1);

    let name = &items[0].properties.get("name").unwrap()[0];
    match name {
        PropertyValue::Text(s) => assert!(s.contains("日本語")),
        _ => panic!("Expected text"),
    }
}

#[test]
fn test_extract_mixed_with_regular_content() {
    let html = r#"
    <div>
        <p>Regular paragraph</p>
        <div itemscope itemtype="https://schema.org/Person">
            <span itemprop="name">Jane Doe</span>
        </div>
        <p>Another paragraph</p>
    </div>
    "#;

    let items = extract(html, None).unwrap();
    assert_eq!(items.len(), 1);
}
