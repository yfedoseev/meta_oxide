"""
Test rel-* link relationships extraction.

Tests for extracting HTML link relationships (rel-author, rel-me, rel-webmention, etc.)
which define relationships between documents.

Run with: pytest python/tests/test_rel_links.py -v
"""

import pytest

try:
    import meta_oxide

    PACKAGE_AVAILABLE = True
except ImportError:
    PACKAGE_AVAILABLE = False


@pytest.mark.skipif(not PACKAGE_AVAILABLE, reason="Package not built yet")
class TestBasicExtraction:
    """Test basic rel-* link extraction."""

    def test_single_rel_link(self):
        """Test extracting a single rel link."""
        html = '<link rel="author" href="/about">'
        links = meta_oxide.extract_rel_links(html)
        assert "author" in links
        assert links["author"] == ["/about"]

    def test_multiple_different_rel_types(self):
        """Test extracting multiple different rel types."""
        html = """
        <link rel="author" href="/about">
        <link rel="license" href="https://creativecommons.org/licenses/by/4.0/">
        <link rel="webmention" href="/webmention">
        """
        links = meta_oxide.extract_rel_links(html)
        assert "author" in links
        assert "license" in links
        assert "webmention" in links
        assert links["author"] == ["/about"]
        assert links["license"] == ["https://creativecommons.org/licenses/by/4.0/"]
        assert links["webmention"] == ["/webmention"]

    def test_same_rel_type_multiple_times(self):
        """Test same rel type appearing multiple times."""
        html = """
        <link rel="me" href="https://twitter.com/user">
        <link rel="me" href="https://github.com/user">
        <a rel="me" href="https://mastodon.social/@user">Mastodon</a>
        """
        links = meta_oxide.extract_rel_links(html)
        assert "me" in links
        assert len(links["me"]) == 3
        assert "https://twitter.com/user" in links["me"]
        assert "https://github.com/user" in links["me"]
        assert "https://mastodon.social/@user" in links["me"]

    def test_from_link_tags(self):
        """Test extracting from <link> tags."""
        html = """
        <head>
            <link rel="canonical" href="https://example.com/page">
            <link rel="alternate" type="application/rss+xml" href="/feed.xml">
        </head>
        """
        links = meta_oxide.extract_rel_links(html)
        assert "canonical" in links
        assert "alternate" in links
        assert links["canonical"] == ["https://example.com/page"]

    def test_from_a_tags(self):
        """Test extracting from <a> tags."""
        html = """
        <a rel="payment" href="https://paypal.me/user">Support Me</a>
        <a rel="author" href="/about">About the Author</a>
        """
        links = meta_oxide.extract_rel_links(html)
        assert "payment" in links
        assert "author" in links
        assert links["payment"] == ["https://paypal.me/user"]
        assert links["author"] == ["/about"]

    def test_mixed_link_and_a_tags(self):
        """Test extracting from both <link> and <a> tags."""
        html = """
        <head>
            <link rel="author" href="/about">
        </head>
        <body>
            <a rel="payment" href="https://paypal.me/user">Support</a>
        </body>
        """
        links = meta_oxide.extract_rel_links(html)
        assert "author" in links
        assert "payment" in links
        assert len(links["author"]) == 1
        assert len(links["payment"]) == 1


