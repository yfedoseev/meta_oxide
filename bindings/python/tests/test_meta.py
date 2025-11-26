"""Tests for standard meta tag extraction (Phase 1)"""

import meta_oxide


def test_basic_title():
    """Test extraction of title tag"""
    html = """
        <!DOCTYPE html>
        <html>
        <head>
            <title>Test Page</title>
        </head>
        </html>
    """
    meta = meta_oxide.extract_meta(html)
    assert meta["title"] == "Test Page"


def test_description():
    """Test extraction of meta description"""
    html = '<meta name="description" content="A test description">'
    meta = meta_oxide.extract_meta(html)
    assert meta["description"] == "A test description"


def test_keywords():
    """Test extraction of keywords"""
    html = '<meta name="keywords" content="rust, python, metadata">'
    meta = meta_oxide.extract_meta(html)
    assert "keywords" in meta
    assert meta["keywords"] == ["rust", "python", "metadata"]


def test_author():
    """Test extraction of author"""
    html = '<meta name="author" content="John Doe">'
    meta = meta_oxide.extract_meta(html)
    assert meta["author"] == "John Doe"


def test_generator():
    """Test extraction of generator"""
    html = '<meta name="generator" content="WordPress 6.0">'
    meta = meta_oxide.extract_meta(html)
    assert meta["generator"] == "WordPress 6.0"


def test_canonical():
    """Test extraction of canonical URL"""
    html = '<link rel="canonical" href="https://example.com/page">'
    meta = meta_oxide.extract_meta(html)
    assert meta["canonical"] == "https://example.com/page"


def test_canonical_relative():
    """Test resolution of relative canonical URL"""
    html = '<link rel="canonical" href="/page">'
    meta = meta_oxide.extract_meta(html, base_url="https://example.com")
    assert meta["canonical"] == "https://example.com/page"


def test_robots_directive():
    """Test robots meta tag parsing"""
    html = '<meta name="robots" content="index, follow">'
    meta = meta_oxide.extract_meta(html)
    assert "robots" in meta
    assert meta["robots"]["index"] is True
    assert meta["robots"]["follow"] is True
    assert meta["robots"]["raw"] == "index, follow"


def test_robots_noindex_nofollow():
    """Test robots noindex, nofollow"""
    html = '<meta name="robots" content="noindex, nofollow">'
    meta = meta_oxide.extract_meta(html)
    assert "robots" in meta
    assert meta["robots"]["index"] is False
    assert meta["robots"]["follow"] is False


def test_googlebot():
    """Test googlebot specific directives"""
    html = '<meta name="googlebot" content="nosnippet, notranslate">'
    meta = meta_oxide.extract_meta(html)
    assert "googlebot" in meta
    assert meta["googlebot"]["snippet"] is False
    assert meta["googlebot"]["translate"] is False


def test_rss_feed():
    """Test RSS feed link extraction"""
    html = '<link rel="alternate" type="application/rss+xml" href="/feed.xml" title="RSS">'
    meta = meta_oxide.extract_meta(html, base_url="https://example.com")
    assert "feeds" in meta
    assert len(meta["feeds"]) == 1
    assert meta["feeds"][0]["href"] == "https://example.com/feed.xml"
    assert meta["feeds"][0]["type"] == "application/rss+xml"
    assert meta["feeds"][0]["title"] == "RSS"


def test_atom_feed():
    """Test Atom feed link extraction"""
    html = '<link rel="alternate" type="application/atom+xml" href="/atom.xml">'
    meta = meta_oxide.extract_meta(html, base_url="https://example.com")
    assert "feeds" in meta
    assert len(meta["feeds"]) == 1
    assert meta["feeds"][0]["type"] == "application/atom+xml"


def test_alternate_link():
    """Test alternate link extraction"""
    html = '<link rel="alternate" href="https://example.com/es" hreflang="es">'
    meta = meta_oxide.extract_meta(html)
    assert "alternate" in meta
    assert len(meta["alternate"]) == 1
    assert meta["alternate"][0]["href"] == "https://example.com/es"
    assert meta["alternate"][0]["hreflang"] == "es"


def test_viewport():
    """Test viewport meta tag"""
    html = '<meta name="viewport" content="width=device-width, initial-scale=1.0">'
    meta = meta_oxide.extract_meta(html)
    assert meta["viewport"] == "width=device-width, initial-scale=1.0"


