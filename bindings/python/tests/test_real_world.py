"""Real-world website tests"""

import meta_oxide


def test_github_repo_page():
    """Test with GitHub repository page HTML"""
    # Simplified GitHub-style HTML
    html = """
        <meta property="og:site_name" content="GitHub">
        <meta property="og:type" content="object">
        <meta property="og:title" content="username/repository">
        <meta property="og:description" content="A Rust library for metadata extraction">
        <meta property="og:image" content="https://opengraph.githubassets.com/abc123/username/repository">

        <meta name="twitter:card" content="summary_large_image">
        <meta name="twitter:site" content="@github">
        <meta name="twitter:title" content="username/repository">

        <meta name="description" content="A Rust library for metadata extraction">
    """

    data = meta_oxide.extract_all(html)

    assert data["opengraph"]["site_name"] == "GitHub"
    assert data["twitter"]["site"] == "@github"


def test_medium_article():
    """Test with Medium-style article"""
    html = """
        <title>How to Learn Rust in 2024 - Medium</title>
        <meta name="description" content="A comprehensive guide to learning Rust">
        <meta property="og:type" content="article">
        <meta property="og:title" content="How to Learn Rust in 2024">
        <meta property="og:site_name" content="Medium">
        <meta property="article:published_time" content="2024-01-10T08:00:00.000Z">
        <meta name="twitter:card" content="summary_large_image">
    """

    data = meta_oxide.extract_all(html)

    assert "Medium" in data["meta"]["title"]
    assert data["opengraph"]["site_name"] == "Medium"


def test_youtube_video():
    """Test with YouTube video page"""
    html = """
        <meta property="og:site_name" content="YouTube">
        <meta property="og:url" content="https://www.youtube.com/watch?v=abc123">
        <meta property="og:title" content="Learn Rust Programming">
        <meta property="og:type" content="video.other">
        <meta property="og:video" content="https://www.youtube.com/embed/abc123">
        <meta property="og:video:width" content="1280">
        <meta property="og:video:height" content="720">

        <meta name="twitter:card" content="player">
        <meta name="twitter:player" content="https://www.youtube.com/embed/abc123">
    """

    data = meta_oxide.extract_all(html)

    assert data["opengraph"]["site_name"] == "YouTube"
    assert len(data["opengraph"]["videos"]) >= 1
    assert data["twitter"]["card"] == "player"


def test_amazon_product():
    """Test with Amazon-style product page"""
    html = """
        <title>Premium Laptop - Amazon.com</title>
        <meta name="description" content="Buy Premium Laptop with fast shipping">
        <meta property="og:type" content="product">
        <meta property="og:title" content="Premium Laptop">
        <meta property="og:site_name" content="Amazon.com">
        <meta name="twitter:card" content="summary">
    """

    data = meta_oxide.extract_all(html)

    assert data["opengraph"]["type"] == "product"
    assert "Amazon" in data["meta"]["title"]


def test_twitter_profile():
    """Test with Twitter/X profile page"""
    html = """
        <title>@username (@username) / X</title>
        <meta name="description" content="Tech enthusiast and developer">

        <meta property="og:type" content="profile">
        <meta property="og:title" content="username">
        <meta property="og:image" content="https://pbs.twimg.com/profile_images/abc123.jpg">
        <meta property="og:url" content="https://twitter.com/username">

        <meta name="twitter:card" content="summary">
        <meta name="twitter:site" content="@username">
        <meta name="twitter:creator" content="@username">
    """

    data = meta_oxide.extract_all(html)

    assert data["opengraph"]["type"] == "profile"
    assert data["twitter"]["card"] == "summary"
    assert "@username" in data["twitter"]["site"]


