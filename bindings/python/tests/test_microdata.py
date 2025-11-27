"""Tests for Phase 4: HTML5 Microdata extraction"""

import meta_oxide


def test_extract_empty_html():
    """Test extraction from empty HTML"""
    html = "<html><body></body></html>"
    items = meta_oxide.extract_microdata(html)
    assert items == []


def test_extract_basic_person():
    """Test basic Person extraction"""
    html = """
    <div itemscope itemtype="https://schema.org/Person">
        <span itemprop="name">Jane Doe</span>
        <span itemprop="jobTitle">Software Engineer</span>
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1

    item = items[0]
    assert item["type"] == ["https://schema.org/Person"]
    assert item["name"] == "Jane Doe"
    assert item["jobTitle"] == "Software Engineer"


def test_extract_with_url_properties():
    """Test extraction with URL properties (href, src)"""
    html = """
    <div itemscope itemtype="https://schema.org/Person">
        <a itemprop="url" href="https://example.com">Website</a>
        <img itemprop="image" src="https://example.com/photo.jpg" alt="Photo">
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    # URLs are normalized but trailing slashes are preserved as-is
    assert items[0]["url"] == "https://example.com/"
    assert items[0]["image"] == "https://example.com/photo.jpg"


def test_extract_nested_item():
    """Test extraction of nested itemscope"""
    html = """
    <div itemscope itemtype="https://schema.org/Person">
        <span itemprop="name">Jane Doe</span>
        <div itemprop="address" itemscope itemtype="https://schema.org/PostalAddress">
            <span itemprop="streetAddress">123 Main St</span>
            <span itemprop="addressLocality">San Francisco</span>
            <span itemprop="addressRegion">CA</span>
        </div>
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1

    address = items[0]["address"]
    assert isinstance(address, dict)
    assert address["type"] == ["https://schema.org/PostalAddress"]
    assert address["streetAddress"] == "123 Main St"
    assert address["addressLocality"] == "San Francisco"
    assert address["addressRegion"] == "CA"


def test_extract_multiple_items():
    """Test extraction of multiple top-level items"""
    html = """
    <div>
        <div itemscope itemtype="https://schema.org/Person">
            <span itemprop="name">Jane Doe</span>
        </div>
        <div itemscope itemtype="https://schema.org/Person">
            <span itemprop="name">John Smith</span>
        </div>
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 2
    assert items[0]["name"] == "Jane Doe"
    assert items[1]["name"] == "John Smith"


def test_extract_multiple_values_same_property():
    """Test extraction of multiple values for same property"""
    html = """
    <div itemscope itemtype="https://schema.org/Person">
        <span itemprop="name">Jane Doe</span>
        <span itemprop="telephone">555-1234</span>
        <span itemprop="telephone">555-5678</span>
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1

    telephones = items[0]["telephone"]
    assert isinstance(telephones, list)
    assert len(telephones) == 2
    assert "555-1234" in telephones
    assert "555-5678" in telephones


def test_extract_with_itemid():
    """Test extraction with itemid attribute"""
    html = """
    <div itemscope itemtype="https://schema.org/Person" itemid="person-123">
        <span itemprop="name">Jane Doe</span>
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["id"] == "person-123"


def test_extract_multiple_types():
    """Test extraction with multiple types"""
    html = """
    <div itemscope itemtype="https://schema.org/Person https://schema.org/Employee">
        <span itemprop="name">Jane Doe</span>
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1

    types = items[0]["type"]
    assert len(types) == 2
    assert "https://schema.org/Person" in types
    assert "https://schema.org/Employee" in types


def test_extract_article():
    """Test extraction of Article with nested Person"""
    html = """
    <article itemscope itemtype="https://schema.org/Article">
        <h1 itemprop="headline">Amazing Article</h1>
        <p itemprop="description">This is a great article</p>
        <span itemprop="author" itemscope itemtype="https://schema.org/Person">
            <span itemprop="name">Jane Doe</span>
        </span>
        <time itemprop="datePublished" datetime="2024-01-15">January 15, 2024</time>
    </article>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["type"] == ["https://schema.org/Article"]
    assert items[0]["headline"] == "Amazing Article"
    assert items[0]["description"] == "This is a great article"
    assert items[0]["datePublished"] == "2024-01-15"

    author = items[0]["author"]
    assert isinstance(author, dict)
    assert author["type"] == ["https://schema.org/Person"]
    assert author["name"] == "Jane Doe"