def test_theme_color():
    """Test theme-color meta tag"""
    html = '<meta name="theme-color" content="#ff0000">'
    meta = meta_oxide.extract_meta(html)
    assert meta["theme_color"] == "#ff0000"


def test_charset():
    """Test charset extraction"""
    html = '<meta charset="UTF-8">'
    meta = meta_oxide.extract_meta(html)
    assert meta["charset"] == "UTF-8"


def test_charset_http_equiv():
    """Test charset from Content-Type"""
    html = '<meta http-equiv="Content-Type" content="text/html; charset=UTF-8">'
    meta = meta_oxide.extract_meta(html)
    assert meta["charset"] == "UTF-8"


def test_language():
    """Test language attribute"""
    html = '<html lang="en-US"></html>'
    meta = meta_oxide.extract_meta(html)
    assert meta["language"] == "en-US"


def test_application_name():
    """Test application-name meta tag"""
    html = '<meta name="application-name" content="MyApp">'
    meta = meta_oxide.extract_meta(html)
    assert meta["application_name"] == "MyApp"


def test_referrer():
    """Test referrer meta tag"""
    html = '<meta name="referrer" content="no-referrer">'
    meta = meta_oxide.extract_meta(html)
    assert meta["referrer"] == "no-referrer"


def test_shortlink():
    """Test shortlink extraction"""
    html = '<link rel="shortlink" href="https://example.com/?p=123">'
    meta = meta_oxide.extract_meta(html)
    assert meta["shortlink"] == "https://example.com/?p=123"


def test_empty_html():
    """Test with empty HTML"""
    html = ""
    meta = meta_oxide.extract_meta(html)
    assert isinstance(meta, dict)
    # Should return empty dict or default values


def test_no_meta_tags():
    """Test with HTML but no meta tags"""
    html = "<html><body><p>No meta tags</p></body></html>"
    meta = meta_oxide.extract_meta(html)
    assert isinstance(meta, dict)


def test_whitespace_trimming():
    """Test that whitespace is trimmed"""
    html = '<meta name="description" content="  Lots of whitespace  ">'
    meta = meta_oxide.extract_meta(html)
    assert meta["description"] == "Lots of whitespace"


def test_case_insensitive_names():
    """Test case-insensitive meta names"""
    html = '<meta name="DESCRIPTION" content="Test">'
    meta = meta_oxide.extract_meta(html)
    assert meta["description"] == "Test"


def test_multiple_feeds():
    """Test multiple feeds (RSS + Atom)"""
    html = """
        <link rel="alternate" type="application/rss+xml" href="/rss.xml" title="RSS">
        <link rel="alternate" type="application/atom+xml" href="/atom.xml" title="Atom">
    """
    meta = meta_oxide.extract_meta(html, base_url="https://example.com")
    assert "feeds" in meta
    assert len(meta["feeds"]) == 2
    assert meta["feeds"][0]["type"] == "application/rss+xml"
    assert meta["feeds"][1]["type"] == "application/atom+xml"


def test_multiple_alternate_links():
    """Test multiple alternate links for different languages"""
    html = """
        <link rel="alternate" hreflang="es" href="https://example.com/es">
        <link rel="alternate" hreflang="fr" href="https://example.com/fr">
        <link rel="alternate" hreflang="de" href="https://example.com/de">
    """
    meta = meta_oxide.extract_meta(html)
    assert "alternate" in meta
    assert len(meta["alternate"]) == 3
    assert meta["alternate"][0]["hreflang"] == "es"
    assert meta["alternate"][1]["hreflang"] == "fr"
    assert meta["alternate"][2]["hreflang"] == "de"


def test_complex_real_world():
    """Test with complex real-world HTML"""
    html = """
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>My Blog Post</title>
            <meta name="description" content="A fascinating blog post about Rust">
            <meta name="keywords" content="rust, programming, tutorial">
            <meta name="author" content="Jane Developer">
            <meta name="robots" content="index, follow">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <link rel="canonical" href="https://blog.example.com/post">
            <link rel="alternate" type="application/rss+xml" href="/feed.xml">
            <link rel="alternate" hreflang="es" href="https://blog.example.com/es/post">
        </head>
        </html>
    """
    meta = meta_oxide.extract_meta(html, base_url="https://blog.example.com")

    assert meta["title"] == "My Blog Post"
    assert meta["description"] == "A fascinating blog post about Rust"
    assert "keywords" in meta
    assert len(meta["keywords"]) == 3
    assert meta["author"] == "Jane Developer"
    assert meta["canonical"] == "https://blog.example.com/post"
    assert "robots" in meta
    assert meta["robots"]["index"] is True
    assert "feeds" in meta
    assert len(meta["feeds"]) == 1
    assert "alternate" in meta
    assert len(meta["alternate"]) == 1


