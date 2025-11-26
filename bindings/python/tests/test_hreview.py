"""Tests for h-review microformat extraction - comprehensive TDD test suite"""

import meta_oxide

# =============================================================================
# CATEGORY 1: Basic Fields (4 tests)
# =============================================================================


def test_hreview_basic_fields():
    """Test basic h-review with name, rating, and content"""
    html = """
        <div class="h-review">
            <span class="p-name">Great Product</span>
            <span class="p-rating">4.5</span>
            <span class="e-content">Highly recommend this product!</span>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0].get("name") == "Great Product"
    assert reviews[0]["rating"] == 4.5
    assert reviews[0].get("content") == "Highly recommend this product!"


def test_hreview_with_summary_backward_compat():
    """Test backward compatibility with old p-summary property"""
    html = """
        <div class="h-review">
            <span class="p-summary">Excellent service</span>
            <span class="p-rating">5</span>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0]["summary"] == "Excellent service"
    assert reviews[0]["rating"] == 5.0


def test_hreview_integer_rating():
    """Test review with integer rating"""
    html = """
        <div class="h-review">
            <span class="p-name">Perfect!</span>
            <span class="p-rating">5</span>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0]["rating"] == 5.0


def test_hreview_decimal_rating():
    """Test review with decimal rating"""
    html = """
        <div class="h-review">
            <span class="p-name">Good product</span>
            <span class="p-rating">3.75</span>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0]["rating"] == 3.75


# =============================================================================
# CATEGORY 2: Rating Range (3 tests)
# =============================================================================


def test_hreview_with_rating_scale():
    """Test h-review with best and worst rating boundaries"""
    html = """
        <div class="h-review">
            <span class="p-name">Restaurant review</span>
            <span class="p-rating">9</span>
            <span class="p-best">10</span>
            <span class="p-worst">1</span>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0]["rating"] == 9.0
    assert reviews[0]["best"] == 10.0
    assert reviews[0]["worst"] == 1.0


def test_hreview_five_star_scale():
    """Test common 5-star rating scale"""
    html = """
        <div class="h-review">
            <span class="p-name">4 out of 5 stars</span>
            <span class="p-rating">4</span>
            <span class="p-best">5</span>
            <span class="p-worst">1</span>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0]["rating"] == 4.0
    assert reviews[0]["best"] == 5.0
    assert reviews[0]["worst"] == 1.0


def test_hreview_hundred_scale():
    """Test 100-point rating scale"""
    html = """
        <div class="h-review">
            <span class="p-name">Game review</span>
            <span class="p-rating">87.5</span>
            <span class="p-best">100</span>
            <span class="p-worst">0</span>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0]["rating"] == 87.5
    assert reviews[0]["best"] == 100.0
    assert reviews[0]["worst"] == 0.0


# =============================================================================
# CATEGORY 3: Nested Items (4 tests)
# =============================================================================


def test_hreview_with_nested_hcard_reviewer():
    """Test h-review with nested h-card for reviewer"""
    html = """
        <div class="h-review">
            <span class="p-name">Excellent product</span>
            <span class="p-rating">5</span>
            <div class="p-reviewer h-card">
                <span class="p-name">John Doe</span>
                <a class="u-url" href="https://johndoe.com">Website</a>
            </div>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0].get("reviewer_card") is not None
    reviewer = reviews[0]["reviewer_card"]
    assert reviewer["name"] == "John Doe"
    assert reviewer["url"] == "https://johndoe.com"


def test_hreview_with_nested_hproduct_item():
    """Test h-review with nested h-product as item being reviewed"""
    html = """
        <div class="h-review">
            <span class="p-name">Love this laptop!</span>
            <span class="p-rating">4.5</span>
            <div class="p-item h-product">
                <span class="p-name">ThinkPad X1 Carbon</span>
                <span class="p-brand">Lenovo</span>
                <span class="p-price">$1,299</span>
            </div>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0].get("item_product") is not None
    product = reviews[0]["item_product"]
    assert product["name"] == "ThinkPad X1 Carbon"
    assert product["brand"] == "Lenovo"
    assert product["price"] == "$1,299"


def test_hreview_simple_reviewer_text():
    """Test h-review with simple text reviewer (not nested h-card)"""
    html = """
        <div class="h-review">
            <span class="p-name">Good movie</span>
            <span class="p-rating">4</span>
            <span class="p-reviewer">Jane Smith</span>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0]["reviewer"] == "Jane Smith"


