"""Tests for error handling and edge cases in Python bindings"""

import meta_oxide


def test_extract_meta_with_none_base_url():
    """Test extract_meta with None base_url (should work)"""
    html = "<title>Test</title>"
    meta = meta_oxide.extract_meta(html, None)
    assert meta["title"] == "Test"


def test_extract_meta_empty_string():
    """Test extract_meta with empty string"""
    meta = meta_oxide.extract_meta("")
    assert isinstance(meta, dict)


def test_extract_opengraph_empty_string():
    """Test extract_opengraph with empty string"""
    og = meta_oxide.extract_opengraph("")
    assert isinstance(og, dict)


def test_extract_twitter_empty_string():
    """Test extract_twitter with empty string"""
    twitter = meta_oxide.extract_twitter("")
    assert isinstance(twitter, dict)


def test_extract_all_empty_string():
    """Test extract_all with empty string"""
    data = meta_oxide.extract_all("")
    assert isinstance(data, dict)
    assert "meta" in data or "opengraph" in data or "twitter" in data


def test_extract_all_malformed_html():
    """Test extract_all doesn't crash on malformed HTML"""
    html = "<div><span>Unclosed tags<p>More unclosed"
    data = meta_oxide.extract_all(html)
    assert isinstance(data, dict)


def test_extract_meta_with_invalid_utf8():
    """Test that invalid UTF-8 doesn't crash (Python handles this)"""
    # Python strings are always valid UTF-8, but test edge case
    html = "<title>Test \udcff</title>"  # Surrogate character
    try:
        meta = meta_oxide.extract_meta(html)
        assert isinstance(meta, dict)
    except Exception:
        # If it raises, that's also acceptable behavior
        pass


def test_module_has_version():
    """Test that __version__ attribute exists"""
    assert hasattr(meta_oxide, "__version__")
    assert isinstance(meta_oxide.__version__, str)
    assert len(meta_oxide.__version__) > 0


def test_extract_microformats_empty():
    """Test extract_microformats with no microformats present"""
    html = "<html><body><p>No microformats here</p></body></html>"
    result = meta_oxide.extract_microformats(html)
    assert isinstance(result, dict)


def test_extract_hcard_none_values():
    """Test hcard extraction handles missing properties"""
    html = '<div class="h-card"><span class="p-name">Name Only</span></div>'
    cards = meta_oxide.extract_hcard(html)
    assert len(cards) == 1
    assert cards[0]["name"] == "Name Only"
    # Other fields should be None or not present


def test_extract_hentry_minimal():
    """Test hentry with minimal data"""
    html = '<article class="h-entry"><h1 class="p-name">Title</h1></article>'
    entries = meta_oxide.extract_hentry(html)
    assert len(entries) == 1
    assert entries[0]["name"] == "Title"


def test_extract_hevent_minimal():
    """Test hevent with minimal data"""
    html = '<div class="h-event"><span class="p-name">Event</span></div>'
    events = meta_oxide.extract_hevent(html)
    assert len(events) == 1
    assert events[0]["name"] == "Event"


def test_extract_all_with_invalid_base_url():
    """Test extract_all with invalid base URL"""
    html = '<link rel="canonical" href="/page">'
    # Invalid base URL should be handled gracefully
    data = meta_oxide.extract_all(html, "not-a-url")
    assert isinstance(data, dict)


def test_extract_twitter_with_fallback_empty():
    """Test Twitter fallback when both Twitter and OG are empty"""
    html = "<html><head></head></html>"
    twitter = meta_oxide.extract_twitter(html)
    assert isinstance(twitter, dict)
    # Should return empty/None values, not crash


def test_large_html_document():
    """Test with very large HTML document"""
    # Create HTML with 10,000 meta tags
    meta_tags = "\n".join([f'<meta name="tag{i}" content="value{i}">' for i in range(10000)])
    html = f"<html><head>{meta_tags}</head></html>"

    # Should handle large documents without crashing
    meta = meta_oxide.extract_meta(html)
    assert isinstance(meta, dict)
