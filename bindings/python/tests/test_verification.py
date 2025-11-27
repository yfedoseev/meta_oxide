"""Tests for Phase 6: Verification & Platform Integration tags"""

import meta_oxide


def test_google_site_verification():
    """Test Google Search Console verification tag"""
    html = '<meta name="google-site-verification" content="abc123xyz456def789">'
    meta = meta_oxide.extract_meta(html)
    assert meta["google_site_verification"] == "abc123xyz456def789"


def test_google_signin_client_id():
    """Test Google Sign-In client ID meta tag"""
    html = '<meta name="google-signin-client_id" content="123456789.apps.googleusercontent.com">'
    meta = meta_oxide.extract_meta(html)
    assert meta["google_signin_client_id"] == "123456789.apps.googleusercontent.com"


def test_facebook_domain_verification():
    """Test Facebook Business Manager domain verification"""
    html = '<meta name="facebook-domain-verification" content="fb123456789abcdef">'
    meta = meta_oxide.extract_meta(html)
    assert meta["facebook_domain_verification"] == "fb123456789abcdef"


def test_pinterest_verification():
    """Test Pinterest domain verification tag"""
    html = '<meta name="p:domain_verify" content="pinterest123456789">'
    meta = meta_oxide.extract_meta(html)
    assert meta["p_domain_verify"] == "pinterest123456789"


def test_yandex_verification():
    """Test Yandex Webmaster verification tag"""
    html = '<meta name="yandex-verification" content="yandex1234567890abcdef">'
    meta = meta_oxide.extract_meta(html)
    assert meta["yandex_verification"] == "yandex1234567890abcdef"


def test_bing_verification():
    """Test Bing Webmaster Tools verification tag (msvalidate.01)"""
    html = '<meta name="msvalidate.01" content="BING123456789ABCDEF">'
    meta = meta_oxide.extract_meta(html)
    assert meta["msvalidate_01"] == "BING123456789ABCDEF"


def test_google_analytics():
    """Test Google Analytics property ID"""
    html = '<meta name="google-analytics" content="UA-123456789-1">'
    meta = meta_oxide.extract_meta(html)
    assert meta["google_analytics"] == "UA-123456789-1"


def test_google_analytics_ga4():
    """Test Google Analytics 4 property ID"""
    html = '<meta name="google-analytics" content="G-XXXXXXXXXX">'
    meta = meta_oxide.extract_meta(html)
    assert meta["google_analytics"] == "G-XXXXXXXXXX"


def test_facebook_app_id():
    """Test Facebook App ID meta tag"""
    html = '<meta property="fb:app_id" content="123456789012345">'
    meta = meta_oxide.extract_meta(html)
    assert meta["fb_app_id"] == "123456789012345"


def test_facebook_pages():
    """Test Facebook Pages ID meta tag"""
    html = '<meta property="fb:pages" content="987654321098765">'
    meta = meta_oxide.extract_meta(html)
    assert meta["fb_pages"] == "987654321098765"


def test_multiple_verification_tags():
    """Test extraction of multiple verification and analytics tags at once"""
    html = """
        <html>
        <head>
            <meta name="google-site-verification" content="google_verify_123">
            <meta name="google-signin-client_id" content="client123.apps.googleusercontent.com">
            <meta name="msvalidate.01" content="bing_verify_456">
            <meta name="yandex-verification" content="yandex_verify_789">
            <meta name="p:domain_verify" content="pinterest_verify_012">
            <meta name="facebook-domain-verification" content="fb_domain_345">
            <meta name="google-analytics" content="UA-12345678-9">
            <meta property="fb:app_id" content="1234567890">
            <meta property="fb:pages" content="9876543210">
        </head>
        </html>
    """
    meta = meta_oxide.extract_meta(html)

    # Verification tags
    assert meta["google_site_verification"] == "google_verify_123"
    assert meta["google_signin_client_id"] == "client123.apps.googleusercontent.com"
    assert meta["msvalidate_01"] == "bing_verify_456"
    assert meta["yandex_verification"] == "yandex_verify_789"
    assert meta["p_domain_verify"] == "pinterest_verify_012"
    assert meta["facebook_domain_verification"] == "fb_domain_345"

    # Analytics tags
    assert meta["google_analytics"] == "UA-12345678-9"
    assert meta["fb_app_id"] == "1234567890"
    assert meta["fb_pages"] == "9876543210"