@pytest.mark.skipif(not PACKAGE_AVAILABLE, reason="Package not built yet")
class TestSpecificRelTypes:
    """Test specific rel-* types commonly used."""

    def test_rel_author(self):
        """Test rel-author extraction."""
        html = '<link rel="author" href="/about-the-author">'
        links = meta_oxide.extract_rel_links(html)
        assert "author" in links
        assert links["author"] == ["/about-the-author"]

    def test_rel_me_multiple(self):
        """Test rel-me for identity consolidation (IndieWeb)."""
        html = """
        <link rel="me" href="https://twitter.com/user">
        <a rel="me" href="https://github.com/user">GitHub</a>
        <a rel="me" href="https://linkedin.com/in/user">LinkedIn</a>
        """
        links = meta_oxide.extract_rel_links(html)
        assert "me" in links
        assert len(links["me"]) == 3
        assert "https://twitter.com/user" in links["me"]
        assert "https://github.com/user" in links["me"]
        assert "https://linkedin.com/in/user" in links["me"]

    def test_rel_webmention(self):
        """Test rel-webmention for IndieWeb webmention endpoint."""
        html = '<link rel="webmention" href="https://example.com/webmention">'
        links = meta_oxide.extract_rel_links(html)
        assert "webmention" in links
        assert links["webmention"] == ["https://example.com/webmention"]

    def test_rel_pingback(self):
        """Test rel-pingback for pingback endpoint."""
        html = '<link rel="pingback" href="https://example.com/xmlrpc.php">'
        links = meta_oxide.extract_rel_links(html)
        assert "pingback" in links
        assert links["pingback"] == ["https://example.com/xmlrpc.php"]

    def test_rel_license(self):
        """Test rel-license for content license."""
        html = '<a rel="license" href="https://creativecommons.org/licenses/by/4.0/">CC BY 4.0</a>'
        links = meta_oxide.extract_rel_links(html)
        assert "license" in links
        assert links["license"] == ["https://creativecommons.org/licenses/by/4.0/"]

    def test_rel_payment(self):
        """Test rel-payment for payment/donation links."""
        html = """
        <a rel="payment" href="https://paypal.me/user">PayPal</a>
        <a rel="payment" href="https://ko-fi.com/user">Ko-fi</a>
        """
        links = meta_oxide.extract_rel_links(html)
        assert "payment" in links
        assert len(links["payment"]) == 2
        assert "https://paypal.me/user" in links["payment"]
        assert "https://ko-fi.com/user" in links["payment"]

    def test_rel_search(self):
        """Test rel-search for OpenSearch description."""
        html = '<link rel="search" type="application/opensearchdescription+xml" href="/opensearch.xml">'
        links = meta_oxide.extract_rel_links(html)
        assert "search" in links
        assert links["search"] == ["/opensearch.xml"]

    def test_rel_alternate(self):
        """Test rel-alternate for alternate versions."""
        html = """
        <link rel="alternate" type="application/rss+xml" href="/feed.xml">
        <link rel="alternate" type="application/atom+xml" href="/atom.xml">
        <link rel="alternate" hreflang="es" href="/es/page">
        """
        links = meta_oxide.extract_rel_links(html)
        assert "alternate" in links
        assert len(links["alternate"]) == 3
        assert "/feed.xml" in links["alternate"]
        assert "/atom.xml" in links["alternate"]
        assert "/es/page" in links["alternate"]

    def test_rel_nofollow(self):
        """Test rel-nofollow for SEO."""
        html = '<a rel="nofollow" href="https://untrusted-site.com">Link</a>'
        links = meta_oxide.extract_rel_links(html)
        assert "nofollow" in links
        assert links["nofollow"] == ["https://untrusted-site.com"]

    def test_rel_noopener(self):
        """Test rel-noopener for security."""
        html = '<a rel="noopener" href="https://external.com" target="_blank">External</a>'
        links = meta_oxide.extract_rel_links(html)
        assert "noopener" in links
        assert links["noopener"] == ["https://external.com"]

    def test_rel_canonical(self):
        """Test rel-canonical for canonical URL."""
        html = '<link rel="canonical" href="https://example.com/canonical-page">'
        links = meta_oxide.extract_rel_links(html)
        assert "canonical" in links
        assert links["canonical"] == ["https://example.com/canonical-page"]

    def test_rel_prev_next(self):
        """Test rel-prev and rel-next for pagination."""
        html = """
        <link rel="prev" href="/page/1">
        <link rel="next" href="/page/3">
        """
        links = meta_oxide.extract_rel_links(html)
        assert "prev" in links
        assert "next" in links
        assert links["prev"] == ["/page/1"]
        assert links["next"] == ["/page/3"]