def test_linkedin_article():
    """Test with LinkedIn article/post"""
    html = """
        <title>Post | LinkedIn</title>
        <meta name="description" content="Exciting announcement about our new product">

        <meta property="og:title" content="Exciting announcement">
        <meta property="og:type" content="article">
        <meta property="og:site_name" content="LinkedIn">
        <meta property="og:image" content="https://media.licdn.com/dms/image/abc123">

        <meta name="twitter:card" content="summary">
    """

    data = meta_oxide.extract_all(html)

    assert "LinkedIn" in data["opengraph"]["site_name"]
    assert data["opengraph"]["type"] == "article"


def test_reddit_post():
    """Test with Reddit post page"""
    html = """
        <title>Post Title - r/rust - Reddit</title>
        <meta name="description" content="Discussion about Rust programming">

        <meta property="og:title" content="Post Title">
        <meta property="og:type" content="website">
        <meta property="og:site_name" content="Reddit">
        <meta property="og:image" content="https://preview.redd.it/abc123.jpg">

        <meta name="twitter:card" content="summary_large_image">
        <meta name="twitter:site" content="@reddit">
    """

    data = meta_oxide.extract_all(html)

    assert "Reddit" in data["meta"]["title"]
    assert data["opengraph"]["site_name"] == "Reddit"
    assert data["twitter"]["site"] == "@reddit"


def test_spotify_track():
    """Test with Spotify track page"""
    html = """
        <title>Song Name - Artist Name - Spotify</title>
        <meta name="description" content="Listen to Song Name on Spotify">

        <meta property="og:type" content="music.song">
        <meta property="og:title" content="Song Name">
        <meta property="og:site_name" content="Spotify">
        <meta property="og:image" content="https://i.scdn.co/image/abc123">
        <meta property="music:musician" content="https://open.spotify.com/artist/123">

        <meta name="twitter:card" content="summary">
        <meta name="twitter:site" content="@spotify">
    """

    data = meta_oxide.extract_all(html)

    assert data["opengraph"]["type"] == "music.song"
    assert "Spotify" in data["opengraph"]["site_name"]
    # Music metadata is extracted but not in a separate 'music' field in current implementation


def test_wikipedia_article():
    """Test with Wikipedia article"""
    html = """
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Rust (programming language) - Wikipedia</title>
            <meta name="description" content="Rust is a multi-paradigm programming language">
            <link rel="canonical" href="https://en.wikipedia.org/wiki/Rust_(programming_language)">

            <meta property="og:title" content="Rust (programming language)">
            <meta property="og:type" content="website">
            <meta property="og:site_name" content="Wikipedia">
            <meta property="og:image" content="https://upload.wikimedia.org/wikipedia/commons/rust.png">
        </head>
        </html>
    """

    data = meta_oxide.extract_all(html)

    assert "Wikipedia" in data["meta"]["title"]
    assert data["opengraph"]["site_name"] == "Wikipedia"


def test_stackoverflow_question():
    """Test with Stack Overflow question page"""
    html = """
        <title>How to handle errors in Rust? - Stack Overflow</title>
        <meta name="description" content="I'm learning Rust and need help with error handling">

        <meta property="og:title" content="How to handle errors in Rust?">
        <meta property="og:type" content="website">
        <meta property="og:site_name" content="Stack Overflow">
        <meta property="og:image" content="https://cdn.sstatic.net/Sites/stackoverflow/img/logo.png">

        <meta name="twitter:card" content="summary">
    """

    data = meta_oxide.extract_all(html)

    assert "Stack Overflow" in data["meta"]["title"]
    assert data["opengraph"]["site_name"] == "Stack Overflow"


def test_dev_to_article():
    """Test with DEV.to article"""
    html = """
        <title>Understanding Rust Ownership - DEV Community</title>
        <meta name="description" content="A deep dive into Rust's ownership system">

        <meta property="og:type" content="article">
        <meta property="og:title" content="Understanding Rust Ownership">
        <meta property="og:site_name" content="DEV Community">
        <meta property="article:published_time" content="2024-01-12T10:00:00Z">
        <meta property="article:tag" content="rust">

        <meta name="twitter:card" content="summary_large_image">
        <meta name="twitter:site" content="@thepracticaldev">
    """

    data = meta_oxide.extract_all(html)

    assert "DEV" in data["meta"]["title"]
    assert data["opengraph"]["type"] == "article"
    assert data["twitter"]["site"] == "@thepracticaldev"