def test_hreview_simple_item_text():
    """Test h-review with simple text item (not nested h-product)"""
    html = """
        <div class="h-review">
            <span class="p-name">Great book</span>
            <span class="p-rating">5</span>
            <span class="p-item">The Rust Programming Language</span>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0]["item"] == "The Rust Programming Language"


# =============================================================================
# CATEGORY 4: Dates (2 tests)
# =============================================================================


def test_hreview_with_published_date():
    """Test h-review with dt-published (modern microformats2)"""
    html = """
        <div class="h-review">
            <span class="p-name">Good experience</span>
            <span class="p-rating">4</span>
            <time class="dt-published" datetime="2024-01-15">Jan 15, 2024</time>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0].get("published") == "2024-01-15"


def test_hreview_with_dtreviewed_backward_compat():
    """Test backward compatibility with dt-reviewed property"""
    html = """
        <div class="h-review">
            <span class="p-summary">Nice hotel</span>
            <span class="p-rating">4.5</span>
            <time class="dt-reviewed" datetime="2024-03-20">March 20, 2024</time>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0]["dtreviewed"] == "2024-03-20"


# =============================================================================
# CATEGORY 5: Real-World Examples (4 tests)
# =============================================================================


def test_hreview_product_review_comprehensive():
    """Test realistic comprehensive product review"""
    html = """
        <article class="h-review">
            <h3 class="p-name">Best laptop for developers</h3>
            <div class="e-content">
                After using this laptop for 6 months, I can confidently say it's perfect
                for software development. Fast, reliable, great battery life.
            </div>
            <div class="p-item h-product">
                <span class="p-name">ThinkPad X1 Carbon</span>
                <span class="p-brand">Lenovo</span>
            </div>
            <span class="p-rating">4.5</span> out of <span class="p-best">5</span> stars
            <p>Reviewed by
               <span class="p-reviewer h-card">
                   <span class="p-name">Alice Developer</span>
                   <a class="u-url" href="https://alice.dev">alice.dev</a>
               </span>
               on <time class="dt-published" datetime="2024-03-20">March 20, 2024</time>
            </p>
        </article>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    review = reviews[0]
    assert review.get("name") == "Best laptop for developers"
    assert "6 months" in review.get("content", "")
    assert "software development" in review.get("content", "")
    assert review["rating"] == 4.5
    assert review["best"] == 5.0
    assert review.get("published") == "2024-03-20"
    # Check nested product
    assert review.get("item_product") is not None
    assert review["item_product"]["name"] == "ThinkPad X1 Carbon"
    # Check nested reviewer h-card
    assert review.get("reviewer_card") is not None
    assert review["reviewer_card"]["name"] == "Alice Developer"


def test_hreview_restaurant_review():
    """Test restaurant review"""
    html = """
        <div class="h-review">
            <span class="p-item">Pizza Palace</span>
            <span class="p-name">Amazing pizza and great atmosphere!</span>
            <div class="e-content">
                The margherita pizza was perfectly cooked with fresh ingredients.
                Service was friendly and quick. Highly recommended!
            </div>
            <span class="p-rating">5</span>
            <span class="p-best">5</span>
            <span class="p-reviewer">Food Critic</span>
            <time class="dt-published" datetime="2024-02-10">Feb 10, 2024</time>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0]["item"] == "Pizza Palace"
    assert reviews[0]["rating"] == 5.0
    assert "margherita pizza" in reviews[0].get("content", "")
    assert reviews[0]["reviewer"] == "Food Critic"


def test_hreview_movie_review():
    """Test movie review"""
    html = """
        <div class="h-review">
            <span class="p-name">Must-see sci-fi masterpiece</span>
            <span class="p-item">Inception</span>
            <span class="p-rating">9.5</span>
            <span class="p-best">10</span>
            <span class="p-worst">1</span>
            <div class="e-content">
                Mind-bending plot, stunning visuals, and excellent performances.
                Christopher Nolan at his best.
            </div>
            <span class="p-reviewer">Movie Buff</span>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0].get("name") == "Must-see sci-fi masterpiece"
    assert reviews[0]["item"] == "Inception"
    assert reviews[0]["rating"] == 9.5
    assert "Christopher Nolan" in reviews[0].get("content", "")