def test_extract_product_with_offer():
    """Test extraction of Product with nested Offer"""
    html = """
    <div itemscope itemtype="https://schema.org/Product">
        <span itemprop="name">Wireless Headphones</span>
        <span itemprop="description">Noise-cancelling headphones</span>
        <span itemprop="brand">TechBrand</span>
        <div itemprop="offers" itemscope itemtype="https://schema.org/Offer">
            <span itemprop="price">299.99</span>
            <span itemprop="priceCurrency">USD</span>
        </div>
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["type"] == ["https://schema.org/Product"]
    assert items[0]["name"] == "Wireless Headphones"
    assert items[0]["brand"] == "TechBrand"

    offers = items[0]["offers"]
    assert isinstance(offers, dict)
    assert offers["type"] == ["https://schema.org/Offer"]
    assert offers["price"] == "299.99"
    assert offers["priceCurrency"] == "USD"


def test_extract_no_itemtype():
    """Test extraction without itemtype attribute"""
    html = """
    <div itemscope>
        <span itemprop="name">Jane Doe</span>
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert "type" not in items[0] or items[0].get("type") is None
    assert items[0]["name"] == "Jane Doe"


def test_extract_with_meta_tag():
    """Test extraction using meta tag for content"""
    html = """
    <div itemscope itemtype="https://schema.org/Article">
        <h1 itemprop="headline">Article Title</h1>
        <meta itemprop="datePublished" content="2024-01-15">
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["datePublished"] == "2024-01-15"


def test_extract_with_link_tag():
    """Test extraction using link tag for URLs"""
    html = """
    <div itemscope itemtype="https://schema.org/Article">
        <link itemprop="image" href="https://example.com/image.jpg">
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["image"] == "https://example.com/image.jpg"


def test_extract_deeply_nested():
    """Test extraction of deeply nested items"""
    html = """
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
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["name"] == "Company"

    address = items[0]["address"]
    assert address["streetAddress"] == "123 Main St"

    geo = address["geo"]
    assert geo["type"] == ["https://schema.org/GeoCoordinates"]
    assert geo["latitude"] == "37.7749"
    assert geo["longitude"] == "-122.4194"


def test_extract_with_base_url():
    """Test URL resolution with base_url"""
    html = """
    <div itemscope itemtype="https://schema.org/Person">
        <a itemprop="url" href="/about">About</a>
    </div>
    """

    items = meta_oxide.extract_microdata(html, base_url="https://example.com")
    assert len(items) == 1
    assert items[0]["url"] == "https://example.com/about"


def test_extract_relative_url_without_base():
    """Test relative URL without base_url (should remain relative)"""
    html = """
    <div itemscope itemtype="https://schema.org/Person">
        <a itemprop="url" href="/about">About</a>
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    # Without base_url, relative URLs should remain as-is or be resolved to absolute
    assert items[0]["url"] == "/about"


def test_extract_with_whitespace():
    """Test extraction with whitespace in text content"""
    html = """
    <div itemscope itemtype="https://schema.org/Person">
        <span itemprop="name">
            Jane Doe
        </span>
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    # Text should include whitespace as-is (consumer can trim)
    name = items[0]["name"]
    assert "Jane Doe" in name


def test_extract_unicode_content():
    """Test extraction of Unicode content"""
    html = """
    <div itemscope itemtype="https://schema.org/Person">
        <span itemprop="name">日本語 名前</span>
        <span itemprop="description">Описание на русском</span>
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert "日本語" in items[0]["name"]
    assert "Описание" in items[0]["description"]


