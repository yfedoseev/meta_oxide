"""Integration tests for combined extraction (Phase E)"""

import meta_oxide


def test_extract_all_complete_page():
    """Test extract_all() with a complete real-world page"""
    html = """
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>My Amazing Blog Post</title>
            <meta name="description" content="A fascinating article about Rust and Python">
            <meta name="keywords" content="rust, python, tutorial">
            <meta name="author" content="Jane Developer">
            <meta name="robots" content="index, follow">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <link rel="canonical" href="https://blog.example.com/post/123">
            <link rel="alternate" type="application/rss+xml" href="/feed.xml">

            <!-- Open Graph -->
            <meta property="og:title" content="My Amazing Blog Post">
            <meta property="og:type" content="article">
            <meta property="og:url" content="https://blog.example.com/post/123">
            <meta property="og:image" content="https://blog.example.com/images/post.jpg">
            <meta property="og:description" content="A fascinating article about Rust and Python">
            <meta property="og:site_name" content="Example Blog">
            <meta property="article:published_time" content="2024-01-15T10:00:00Z">
            <meta property="article:author" content="https://blog.example.com/author/jane">

            <!-- Twitter -->
            <meta name="twitter:card" content="summary_large_image">
            <meta name="twitter:site" content="@exampleblog">
            <meta name="twitter:creator" content="@janedev">
        </head>
        <body>
            <!-- Microformats -->
            <div class="h-card">
                <span class="p-name">Jane Developer</span>
                <a class="u-url" href="https://jane.example.com">Website</a>
            </div>

            <article class="h-entry">
                <h1 class="p-name">My Amazing Blog Post</h1>
                <time class="dt-published" datetime="2024-01-15">January 15, 2024</time>
                <div class="p-author h-card">
                    <span class="p-name">Jane Developer</span>
                </div>
            </article>
        </body>
        </html>
    """

    data = meta_oxide.extract_all(html, "https://blog.example.com")

    # Verify all sections extracted
    assert "meta" in data
    assert "opengraph" in data
    assert "twitter" in data
    assert "microformats" in data

    # Verify Phase 1 (Standard Meta)
    assert data["meta"]["title"] == "My Amazing Blog Post"
    assert data["meta"]["description"] == "A fascinating article about Rust and Python"
    assert "keywords" in data["meta"]
    assert data["meta"]["canonical"] == "https://blog.example.com/post/123"
    assert data["meta"]["charset"] == "UTF-8"
    assert data["meta"]["language"] == "en"

    # Verify Phase 2 (Open Graph)
    assert data["opengraph"]["title"] == "My Amazing Blog Post"
    assert data["opengraph"]["type"] == "article"
    assert data["opengraph"]["url"] == "https://blog.example.com/post/123"
    assert data["opengraph"]["image"] == "https://blog.example.com/images/post.jpg"
    assert "article" in data["opengraph"]

    # Verify Phase 2 (Twitter)
    assert data["twitter"]["card"] == "summary_large_image"
    assert data["twitter"]["site"] == "@exampleblog"
    assert data["twitter"]["creator"] == "@janedev"

    # Verify Phase 7 (Microformats)
    assert "h-card" in data["microformats"]
    assert "h-entry" in data["microformats"]
    assert len(data["microformats"]["h-card"]) == 2  # One in head, one in body
    assert len(data["microformats"]["h-entry"]) == 1


def test_extract_all_minimal_page():
    """Test with minimal HTML"""
    html = """<html><head><title>Minimal</title></head></html>"""
    data = meta_oxide.extract_all(html)

    assert "meta" in data
    assert data["meta"]["title"] == "Minimal"


def test_extract_all_partial_data():
    """Test with only some metadata present"""
    html = """
        <meta property="og:title" content="Only OG">
        <meta name="description" content="Only meta description">
    """
    data = meta_oxide.extract_all(html)

    assert "meta" in data
    assert data["meta"]["description"] == "Only meta description"

    assert "opengraph" in data
    assert data["opengraph"]["title"] == "Only OG"


def test_extract_all_real_world_ecommerce():
    """Test with real e-commerce product page"""
    html = """
        <!DOCTYPE html>
        <html>
        <head>
            <title>Premium Headphones - $299</title>
            <meta
                name="description"
                content="High-quality wireless headphones with noise cancellation"
            >
            <link rel="canonical" href="https://shop.example.com/products/headphones">

            <meta property="og:type" content="product">
            <meta property="og:title" content="Premium Headphones">
            <meta property="og:image" content="https://shop.example.com/images/headphones.jpg">
            <meta property="og:price:amount" content="299.00">
            <meta property="og:price:currency" content="USD">

            <meta name="twitter:card" content="summary">
            <meta name="twitter:title" content="Premium Headphones - $299">
        </head>
        </html>
    """

    data = meta_oxide.extract_all(html, "https://shop.example.com")

    assert data["meta"]["title"] == "Premium Headphones - $299"
    assert data["opengraph"]["type"] == "product"
    assert data["twitter"]["card"] == "summary"


