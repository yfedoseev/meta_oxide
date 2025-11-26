"""Tests for PWA and Mobile metadata extraction (Phase 8)"""

import meta_oxide


def test_manifest_link():
    """Test PWA manifest link extraction"""
    html = '<link rel="manifest" href="/manifest.json">'
    meta = meta_oxide.extract_meta(html)
    assert meta["manifest"] == "/manifest.json"


def test_manifest_link_with_base_url():
    """Test PWA manifest link with base URL resolution"""
    html = '<link rel="manifest" href="/manifest.webmanifest">'
    meta = meta_oxide.extract_meta(html, base_url="https://example.com")
    assert meta["manifest"] == "https://example.com/manifest.webmanifest"


def test_mobile_web_app_capable():
    """Test mobile-web-app-capable meta tag"""
    html = '<meta name="mobile-web-app-capable" content="yes">'
    meta = meta_oxide.extract_meta(html)
    assert meta["mobile_web_app_capable"] == "yes"


def test_mobile_web_app_capable_no():
    """Test mobile-web-app-capable with no value"""
    html = '<meta name="mobile-web-app-capable" content="no">'
    meta = meta_oxide.extract_meta(html)
    assert meta["mobile_web_app_capable"] == "no"


def test_apple_mobile_web_app_capable():
    """Test Apple mobile web app capable tag"""
    html = '<meta name="apple-mobile-web-app-capable" content="yes">'
    meta = meta_oxide.extract_meta(html)
    assert meta["apple_mobile_web_app_capable"] == "yes"


def test_apple_mobile_web_app_status_bar():
    """Test Apple mobile status bar style tag"""
    html = '<meta name="apple-mobile-web-app-status-bar-style" content="black-translucent">'
    meta = meta_oxide.extract_meta(html)
    assert meta["apple_mobile_web_app_status_bar_style"] == "black-translucent"


def test_apple_mobile_web_app_status_bar_default():
    """Test Apple mobile status bar style with default value"""
    html = '<meta name="apple-mobile-web-app-status-bar-style" content="default">'
    meta = meta_oxide.extract_meta(html)
    assert meta["apple_mobile_web_app_status_bar_style"] == "default"


def test_apple_mobile_web_app_status_bar_black():
    """Test Apple mobile status bar style with black value"""
    html = '<meta name="apple-mobile-web-app-status-bar-style" content="black">'
    meta = meta_oxide.extract_meta(html)
    assert meta["apple_mobile_web_app_status_bar_style"] == "black"


def test_apple_mobile_web_app_title():
    """Test Apple mobile web app title tag"""
    html = '<meta name="apple-mobile-web-app-title" content="My App">'
    meta = meta_oxide.extract_meta(html)
    assert meta["apple_mobile_web_app_title"] == "My App"


def test_apple_itunes_app():
    """Test Apple iTunes app meta tag"""
    html = '<meta name="apple-itunes-app" content="app-id=123456789">'
    meta = meta_oxide.extract_meta(html)
    assert meta["apple_itunes_app"] == "app-id=123456789"


def test_apple_itunes_app_with_affiliate():
    """Test Apple iTunes app with affiliate data"""
    html = (
        '<meta name="apple-itunes-app" content="app-id=123456789, affiliate-data=myAffiliateData">'
    )
    meta = meta_oxide.extract_meta(html)
    assert meta["apple_itunes_app"] == "app-id=123456789, affiliate-data=myAffiliateData"


def test_google_play_app():
    """Test Google Play app meta tag"""
    html = '<meta name="google-play-app" content="app-id=com.example.android">'
    meta = meta_oxide.extract_meta(html)
    assert meta["google_play_app"] == "app-id=com.example.android"


def test_format_detection():
    """Test format-detection meta tag"""
    html = '<meta name="format-detection" content="telephone=no">'
    meta = meta_oxide.extract_meta(html)
    assert meta["format_detection"] == "telephone=no"


def test_format_detection_multiple():
    """Test format-detection with multiple values"""
    html = '<meta name="format-detection" content="telephone=no, email=no, address=no">'
    meta = meta_oxide.extract_meta(html)
    assert meta["format_detection"] == "telephone=no, email=no, address=no"


def test_complete_pwa_tags():
    """Test extraction of complete PWA tag set"""
    html = """
        <html>
        <head>
            <meta name="mobile-web-app-capable" content="yes">
            <meta name="apple-mobile-web-app-capable" content="yes">
            <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent">
            <meta name="apple-mobile-web-app-title" content="My PWA">
            <meta name="theme-color" content="#000000">
            <link rel="manifest" href="/manifest.json">
            <link rel="apple-touch-icon" href="/icon-192.png">
        </head>
        </html>
    """
    meta = meta_oxide.extract_meta(html)
    assert meta["mobile_web_app_capable"] == "yes"
    assert meta["apple_mobile_web_app_capable"] == "yes"
    assert meta["apple_mobile_web_app_status_bar_style"] == "black-translucent"
    assert meta["apple_mobile_web_app_title"] == "My PWA"
    assert meta["theme_color"] == "#000000"
    assert meta["manifest"] == "/manifest.json"
    assert meta["apple_touch_icon"] == "/icon-192.png"