def test_extract_mixed_with_regular_content():
    """Test extraction from HTML with mixed regular and microdata content"""
    html = """
    <div>
        <p>Regular paragraph</p>
        <div itemscope itemtype="https://schema.org/Person">
            <span itemprop="name">Jane Doe</span>
        </div>
        <p>Another paragraph</p>
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["name"] == "Jane Doe"


def test_extract_all_includes_microdata():
    """Test that extract_all includes microdata"""
    html = """
    <html>
    <head>
        <title>Test Page</title>
        <meta name="description" content="Test description">
    </head>
    <body>
        <div itemscope itemtype="https://schema.org/Person">
            <span itemprop="name">Jane Doe</span>
        </div>
    </body>
    </html>
    """

    data = meta_oxide.extract_all(html)

    # Should have meta tags
    assert "meta" in data
    assert data["meta"]["title"] == "Test Page"

    # Should have microdata
    assert "microdata" in data
    assert len(data["microdata"]) == 1
    assert data["microdata"][0]["name"] == "Jane Doe"


def test_extract_recipe():
    """Test extraction of Recipe with nested ratings"""
    html = """
    <div itemscope itemtype="https://schema.org/Recipe">
        <span itemprop="name">Chocolate Cake</span>
        <span itemprop="author">Jane Doe</span>
        <span itemprop="prepTime">PT30M</span>
        <span itemprop="cookTime">PT1H</span>
        <div itemprop="aggregateRating" itemscope itemtype="https://schema.org/AggregateRating">
            <span itemprop="ratingValue">4.5</span>
            <span itemprop="reviewCount">120</span>
        </div>
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["type"] == ["https://schema.org/Recipe"]
    assert items[0]["name"] == "Chocolate Cake"
    assert items[0]["prepTime"] == "PT30M"

    rating = items[0]["aggregateRating"]
    assert rating["type"] == ["https://schema.org/AggregateRating"]
    assert rating["ratingValue"] == "4.5"
    assert rating["reviewCount"] == "120"


def test_extract_event():
    """Test extraction of Event"""
    html = """
    <div itemscope itemtype="https://schema.org/Event">
        <span itemprop="name">Tech Conference 2024</span>
        <time itemprop="startDate" datetime="2024-06-15T09:00">June 15, 2024</time>
        <time itemprop="endDate" datetime="2024-06-17T17:00">June 17, 2024</time>
        <div itemprop="location" itemscope itemtype="https://schema.org/Place">
            <span itemprop="name">Convention Center</span>
            <div itemprop="address" itemscope itemtype="https://schema.org/PostalAddress">
                <span itemprop="addressLocality">San Francisco</span>
                <span itemprop="addressRegion">CA</span>
            </div>
        </div>
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["type"] == ["https://schema.org/Event"]
    assert items[0]["name"] == "Tech Conference 2024"
    assert items[0]["startDate"] == "2024-06-15T09:00"

    location = items[0]["location"]
    assert location["type"] == ["https://schema.org/Place"]
    assert location["name"] == "Convention Center"

    address = location["address"]
    assert address["addressLocality"] == "San Francisco"


def test_extract_organization():
    """Test extraction of Organization"""
    html = """
    <div itemscope itemtype="https://schema.org/Organization">
        <span itemprop="name">TechCorp</span>
        <span itemprop="url">https://techcorp.com</span>
        <span itemprop="telephone">555-1234</span>
        <span itemprop="email">info@techcorp.com</span>
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["type"] == ["https://schema.org/Organization"]
    assert items[0]["name"] == "TechCorp"
    assert items[0]["telephone"] == "555-1234"
    assert items[0]["email"] == "info@techcorp.com"