def test_extract_all_real_world_news():
    """Test with real news article"""
    html = """
        <!DOCTYPE html>
        <html lang="en-US">
        <head>
            <meta charset="UTF-8">
            <title>Breaking: Important News Event</title>
            <meta name="description" content="Details about the important news event">
            <meta name="author" content="News Reporter">
            <link rel="canonical" href="https://news.example.com/2024/01/15/event">

            <meta property="og:type" content="article">
            <meta property="og:title" content="Breaking: Important News Event">
            <meta property="og:site_name" content="Example News">
            <meta property="article:published_time" content="2024-01-15T14:30:00Z">
            <meta property="article:section" content="World">

            <meta name="twitter:card" content="summary_large_image">
            <meta name="twitter:site" content="@examplenews">
        </head>
        </html>
    """

    data = meta_oxide.extract_all(html, "https://news.example.com")

    # Comprehensive assertions
    assert data["meta"]["author"] == "News Reporter"
    assert data["meta"]["language"] == "en-US"
    assert data["opengraph"]["site_name"] == "Example News"
    assert data["opengraph"]["article"]["section"] == "World"


def test_extract_all_with_relative_urls():
    """Test URL resolution across all extractors"""
    html = """
        <link rel="canonical" href="/blog/post">
        <meta property="og:image" content="/images/post.jpg">
        <meta name="twitter:image" content="/images/twitter.jpg">
    """

    data = meta_oxide.extract_all(html, "https://example.com")

    assert data["meta"]["canonical"] == "https://example.com/blog/post"
    assert data["opengraph"]["image"] == "https://example.com/images/post.jpg"
    assert data["twitter"]["image"] == "https://example.com/images/twitter.jpg"


def test_extract_all_empty_html():
    """Test with empty HTML"""
    data = meta_oxide.extract_all("")
    assert isinstance(data, dict)


def test_extract_all_malformed_html():
    """Test with malformed HTML (should not crash)"""
    html = """
        <meta property="og:title" content="Unclosed tag
        <meta name="description" content="Missing quote>
        <div class="h-card">
            <span class="p-name">No closing div
    """
    data = meta_oxide.extract_all(html)

    # Should extract what it can without crashing
    assert isinstance(data, dict)


def test_extract_all_unicode_content():
    """Test with international characters"""
    html = """
        <meta charset="UTF-8">
        <title>日本語のタイトル</title>
        <meta name="description" content="Ça c'est français">
        <meta property="og:title" content="Русский язык">
        <meta name="twitter:title" content="العربية">
    """

    data = meta_oxide.extract_all(html)

    assert "日本語" in data["meta"]["title"]
    assert "français" in data["meta"]["description"]
    assert "Русский" in data["opengraph"]["title"]
    assert "العربية" in data["twitter"]["title"]


def test_extract_all_blog_platform():
    """Test with blog platform (WordPress/Medium style)"""
    html = """
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>10 Tips for Learning Rust</title>
            <meta name="description" content="Essential tips for beginners">
            <meta name="author" content="Tech Blogger">
            <meta name="keywords" content="rust, programming, tutorial, beginner">
            <link rel="canonical" href="https://blog.example.com/rust-tips">

            <meta property="og:type" content="article">
            <meta property="og:title" content="10 Tips for Learning Rust">
            <meta property="og:description" content="Essential tips for beginners">
            <meta property="og:image" content="https://blog.example.com/images/rust-tips.jpg">
            <meta property="og:url" content="https://blog.example.com/rust-tips">
            <meta property="article:published_time" content="2024-01-10T09:00:00Z">
            <meta property="article:author" content="Tech Blogger">
            <meta property="article:tag" content="rust">
            <meta property="article:tag" content="programming">

            <meta name="twitter:card" content="summary_large_image">
            <meta name="twitter:title" content="10 Tips for Learning Rust">
            <meta name="twitter:description" content="Essential tips for beginners">
            <meta name="twitter:image" content="https://blog.example.com/images/rust-tips.jpg">
        </head>
        </html>
    """

    data = meta_oxide.extract_all(html, "https://blog.example.com")

    assert data["meta"]["title"] == "10 Tips for Learning Rust"
    assert len(data["meta"]["keywords"]) == 4
    assert data["opengraph"]["type"] == "article"
    assert len(data["opengraph"]["article"]["tag"]) == 2  # Fixed: 'tag' not 'tags'
    assert data["twitter"]["card"] == "summary_large_image"


