"""Tests for MetaTags type conversions to Python dictionaries"""

import meta_oxide


def test_meta_tags_all_fields_present():
    """Test that all MetaTags fields are properly converted"""
    html = """
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Complete Test Page</title>
            <meta name="description" content="Full description">
            <meta name="keywords" content="key1, key2, key3">
            <meta name="author" content="Test Author">
            <meta name="generator" content="Test Gen">
            <meta name="application-name" content="Test App">
            <meta name="referrer" content="no-referrer">
            <meta name="robots" content="index, follow, noarchive">
            <meta name="googlebot" content="nosnippet">
            <meta name="viewport" content="width=device-width">
            <meta name="theme-color" content="#ffffff">
            <link rel="canonical" href="https://example.com/page">
            <link rel="shortlink" href="https://example.com/?p=123">
            <link rel="alternate" href="https://example.com/es" hreflang="es">
            <link rel="alternate" type="application/rss+xml" href="/feed.xml" title="RSS">
        </head>
        </html>
    """

    meta = meta_oxide.extract_meta(html, "https://example.com")

    # Test all basic fields are present and correct type
    assert isinstance(meta, dict)
    assert meta["title"] == "Complete Test Page"
    assert meta["description"] == "Full description"
    assert isinstance(meta["keywords"], list)
    assert len(meta["keywords"]) == 3
    assert meta["keywords"][0] == "key1"
    assert meta["author"] == "Test Author"
    assert meta["generator"] == "Test Gen"
    assert meta["application_name"] == "Test App"
    assert meta["referrer"] == "no-referrer"
    assert meta["viewport"] == "width=device-width"
    assert meta["theme_color"] == "#ffffff"
    assert meta["charset"] == "UTF-8"
    assert meta["language"] == "en"
    assert meta["canonical"] == "https://example.com/page"
    assert meta["shortlink"] == "https://example.com/?p=123"

    # Test nested dictionaries
    assert "robots" in meta
    assert isinstance(meta["robots"], dict)
    assert meta["robots"]["index"] is True
    assert meta["robots"]["follow"] is True
    assert meta["robots"]["archive"] is False
    assert "raw" in meta["robots"]

    assert "googlebot" in meta
    assert isinstance(meta["googlebot"], dict)
    assert meta["googlebot"]["snippet"] is False

    # Test lists of complex types
    assert "alternate" in meta
    assert isinstance(meta["alternate"], list)
    assert len(meta["alternate"]) == 1
    assert isinstance(meta["alternate"][0], dict)
    assert meta["alternate"][0]["href"] == "https://example.com/es"
    assert meta["alternate"][0]["hreflang"] == "es"

    assert "feeds" in meta
    assert isinstance(meta["feeds"], list)
    assert len(meta["feeds"]) == 1
    assert isinstance(meta["feeds"][0], dict)
    assert meta["feeds"][0]["type"] == "application/rss+xml"
    assert meta["feeds"][0]["title"] == "RSS"


def test_robots_directive_all_directives():
    """Test RobotsDirective parsing all possible directives"""
    html_tests = [
        ('<meta name="robots" content="index, follow">', {"index": True, "follow": True}),
        ('<meta name="robots" content="noindex, nofollow">', {"index": False, "follow": False}),
        ('<meta name="robots" content="all">', {"index": True, "follow": True}),
        ('<meta name="robots" content="none">', {"index": False, "follow": False}),
        ('<meta name="robots" content="noarchive">', {"archive": False}),
        ('<meta name="robots" content="nosnippet">', {"snippet": False}),
        ('<meta name="robots" content="notranslate">', {"translate": False}),
        ('<meta name="robots" content="noimageindex">', {"imageindex": False}),
    ]

    for html, expected_fields in html_tests:
        meta = meta_oxide.extract_meta(html)
        assert "robots" in meta
        robots = meta["robots"]
        assert isinstance(robots, dict)
        assert "raw" in robots

        for field, value in expected_fields.items():
            assert field in robots
            assert robots[field] == value


def test_alternate_link_all_fields():
    """Test AlternateLink with all possible fields"""
    html = """
        <link rel="alternate"
              href="https://example.com/mobile"
              hreflang="en"
              media="only screen and (max-width: 640px)"
              type="text/html">
    """
    meta = meta_oxide.extract_meta(html)

    assert "alternate" in meta
    assert len(meta["alternate"]) == 1
    alt = meta["alternate"][0]

    assert isinstance(alt, dict)
    assert alt["href"] == "https://example.com/mobile"
    assert alt["hreflang"] == "en"
    assert alt["media"] == "only screen and (max-width: 640px)"
    assert alt["type"] == "text/html"


def test_feed_link_all_fields():
    """Test FeedLink with all fields"""
    html = '<link rel="alternate" type="application/rss+xml" href="/feed.xml" title="My RSS Feed">'
    meta = meta_oxide.extract_meta(html, "https://example.com")

    assert "feeds" in meta
    assert len(meta["feeds"]) == 1
    feed = meta["feeds"][0]

    assert isinstance(feed, dict)
    assert feed["href"] == "https://example.com/feed.xml"
    assert feed["type"] == "application/rss+xml"
    assert feed["title"] == "My RSS Feed"


def test_meta_tags_with_none_values():
    """Test MetaTags with mostly None values"""
    html = "<title>Only Title</title>"
    meta = meta_oxide.extract_meta(html)

    assert isinstance(meta, dict)
    assert meta["title"] == "Only Title"
    # Other fields should not be present or should handle None gracefully
    assert "description" not in meta or meta.get("description") is None


def test_meta_tags_empty_keywords():
    """Test keywords field when empty"""
    html = '<meta name="keywords" content="">'
    meta = meta_oxide.extract_meta(html)
    # Empty keywords should not be present or should be empty list
    assert "keywords" not in meta or meta.get("keywords") == [] or meta.get("keywords") is None


def test_multiple_alternate_links():
    """Test multiple alternate links conversion"""
    html = """
        <link rel="alternate" href="/es" hreflang="es">
        <link rel="alternate" href="/fr" hreflang="fr">
        <link rel="alternate" href="/de" hreflang="de">
    """
    meta = meta_oxide.extract_meta(html, "https://example.com")

    assert "alternate" in meta
    assert len(meta["alternate"]) == 3

    hreflangs = [alt["hreflang"] for alt in meta["alternate"]]
    assert "es" in hreflangs
    assert "fr" in hreflangs
    assert "de" in hreflangs


def test_multiple_feeds():
    """Test multiple feed links conversion"""
    html = """
        <link rel="alternate" type="application/rss+xml" href="/rss.xml" title="RSS">
        <link rel="alternate" type="application/atom+xml" href="/atom.xml" title="Atom">
    """
    meta = meta_oxide.extract_meta(html, "https://example.com")

    assert "feeds" in meta
    assert len(meta["feeds"]) == 2

    types = [feed["type"] for feed in meta["feeds"]]
    assert "application/rss+xml" in types
    assert "application/atom+xml" in types