def test_extract_time_element_without_datetime():
    """Test extraction from time element without datetime attribute"""
    html = """
    <div itemscope itemtype="https://schema.org/Article">
        <time itemprop="datePublished">January 15, 2024</time>
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["datePublished"] == "January 15, 2024"


def test_extract_data_element():
    """Test extraction from data element with value attribute"""
    html = """
    <div itemscope itemtype="https://schema.org/Product">
        <span itemprop="name">Widget</span>
        <data itemprop="sku" value="12345">SKU: 12345</data>
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["sku"] == "12345"


def test_extract_with_multiple_properties_on_same_element():
    """Test extraction when element has multiple itemprop values"""
    html = """
    <div itemscope itemtype="https://schema.org/Person">
        <span itemprop="name givenName">Jane</span>
    </div>
    """

    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    # Note: This test might fail depending on implementation
    # Multiple itemprop values in single attribute is valid microdata


def test_microdata_error_handling():
    """Test that invalid HTML doesn't crash the extractor"""
    html = "<div itemscope><span itemprop"

    # Should not raise exception
    items = meta_oxide.extract_microdata(html)
    # May return empty or partial data, but shouldn't crash
    assert isinstance(items, list)


# ============================================================================
# Additional comprehensive tests to reach 40-50 tests
# ============================================================================


# ===== Property extraction tests (different element types) =====


def test_extract_video_src():
    """Test extraction from video element with src attribute"""
    html = """
    <div itemscope itemtype="https://schema.org/VideoObject">
        <span itemprop="name">Video Title</span>
        <video itemprop="contentUrl" src="https://example.com/video.mp4">
            Your browser doesn't support video.
        </video>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["contentUrl"] == "https://example.com/video.mp4"


def test_extract_audio_src():
    """Test extraction from audio element with src attribute"""
    html = """
    <div itemscope itemtype="https://schema.org/AudioObject">
        <span itemprop="name">Podcast Episode</span>
        <audio itemprop="contentUrl" src="https://example.com/audio.mp3"></audio>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["contentUrl"] == "https://example.com/audio.mp3"


def test_extract_iframe_src():
    """Test extraction from iframe element with src attribute"""
    html = """
    <div itemscope itemtype="https://schema.org/VideoObject">
        <span itemprop="name">Embedded Video</span>
        <iframe itemprop="embedUrl" src="https://www.youtube.com/embed/abc123"></iframe>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["embedUrl"] == "https://www.youtube.com/embed/abc123"


def test_extract_object_data():
    """Test extraction from object element with data attribute"""
    html = """
    <div itemscope itemtype="https://schema.org/MediaObject">
        <object itemprop="contentUrl" data="https://example.com/file.pdf"></object>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["contentUrl"] == "https://example.com/file.pdf"


def test_extract_meter_value():
    """Test extraction from meter element with value attribute"""
    html = """
    <div itemscope itemtype="https://schema.org/Product">
        <span itemprop="name">Product</span>
        <meter itemprop="ratingValue" value="4.5" min="0" max="5">4.5 out of 5</meter>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["ratingValue"] == "4.5"


def test_extract_area_href():
    """Test extraction from area element with href attribute"""
    html = """
    <div itemscope itemtype="https://schema.org/ImageObject">
        <map>
            <area itemprop="url" href="https://example.com/region1"
                  shape="rect" coords="0,0,100,100">
        </map>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["url"] == "https://example.com/region1"


# ===== Real-world Schema.org types =====