def test_hackernews_item():
    """Test with Hacker News item"""
    html = """
        <title>Show HN: MetaOxide - Fast Rust metadata extractor | Hacker News</title>
        <meta name="description" content="Discussion about MetaOxide on Hacker News">

        <meta property="og:title" content="Show HN: MetaOxide">
        <meta property="og:type" content="website">
        <meta property="og:site_name" content="Hacker News">

        <meta name="twitter:card" content="summary">
    """

    data = meta_oxide.extract_all(html)

    assert "Hacker News" in data["meta"]["title"]


def test_producthunt_product():
    """Test with Product Hunt product page"""
    html = """
        <title>MetaOxide - Fast metadata extraction | Product Hunt</title>
        <meta name="description" content="Extract metadata from HTML with blazing speed">

        <meta property="og:title" content="MetaOxide">
        <meta property="og:type" content="product">
        <meta property="og:site_name" content="Product Hunt">
        <meta property="og:image" content="https://ph-files.imgix.com/abc123.png">

        <meta name="twitter:card" content="summary_large_image">
        <meta name="twitter:site" content="@ProductHunt">
    """

    data = meta_oxide.extract_all(html)

    assert "Product Hunt" in data["meta"]["title"]
    assert data["twitter"]["site"] == "@ProductHunt"


def test_nytimes_article():
    """Test with New York Times article"""
    html = """
        <!DOCTYPE html>
        <html lang="en-US">
        <head>
            <meta charset="UTF-8">
            <title>Breaking News Article - The New York Times</title>
            <meta name="description" content="Latest developments in the story">
            <meta name="author" content="Reporter Name">
            <link rel="canonical" href="https://www.nytimes.com/2024/01/15/article.html">

            <meta property="og:type" content="article">
            <meta property="og:title" content="Breaking News Article">
            <meta property="og:site_name" content="The New York Times">
            <meta property="article:published_time" content="2024-01-15T12:00:00Z">
            <meta property="article:section" content="World">

            <meta name="twitter:card" content="summary_large_image">
            <meta name="twitter:site" content="@nytimes">
        </head>
        </html>
    """

    data = meta_oxide.extract_all(html, "https://www.nytimes.com")

    assert "New York Times" in data["meta"]["title"]
    assert data["opengraph"]["site_name"] == "The New York Times"
    assert data["twitter"]["site"] == "@nytimes"


def test_shopify_store():
    """Test with Shopify store product page"""
    html = """
        <!DOCTYPE html>
        <html>
        <head>
            <title>Cool Product - My Shop</title>
            <meta name="description" content="Amazing product with great features">

            <meta property="og:type" content="product">
            <meta property="og:title" content="Cool Product">
            <meta property="og:image" content="https://cdn.shopify.com/product.jpg">
            <meta property="og:price:amount" content="49.99">
            <meta property="og:price:currency" content="USD">
            <meta property="og:availability" content="instock">

            <meta name="twitter:card" content="summary">
        </head>
        </html>
    """

    data = meta_oxide.extract_all(html)

    assert data["opengraph"]["type"] == "product"
    assert data["meta"]["title"] == "Cool Product - My Shop"


def test_vercel_docs():
    """Test with Vercel documentation style"""
    html = """
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <title>Getting Started - Documentation</title>
            <meta name="description" content="Learn how to get started with our platform">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <link rel="canonical" href="https://docs.example.com/getting-started">

            <meta property="og:title" content="Getting Started">
            <meta property="og:type" content="website">
            <meta property="og:site_name" content="Documentation">
            <meta property="og:image" content="https://docs.example.com/og-image.png">

            <meta name="twitter:card" content="summary_large_image">
        </head>
        </html>
    """

    data = meta_oxide.extract_all(html)

    assert "Documentation" in data["meta"]["title"]
    assert data["twitter"]["card"] == "summary_large_image"