def test_wordpress_site():
    """Test with WordPress site structure"""
    html = """
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>My WordPress Blog</title>
            <meta name="description" content="A blog about Rust and Python">
            <meta name="robots" content="index, follow">
            <meta name="generator" content="WordPress 6.0">
            <link rel="canonical" href="https://example.com/blog">
            <link rel="alternate" type="application/rss+xml" title="RSS" href="/feed">
            <meta name="viewport" content="width=device-width, initial-scale=1">
        </head>
        </html>
    """
    meta = meta_oxide.extract_meta(html, base_url="https://example.com")

    assert meta["title"] == "My WordPress Blog"
    assert meta["description"] == "A blog about Rust and Python"
    assert meta["charset"] == "UTF-8"
    assert meta["language"] == "en"
    assert meta["generator"] == "WordPress 6.0"
    assert meta["canonical"] == "https://example.com/blog"
    assert len(meta["feeds"]) == 1


def test_e_commerce_site():
    """Test with e-commerce site structure"""
    html = """
        <html>
        <head>
            <meta charset="utf-8">
            <title>Product Name - Shop</title>
            <meta name="description" content="Buy Product Name at great prices">
            <link rel="canonical" href="https://shop.example.com/product/123">
            <meta name="robots" content="index, follow, noarchive">
            <link rel="alternate" hreflang="es" href="https://shop.example.com/es/product/123">
            <link rel="alternate" hreflang="fr" href="https://shop.example.com/fr/product/123">
        </head>
        </html>
    """
    meta = meta_oxide.extract_meta(html)

    assert meta["title"] == "Product Name - Shop"
    assert len(meta["alternate"]) == 2
    assert "robots" in meta
    assert meta["robots"]["archive"] is False


def test_international_characters():
    """Test with international characters"""
    html = """
        <meta name="description" content="Это тест на русском языке">
        <meta name="author" content="José García">
    """
    meta = meta_oxide.extract_meta(html)
    assert meta["description"] == "Это тест на русском языке"
    assert meta["author"] == "José García"


def test_mobile_app_meta():
    """Test mobile app specific meta tags"""
    html = """
        <meta name="application-name" content="MyMobileApp">
        <meta name="theme-color" content="#1a1a1a">
        <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=5">
    """
    meta = meta_oxide.extract_meta(html)
    assert meta["application_name"] == "MyMobileApp"
    assert meta["theme_color"] == "#1a1a1a"
    assert "viewport" in meta


def test_robots_all_directive():
    """Test robots 'all' directive"""
    html = '<meta name="robots" content="all">'
    meta = meta_oxide.extract_meta(html)
    assert meta["robots"]["index"] is True
    assert meta["robots"]["follow"] is True


def test_robots_none_directive():
    """Test robots 'none' directive"""
    html = '<meta name="robots" content="none">'
    meta = meta_oxide.extract_meta(html)
    assert meta["robots"]["index"] is False
    assert meta["robots"]["follow"] is False


def test_multiple_canonical_uses_first():
    """Test that first canonical link is used"""
    html = """
        <link rel="canonical" href="https://example.com/first">
        <link rel="canonical" href="https://example.com/second">
    """
    meta = meta_oxide.extract_meta(html)
    assert meta["canonical"] == "https://example.com/first"


def test_empty_content_ignored():
    """Test that empty content is ignored"""
    html = '<meta name="description" content="">'
    meta = meta_oxide.extract_meta(html)
    assert "description" not in meta


def test_keywords_empty_items_filtered():
    """Test that empty keyword items are filtered"""
    html = '<meta name="keywords" content="rust, , python,  , metadata">'
    meta = meta_oxide.extract_meta(html)
    assert meta["keywords"] == ["rust", "python", "metadata"]


def test_alternate_with_media_query():
    """Test alternate link with media query"""
    html = '<link rel="alternate" media="only screen and (max-width: 640px)" href="https://m.example.com">'
    meta = meta_oxide.extract_meta(html)
    assert "alternate" in meta
    assert len(meta["alternate"]) == 1
    assert meta["alternate"][0]["media"] == "only screen and (max-width: 640px)"