def test_extract_all_documentation_site():
    """Test with documentation site"""
    html = """
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <title>API Reference - MyProject Documentation</title>
            <meta name="description" content="Complete API reference for MyProject">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <link rel="canonical" href="https://docs.example.com/api">

            <meta property="og:title" content="API Reference">
            <meta property="og:type" content="website">
            <meta property="og:site_name" content="MyProject Docs">

            <meta name="twitter:card" content="summary">
        </head>
        </html>
    """

    data = meta_oxide.extract_all(html, "https://docs.example.com")

    assert "Documentation" in data["meta"]["title"]
    assert data["opengraph"]["site_name"] == "MyProject Docs"
    assert data["twitter"]["card"] == "summary"


def test_extract_all_portfolio_site():
    """Test with portfolio/personal website"""
    html = """
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>John Doe - Software Engineer</title>
            <meta name="description" content="Portfolio of John Doe, full-stack developer">
            <meta name="author" content="John Doe">
            <link rel="canonical" href="https://johndoe.com">

            <meta property="og:type" content="profile">
            <meta property="og:title" content="John Doe - Software Engineer">
            <meta property="og:image" content="https://johndoe.com/profile.jpg">
            <meta property="profile:first_name" content="John">
            <meta property="profile:last_name" content="Doe">

            <meta name="twitter:card" content="summary">
            <meta name="twitter:creator" content="@johndoe">
        </head>
        <body>
            <div class="h-card">
                <h1 class="p-name">John Doe</h1>
                <p class="p-note">Full-stack developer</p>
                <a class="u-url" href="https://johndoe.com">Website</a>
                <a class="u-email" href="mailto:john@example.com">Email</a>
            </div>
        </body>
        </html>
    """

    data = meta_oxide.extract_all(html, "https://johndoe.com")

    assert data["meta"]["title"] == "John Doe - Software Engineer"
    assert data["opengraph"]["type"] == "profile"
    assert "profile" in data["opengraph"]
    assert "h-card" in data["microformats"]


def test_extract_all_social_media_page():
    """Test with social media-style page"""
    html = """
        <!DOCTYPE html>
        <html>
        <head>
            <title>@username on SocialNet</title>
            <meta name="description" content="Follow @username for tech updates">

            <meta property="og:type" content="profile">
            <meta property="og:title" content="@username on SocialNet">
            <meta property="og:image" content="https://social.example.com/avatars/user123.jpg">
            <meta property="og:url" content="https://social.example.com/username">
            <meta property="profile:username" content="username">

            <meta name="twitter:card" content="summary">
            <meta name="twitter:site" content="@socialnet">
            <meta name="twitter:creator" content="@username">
        </head>
        </html>
    """

    data = meta_oxide.extract_all(html, "https://social.example.com")

    assert "@username" in data["meta"]["title"]
    assert data["opengraph"]["type"] == "profile"
    assert data["twitter"]["creator"] == "@username"


def test_extract_all_video_platform():
    """Test with video platform page"""
    html = """
        <!DOCTYPE html>
        <html>
        <head>
            <title>Amazing Video Title - VideoSite</title>
            <meta name="description" content="Watch this amazing video">

            <meta property="og:type" content="video.other">
            <meta property="og:title" content="Amazing Video Title">
            <meta property="og:url" content="https://video.example.com/watch?v=abc123">
            <meta property="og:image" content="https://video.example.com/thumbs/abc123.jpg">
            <meta property="og:video" content="https://video.example.com/embed/abc123">
            <meta property="og:video:width" content="1280">
            <meta property="og:video:height" content="720">
            <meta property="og:video:type" content="video/mp4">

            <meta name="twitter:card" content="player">
            <meta name="twitter:player" content="https://video.example.com/embed/abc123">
            <meta name="twitter:player:width" content="1280">
            <meta name="twitter:player:height" content="720">
        </head>
        </html>
    """

    data = meta_oxide.extract_all(html, "https://video.example.com")

    assert "VideoSite" in data["meta"]["title"]
    assert data["opengraph"]["type"] == "video.other"
    assert len(data["opengraph"]["videos"]) >= 1
    assert data["twitter"]["card"] == "player"


