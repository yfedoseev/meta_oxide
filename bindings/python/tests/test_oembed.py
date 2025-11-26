"""Tests for oEmbed endpoint discovery"""

import meta_oxide


def test_oembed_json_endpoint():
    """Test JSON oEmbed endpoint discovery"""
    html = '<link rel="alternate" type="application/json+oembed" href="https://example.com/oembed?format=json&url=...">'
    oembed = meta_oxide.extract_oembed(html)
    assert "json_endpoints" in oembed
    assert len(oembed["json_endpoints"]) == 1
    assert oembed["json_endpoints"][0]["href"] == "https://example.com/oembed?format=json&url=..."
    assert oembed["json_endpoints"][0]["format"] == "json"


def test_oembed_xml_endpoint():
    """Test XML oEmbed endpoint discovery"""
    html = '<link rel="alternate" type="text/xml+oembed" href="https://example.com/oembed?format=xml&url=...">'
    oembed = meta_oxide.extract_oembed(html)
    assert "xml_endpoints" in oembed
    assert len(oembed["xml_endpoints"]) == 1
    assert oembed["xml_endpoints"][0]["href"] == "https://example.com/oembed?format=xml&url=..."
    assert oembed["xml_endpoints"][0]["format"] == "xml"


def test_oembed_with_title():
    """Test oEmbed endpoint with title attribute"""
    html = (
        '<link rel="alternate" type="application/json+oembed" '
        'href="https://example.com/oembed" title="Example oEmbed">'
    )
    oembed = meta_oxide.extract_oembed(html)
    assert len(oembed["json_endpoints"]) == 1
    assert oembed["json_endpoints"][0]["title"] == "Example oEmbed"


def test_oembed_multiple_endpoints():
    """Test discovery of multiple oEmbed endpoints"""
    html = """
        <html>
        <head>
            <link rel="alternate" type="application/json+oembed" href="https://example.com/oembed.json">
            <link rel="alternate" type="text/xml+oembed" href="https://example.com/oembed.xml">
        </head>
        </html>
    """
    oembed = meta_oxide.extract_oembed(html)
    assert len(oembed["json_endpoints"]) == 1
    assert len(oembed["xml_endpoints"]) == 1


def test_oembed_youtube_style():
    """Test YouTube-style oEmbed endpoint"""
    html = (
        '<link rel="alternate" type="application/json+oembed" '
        'href="https://www.youtube.com/oembed?format=json&'
        'url=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DdQw4w9WgXcQ" '
        'title="Rick Astley - Never Gonna Give You Up">'
    )
    oembed = meta_oxide.extract_oembed(html)
    assert len(oembed["json_endpoints"]) == 1
    assert "youtube.com/oembed" in oembed["json_endpoints"][0]["href"]
    assert oembed["json_endpoints"][0]["title"] == "Rick Astley - Never Gonna Give You Up"


def test_oembed_url_resolution():
    """Test that relative URLs in oEmbed endpoints are resolved"""
    html = '<link rel="alternate" type="application/json+oembed" href="/oembed?url=...">'
    oembed = meta_oxide.extract_oembed(html, "https://example.com")
    assert len(oembed["json_endpoints"]) == 1
    assert oembed["json_endpoints"][0]["href"] == "https://example.com/oembed?url=..."


def test_oembed_empty():
    """Test page with no oEmbed endpoints"""
    html = "<html><head><title>No oEmbed</title></head></html>"
    oembed = meta_oxide.extract_oembed(html)
    assert "json_endpoints" not in oembed or len(oembed.get("json_endpoints", [])) == 0
    assert "xml_endpoints" not in oembed or len(oembed.get("xml_endpoints", [])) == 0


def test_oembed_no_href():
    """Test oEmbed link without href attribute"""
    html = '<link rel="alternate" type="application/json+oembed">'
    oembed = meta_oxide.extract_oembed(html)
    assert "json_endpoints" not in oembed or len(oembed.get("json_endpoints", [])) == 0


def test_oembed_no_type():
    """Test link without type attribute"""
    html = '<link rel="alternate" href="https://example.com/oembed">'
    oembed = meta_oxide.extract_oembed(html)
    assert "json_endpoints" not in oembed or len(oembed.get("json_endpoints", [])) == 0


def test_oembed_wrong_rel():
    """Test oEmbed type link with wrong rel attribute"""
    html = (
        '<link rel="stylesheet" type="application/json+oembed" href="https://example.com/oembed">'
    )
    oembed = meta_oxide.extract_oembed(html)
    assert "json_endpoints" not in oembed or len(oembed.get("json_endpoints", [])) == 0


