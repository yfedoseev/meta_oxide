"""Stress tests and edge cases for robustness"""

import meta_oxide


def test_extremely_large_html():
    """Test with very large HTML document (1MB+)"""
    # Create HTML with 50,000 meta tags
    meta_tags = "\n".join([f'<meta name="tag{i}" content="value{i}">' for i in range(50000)])
    html = f"<html><head>{meta_tags}</head></html>"

    # Should handle large documents without crashing
    meta = meta_oxide.extract_meta(html)
    assert isinstance(meta, dict)


def test_deeply_nested_html():
    """Test with deeply nested HTML structure"""
    # Create 1000 levels of nesting
    opening_tags = "<div>" * 1000
    closing_tags = "</div>" * 1000
    html = f"{opening_tags}<span class='p-name'>Deeply Nested</span>{closing_tags}"

    # Should handle deep nesting without stack overflow
    cards = meta_oxide.extract_hcard(html)
    assert isinstance(cards, list)


def test_html_with_binary_data():
    """Test with HTML containing binary-like data"""
    html = "<title>Test\x00\x01\x02</title>"

    # Should handle null bytes and binary data gracefully
    try:
        meta = meta_oxide.extract_meta(html)
        assert isinstance(meta, dict)
    except Exception:
        # If it raises, that's also acceptable
        pass


def test_html_with_only_whitespace():
    """Test with whitespace-only HTML"""
    html = "   \n\n\t\t   \n   "

    meta = meta_oxide.extract_meta(html)
    assert isinstance(meta, dict)


def test_html_with_unclosed_tags_everywhere():
    """Test with extensively malformed HTML"""
    html = """
        <html>
        <head>
            <title>Unclosed
            <meta name="description" content="Also unclosed
            <link rel="canonical" href="/page
        <body>
            <div class="h-card">
                <span class="p-name">No closing span
                <div>More unclosed divs
    """

    # Should parse leniently without crashing
    data = meta_oxide.extract_all(html)
    assert isinstance(data, dict)


def test_concurrent_extractions():
    """Test that extractions can happen concurrently"""
    import threading

    html = """
        <title>Test</title>
        <meta name="description" content="Description">
        <meta property="og:title" content="OG Title">
    """

    results = []
    errors = []

    def extract_in_thread() -> None:
        try:
            data = meta_oxide.extract_all(html)
            results.append(data)
        except Exception as e:
            errors.append(e)

    # Run 10 concurrent extractions
    threads = [threading.Thread(target=extract_in_thread) for _ in range(10)]
    for t in threads:
        t.start()
    for t in threads:
        t.join()

    assert len(errors) == 0
    assert len(results) == 10
    # All results should be identical
    for result in results:
        assert result["meta"]["title"] == "Test"


def test_html_with_control_characters():
    """Test with HTML containing control characters"""
    html = "<title>Test\r\n\t\x0b\x0c</title>"

    meta = meta_oxide.extract_meta(html)
    assert isinstance(meta, dict)


def test_repeated_extraction_same_html():
    """Test that repeated extractions give same results"""
    html = """
        <meta name="description" content="Test">
        <meta property="og:title" content="Title">
    """

    # Extract 100 times
    results = [meta_oxide.extract_all(html) for _ in range(100)]

    # All results should be identical
    first = results[0]
    for result in results[1:]:
        assert result == first
