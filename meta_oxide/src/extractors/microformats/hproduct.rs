use crate::microformat_extractor;
use crate::types::HProduct;

microformat_extractor! {
    HProduct, ".h-product" {
        name: text(".p-name"),
        description: text(".p-description, .e-description"),
        photo: url(".u-photo"),
        price: text(".p-price"),
        brand: text(".p-brand"),
        category: multi_text(".p-category"),
        rating: number(".p-rating"),
        url: url(".u-url"),
        identifier: text(".p-identifier"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hproduct() {
        let html = r#"
            <div class="h-product">
                <span class="p-name">Laptop Pro</span>
                <span class="p-brand">TechBrand</span>
                <span class="p-price">$999.99</span>
            </div>
        "#;

        let products = extract(html, None).unwrap();
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].name, Some("Laptop Pro".to_string()));
        assert_eq!(products[0].brand, Some("TechBrand".to_string()));
        assert_eq!(products[0].price, Some("$999.99".to_string()));
    }

    #[test]
    fn test_hproduct_with_description() {
        let html = r#"
            <div class="h-product">
                <span class="p-name">Smartphone X</span>
                <div class="p-description">
                    A powerful smartphone with advanced features.
                </div>
            </div>
        "#;

        let products = extract(html, None).unwrap();
        assert_eq!(products.len(), 1);
        assert!(products[0].description.as_ref().unwrap().contains("powerful smartphone"));
    }

    #[test]
    fn test_hproduct_with_photo() {
        let html = r#"
            <div class="h-product">
                <span class="p-name">Camera</span>
                <img class="u-photo" src="https://example.com/camera.jpg" alt="Camera" />
            </div>
        "#;

        let products = extract(html, None).unwrap();
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].photo, Some("https://example.com/camera.jpg".to_string()));
    }

    #[test]
    fn test_hproduct_with_rating() {
        let html = r#"
            <div class="h-product">
                <span class="p-name">Headphones</span>
                <span class="p-rating">4.5</span>
            </div>
        "#;

        let products = extract(html, None).unwrap();
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].rating, Some(4.5));
    }

    #[test]
    fn test_hproduct_with_categories() {
        let html = r#"
            <div class="h-product">
                <span class="p-name">Running Shoes</span>
                <span class="p-category">Footwear</span>
                <span class="p-category">Sports</span>
            </div>
        "#;

        let products = extract(html, None).unwrap();
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].category.len(), 2);
        assert_eq!(products[0].category[0], "Footwear");
        assert_eq!(products[0].category[1], "Sports");
    }

    #[test]
    fn test_hproduct_with_url() {
        let html = r#"
            <div class="h-product">
                <a class="u-url" href="https://example.com/product/123">
                    <span class="p-name">Watch</span>
                </a>
            </div>
        "#;

        let products = extract(html, None).unwrap();
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].url, Some("https://example.com/product/123".to_string()));
    }

    #[test]
    fn test_hproduct_with_identifier() {
        let html = r#"
            <div class="h-product">
                <span class="p-name">Tablet</span>
                <span class="p-identifier">SKU-12345</span>
            </div>
        "#;

        let products = extract(html, None).unwrap();
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].identifier, Some("SKU-12345".to_string()));
    }

    #[test]
    fn test_multiple_hproducts() {
        let html = r#"
            <div class="h-product">
                <span class="p-name">Product 1</span>
            </div>
            <div class="h-product">
                <span class="p-name">Product 2</span>
            </div>
        "#;

        let products = extract(html, None).unwrap();
        assert_eq!(products.len(), 2);
        assert_eq!(products[0].name, Some("Product 1".to_string()));
        assert_eq!(products[1].name, Some("Product 2".to_string()));
    }

    #[test]
    fn test_hproduct_empty() {
        let html = "<html><body><p>No products here</p></body></html>";
        let products = extract(html, None).unwrap();
        assert_eq!(products.len(), 0);
    }
}