@pytest.mark.skipif(not PACKAGE_AVAILABLE, reason="Package not built yet")
class TestURLHandling:
    """Test URL resolution and handling."""

    def test_absolute_urls(self):
        """Test that absolute URLs are preserved."""
        html = '<link rel="author" href="https://example.com/about">'
        links = meta_oxide.extract_rel_links(html)
        assert links["author"] == ["https://example.com/about"]

    def test_relative_urls_with_base(self):
        """Test relative URL resolution with base_url."""
        html = '<link rel="author" href="/about">'
        links = meta_oxide.extract_rel_links(html, base_url="https://example.com")
        assert links["author"] == ["https://example.com/about"]

    def test_relative_urls_without_base(self):
        """Test relative URLs without base_url remain relative."""
        html = '<link rel="author" href="/about">'
        links = meta_oxide.extract_rel_links(html)
        assert links["author"] == ["/about"]

    def test_fragment_urls(self):
        """Test URLs with fragments."""
        html = '<a rel="author" href="/about#bio">Author Bio</a>'
        links = meta_oxide.extract_rel_links(html, base_url="https://example.com")
        assert links["author"] == ["https://example.com/about#bio"]

    def test_query_parameters(self):
        """Test URLs with query parameters."""
        html = '<link rel="search" href="/search?type=opensearch">'
        links = meta_oxide.extract_rel_links(html, base_url="https://example.com")
        assert links["search"] == ["https://example.com/search?type=opensearch"]


@pytest.mark.skipif(not PACKAGE_AVAILABLE, reason="Package not built yet")
class TestMultipleValues:
    """Test handling of multiple rel values."""

    def test_space_separated_rel_values(self):
        """Test space-separated rel values (e.g., rel='me noopener')."""
        html = '<a rel="me noopener" href="https://twitter.com/user">Twitter</a>'
        links = meta_oxide.extract_rel_links(html)
        assert "me" in links
        assert "noopener" in links
        assert links["me"] == ["https://twitter.com/user"]
        assert links["noopener"] == ["https://twitter.com/user"]

    def test_multiple_rel_values_same_url(self):
        """Test that same URL appears in multiple rel types when specified."""
        html = '<a rel="external nofollow noopener" href="https://untrusted.com">Link</a>'
        links = meta_oxide.extract_rel_links(html)
        assert "external" in links
        assert "nofollow" in links
        assert "noopener" in links
        assert links["external"] == ["https://untrusted.com"]
        assert links["nofollow"] == ["https://untrusted.com"]
        assert links["noopener"] == ["https://untrusted.com"]

    def test_case_insensitive_rel_values(self):
        """Test that rel values are normalized to lowercase."""
        html = """
        <link rel="Author" href="/about">
        <a rel="LICENSE" href="/license">License</a>
        """
        links = meta_oxide.extract_rel_links(html)
        assert "author" in links
        assert "license" in links
        # Should not have uppercase variants
        assert "Author" not in links
        assert "LICENSE" not in links

    def test_extra_whitespace_in_rel(self):
        """Test handling extra whitespace in rel attribute."""
        html = '<a rel="  me   noopener  " href="https://github.com/user">GitHub</a>'
        links = meta_oxide.extract_rel_links(html)
        assert "me" in links
        assert "noopener" in links
        # Should only have 2 rel types, whitespace shouldn't create empty entries
        assert len(links) == 2


