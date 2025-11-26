"""Tests for OpenGraph and TwitterCard type conversions"""

import meta_oxide


def test_opengraph_all_basic_fields():
    """Test all OpenGraph basic fields"""
    html = """
        <meta property="og:title" content="Test Title">
        <meta property="og:type" content="article">
        <meta property="og:url" content="https://example.com">
        <meta property="og:image" content="https://example.com/image.jpg">
        <meta property="og:description" content="Test Description">
        <meta property="og:site_name" content="Test Site">
        <meta property="og:locale" content="en_US">
        <meta property="og:locale:alternate" content="es_ES">
        <meta property="og:locale:alternate" content="fr_FR">
    """

    og = meta_oxide.extract_opengraph(html)

    assert isinstance(og, dict)
    assert og["title"] == "Test Title"
    assert og["type"] == "article"
    assert og["url"] == "https://example.com"
    assert og["image"] == "https://example.com/image.jpg"
    assert og["description"] == "Test Description"
    assert og["site_name"] == "Test Site"
    assert og["locale"] == "en_US"

    assert "locale_alternate" in og
    assert isinstance(og["locale_alternate"], list)
    assert len(og["locale_alternate"]) == 2
    assert "es_ES" in og["locale_alternate"]
    assert "fr_FR" in og["locale_alternate"]


def test_og_image_with_all_metadata():
    """Test OgImage type conversion with all fields"""
    html = """
        <meta property="og:image" content="https://example.com/image.jpg">
        <meta property="og:image:secure_url" content="https://example.com/image.jpg">
        <meta property="og:image:type" content="image/jpeg">
        <meta property="og:image:width" content="1200">
        <meta property="og:image:height" content="630">
        <meta property="og:image:alt" content="Image description">
    """

    og = meta_oxide.extract_opengraph(html)

    assert "images" in og
    assert len(og["images"]) >= 1
    img = og["images"][0]

    assert isinstance(img, dict)
    assert img["url"] == "https://example.com/image.jpg"
    assert img["secure_url"] == "https://example.com/image.jpg"
    assert img["type"] == "image/jpeg"
    assert img["width"] == 1200
    assert img["height"] == 630
    assert img["alt"] == "Image description"


def test_og_video_with_all_metadata():
    """Test OgVideo type conversion with all fields"""
    html = """
        <meta property="og:video" content="https://example.com/video.mp4">
        <meta property="og:video:secure_url" content="https://example.com/video.mp4">
        <meta property="og:video:type" content="video/mp4">
        <meta property="og:video:width" content="1280">
        <meta property="og:video:height" content="720">
    """

    og = meta_oxide.extract_opengraph(html)

    assert "videos" in og
    assert len(og["videos"]) >= 1
    video = og["videos"][0]

    assert isinstance(video, dict)
    assert video["url"] == "https://example.com/video.mp4"
    assert video["secure_url"] == "https://example.com/video.mp4"
    assert video["type"] == "video/mp4"
    assert video["width"] == 1280
    assert video["height"] == 720


def test_og_audio_with_all_metadata():
    """Test OgAudio type conversion"""
    html = """
        <meta property="og:audio" content="https://example.com/audio.mp3">
        <meta property="og:audio:secure_url" content="https://example.com/audio.mp3">
        <meta property="og:audio:type" content="audio/mpeg">
    """

    og = meta_oxide.extract_opengraph(html)

    assert "audios" in og
    assert len(og["audios"]) >= 1
    audio = og["audios"][0]

    assert isinstance(audio, dict)
    assert audio["url"] == "https://example.com/audio.mp3"
    assert audio["secure_url"] == "https://example.com/audio.mp3"
    assert audio["type"] == "audio/mpeg"


def test_og_article_all_fields():
    """Test OgArticle type conversion with all fields"""
    html = """
        <meta property="og:type" content="article">
        <meta property="article:published_time" content="2024-01-15T10:00:00Z">
        <meta property="article:modified_time" content="2024-01-16T12:00:00Z">
        <meta property="article:expiration_time" content="2025-01-15T10:00:00Z">
        <meta property="article:author" content="https://example.com/author1">
        <meta property="article:author" content="https://example.com/author2">
        <meta property="article:section" content="Technology">
        <meta property="article:tag" content="rust">
        <meta property="article:tag" content="python">
        <meta property="article:tag" content="testing">
    """

    og = meta_oxide.extract_opengraph(html)

    assert "article" in og
    article = og["article"]

    assert isinstance(article, dict)
    assert article["published_time"] == "2024-01-15T10:00:00Z"
    assert article["modified_time"] == "2024-01-16T12:00:00Z"
    assert article["expiration_time"] == "2025-01-15T10:00:00Z"

    assert isinstance(article["author"], list)
    assert len(article["author"]) == 2
    assert "https://example.com/author1" in article["author"]

    assert article["section"] == "Technology"

    assert isinstance(article["tag"], list)
    assert len(article["tag"]) == 3
    assert "rust" in article["tag"]
    assert "python" in article["tag"]
    assert "testing" in article["tag"]


