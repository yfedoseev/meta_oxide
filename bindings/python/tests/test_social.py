"""Tests for social media tag extraction (Phase 2)"""

import meta_oxide


def test_opengraph_basic():
    """Test basic Open Graph extraction"""
    html = """
        <meta property="og:title" content="Test Article">
        <meta property="og:type" content="article">
        <meta property="og:url" content="https://example.com/article">
        <meta property="og:image" content="https://example.com/image.jpg">
    """
    og = meta_oxide.extract_opengraph(html)
    assert og["title"] == "Test Article"
    assert og["type"] == "article"
    assert og["url"] == "https://example.com/article"
    assert og["image"] == "https://example.com/image.jpg"


def test_opengraph_with_metadata():
    """Test Open Graph with rich metadata"""
    html = """
        <meta property="og:title" content="Amazing Recipe">
        <meta property="og:type" content="article">
        <meta property="og:description" content="Delicious recipe">
        <meta property="og:site_name" content="Recipe Blog">
        <meta property="og:locale" content="en_US">
        <meta property="og:image" content="https://example.com/food.jpg">
        <meta property="og:image:width" content="1200">
        <meta property="og:image:height" content="630">
    """
    og = meta_oxide.extract_opengraph(html)
    assert og["title"] == "Amazing Recipe"
    assert og["description"] == "Delicious recipe"
    assert og["site_name"] == "Recipe Blog"
    assert "images" in og
    assert len(og["images"]) >= 1


def test_opengraph_multiple_images():
    """Test Open Graph with multiple images"""
    html = """
        <meta property="og:image" content="https://example.com/image1.jpg">
        <meta property="og:image" content="https://example.com/image2.jpg">
        <meta property="og:image" content="https://example.com/image3.jpg">
    """
    og = meta_oxide.extract_opengraph(html)
    assert len(og["images"]) == 3
    assert og["images"][0]["url"] == "https://example.com/image1.jpg"
    assert og["images"][1]["url"] == "https://example.com/image2.jpg"


def test_opengraph_image_with_full_metadata():
    """Test Open Graph image with complete metadata"""
    html = """
        <meta property="og:image" content="https://example.com/image.jpg">
        <meta property="og:image:secure_url" content="https://example.com/secure.jpg">
        <meta property="og:image:type" content="image/jpeg">
        <meta property="og:image:width" content="1200">
        <meta property="og:image:height" content="630">
        <meta property="og:image:alt" content="Test image">
    """
    og = meta_oxide.extract_opengraph(html)
    assert len(og["images"]) == 1
    img = og["images"][0]
    assert img["url"] == "https://example.com/image.jpg"
    assert img["secure_url"] == "https://example.com/secure.jpg"
    assert img["type"] == "image/jpeg"
    assert img["width"] == 1200
    assert img["height"] == 630
    assert img["alt"] == "Test image"


def test_opengraph_article_metadata():
    """Test Open Graph article metadata"""
    html = """
        <meta property="og:type" content="article">
        <meta property="article:published_time" content="2024-01-15T10:00:00Z">
        <meta property="article:modified_time" content="2024-01-16T12:00:00Z">
        <meta property="article:author" content="https://example.com/author">
        <meta property="article:section" content="Technology">
        <meta property="article:tag" content="rust">
        <meta property="article:tag" content="python">
    """
    og = meta_oxide.extract_opengraph(html)
    assert "article" in og
    article = og["article"]
    assert article["published_time"] == "2024-01-15T10:00:00Z"
    assert article["modified_time"] == "2024-01-16T12:00:00Z"
    assert article["section"] == "Technology"
    assert len(article["tag"]) == 2
    assert "rust" in article["tag"]
    assert "python" in article["tag"]