@pytest.mark.skipif(not PACKAGE_AVAILABLE, reason="Package not built yet")
class TestEdgeCases:
    """Test edge cases and malformed input."""

    def test_missing_href(self):
        """Test that links without href are ignored."""
        html = """
        <link rel="author">
        <a rel="license">License</a>
        """
        links = meta_oxide.extract_rel_links(html)
        # Should not extract links without href
        assert "author" not in links
        assert "license" not in links

    def test_empty_rel(self):
        """Test that empty rel attribute is ignored."""
        html = '<link rel="" href="/page">'
        links = meta_oxide.extract_rel_links(html)
        # Should not create an empty key
        assert "" not in links
        assert len(links) == 0

    def test_whitespace_only_rel(self):
        """Test that whitespace-only rel is ignored."""
        html = '<link rel="   " href="/page">'
        links = meta_oxide.extract_rel_links(html)
        # Should not create entries for whitespace
        assert len(links) == 0

    def test_empty_href(self):
        """Test handling of empty href."""
        html = '<link rel="author" href="">'
        links = meta_oxide.extract_rel_links(html)
        # Depends on implementation - might have empty string or be filtered
        # At minimum should not crash
        assert isinstance(links, dict)

    def test_no_rel_links(self):
        """Test HTML with no rel links."""
        html = "<html><body><p>No links here</p></body></html>"
        links = meta_oxide.extract_rel_links(html)
        assert links == {}

    def test_malformed_html(self):
        """Test with malformed HTML."""
        html = '<link rel="author" href="/about"'  # Missing closing >
        links = meta_oxide.extract_rel_links(html)
        # Should handle gracefully
        assert isinstance(links, dict)


@pytest.mark.skipif(not PACKAGE_AVAILABLE, reason="Package not built yet")
class TestIntegration:
    """Test integration scenarios."""

    def test_real_world_blog_example(self):
        """Test a realistic blog page with multiple rel links."""
        html = """
        <html>
        <head>
            <link rel="author" href="/about">
            <link rel="license" href="https://creativecommons.org/licenses/by/4.0/">
            <link rel="webmention" href="https://webmention.io/example.com/webmention">
            <link rel="pingback" href="https://webmention.io/example.com/xmlrpc">
            <link rel="alternate" type="application/rss+xml" href="/feed.xml">
            <link rel="canonical" href="https://example.com/blog/post">
            <link rel="me" href="https://twitter.com/author">
        </head>
        <body>
            <article>
                <p>Content here</p>
                <a rel="payment" href="https://ko-fi.com/author">Support</a>
                <a rel="license" href="https://creativecommons.org/licenses/by/4.0/">CC BY 4.0</a>
            </article>
        </body>
        </html>
        """
        links = meta_oxide.extract_rel_links(html, base_url="https://example.com")

        assert "author" in links
        assert "license" in links
        assert "webmention" in links
        assert "pingback" in links
        assert "alternate" in links
        assert "canonical" in links
        assert "me" in links
        assert "payment" in links

        # Check license appears twice (link and a tag)
        assert len(links["license"]) == 2

        # Verify URL resolution worked
        assert links["author"] == ["https://example.com/about"]
        assert links["canonical"] == ["https://example.com/blog/post"]

    def test_indieweb_profile_example(self):
        """Test IndieWeb h-card with rel-me links."""
        html = """
        <div class="h-card">
            <a class="p-name u-url" rel="me" href="https://example.com">Jane Doe</a>
            <a rel="me" href="https://twitter.com/janedoe">Twitter</a>
            <a rel="me" href="https://github.com/janedoe">GitHub</a>
            <a rel="me" href="https://mastodon.social/@janedoe">Mastodon</a>
        </div>
        """
        links = meta_oxide.extract_rel_links(html)

        assert "me" in links
        assert len(links["me"]) == 4
        assert "https://example.com" in links["me"]
        assert "https://twitter.com/janedoe" in links["me"]
        assert "https://github.com/janedoe" in links["me"]
        assert "https://mastodon.social/@janedoe" in links["me"]

    def test_extract_all_includes_rel_links(self):
        """Test that extract_all() includes rel_links."""
        html = """
        <html>
        <head>
            <title>Test Page</title>
            <meta name="description" content="Test description">
            <link rel="author" href="/about">
            <link rel="license" href="https://creativecommons.org/licenses/by/4.0/">
        </head>
        </html>
        """
        result = meta_oxide.extract_all(html, base_url="https://example.com")

        # Should have standard extractions
        assert "meta" in result
        assert result["meta"]["title"] == "Test Page"

        # Should also have rel_links
        assert "rel_links" in result
        assert "author" in result["rel_links"]
        assert "license" in result["rel_links"]
        assert result["rel_links"]["author"] == ["https://example.com/about"]
