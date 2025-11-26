"""Tests for h-product microformat extraction"""

import meta_oxide


def test_extract_hproduct_basic():
    """Test basic h-product extraction"""
    html = """
        <div class="h-product">
            <span class="p-name">Laptop Pro</span>
            <span class="p-brand">TechBrand</span>
            <span class="p-price">$999.99</span>
        </div>
    """
    products = meta_oxide.extract_hproduct(html)
    assert len(products) == 1
    assert products[0]["name"] == "Laptop Pro"
    assert products[0]["brand"] == "TechBrand"
    assert products[0]["price"] == "$999.99"


def test_hproduct_with_description():
    """Test h-product with description"""
    html = """
        <div class="h-product">
            <span class="p-name">Smartphone X</span>
            <div class="p-description">
                A powerful smartphone with advanced features.
            </div>
        </div>
    """
    products = meta_oxide.extract_hproduct(html)
    assert len(products) == 1
    assert "powerful smartphone" in products[0]["description"]
    assert "advanced features" in products[0]["description"]


def test_hproduct_with_photo():
    """Test h-product with photo"""
    html = """
        <div class="h-product">
            <span class="p-name">Camera</span>
            <img class="u-photo" src="https://example.com/camera.jpg" alt="Camera" />
        </div>
    """
    products = meta_oxide.extract_hproduct(html)
    assert len(products) == 1
    assert products[0]["photo"] == "https://example.com/camera.jpg"


def test_hproduct_with_rating():
    """Test h-product with rating"""
    html = """
        <div class="h-product">
            <span class="p-name">Headphones</span>
            <span class="p-rating">4.5</span>
        </div>
    """
    products = meta_oxide.extract_hproduct(html)
    assert len(products) == 1
    assert products[0]["rating"] == 4.5


def test_hproduct_with_categories():
    """Test h-product with multiple categories"""
    html = """
        <div class="h-product">
            <span class="p-name">Running Shoes</span>
            <span class="p-category">Footwear</span>
            <span class="p-category">Sports</span>
        </div>
    """
    products = meta_oxide.extract_hproduct(html)
    assert len(products) == 1
    assert len(products[0]["category"]) == 2
    assert products[0]["category"][0] == "Footwear"
    assert products[0]["category"][1] == "Sports"


def test_hproduct_with_url():
    """Test h-product with URL"""
    html = """
        <div class="h-product">
            <a class="u-url" href="https://example.com/product/123">
                <span class="p-name">Watch</span>
            </a>
        </div>
    """
    products = meta_oxide.extract_hproduct(html)
    assert len(products) == 1
    assert products[0]["url"] == "https://example.com/product/123"


def test_hproduct_with_identifier():
    """Test h-product with identifier/SKU"""
    html = """
        <div class="h-product">
            <span class="p-name">Tablet</span>
            <span class="p-identifier">SKU-12345</span>
        </div>
    """
    products = meta_oxide.extract_hproduct(html)
    assert len(products) == 1
    assert products[0]["identifier"] == "SKU-12345"


def test_multiple_hproducts():
    """Test extraction of multiple products"""
    html = """
        <div class="h-product">
            <span class="p-name">Product 1</span>
        </div>
        <div class="h-product">
            <span class="p-name">Product 2</span>
        </div>
    """
    products = meta_oxide.extract_hproduct(html)
    assert len(products) == 2
    assert products[0]["name"] == "Product 1"
    assert products[1]["name"] == "Product 2"


def test_hproduct_empty():
    """Test page with no products"""
    html = "<html><body><p>No products here</p></body></html>"
    products = meta_oxide.extract_hproduct(html)
    assert len(products) == 0


def test_hproduct_ecommerce():
    """Test realistic e-commerce product"""
    html = """
        <article class="h-product">
            <h1 class="p-name">Wireless Noise-Cancelling Headphones</h1>

            <img class="u-photo" src="https://example.com/headphones.jpg" alt="Headphones" />

            <span class="p-brand">AudioTech</span>

            <div class="p-category">Electronics</div>
            <div class="p-category">Audio</div>

            <div class="p-description">
                Premium wireless headphones with active noise cancellation.
                Features 30-hour battery life and comfortable over-ear design.
            </div>

            <span class="p-price">$299.99</span>

            <div class="p-rating">4.8</div>

            <a class="u-url" href="https://example.com/products/headphones-pro">View Product</a>

            <span class="p-identifier">PROD-WH-NC-001</span>
        </article>
    """
    products = meta_oxide.extract_hproduct(html)
    assert len(products) == 1

    product = products[0]
    assert product["name"] == "Wireless Noise-Cancelling Headphones"
    assert product["brand"] == "AudioTech"
    assert product["price"] == "$299.99"
    assert abs(product["rating"] - 4.8) < 0.01
    assert "noise cancellation" in product["description"]
    assert "30-hour battery" in product["description"]
    assert product["photo"] == "https://example.com/headphones.jpg"
    assert product["url"] == "https://example.com/products/headphones-pro"
    assert product["identifier"] == "PROD-WH-NC-001"

    # Check categories
    assert len(product["category"]) == 2
    assert "Electronics" in product["category"]
    assert "Audio" in product["category"]


def test_hproduct_book():
    """Test book product"""
    html = """
        <div class="h-product">
            <span class="p-name">The Rust Programming Language</span>
            <span class="p-brand">No Starch Press</span>
            <span class="p-price">$39.99</span>
            <span class="p-category">Books</span>
            <span class="p-category">Programming</span>
            <span class="p-identifier">ISBN-978-1593278281</span>
            <span class="p-rating">4.9</span>
        </div>
    """
    products = meta_oxide.extract_hproduct(html)
    assert len(products) == 1
    assert products[0]["name"] == "The Rust Programming Language"
    assert products[0]["brand"] == "No Starch Press"
    assert products[0]["identifier"] == "ISBN-978-1593278281"
    assert abs(products[0]["rating"] - 4.9) < 0.01


def test_hproduct_in_extract_all():
    """Test that h-product is included in extract_all"""
    html = """
        <html>
        <head>
            <title>Product Page</title>
        </head>
        <body>
            <div class="h-product">
                <span class="p-name">Amazing Widget</span>
                <span class="p-brand">TechCorp</span>
                <span class="p-price">$99.99</span>
            </div>
        </body>
        </html>
    """
    data = meta_oxide.extract_all(html)

    assert "microformats" in data
    assert "h-product" in data["microformats"]
    assert len(data["microformats"]["h-product"]) == 1
    assert data["microformats"]["h-product"][0]["name"] == "Amazing Widget"
    assert data["microformats"]["h-product"][0]["brand"] == "TechCorp"
    assert data["microformats"]["h-product"][0]["price"] == "$99.99"