def test_opengraph_video():
    """Test Open Graph video metadata"""
    html = """
        <meta property="og:video" content="https://example.com/video.mp4">
        <meta property="og:video:secure_url" content="https://example.com/secure-video.mp4">
        <meta property="og:video:type" content="video/mp4">
        <meta property="og:video:width" content="1280">
        <meta property="og:video:height" content="720">
    """
    og = meta_oxide.extract_opengraph(html)
    assert len(og["videos"]) == 1
    video = og["videos"][0]
    assert video["url"] == "https://example.com/video.mp4"
    assert video["width"] == 1280
    assert video["height"] == 720


def test_opengraph_audio():
    """Test Open Graph audio metadata"""
    html = """
        <meta property="og:audio" content="https://example.com/audio.mp3">
        <meta property="og:audio:secure_url" content="https://example.com/secure-audio.mp3">
        <meta property="og:audio:type" content="audio/mpeg">
    """
    og = meta_oxide.extract_opengraph(html)
    assert len(og["audios"]) == 1
    audio = og["audios"][0]
    assert audio["url"] == "https://example.com/audio.mp3"
    assert audio["type"] == "audio/mpeg"


def test_opengraph_book_metadata():
    """Test Open Graph book metadata"""
    html = """
        <meta property="og:type" content="book">
        <meta property="book:author" content="https://example.com/author1">
        <meta property="book:author" content="https://example.com/author2">
        <meta property="book:isbn" content="978-3-16-148410-0">
        <meta property="book:release_date" content="2024-01-01">
        <meta property="book:tag" content="fiction">
        <meta property="book:tag" content="mystery">
    """
    og = meta_oxide.extract_opengraph(html)
    assert "book" in og
    book = og["book"]
    assert len(book["author"]) == 2
    assert book["isbn"] == "978-3-16-148410-0"
    assert len(book["tag"]) == 2


def test_opengraph_profile_metadata():
    """Test Open Graph profile metadata"""
    html = """
        <meta property="og:type" content="profile">
        <meta property="profile:first_name" content="John">
        <meta property="profile:last_name" content="Doe">
        <meta property="profile:username" content="johndoe">
        <meta property="profile:gender" content="male">
    """
    og = meta_oxide.extract_opengraph(html)
    assert "profile" in og
    profile = og["profile"]
    assert profile["first_name"] == "John"
    assert profile["last_name"] == "Doe"
    assert profile["username"] == "johndoe"


def test_opengraph_locale():
    """Test Open Graph locale and alternates"""
    html = """
        <meta property="og:locale" content="en_US">
        <meta property="og:locale:alternate" content="es_ES">
        <meta property="og:locale:alternate" content="fr_FR">
    """
    og = meta_oxide.extract_opengraph(html)
    assert og["locale"] == "en_US"
    assert len(og["locale_alternate"]) == 2
    assert "es_ES" in og["locale_alternate"]
    assert "fr_FR" in og["locale_alternate"]


def test_opengraph_relative_urls():
    """Test Open Graph with relative URL resolution"""
    html = """
        <meta property="og:url" content="/article">
        <meta property="og:image" content="/images/photo.jpg">
    """
    og = meta_oxide.extract_opengraph(html, "https://example.com")
    assert og["url"] == "https://example.com/article"
    assert og["image"] == "https://example.com/images/photo.jpg"


def test_twitter_card_summary():
    """Test Twitter Card extraction"""
    html = """
        <meta name="twitter:card" content="summary">
        <meta name="twitter:title" content="Test Tweet">
        <meta name="twitter:description" content="Tweet description">
        <meta name="twitter:image" content="https://example.com/image.jpg">
        <meta name="twitter:site" content="@example">
    """
    card = meta_oxide.extract_twitter(html)
    assert card["card"] == "summary"
    assert card["title"] == "Test Tweet"
    assert card["description"] == "Tweet description"
    assert card["image"] == "https://example.com/image.jpg"
    assert card["site"] == "@example"


def test_twitter_large_image():
    """Test Twitter Card with large image"""
    html = '<meta name="twitter:card" content="summary_large_image">'
    card = meta_oxide.extract_twitter(html)
    assert card["card"] == "summary_large_image"