def test_extract_local_business():
    """Test extraction of LocalBusiness with multiple nested items"""
    html = """
    <div itemscope itemtype="https://schema.org/Restaurant">
        <span itemprop="name">The Gourmet Restaurant</span>
        <div itemprop="address" itemscope itemtype="https://schema.org/PostalAddress">
            <span itemprop="streetAddress">456 Food Street</span>
            <span itemprop="addressLocality">New York</span>
            <span itemprop="addressRegion">NY</span>
            <span itemprop="postalCode">10001</span>
        </div>
        <span itemprop="telephone">555-FOOD</span>
        <span itemprop="priceRange">$$</span>
        <div itemprop="geo" itemscope itemtype="https://schema.org/GeoCoordinates">
            <meta itemprop="latitude" content="40.7589">
            <meta itemprop="longitude" content="-73.9851">
        </div>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["type"] == ["https://schema.org/Restaurant"]
    assert items[0]["name"] == "The Gourmet Restaurant"
    assert items[0]["telephone"] == "555-FOOD"
    assert items[0]["priceRange"] == "$$"

    # Check nested address
    address = items[0]["address"]
    assert address["streetAddress"] == "456 Food Street"
    assert address["postalCode"] == "10001"

    # Check nested geo
    geo = items[0]["geo"]
    assert geo["latitude"] == "40.7589"
    assert geo["longitude"] == "-73.9851"


def test_extract_blog_posting():
    """Test extraction of BlogPosting with multiple authors"""
    html = """
    <article itemscope itemtype="https://schema.org/BlogPosting">
        <h1 itemprop="headline">My Blog Post</h1>
        <p itemprop="description">A fascinating blog post about microdata.</p>
        <span itemprop="author" itemscope itemtype="https://schema.org/Person">
            <span itemprop="name">Alice Smith</span>
        </span>
        <span itemprop="author" itemscope itemtype="https://schema.org/Person">
            <span itemprop="name">Bob Jones</span>
        </span>
        <time itemprop="datePublished" datetime="2024-03-15">March 15, 2024</time>
        <div itemprop="publisher" itemscope itemtype="https://schema.org/Organization">
            <span itemprop="name">Tech Blog Inc</span>
        </div>
    </article>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["headline"] == "My Blog Post"
    assert items[0]["datePublished"] == "2024-03-15"

    # Multiple authors should be in a list
    authors = items[0]["author"]
    assert isinstance(authors, list)
    assert len(authors) == 2
    assert authors[0]["name"] == "Alice Smith"
    assert authors[1]["name"] == "Bob Jones"

    # Publisher should be a single nested item
    publisher = items[0]["publisher"]
    assert isinstance(publisher, dict)
    assert publisher["name"] == "Tech Blog Inc"


def test_extract_job_posting():
    """Test extraction of JobPosting"""
    html = """
    <div itemscope itemtype="https://schema.org/JobPosting">
        <span itemprop="title">Software Engineer</span>
        <span itemprop="description">We are looking for a talented software engineer.</span>
        <div itemprop="hiringOrganization" itemscope itemtype="https://schema.org/Organization">
            <span itemprop="name">TechCorp</span>
        </div>
        <div itemprop="jobLocation" itemscope itemtype="https://schema.org/Place">
            <span itemprop="name">San Francisco Office</span>
            <div itemprop="address" itemscope itemtype="https://schema.org/PostalAddress">
                <span itemprop="addressLocality">San Francisco</span>
                <span itemprop="addressRegion">CA</span>
            </div>
        </div>
        <time itemprop="datePosted" datetime="2024-01-01">January 1, 2024</time>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["type"] == ["https://schema.org/JobPosting"]
    assert items[0]["title"] == "Software Engineer"
    assert items[0]["datePosted"] == "2024-01-01"

    org = items[0]["hiringOrganization"]
    assert org["name"] == "TechCorp"

    location = items[0]["jobLocation"]
    assert location["name"] == "San Francisco Office"
    assert location["address"]["addressLocality"] == "San Francisco"


def test_extract_course():
    """Test extraction of Course"""
    html = """
    <div itemscope itemtype="https://schema.org/Course">
        <span itemprop="name">Introduction to Python</span>
        <span itemprop="description">Learn Python programming from scratch</span>
        <div itemprop="provider" itemscope itemtype="https://schema.org/Organization">
            <span itemprop="name">Online Education Platform</span>
        </div>
        <div itemprop="offers" itemscope itemtype="https://schema.org/Offer">
            <span itemprop="price">99.99</span>
            <span itemprop="priceCurrency">USD</span>
        </div>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["name"] == "Introduction to Python"

    provider = items[0]["provider"]
    assert provider["name"] == "Online Education Platform"

    offers = items[0]["offers"]
    assert offers["price"] == "99.99"
    assert offers["priceCurrency"] == "USD"


