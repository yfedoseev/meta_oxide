"""Tests for h-feed microformat extraction"""

import meta_oxide


def test_extract_hfeed_basic():
    """Test basic h-feed extraction"""
    html = """
        <div class="h-feed">
            <span class="p-name">My Blog</span>
            <span class="p-author">John Doe</span>
        </div>
    """
    feeds = meta_oxide.extract_hfeed(html)
    assert len(feeds) == 1
    assert feeds[0]["name"] == "My Blog"
    assert feeds[0]["author"] == "John Doe"


def test_hfeed_with_url():
    """Test h-feed with URL"""
    html = """
        <div class="h-feed">
            <span class="p-name">Tech News</span>
            <a class="u-url" href="https://example.com/feed">Feed URL</a>
        </div>
    """
    feeds = meta_oxide.extract_hfeed(html)
    assert len(feeds) == 1
    assert feeds[0]["name"] == "Tech News"
    assert feeds[0]["url"] == "https://example.com/feed"


def test_hfeed_with_photo():
    """Test h-feed with photo"""
    html = """
        <div class="h-feed">
            <span class="p-name">Photo Blog</span>
            <img class="u-photo" src="https://example.com/logo.jpg" alt="Logo" />
        </div>
    """
    feeds = meta_oxide.extract_hfeed(html)
    assert len(feeds) == 1
    assert feeds[0]["photo"] == "https://example.com/logo.jpg"


def test_hfeed_complete():
    """Test h-feed with all properties"""
    html = """
        <div class="h-feed">
            <h1 class="p-name">The Daily Post</h1>
            <span class="p-author">Jane Smith</span>
            <a class="u-url" href="https://example.com/blog">Blog</a>
            <img class="u-photo" src="https://example.com/blog-logo.png" alt="Logo" />
        </div>
    """
    feeds = meta_oxide.extract_hfeed(html)
    assert len(feeds) == 1
    feed = feeds[0]
    assert feed["name"] == "The Daily Post"
    assert feed["author"] == "Jane Smith"
    assert feed["url"] == "https://example.com/blog"
    assert feed["photo"] == "https://example.com/blog-logo.png"


def test_multiple_hfeeds():
    """Test extraction of multiple feeds"""
    html = """
        <div class="h-feed">
            <span class="p-name">Feed 1</span>
        </div>
        <div class="h-feed">
            <span class="p-name">Feed 2</span>
        </div>
    """
    feeds = meta_oxide.extract_hfeed(html)
    assert len(feeds) == 2
    assert feeds[0]["name"] == "Feed 1"
    assert feeds[1]["name"] == "Feed 2"


def test_hfeed_empty():
    """Test page with no feeds"""
    html = "<html><body><p>No feeds here</p></body></html>"
    feeds = meta_oxide.extract_hfeed(html)
    assert len(feeds) == 0


def test_hfeed_minimal():
    """Test h-feed with minimal properties"""
    html = """
        <div class="h-feed">
            <span class="p-name">Minimal Feed</span>
        </div>
    """
    feeds = meta_oxide.extract_hfeed(html)
    assert len(feeds) == 1
    assert feeds[0]["name"] == "Minimal Feed"