def test_twitter_site_and_creator():
    """Test Twitter site and creator metadata"""
    html = """
        <meta name="twitter:site" content="@example">
        <meta name="twitter:site:id" content="123456">
        <meta name="twitter:creator" content="@author">
        <meta name="twitter:creator:id" content="789012">
    """
    card = meta_oxide.extract_twitter(html)
    assert card["site"] == "@example"
    assert card["site_id"] == "123456"
    assert card["creator"] == "@author"
    assert card["creator_id"] == "789012"


def test_twitter_image_alt():
    """Test Twitter image alt text"""
    html = """
        <meta name="twitter:image" content="https://example.com/image.jpg">
        <meta name="twitter:image:alt" content="Image description">
    """
    card = meta_oxide.extract_twitter(html)
    assert card["image_alt"] == "Image description"


def test_twitter_player_card():
    """Test Twitter player card"""
    html = """
        <meta name="twitter:card" content="player">
        <meta name="twitter:player" content="https://example.com/player">
        <meta name="twitter:player:width" content="1280">
        <meta name="twitter:player:height" content="720">
        <meta name="twitter:player:stream" content="https://example.com/stream.mp4">
    """
    card = meta_oxide.extract_twitter(html)
    assert card["card"] == "player"
    assert "player" in card
    player = card["player"]
    assert player["url"] == "https://example.com/player"
    assert player["width"] == 1280
    assert player["height"] == 720
    assert player["stream"] == "https://example.com/stream.mp4"


def test_twitter_app_card_iphone():
    """Test Twitter app card for iPhone"""
    html = """
        <meta name="twitter:card" content="app">
        <meta name="twitter:app:name:iphone" content="MyApp">
        <meta name="twitter:app:id:iphone" content="123456">
        <meta name="twitter:app:url:iphone" content="myapp://open">
    """
    card = meta_oxide.extract_twitter(html)
    assert "app" in card
    app = card["app"]
    assert app["name_iphone"] == "MyApp"
    assert app["id_iphone"] == "123456"
    assert app["url_iphone"] == "myapp://open"


def test_twitter_app_card_all_platforms():
    """Test Twitter app card with all platforms"""
    html = """
        <meta name="twitter:card" content="app">
        <meta name="twitter:app:name:iphone" content="MyApp">
        <meta name="twitter:app:id:iphone" content="123456">
        <meta name="twitter:app:name:ipad" content="MyApp HD">
        <meta name="twitter:app:id:ipad" content="234567">
        <meta name="twitter:app:name:googleplay" content="My Android App">
        <meta name="twitter:app:id:googleplay" content="com.example.app">
        <meta name="twitter:app:country" content="US">
    """
    card = meta_oxide.extract_twitter(html)
    assert "app" in card
    app = card["app"]
    assert app["name_iphone"] == "MyApp"
    assert app["name_ipad"] == "MyApp HD"
    assert app["name_googleplay"] == "My Android App"
    assert app["country"] == "US"


def test_twitter_relative_urls():
    """Test Twitter card with relative URL resolution"""
    html = """
        <meta name="twitter:image" content="/images/photo.jpg">
    """
    card = meta_oxide.extract_twitter(html, "https://example.com")
    assert card["image"] == "https://example.com/images/photo.jpg"


def test_twitter_fallback_to_opengraph():
    """Test Twitter fallback to Open Graph"""
    html = """
        <meta property="og:title" content="OG Title">
        <meta property="og:description" content="OG Description">
        <meta property="og:image" content="https://example.com/og-image.jpg">
    """
    card = meta_oxide.extract_twitter_with_fallback(html)
    # Should fall back to OG values
    assert card["title"] == "OG Title"
    assert card["description"] == "OG Description"
    assert card["image"] == "https://example.com/og-image.jpg"