def test_extract_all_music_platform():
    """Test with music streaming platform"""
    html = """
        <!DOCTYPE html>
        <html>
        <head>
            <title>Song Title - Artist Name</title>
            <meta name="description" content="Listen to Song Title by Artist Name">

            <meta property="og:type" content="music.song">
            <meta property="og:title" content="Song Title">
            <meta property="og:image" content="https://music.example.com/album-art.jpg">
            <meta property="music:duration" content="180">
            <meta property="music:album" content="Album Name">
            <meta property="music:musician" content="https://music.example.com/artist/123">

            <meta name="twitter:card" content="summary">
            <meta name="twitter:audio:src" content="https://music.example.com/preview/song123.mp3">
        </head>
        </html>
    """

    data = meta_oxide.extract_all(html, "https://music.example.com")

    assert "Song Title" in data["meta"]["title"]
    assert data["opengraph"]["type"] == "music.song"
    # Music metadata is extracted but not in a separate 'music' field in current implementation


def test_extract_all_recipe_site():
    """Test with recipe/cooking site"""
    html = """
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <title>Delicious Chocolate Cake Recipe</title>
            <meta
                name="description"
                content="Easy chocolate cake recipe with step-by-step instructions"
            >
            <meta name="author" content="Chef Smith">
            <meta name="keywords" content="recipe, chocolate, cake, dessert, baking">

            <meta property="og:type" content="article">
            <meta property="og:title" content="Delicious Chocolate Cake Recipe">
            <meta property="og:image" content="https://recipes.example.com/images/chocolate-cake.jpg">
            <meta property="article:author" content="Chef Smith">
            <meta property="article:section" content="Desserts">

            <meta name="twitter:card" content="summary_large_image">
        </head>
        <body>
            <div class="h-recipe">
                <h1 class="p-name">Chocolate Cake</h1>
                <p class="p-summary">Delicious and easy chocolate cake</p>
            </div>
        </body>
        </html>
    """

    data = meta_oxide.extract_all(html, "https://recipes.example.com")

    assert "Chocolate Cake" in data["meta"]["title"]
    assert data["opengraph"]["article"]["section"] == "Desserts"
    assert data["twitter"]["card"] == "summary_large_image"


def test_extract_all_no_metadata():
    """Test with page that has no metadata at all"""
    html = """
        <html>
        <head></head>
        <body><h1>Plain Page</h1></body>
        </html>
    """

    data = meta_oxide.extract_all(html)

    # Should still return dictionary structure
    assert isinstance(data, dict)
    assert "meta" in data
    assert "opengraph" in data
    assert "twitter" in data


def test_extract_all_only_twitter_no_og():
    """Test with Twitter cards but no Open Graph"""
    html = """
        <meta name="twitter:card" content="summary">
        <meta name="twitter:title" content="Twitter Only">
        <meta name="twitter:description" content="No OG tags here">
    """

    data = meta_oxide.extract_all(html)

    assert data["twitter"]["card"] == "summary"
    assert data["twitter"]["title"] == "Twitter Only"


def test_extract_all_mixed_microformats():
    """Test with multiple microformat types"""
    html = """
        <html>
        <body>
            <div class="h-card">
                <span class="p-name">Person One</span>
            </div>

            <article class="h-entry">
                <h1 class="p-name">Blog Post</h1>
                <time class="dt-published" datetime="2024-01-15">Jan 15</time>
            </article>

            <div class="h-event">
                <h2 class="p-name">Conference 2024</h2>
                <time class="dt-start" datetime="2024-03-01">March 1, 2024</time>
            </div>

            <div class="h-card">
                <span class="p-name">Person Two</span>
            </div>
        </body>
        </html>
    """

    data = meta_oxide.extract_all(html)

    assert "microformats" in data
    assert len(data["microformats"]["h-card"]) == 2
    assert len(data["microformats"]["h-entry"]) == 1
    assert len(data["microformats"]["h-event"]) == 1


def test_extract_all_special_characters():
    """Test with special HTML characters and entities"""
    html = """
        <title>Test &amp; Example &lt;Company&gt;</title>
        <meta name="description" content="We're #1 in &quot;quality&quot;">
        <meta property="og:title" content="Test &amp; Example">
        <meta name="twitter:title" content="&lt;Test&gt;">
    """

    data = meta_oxide.extract_all(html)

    # HTML entities should be properly decoded
    assert "&" in data["meta"]["title"]
    assert "<" in data["meta"]["title"]
    assert ">" in data["meta"]["title"]