def test_hreview_book_review():
    """Test book review with URL"""
    html = """
        <div class="h-review">
            <a class="u-url" href="https://reviews.example.com/rust-book">
                <span class="p-name">Essential reading for Rust developers</span>
            </a>
            <span class="p-item">The Rust Programming Language</span>
            <span class="p-rating">5</span>
            <span class="p-best">5</span>
            <div class="e-content">
                This book is the definitive guide to Rust. Clear explanations,
                practical examples, and comprehensive coverage of the language.
            </div>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0].get("name") == "Essential reading for Rust developers"
    assert reviews[0]["url"] == "https://reviews.example.com/rust-book"
    assert reviews[0]["item"] == "The Rust Programming Language"


# =============================================================================
# CATEGORY 6: Edge Cases (3 tests)
# =============================================================================


def test_hreview_minimal():
    """Test minimal h-review with only required fields"""
    html = """
        <div class="h-review">
            <span class="p-name">Quick review</span>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0].get("name") == "Quick review"


def test_hreview_missing_rating():
    """Test h-review without rating"""
    html = """
        <div class="h-review">
            <span class="p-name">Review without rating</span>
            <span class="e-content">Just some comments without a score</span>
            <span class="p-item">Product X</span>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0].get("name") == "Review without rating"
    assert reviews[0].get("rating") is None
    assert reviews[0]["item"] == "Product X"


def test_multiple_hreviews():
    """Test extraction of multiple reviews on same page"""
    html = """
        <div class="h-review">
            <span class="p-name">Review 1</span>
            <span class="p-rating">4</span>
        </div>
        <div class="h-review">
            <span class="p-name">Review 2</span>
            <span class="p-rating">5</span>
        </div>
        <div class="h-review">
            <span class="p-name">Review 3</span>
            <span class="p-rating">3.5</span>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 3
    assert reviews[0].get("name") == "Review 1"
    assert reviews[1].get("name") == "Review 2"
    assert reviews[2].get("name") == "Review 3"
    assert reviews[0]["rating"] == 4.0
    assert reviews[1]["rating"] == 5.0
    assert reviews[2]["rating"] == 3.5


# =============================================================================
# CATEGORY 7: Additional Edge Cases (2 tests)
# =============================================================================


def test_hreview_empty_page():
    """Test page with no reviews"""
    html = "<html><body><p>No reviews here</p></body></html>"
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 0


def test_hreview_with_html_content():
    """Test review with HTML content in e-content"""
    html = """
        <div class="h-review">
            <span class="p-name">Detailed review</span>
            <div class="e-content">
                <h3>Pros:</h3>
                <ul>
                    <li>Fast performance</li>
                    <li>Great battery life</li>
                </ul>
                <h3>Cons:</h3>
                <ul>
                    <li>Expensive</li>
                </ul>
            </div>
            <span class="p-rating">4</span>
        </div>
    """
    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    content = reviews[0].get("content", "")
    assert "Pros:" in content
    assert "Cons:" in content
    assert "<li>" in content or "Fast performance" in content


# =============================================================================
# CATEGORY 8: Integration Test (1 test)
# =============================================================================


def test_hreview_in_extract_all():
    """Test that h-review is included in extract_all() results"""
    html = """
        <html>
        <head>
            <title>Review Page</title>
        </head>
        <body>
            <div class="h-review">
                <span class="p-name">Great product!</span>
                <span class="p-rating">5</span>
                <span class="p-item">Widget Pro</span>
            </div>
        </body>
        </html>
    """
    result = meta_oxide.extract_all(html)
    assert "microformats" in result
    microformats = result["microformats"]
    assert "h-review" in microformats
    reviews = microformats["h-review"]
    assert len(reviews) == 1
    assert reviews[0].get("name") == "Great product!"
    assert reviews[0]["rating"] == 5.0