def test_twitter_takes_precedence_over_og():
    """Test that Twitter tags take precedence over OG"""
    html = """
        <meta name="twitter:title" content="Twitter Title">
        <meta property="og:title" content="OG Title">
        <meta property="og:description" content="OG Description">
    """
    card = meta_oxide.extract_twitter_with_fallback(html)
    # Twitter title should be used
    assert card["title"] == "Twitter Title"
    # But should fall back to OG for missing fields
    assert card["description"] == "OG Description"


def test_real_world_facebook_post():
    """Test with real Facebook-style Open Graph"""
    html = """
        <!DOCTYPE html>
        <html>
        <head>
            <meta property="og:title" content="Breaking News Story">
            <meta property="og:type" content="article">
            <meta property="og:url" content="https://news.example.com/story/123">
            <meta property="og:image" content="https://news.example.com/images/story.jpg">
            <meta property="og:description" content="Important news breaking now">
            <meta property="og:site_name" content="Example News">
            <meta property="og:locale" content="en_US">
            <meta property="article:published_time" content="2024-01-15T10:00:00Z">
            <meta property="article:author" content="https://news.example.com/author/jane">
            <meta property="article:section" content="World">
            <meta property="article:tag" content="breaking">
            <meta property="article:tag" content="news">
        </head>
        </html>
    """
    og = meta_oxide.extract_opengraph(html)

    assert og["title"] == "Breaking News Story"
    assert og["type"] == "article"
    assert og["site_name"] == "Example News"
    assert og["locale"] == "en_US"
    assert "article" in og
    assert og["article"]["section"] == "World"
    assert len(og["article"]["tag"]) == 2


def test_real_world_twitter_post():
    """Test with real Twitter-style metadata"""
    html = """
        <meta name="twitter:card" content="summary_large_image">
        <meta name="twitter:site" content="@example">
        <meta name="twitter:creator" content="@author">
        <meta name="twitter:title" content="Check out this article">
        <meta name="twitter:description" content="Great insights on Rust">
        <meta name="twitter:image" content="https://example.com/article-image.jpg">
        <meta name="twitter:image:alt" content="Rust logo">
    """
    card = meta_oxide.extract_twitter(html)

    assert card["card"] == "summary_large_image"
    assert card["site"] == "@example"
    assert card["creator"] == "@author"
    assert card["image_alt"] == "Rust logo"


def test_combined_og_and_twitter():
    """Test site with both Open Graph and Twitter Cards"""
    html = """
        <meta property="og:title" content="Article Title">
        <meta property="og:image" content="https://example.com/og-image.jpg">
        <meta name="twitter:card" content="summary">
        <meta name="twitter:site" content="@example">
    """
    og = meta_oxide.extract_opengraph(html)
    card = meta_oxide.extract_twitter(html)

    assert og["title"] == "Article Title"
    assert card["card"] == "summary"


def test_empty_html():
    """Test with empty HTML"""
    html = ""
    og = meta_oxide.extract_opengraph(html)
    card = meta_oxide.extract_twitter(html)

    # Should return empty dicts, not fail
    assert isinstance(og, dict)
    assert isinstance(card, dict)


def test_no_social_tags():
    """Test with HTML that has no social tags"""
    html = """
        <html>
        <head>
            <title>Regular HTML</title>
            <meta name="description" content="Not social">
        </head>
        </html>
    """
    og = meta_oxide.extract_opengraph(html)
    card = meta_oxide.extract_twitter(html)

    assert isinstance(og, dict)
    assert isinstance(card, dict)


def test_opengraph_whitespace_trimming():
    """Test that OG content whitespace is trimmed"""
    html = """
        <meta property="og:title" content="  Title with spaces  ">
        <meta property="og:description" content="
            Description with
            newlines
        ">
    """
    og = meta_oxide.extract_opengraph(html)
    assert og["title"] == "Title with spaces"
    # Description should be trimmed
    assert not og["description"].startswith(" ")
    assert not og["description"].endswith(" ")