def test_oembed_vimeo_style():
    """Test Vimeo-style oEmbed endpoint"""
    json_url = "https://vimeo.com/api/oembed.json?url=https%3A%2F%2Fvimeo.com%2F123456"
    xml_url = "https://vimeo.com/api/oembed.xml?url=https%3A%2F%2Fvimeo.com%2F123456"
    html = f"""
        <html>
        <head>
            <link rel="alternate" type="application/json+oembed"
                  href="{json_url}" title="Cool Video">
            <link rel="alternate" type="text/xml+oembed"
                  href="{xml_url}" title="Cool Video">
        </head>
        </html>
    """
    oembed = meta_oxide.extract_oembed(html)
    assert len(oembed["json_endpoints"]) == 1
    assert len(oembed["xml_endpoints"]) == 1
    assert "vimeo.com/api/oembed" in oembed["json_endpoints"][0]["href"]


def test_oembed_multiple_json_endpoints():
    """Test multiple JSON oEmbed endpoints (rare but possible)"""
    html = """
        <link rel="alternate" type="application/json+oembed" href="https://example.com/oembed1.json">
        <link rel="alternate" type="application/json+oembed" href="https://example.com/oembed2.json">
    """
    oembed = meta_oxide.extract_oembed(html)
    assert len(oembed["json_endpoints"]) == 2


def test_oembed_twitter_x_style():
    """Test Twitter/X-style oEmbed endpoint"""
    html = (
        '<link rel="alternate" type="application/json+oembed" '
        'href="https://publish.twitter.com/oembed?url=https%3A%2F%2Ftwitter.com%2Fuser%2Fstatus%2F123456" '
        'title="Tweet from @user">'
    )
    oembed = meta_oxide.extract_oembed(html)
    assert len(oembed["json_endpoints"]) == 1
    assert "twitter.com/oembed" in oembed["json_endpoints"][0]["href"]
    assert oembed["json_endpoints"][0]["title"] == "Tweet from @user"


def test_oembed_soundcloud_style():
    """Test SoundCloud-style oEmbed endpoint"""
    html = (
        '<link rel="alternate" type="application/json+oembed" '
        'href="https://soundcloud.com/oembed?format=json&url=https%3A%2F%2Fsoundcloud.com%2Fartist%2Ftrack" '
        'title="Artist - Track Name">'
    )
    oembed = meta_oxide.extract_oembed(html)
    assert len(oembed["json_endpoints"]) == 1
    assert "soundcloud.com/oembed" in oembed["json_endpoints"][0]["href"]
    assert oembed["json_endpoints"][0]["format"] == "json"


def test_oembed_instagram_style():
    """Test Instagram-style oEmbed endpoint"""
    html = (
        '<link rel="alternate" type="application/json+oembed" '
        'href="https://api.instagram.com/oembed?url=https%3A%2F%2Fwww.instagram.com%2Fp%2FAbc123%2F">'
    )
    oembed = meta_oxide.extract_oembed(html)
    assert len(oembed["json_endpoints"]) == 1
    assert "instagram.com/oembed" in oembed["json_endpoints"][0]["href"]


def test_oembed_flickr_style():
    """Test Flickr-style oEmbed endpoint"""
    html = """
        <link rel="alternate" type="application/json+oembed"
              href="https://www.flickr.com/services/oembed?format=json&url=https%3A%2F%2Fwww.flickr.com%2Fphotos%2Fuser%2F123456"
              title="Photo Title">
    """
    oembed = meta_oxide.extract_oembed(html)
    assert len(oembed["json_endpoints"]) == 1
    assert "flickr.com/services/oembed" in oembed["json_endpoints"][0]["href"]


def test_oembed_spotify_style():
    """Test Spotify-style oEmbed endpoint"""
    html = (
        '<link rel="alternate" type="application/json+oembed" '
        'href="https://open.spotify.com/oembed?url=https://open.spotify.com/track/123abc">'
    )
    oembed = meta_oxide.extract_oembed(html)
    assert len(oembed["json_endpoints"]) == 1
    assert "spotify.com/oembed" in oembed["json_endpoints"][0]["href"]


def test_oembed_case_insensitive_type():
    """Test case-insensitive type attribute matching"""
    html = '<link rel="alternate" type="APPLICATION/JSON+OEMBED" href="https://example.com/oembed">'
    oembed = meta_oxide.extract_oembed(html)
    assert len(oembed["json_endpoints"]) == 1
    assert oembed["json_endpoints"][0]["format"] == "json"


def test_oembed_whitespace_in_attributes():
    """Test handling of extra whitespace in attributes"""
    html = (
        '<link rel="alternate" type=" application/json+oembed " '
        'href=" https://example.com/oembed " title=" Video Title ">'
    )
    oembed = meta_oxide.extract_oembed(html)
    assert len(oembed["json_endpoints"]) == 1
    # The href should be the URL without leading/trailing whitespace
    assert oembed["json_endpoints"][0]["href"].strip() == "https://example.com/oembed"