def test_blog_with_author_microformat():
    """Test blog with h-card author"""
    html = """
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <title>My Blog Post</title>
            <meta name="description" content="An interesting article">

            <meta property="og:type" content="article">
            <meta property="og:title" content="My Blog Post">

            <meta name="twitter:card" content="summary">
        </head>
        <body>
            <article class="h-entry">
                <h1 class="p-name">My Blog Post</h1>
                <time class="dt-published" datetime="2024-01-15">January 15, 2024</time>
                <div class="p-author h-card">
                    <span class="p-name">Jane Blogger</span>
                    <a class="u-url" href="https://jane.example.com">Website</a>
                </div>
                <div class="e-content">
                    <p>Article content here...</p>
                </div>
            </article>
        </body>
        </html>
    """

    data = meta_oxide.extract_all(html)

    assert data["meta"]["title"] == "My Blog Post"
    assert "h-entry" in data["microformats"]
    assert "h-card" in data["microformats"]


def test_event_website():
    """Test event website with h-event"""
    html = """
        <!DOCTYPE html>
        <html>
        <head>
            <title>Tech Conference 2024</title>
            <meta name="description" content="Join us for the biggest tech conference">

            <meta property="og:type" content="website">
            <meta property="og:title" content="Tech Conference 2024">
        </head>
        <body>
            <div class="h-event">
                <h1 class="p-name">Tech Conference 2024</h1>
                <time class="dt-start" datetime="2024-03-01T09:00:00">
                    March 1, 2024 at 9:00 AM
                </time>
                <time class="dt-end" datetime="2024-03-03T17:00:00">
                    March 3, 2024 at 5:00 PM
                </time>
                <div class="p-location h-card">
                    <span class="p-name">Convention Center</span>
                    <span class="p-street-address">123 Main St</span>
                </div>
            </div>
        </body>
        </html>
    """

    data = meta_oxide.extract_all(html)

    assert data["meta"]["title"] == "Tech Conference 2024"
    assert "h-event" in data["microformats"]


def test_podcast_episode():
    """Test podcast episode page"""
    html = """
        <!DOCTYPE html>
        <html>
        <head>
            <title>Episode 42: Understanding Rust - My Podcast</title>
            <meta name="description" content="In this episode we discuss Rust programming">
            <meta name="author" content="Podcast Host">

            <meta property="og:type" content="music.song">
            <meta property="og:title" content="Episode 42: Understanding Rust">
            <meta property="og:site_name" content="My Podcast">
            <meta property="og:audio" content="https://podcast.example.com/episode42.mp3">
            <meta property="og:image" content="https://podcast.example.com/artwork.jpg">

            <meta name="twitter:card" content="summary">
            <meta name="twitter:site" content="@mypodcast">
        </head>
        </html>
    """

    data = meta_oxide.extract_all(html)

    assert "Episode 42" in data["meta"]["title"]
    assert data["opengraph"]["site_name"] == "My Podcast"


def test_restaurant_with_address():
    """Test restaurant website with h-card"""
    html = """
        <!DOCTYPE html>
        <html>
        <head>
            <title>Best Restaurant - Fine Dining</title>
            <meta name="description" content="Experience exquisite cuisine">

            <meta property="og:type" content="business.business">
            <meta property="og:title" content="Best Restaurant">
        </head>
        <body>
            <div class="h-card">
                <h1 class="p-name">Best Restaurant</h1>
                <div class="p-adr h-adr">
                    <span class="p-street-address">456 Food Street</span>
                    <span class="p-locality">Food City</span>
                </div>
                <span class="p-tel">555-1234</span>
                <a class="u-url" href="https://restaurant.example.com">Visit Us</a>
            </div>
        </body>
        </html>
    """

    data = meta_oxide.extract_all(html)

    assert "Best Restaurant" in data["meta"]["title"]
    assert "h-card" in data["microformats"]