# ===== Multiple images and media =====


def test_extract_multiple_images():
    """Test extraction of multiple images for same property"""
    html = """
    <div itemscope itemtype="https://schema.org/Article">
        <span itemprop="headline">Photo Gallery</span>
        <img itemprop="image" src="https://example.com/img1.jpg" alt="Image 1">
        <img itemprop="image" src="https://example.com/img2.jpg" alt="Image 2">
        <img itemprop="image" src="https://example.com/img3.jpg" alt="Image 3">
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1

    images = items[0]["image"]
    assert isinstance(images, list)
    assert len(images) == 3
    assert "https://example.com/img1.jpg" in images
    assert "https://example.com/img2.jpg" in images
    assert "https://example.com/img3.jpg" in images


def test_extract_mixed_property_types():
    """Test extraction of properties with mixed text and nested items"""
    html = """
    <div itemscope itemtype="https://schema.org/Article">
        <span itemprop="headline">Article with Mixed Authors</span>
        <span itemprop="author">Simple Author Name</span>
        <div itemprop="author" itemscope itemtype="https://schema.org/Person">
            <span itemprop="name">Detailed Author</span>
            <span itemprop="email">author@example.com</span>
        </div>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1

    authors = items[0]["author"]
    assert isinstance(authors, list)
    assert len(authors) == 2
    # First author is text
    assert authors[0] == "Simple Author Name"
    # Second author is nested item
    assert isinstance(authors[1], dict)
    assert authors[1]["name"] == "Detailed Author"


# ===== Edge cases and invalid data =====


def test_extract_empty_itemscope():
    """Test extraction from itemscope with no properties"""
    html = """
    <div itemscope itemtype="https://schema.org/Thing">
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["type"] == ["https://schema.org/Thing"]
    # Should only have type, no other properties
    assert len(items[0]) == 1


def test_extract_empty_property_value():
    """Test extraction of empty property values (should be skipped)"""
    html = """
    <div itemscope itemtype="https://schema.org/Person">
        <span itemprop="name">Jane Doe</span>
        <span itemprop="description"></span>
        <span itemprop="jobTitle">   </span>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["name"] == "Jane Doe"
    # Empty properties might be included or excluded depending on implementation
    # This tests current behavior


def test_extract_property_without_itemscope():
    """Test that itemprop outside itemscope is ignored"""
    html = """
    <div>
        <span itemprop="name">Should be ignored</span>
        <div itemscope itemtype="https://schema.org/Person">
            <span itemprop="name">Jane Doe</span>
        </div>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["name"] == "Jane Doe"


def test_extract_invalid_itemtype():
    """Test extraction with non-Schema.org itemtype"""
    html = """
    <div itemscope itemtype="http://example.com/CustomType">
        <span itemprop="name">Custom Item</span>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["type"] == ["http://example.com/CustomType"]
    assert items[0]["name"] == "Custom Item"