def test_verification_in_extract_all():
    """Test that verification tags work in extract_all() integration"""
    html = """
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <title>Verified Site</title>
            <meta name="description" content="A fully verified website">
            <meta name="google-site-verification" content="google123">
            <meta name="google-signin-client_id" content="app.googleusercontent.com">
            <meta name="facebook-domain-verification" content="fb456">
            <meta name="google-analytics" content="UA-999999-1">
            <meta property="fb:app_id" content="999999999">
        </head>
        </html>
    """
    result = meta_oxide.extract_all(html)

    # Ensure meta tags are extracted
    assert "meta" in result
    meta = result["meta"]

    # Basic tags should be present
    assert meta["title"] == "Verified Site"
    assert meta["description"] == "A fully verified website"

    # Verification tags should be present
    assert meta["google_site_verification"] == "google123"
    assert meta["google_signin_client_id"] == "app.googleusercontent.com"
    assert meta["facebook_domain_verification"] == "fb456"
    assert meta["google_analytics"] == "UA-999999-1"
    assert meta["fb_app_id"] == "999999999"


def test_empty_verification_tags_ignored():
    """Test that verification tags with empty content are ignored"""
    html = """
        <meta name="google-site-verification" content="">
        <meta name="google-analytics" content="  ">
    """
    meta = meta_oxide.extract_meta(html)
    assert "google_site_verification" not in meta
    assert "google_analytics" not in meta


def test_case_insensitive_verification_tags():
    """Test that verification tag names are case-insensitive"""
    html = """
        <meta name="GOOGLE-SITE-VERIFICATION" content="uppercase123">
        <meta name="Google-Analytics" content="MixedCase456">
    """
    meta = meta_oxide.extract_meta(html)
    assert meta["google_site_verification"] == "uppercase123"
    assert meta["google_analytics"] == "MixedCase456"


def test_whitespace_trimming_verification():
    """Test that whitespace is trimmed from verification tag content"""
    html = '<meta name="google-site-verification" content="  trimmed123  ">'
    meta = meta_oxide.extract_meta(html)
    assert meta["google_site_verification"] == "trimmed123"


def test_real_world_wordpress_with_verification():
    """Test real-world WordPress site with verification tags"""
    html = """
        <!DOCTYPE html>
        <html lang="en-US">
        <head>
            <meta charset="UTF-8">
            <title>My WordPress Site</title>
            <meta name="description" content="Welcome to my verified WordPress site">
            <meta name="generator" content="WordPress 6.4">
            <meta name="google-site-verification" content="wordpress_google_123">
            <meta name="google-analytics" content="UA-WORDPRESS-1">
            <meta property="fb:app_id" content="1234567890123">
            <link rel="canonical" href="https://example.com">
        </head>
        </html>
    """
    meta = meta_oxide.extract_meta(html, base_url="https://example.com")

    assert meta["title"] == "My WordPress Site"
    assert meta["generator"] == "WordPress 6.4"
    assert meta["google_site_verification"] == "wordpress_google_123"
    assert meta["google_analytics"] == "UA-WORDPRESS-1"
    assert meta["fb_app_id"] == "1234567890123"
    assert meta["canonical"] == "https://example.com/"


def test_real_world_shopify_with_verification():
    """Test real-world Shopify e-commerce site with verification and analytics"""
    html = """
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="utf-8">
            <title>My Shop - Product Page</title>
            <meta name="description" content="Buy amazing products">
            <meta name="google-site-verification" content="shopify_verify_xyz">
            <meta name="google-analytics" content="G-SHOPIFY123">
            <meta property="fb:app_id" content="9999999999">
            <meta property="fb:pages" content="8888888888">
            <meta name="pinterest-domain-verify" content="shop_pinterest_123">
            <link rel="canonical" href="https://shop.example.com/product">
        </head>
        </html>
    """
    meta = meta_oxide.extract_meta(html, base_url="https://shop.example.com")

    assert meta["title"] == "My Shop - Product Page"
    assert meta["google_site_verification"] == "shopify_verify_xyz"
    assert meta["google_analytics"] == "G-SHOPIFY123"
    assert meta["fb_app_id"] == "9999999999"
    assert meta["fb_pages"] == "8888888888"


def test_facebook_property_tags():
    """Test Facebook property tags (using property attribute instead of name)"""
    html = """
        <meta property="fb:app_id" content="123456789">
        <meta property="fb:pages" content="987654321">
    """
    meta = meta_oxide.extract_meta(html)
    assert meta["fb_app_id"] == "123456789"
    assert meta["fb_pages"] == "987654321"


def test_mixed_verification_analytics():
    """Test mix of verification tags and analytics tags"""
    html = """
        <html>
        <head>
            <title>Business Site</title>
            <meta name="google-site-verification" content="business_google">
            <meta name="google-analytics" content="UA-BUSINESS-1">
            <meta name="facebook-domain-verification" content="business_fb">
            <meta property="fb:app_id" content="111111111">
        </head>
        </html>
    """
    meta = meta_oxide.extract_meta(html)

    assert meta["title"] == "Business Site"
    assert meta["google_site_verification"] == "business_google"
    assert meta["google_analytics"] == "UA-BUSINESS-1"
    assert meta["facebook_domain_verification"] == "business_fb"
    assert meta["fb_app_id"] == "111111111"