def test_og_book_all_fields():
    """Test OgBook type conversion"""
    html = """
        <meta property="og:type" content="book">
        <meta property="book:author" content="https://example.com/author">
        <meta property="book:isbn" content="978-3-16-148410-0">
        <meta property="book:release_date" content="2024-01-15">
        <meta property="book:tag" content="fiction">
        <meta property="book:tag" content="mystery">
    """

    og = meta_oxide.extract_opengraph(html)

    assert "book" in og
    book = og["book"]

    assert isinstance(book, dict)
    assert isinstance(book["author"], list)
    assert "https://example.com/author" in book["author"]
    assert book["isbn"] == "978-3-16-148410-0"
    assert book["release_date"] == "2024-01-15"
    assert isinstance(book["tag"], list)
    assert len(book["tag"]) == 2


def test_og_profile_all_fields():
    """Test OgProfile type conversion"""
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

    assert isinstance(profile, dict)
    assert profile["first_name"] == "John"
    assert profile["last_name"] == "Doe"
    assert profile["username"] == "johndoe"
    assert profile["gender"] == "male"


def test_twitter_card_all_basic_fields():
    """Test all TwitterCard basic fields"""
    html = """
        <meta name="twitter:card" content="summary_large_image">
        <meta name="twitter:title" content="Tweet Title">
        <meta name="twitter:description" content="Tweet description">
        <meta name="twitter:image" content="https://example.com/image.jpg">
        <meta name="twitter:image:alt" content="Image alt text">
        <meta name="twitter:site" content="@example">
        <meta name="twitter:site:id" content="123456">
        <meta name="twitter:creator" content="@creator">
        <meta name="twitter:creator:id" content="789012">
    """

    card = meta_oxide.extract_twitter(html)

    assert isinstance(card, dict)
    assert card["card"] == "summary_large_image"
    assert card["title"] == "Tweet Title"
    assert card["description"] == "Tweet description"
    assert card["image"] == "https://example.com/image.jpg"
    assert card["image_alt"] == "Image alt text"
    assert card["site"] == "@example"
    assert card["site_id"] == "123456"
    assert card["creator"] == "@creator"
    assert card["creator_id"] == "789012"


def test_twitter_player_all_fields():
    """Test TwitterPlayer type conversion"""
    html = """
        <meta name="twitter:card" content="player">
        <meta name="twitter:player" content="https://example.com/player">
        <meta name="twitter:player:width" content="1280">
        <meta name="twitter:player:height" content="720">
        <meta name="twitter:player:stream" content="https://example.com/stream.mp4">
    """

    card = meta_oxide.extract_twitter(html)

    assert "player" in card
    player = card["player"]

    assert isinstance(player, dict)
    assert player["url"] == "https://example.com/player"
    assert player["width"] == 1280
    assert player["height"] == 720
    assert player["stream"] == "https://example.com/stream.mp4"


def test_twitter_app_all_fields():
    """Test TwitterApp type conversion with all platform fields"""
    html = """
        <meta name="twitter:card" content="app">
        <meta name="twitter:app:name:iphone" content="MyApp iOS">
        <meta name="twitter:app:id:iphone" content="123456">
        <meta name="twitter:app:url:iphone" content="myapp://open">
        <meta name="twitter:app:name:ipad" content="MyApp iPad">
        <meta name="twitter:app:id:ipad" content="123457">
        <meta name="twitter:app:url:ipad" content="myapp://open">
        <meta name="twitter:app:name:googleplay" content="MyApp Android">
        <meta name="twitter:app:id:googleplay" content="com.example.myapp">
        <meta name="twitter:app:url:googleplay" content="myapp://open">
        <meta name="twitter:app:country" content="US">
    """

    card = meta_oxide.extract_twitter(html)

    assert "app" in card
    app = card["app"]

    assert isinstance(app, dict)
    assert app["name_iphone"] == "MyApp iOS"
    assert app["id_iphone"] == "123456"
    assert app["url_iphone"] == "myapp://open"
    assert app["name_ipad"] == "MyApp iPad"
    assert app["id_ipad"] == "123457"
    assert app["url_ipad"] == "myapp://open"
    assert app["name_googleplay"] == "MyApp Android"
    assert app["id_googleplay"] == "com.example.myapp"
    assert app["url_googleplay"] == "myapp://open"
    assert app["country"] == "US"


def test_opengraph_with_none_values():
    """Test OpenGraph with minimal data (mostly None)"""
    html = '<meta property="og:title" content="Only Title">'
    og = meta_oxide.extract_opengraph(html)

    assert isinstance(og, dict)
    assert og["title"] == "Only Title"
    # Other fields should not be present or handle None


def test_twitter_with_none_values():
    """Test TwitterCard with minimal data"""
    html = '<meta name="twitter:card" content="summary">'
    card = meta_oxide.extract_twitter(html)

    assert isinstance(card, dict)
    assert card["card"] == "summary"
    # Other fields should not be present or handle None