def test_twitter_whitespace_trimming():
    """Test that Twitter content whitespace is trimmed"""
    html = """
        <meta name="twitter:title" content="  Title with spaces  ">
    """
    card = meta_oxide.extract_twitter(html)
    assert card["title"] == "Title with spaces"


def test_real_world_product_page():
    """Test Open Graph for product page"""
    html = """
        <meta property="og:title" content="Amazing Product">
        <meta property="og:type" content="product">
        <meta property="og:url" content="https://shop.example.com/product/123">
        <meta property="og:image" content="https://shop.example.com/images/product.jpg">
        <meta property="og:image" content="https://shop.example.com/images/product-alt.jpg">
        <meta property="og:description" content="Best product ever">
        <meta property="og:site_name" content="Example Shop">
    """
    og = meta_oxide.extract_opengraph(html)

    assert og["title"] == "Amazing Product"
    assert og["type"] == "product"
    assert len(og["images"]) == 2


def test_real_world_video_page():
    """Test Open Graph for video page"""
    html = """
        <meta property="og:title" content="Funny Cat Video">
        <meta property="og:type" content="video.other">
        <meta property="og:url" content="https://videos.example.com/watch/123">
        <meta property="og:image" content="https://videos.example.com/thumbnail.jpg">
        <meta property="og:video" content="https://videos.example.com/video.mp4">
        <meta property="og:video:secure_url" content="https://videos.example.com/video.mp4">
        <meta property="og:video:type" content="video/mp4">
        <meta property="og:video:width" content="1920">
        <meta property="og:video:height" content="1080">
        <meta property="og:description" content="Watch this hilarious cat">
        <meta property="og:site_name" content="Video Site">
    """
    og = meta_oxide.extract_opengraph(html)

    assert og["title"] == "Funny Cat Video"
    assert og["type"] == "video.other"
    assert len(og["videos"]) == 1
    assert og["videos"][0]["width"] == 1920
    assert og["videos"][0]["height"] == 1080


def test_real_world_music_page():
    """Test Open Graph for music page"""
    html = """
        <meta property="og:title" content="Great Song">
        <meta property="og:type" content="music.song">
        <meta property="og:url" content="https://music.example.com/song/123">
        <meta property="og:image" content="https://music.example.com/album-art.jpg">
        <meta property="og:audio" content="https://music.example.com/preview.mp3">
        <meta property="og:audio:type" content="audio/mpeg">
        <meta property="og:description" content="Listen to this amazing track">
        <meta property="og:site_name" content="Music Streaming">
    """
    og = meta_oxide.extract_opengraph(html)

    assert og["title"] == "Great Song"
    assert og["type"] == "music.song"
    assert len(og["audios"]) == 1
    assert og["audios"][0]["type"] == "audio/mpeg"


# Phase 6: Facebook Platform Integration Tests
def test_facebook_app_id():
    """Test Facebook App ID extraction"""
    html = '<meta property="fb:app_id" content="123456789">'
    og = meta_oxide.extract_opengraph(html)
    assert og["fb_app_id"] == "123456789"


def test_facebook_admins():
    """Test Facebook Admins extraction"""
    html = '<meta property="fb:admins" content="user1,user2,user3">'
    og = meta_oxide.extract_opengraph(html)
    assert og["fb_admins"] == "user1,user2,user3"


def test_facebook_platform_with_og():
    """Test Facebook platform IDs with Open Graph tags"""
    html = """
        <meta property="og:title" content="My Website">
        <meta property="og:type" content="website">
        <meta property="og:url" content="https://example.com">
        <meta property="fb:app_id" content="987654321">
        <meta property="fb:admins" content="admin1,admin2">
    """
    og = meta_oxide.extract_opengraph(html)
    assert og["title"] == "My Website"
    assert og["type"] == "website"
    assert og["fb_app_id"] == "987654321"
    assert og["fb_admins"] == "admin1,admin2"