def test_feed_without_title():
    """Test feed without title attribute"""
    html = '<link rel="alternate" type="application/rss+xml" href="/feed.xml">'
    meta = meta_oxide.extract_meta(html, base_url="https://example.com")
    assert len(meta["feeds"]) == 1
    assert "title" not in meta["feeds"][0]


def test_referrer_policies():
    """Test various referrer policies"""
    policies = [
        "no-referrer",
        "no-referrer-when-downgrade",
        "origin",
        "origin-when-cross-origin",
        "same-origin",
        "strict-origin",
    ]

    for policy in policies:
        html = f'<meta name="referrer" content="{policy}">'
        meta = meta_oxide.extract_meta(html)
        assert meta["referrer"] == policy


def test_relative_canonical_with_path():
    """Test relative canonical URL with path resolution"""
    html = '<link rel="canonical" href="../other/page.html">'
    meta = meta_oxide.extract_meta(html, base_url="https://example.com/current/page")
    assert meta["canonical"] == "https://example.com/other/page.html"


def test_complex_robots_directives():
    """Test complex robots directives"""
    html = '<meta name="robots" content="noindex, follow, noarchive, nosnippet, notranslate">'
    meta = meta_oxide.extract_meta(html)
    robots = meta["robots"]
    assert robots["index"] is False
    assert robots["follow"] is True
    assert robots["archive"] is False
    assert robots["snippet"] is False
    assert robots["translate"] is False


# Phase 6: Site Verification Tests
def test_google_site_verification():
    """Test Google Search Console verification tag"""
    html = '<meta name="google-site-verification" content="abc123xyz456def789">'
    meta = meta_oxide.extract_meta(html)
    assert meta["google_site_verification"] == "abc123xyz456def789"


def test_bing_verification():
    """Test Bing Webmaster Tools verification tag"""
    html = '<meta name="msvalidate.01" content="BING123456789">'
    meta = meta_oxide.extract_meta(html)
    assert meta["msvalidate_01"] == "BING123456789"


def test_yandex_verification():
    """Test Yandex Webmaster verification tag"""
    html = '<meta name="yandex-verification" content="yandex123456">'
    meta = meta_oxide.extract_meta(html)
    assert meta["yandex_verification"] == "yandex123456"


def test_pinterest_verification():
    """Test Pinterest domain verification tag"""
    html = '<meta name="p:domain_verify" content="pinterest123">'
    meta = meta_oxide.extract_meta(html)
    assert meta["p_domain_verify"] == "pinterest123"


def test_facebook_domain_verification():
    """Test Facebook Business Manager domain verification"""
    html = '<meta name="facebook-domain-verification" content="fb123456789">'
    meta = meta_oxide.extract_meta(html)
    assert meta["facebook_domain_verification"] == "fb123456789"


def test_multiple_verification_tags():
    """Test extraction of multiple verification tags at once"""
    html = """
        <html>
        <head>
            <meta name="google-site-verification" content="google123">
            <meta name="msvalidate.01" content="bing456">
            <meta name="yandex-verification" content="yandex789">
            <meta name="p:domain_verify" content="pinterest012">
            <meta name="facebook-domain-verification" content="fb345">
        </head>
        </html>
    """
    meta = meta_oxide.extract_meta(html)
    assert meta["google_site_verification"] == "google123"
    assert meta["msvalidate_01"] == "bing456"
    assert meta["yandex_verification"] == "yandex789"
    assert meta["p_domain_verify"] == "pinterest012"
    assert meta["facebook_domain_verification"] == "fb345"


# Phase 8: Apple Mobile Meta Tests
def test_apple_mobile_web_app_capable():
    """Test Apple mobile web app capable tag"""
    html = '<meta name="apple-mobile-web-app-capable" content="yes">'
    meta = meta_oxide.extract_meta(html)
    assert meta["apple_mobile_web_app_capable"] == "yes"


def test_apple_mobile_web_app_status_bar_style():
    """Test Apple mobile status bar style tag"""
    html = '<meta name="apple-mobile-web-app-status-bar-style" content="black-translucent">'
    meta = meta_oxide.extract_meta(html)
    assert meta["apple_mobile_web_app_status_bar_style"] == "black-translucent"