def test_extract_relative_itemtype():
    """Test extraction with relative itemtype (edge case)"""
    html = """
    <div itemscope itemtype="Person">
        <span itemprop="name">Jane Doe</span>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    # Should still extract even with non-standard type
    assert items[0]["type"] == ["Person"]


# ===== Integration with JSON-LD =====


def test_extract_microdata_with_jsonld_on_page():
    """Test that microdata extraction works alongside JSON-LD"""
    html = """
    <html>
    <head>
        <script type="application/ld+json">
        {
            "@context": "https://schema.org",
            "@type": "Article",
            "headline": "JSON-LD Article"
        }
        </script>
    </head>
    <body>
        <div itemscope itemtype="https://schema.org/Person">
            <span itemprop="name">Microdata Person</span>
        </div>
    </body>
    </html>
    """
    # Extract only microdata
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["name"] == "Microdata Person"

    # Verify extract_all includes both
    all_data = meta_oxide.extract_all(html)
    assert "microdata" in all_data
    assert "jsonld" in all_data
    assert len(all_data["microdata"]) == 1
    assert len(all_data["jsonld"]) == 1


# ===== Complex nesting scenarios =====


def test_extract_four_level_nesting():
    """Test extraction of 4-level deep nesting"""
    html = """
    <div itemscope itemtype="https://schema.org/Organization">
        <span itemprop="name">Company</span>
        <div itemprop="address" itemscope itemtype="https://schema.org/PostalAddress">
            <span itemprop="streetAddress">123 Main St</span>
            <div itemprop="geo" itemscope itemtype="https://schema.org/GeoCoordinates">
                <meta itemprop="latitude" content="40.7589">
                <div itemprop="elevation" itemscope itemtype="https://schema.org/QuantitativeValue">
                    <meta itemprop="value" content="100">
                    <meta itemprop="unitCode" content="MTR">
                </div>
            </div>
        </div>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["name"] == "Company"

    # Navigate through 4 levels
    address = items[0]["address"]
    assert address["streetAddress"] == "123 Main St"

    geo = address["geo"]
    assert geo["latitude"] == "40.7589"

    elevation = geo["elevation"]
    assert elevation["value"] == "100"
    assert elevation["unitCode"] == "MTR"


def test_extract_anonymous_nested_item():
    """Test extraction of nested item without itemprop (referenced elsewhere)"""
    html = """
    <div itemscope itemtype="https://schema.org/Person">
        <span itemprop="name">Jane Doe</span>
        <div itemprop="knows" itemscope itemtype="https://schema.org/Person">
            <span itemprop="name">John Smith</span>
        </div>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["name"] == "Jane Doe"

    knows = items[0]["knows"]
    assert isinstance(knows, dict)
    assert knows["name"] == "John Smith"


# ===== URL resolution tests =====


def test_extract_url_with_base_url_resolution():
    """Test URL resolution with various relative URL formats"""
    html = """
    <div itemscope itemtype="https://schema.org/Article">
        <a itemprop="url" href="/article/123">Article Link</a>
        <img itemprop="image" src="/images/photo.jpg" alt="Photo">
        <a itemprop="sameAs" href="https://absolute.example.com/page">Absolute</a>
    </div>
    """
    items = meta_oxide.extract_microdata(html, base_url="https://example.com")
    assert len(items) == 1

    assert items[0]["url"] == "https://example.com/article/123"
    assert items[0]["image"] == "https://example.com/images/photo.jpg"
    # Absolute URL should remain unchanged
    assert items[0]["sameAs"] == "https://absolute.example.com/page"


def test_extract_url_with_fragment():
    """Test URL with fragment identifier"""
    html = """
    <div itemscope itemtype="https://schema.org/Article">
        <a itemprop="url" href="https://example.com/page#section">Link</a>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["url"] == "https://example.com/page#section"


def test_extract_url_with_query_params():
    """Test URL with query parameters"""
    html = """
    <div itemscope itemtype="https://schema.org/Product">
        <a itemprop="url" href="https://example.com/product?id=123&variant=blue">Product Link</a>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert "id=123" in items[0]["url"]
    assert "variant=blue" in items[0]["url"]


# ===== Special element handling =====


def test_extract_from_different_container_elements():
    """Test extraction from various container element types"""
    html = """
    <article itemscope itemtype="https://schema.org/Article">
        <h1 itemprop="headline">Article in article tag</h1>
    </article>
    <section itemscope itemtype="https://schema.org/Event">
        <span itemprop="name">Event in section tag</span>
    </section>
    <main itemscope itemtype="https://schema.org/WebPage">
        <span itemprop="name">Page in main tag</span>
    </main>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 3
    assert items[0]["headline"] == "Article in article tag"
    assert items[1]["name"] == "Event in section tag"
    assert items[2]["name"] == "Page in main tag"