def test_complete_mobile_app_links():
    """Test extraction of mobile app link tags"""
    html = """
        <html>
        <head>
            <meta name="apple-itunes-app" content="app-id=123456789">
            <meta name="google-play-app" content="app-id=com.example.android">
            <meta name="format-detection" content="telephone=no">
        </head>
        </html>
    """
    meta = meta_oxide.extract_meta(html)
    assert meta["apple_itunes_app"] == "app-id=123456789"
    assert meta["google_play_app"] == "app-id=com.example.android"
    assert meta["format_detection"] == "telephone=no"


def test_pwa_in_extract_all():
    """Test that PWA tags are included in extract_all"""
    html = """
        <html>
        <head>
            <title>PWA Example</title>
            <meta name="description" content="A Progressive Web App">
            <meta name="mobile-web-app-capable" content="yes">
            <meta name="apple-mobile-web-app-capable" content="yes">
            <meta name="apple-itunes-app" content="app-id=987654321">
            <link rel="manifest" href="/app.webmanifest">
        </head>
        </html>
    """
    result = meta_oxide.extract_all(html)

    # Check that standard meta is extracted
    assert "meta" in result
    assert result["meta"]["title"] == "PWA Example"
    assert result["meta"]["description"] == "A Progressive Web App"

    # Check that PWA/mobile meta is extracted
    assert result["meta"]["mobile_web_app_capable"] == "yes"
    assert result["meta"]["apple_mobile_web_app_capable"] == "yes"
    assert result["meta"]["apple_itunes_app"] == "app-id=987654321"
    assert result["meta"]["manifest"] == "/app.webmanifest"


def test_real_world_pwa():
    """Test with a real-world PWA example"""
    html = """
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>My Progressive Web App</title>
            <meta name="description" content="An awesome PWA">

            <!-- PWA Meta Tags -->
            <meta name="mobile-web-app-capable" content="yes">
            <meta name="apple-mobile-web-app-capable" content="yes">
            <meta name="apple-mobile-web-app-status-bar-style" content="black">
            <meta name="apple-mobile-web-app-title" content="MyPWA">
            <meta name="theme-color" content="#4285f4">

            <!-- App Store Links -->
            <meta name="apple-itunes-app" content="app-id=123456789, affiliate-data=partnerId=30&siteID=xyz">
            <meta name="google-play-app" content="app-id=com.example.myapp">

            <!-- Format Detection -->
            <meta name="format-detection" content="telephone=no">

            <!-- Icons and Manifest -->
            <link rel="icon" href="/favicon.ico">
            <link rel="apple-touch-icon" href="/icon-180.png">
            <link rel="manifest" href="/manifest.webmanifest">
        </head>
        </html>
    """
    meta = meta_oxide.extract_meta(html)

    # Basic tags
    assert meta["title"] == "My Progressive Web App"
    assert meta["description"] == "An awesome PWA"
    assert meta["viewport"] == "width=device-width, initial-scale=1.0"

    # PWA tags
    assert meta["mobile_web_app_capable"] == "yes"
    assert meta["apple_mobile_web_app_capable"] == "yes"
    assert meta["apple_mobile_web_app_status_bar_style"] == "black"
    assert meta["apple_mobile_web_app_title"] == "MyPWA"
    assert meta["theme_color"] == "#4285f4"

    # App store links
    assert meta["apple_itunes_app"] == "app-id=123456789, affiliate-data=partnerId=30&siteID=xyz"
    assert meta["google_play_app"] == "app-id=com.example.myapp"

    # Format detection
    assert meta["format_detection"] == "telephone=no"

    # Links
    assert meta["icon"] == "/favicon.ico"
    assert meta["apple_touch_icon"] == "/icon-180.png"
    assert meta["manifest"] == "/manifest.webmanifest"


def test_case_insensitive_pwa_tags():
    """Test case-insensitive handling of PWA meta tags"""
    html = """
        <meta name="Mobile-Web-App-Capable" content="yes">
        <meta name="APPLE-ITUNES-APP" content="app-id=123">
    """
    meta = meta_oxide.extract_meta(html)
    assert meta["mobile_web_app_capable"] == "yes"
    assert meta["apple_itunes_app"] == "app-id=123"