def test_oembed_relative_url_with_path():
    """Test relative URL resolution with path"""
    html = '<link rel="alternate" type="application/json+oembed" href="../oembed/endpoint">'
    oembed = meta_oxide.extract_oembed(html, "https://example.com/video/watch")
    assert len(oembed["json_endpoints"]) == 1
    # Should resolve relative to base URL
    assert "example.com" in oembed["json_endpoints"][0]["href"]


def test_oembed_query_parameters_preserved():
    """Test that complex query parameters are preserved"""
    query = "format=json&url=https%3A%2F%2Fexample.com%2Fvideo%2F123&maxwidth=500&maxheight=400"
    html = f'<link rel="alternate" type="application/json+oembed" href="https://example.com/oembed?{query}">'
    oembed = meta_oxide.extract_oembed(html)
    assert len(oembed["json_endpoints"]) == 1
    assert "maxwidth=500" in oembed["json_endpoints"][0]["href"]
    assert "maxheight=400" in oembed["json_endpoints"][0]["href"]


def test_oembed_mixed_providers():
    """Test page with oEmbed links from different providers"""
    html = """
        <html>
        <head>
            <link rel="alternate" type="application/json+oembed"
                  href="https://www.youtube.com/oembed?url=https://youtube.com/watch?v=123"
                  title="YouTube Video">
            <link rel="alternate" type="application/json+oembed"
                  href="https://vimeo.com/api/oembed.json?url=https://vimeo.com/456"
                  title="Vimeo Video">
            <link rel="alternate" type="text/xml+oembed"
                  href="https://soundcloud.com/oembed?format=xml&url=https://soundcloud.com/track/789"
                  title="SoundCloud Track">
        </head>
        </html>
    """
    oembed = meta_oxide.extract_oembed(html)
    assert len(oembed["json_endpoints"]) == 2
    assert len(oembed["xml_endpoints"]) == 1


def test_oembed_empty_href():
    """Test oEmbed link with empty href attribute"""
    html = '<link rel="alternate" type="application/json+oembed" href="">'
    oembed = meta_oxide.extract_oembed(html)
    # Empty href should not create an endpoint
    assert "json_endpoints" not in oembed or len(oembed.get("json_endpoints", [])) == 0


def test_oembed_in_extract_all():
    """Test that oEmbed is included in extract_all() results"""
    html = """
        <html>
        <head>
            <title>Test Page</title>
            <meta name="description" content="A test page">
            <link rel="alternate" type="application/json+oembed"
                  href="https://www.youtube.com/oembed?url=https://youtube.com/watch?v=123"
                  title="Embedded Video">
        </head>
        <body>
            <h1>Test</h1>
        </body>
        </html>
    """
    result = meta_oxide.extract_all(html, "https://example.com")
    assert "oembed" in result
    assert "json_endpoints" in result["oembed"]
    assert len(result["oembed"]["json_endpoints"]) == 1
    assert result["oembed"]["json_endpoints"][0]["title"] == "Embedded Video"


def test_oembed_real_world_blog_post():
    """Test real-world scenario: blog post with embedded YouTube video"""
    html = """
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8">
            <title>My Blog Post About Videos</title>
            <meta name="description" content="Check out this amazing video!">
            <meta property="og:title" content="My Blog Post About Videos">
            <meta property="og:type" content="article">
            <link rel="alternate" type="application/json+oembed"
                  href="https://www.youtube.com/oembed?format=json&url=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DdQw4w9WgXcQ"
                  title="Rick Astley - Never Gonna Give You Up (Official Video)">
        </head>
        <body>
            <article>
                <h1>My Blog Post About Videos</h1>
                <p>Check out this video...</p>
            </article>
        </body>
        </html>
    """
    oembed = meta_oxide.extract_oembed(html, "https://blog.example.com")
    assert len(oembed["json_endpoints"]) == 1
    assert "youtube.com/oembed" in oembed["json_endpoints"][0]["href"]
    assert "Rick Astley" in oembed["json_endpoints"][0]["title"]
    assert oembed["json_endpoints"][0]["format"] == "json"


def test_oembed_no_title_attribute():
    """Test oEmbed endpoint without title (optional field)"""
    html = '<link rel="alternate" type="application/json+oembed" href="https://example.com/oembed">'
    oembed = meta_oxide.extract_oembed(html)
    assert len(oembed["json_endpoints"]) == 1
    assert (
        "title" not in oembed["json_endpoints"][0]
        or oembed["json_endpoints"][0].get("title") is None
    )