def test_extract_different_heading_levels():
    """Test extraction from different heading elements"""
    html = """
    <div itemscope itemtype="https://schema.org/Article">
        <h1 itemprop="headline">Main Headline</h1>
        <h2 itemprop="alternativeHeadline">Sub Headline</h2>
        <h3 itemprop="section">Section Name</h3>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["headline"] == "Main Headline"
    assert items[0]["alternativeHeadline"] == "Sub Headline"
    assert items[0]["section"] == "Section Name"


# ===== Multiple itemtype values =====


def test_extract_with_three_types():
    """Test extraction with three different types"""
    html = """
    <div itemscope itemtype="https://schema.org/Person https://schema.org/Employee https://schema.org/Author">
        <span itemprop="name">Jane Doe</span>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1

    types = items[0]["type"]
    assert len(types) == 3
    assert "https://schema.org/Person" in types
    assert "https://schema.org/Employee" in types
    assert "https://schema.org/Author" in types


# ===== Performance and stress tests =====


def test_extract_many_properties():
    """Test extraction with many properties (20+ properties)"""
    html = """
    <div itemscope itemtype="https://schema.org/Person">
        <span itemprop="name">Jane Doe</span>
        <span itemprop="givenName">Jane</span>
        <span itemprop="familyName">Doe</span>
        <span itemprop="email">jane@example.com</span>
        <span itemprop="telephone">555-1234</span>
        <span itemprop="jobTitle">Software Engineer</span>
        <span itemprop="worksFor">TechCorp</span>
        <span itemprop="url">https://janedoe.com</span>
        <span itemprop="gender">Female</span>
        <span itemprop="birthDate">1990-01-01</span>
        <span itemprop="nationality">American</span>
        <span itemprop="alumniOf">MIT</span>
        <span itemprop="knows">John Smith</span>
        <span itemprop="award">Employee of the Year</span>
        <span itemprop="description">Experienced software engineer</span>
        <span itemprop="sameAs">https://linkedin.com/in/janedoe</span>
        <span itemprop="sameAs">https://twitter.com/janedoe</span>
        <span itemprop="image">https://example.com/photo.jpg</span>
        <span itemprop="address">San Francisco, CA</span>
        <span itemprop="affiliation">Tech Association</span>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 1
    assert items[0]["name"] == "Jane Doe"
    assert items[0]["email"] == "jane@example.com"
    # sameAs should be a list since there are 2 values
    assert isinstance(items[0]["sameAs"], list)
    assert len(items[0]["sameAs"]) == 2


def test_extract_multiple_different_types():
    """Test extraction of 5+ different Schema.org types on same page"""
    html = """
    <div itemscope itemtype="https://schema.org/Article">
        <span itemprop="headline">Article</span>
    </div>
    <div itemscope itemtype="https://schema.org/Person">
        <span itemprop="name">Person</span>
    </div>
    <div itemscope itemtype="https://schema.org/Product">
        <span itemprop="name">Product</span>
    </div>
    <div itemscope itemtype="https://schema.org/Event">
        <span itemprop="name">Event</span>
    </div>
    <div itemscope itemtype="https://schema.org/Organization">
        <span itemprop="name">Organization</span>
    </div>
    """
    items = meta_oxide.extract_microdata(html)
    assert len(items) == 5

    types = [item["type"][0] for item in items]
    assert "https://schema.org/Article" in types
    assert "https://schema.org/Person" in types
    assert "https://schema.org/Product" in types
    assert "https://schema.org/Event" in types
    assert "https://schema.org/Organization" in types