def test_apple_mobile_web_app_title():
    """Test Apple mobile web app title tag"""
    html = '<meta name="apple-mobile-web-app-title" content="My App">'
    meta = meta_oxide.extract_meta(html)
    assert meta["apple_mobile_web_app_title"] == "My App"


def test_all_apple_mobile_tags():
    """Test extraction of all Apple mobile tags at once"""
    html = """
        <html>
        <head>
            <meta name="apple-mobile-web-app-capable" content="yes">
            <meta name="apple-mobile-web-app-status-bar-style" content="black">
            <meta name="apple-mobile-web-app-title" content="My PWA">
        </head>
        </html>
    """
    meta = meta_oxide.extract_meta(html)
    assert meta["apple_mobile_web_app_capable"] == "yes"
    assert meta["apple_mobile_web_app_status_bar_style"] == "black"
    assert meta["apple_mobile_web_app_title"] == "My PWA"


def test_msapplication_tile_color():
    """Test Microsoft tile color for Windows start screen"""
    html = '<meta name="msapplication-TileColor" content="#da532c">'
    meta = meta_oxide.extract_meta(html)
    assert meta["msapplication_tile_color"] == "#da532c"


def test_msapplication_tile_image():
    """Test Microsoft tile image for Windows start screen"""
    html = '<meta name="msapplication-TileImage" content="/mstile-144x144.png">'
    meta = meta_oxide.extract_meta(html)
    assert meta["msapplication_tile_image"] == "/mstile-144x144.png"


def test_msapplication_config():
    """Test Microsoft browserconfig.xml URL"""
    html = '<meta name="msapplication-config" content="/browserconfig.xml">'
    meta = meta_oxide.extract_meta(html)
    assert meta["msapplication_config"] == "/browserconfig.xml"


def test_all_microsoft_meta_tags():
    """Test extraction of all Microsoft meta tags at once"""
    html = """
        <html>
        <head>
            <meta name="msapplication-TileColor" content="#2b5797">
            <meta name="msapplication-TileImage" content="/mstile.png">
            <meta name="msapplication-config" content="/config.xml">
        </head>
        </html>
    """
    meta = meta_oxide.extract_meta(html)
    assert meta["msapplication_tile_color"] == "#2b5797"
    assert meta["msapplication_tile_image"] == "/mstile.png"
    assert meta["msapplication_config"] == "/config.xml"


def test_link_icon():
    """Test favicon link extraction"""
    html = '<link rel="icon" href="/favicon.ico">'
    meta = meta_oxide.extract_meta(html)
    assert meta["icon"] == "/favicon.ico"


def test_link_apple_touch_icon():
    """Test Apple touch icon link extraction"""
    html = '<link rel="apple-touch-icon" href="/apple-touch-icon.png">'
    meta = meta_oxide.extract_meta(html)
    assert meta["apple_touch_icon"] == "/apple-touch-icon.png"


def test_link_manifest():
    """Test PWA manifest link extraction"""
    html = '<link rel="manifest" href="/manifest.json">'
    meta = meta_oxide.extract_meta(html)
    assert meta["manifest"] == "/manifest.json"


def test_link_prev_next():
    """Test pagination prev/next links"""
    html = """
        <html>
        <head>
            <link rel="prev" href="/page/1">
            <link rel="next" href="/page/3">
        </head>
        </html>
    """
    meta = meta_oxide.extract_meta(html)
    assert meta["prev"] == "/page/1"
    assert meta["next"] == "/page/3"


def test_all_additional_links():
    """Test extraction of all additional link types at once"""
    html = """
        <html>
        <head>
            <link rel="icon" href="/favicon.ico">
            <link rel="apple-touch-icon" href="/icon.png">
            <link rel="manifest" href="/manifest.webmanifest">
            <link rel="prev" href="/previous">
            <link rel="next" href="/following">
        </head>
        </html>
    """
    meta = meta_oxide.extract_meta(html)
    assert meta["icon"] == "/favicon.ico"
    assert meta["apple_touch_icon"] == "/icon.png"
    assert meta["manifest"] == "/manifest.webmanifest"
    assert meta["prev"] == "/previous"
    assert meta["next"] == "/following"


def test_link_url_resolution():
    """Test that relative URLs in links are resolved correctly"""
    html = '<link rel="icon" href="favicon.ico">'
    meta = meta_oxide.extract_meta(html, "https://example.com/subdir/")
    assert meta["icon"] == "https://example.com/subdir/favicon.ico"
