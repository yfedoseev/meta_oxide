"""
Basic tests for MetaOxide Python bindings.

Run with: pytest python/tests/
"""

import pytest

# This will fail until the package is built with maturin
try:
    import meta_oxide
    PACKAGE_AVAILABLE = True
except ImportError:
    PACKAGE_AVAILABLE = False


@pytest.mark.skipif(not PACKAGE_AVAILABLE, reason="Package not built yet")
class TestHCard:
    """Test h-card extraction."""

    def test_basic_hcard(self):
        """Test extracting a basic h-card."""
        html = """
        <div class="h-card">
            <span class="p-name">John Doe</span>
            <a class="u-url" href="https://example.com">Website</a>
            <a class="u-email" href="mailto:john@example.com">Email</a>
        </div>
        """

        cards = meta_oxide.extract_hcard(html)

        assert len(cards) == 1
        assert cards[0]['name'] == 'John Doe'
        assert cards[0]['url'] == 'https://example.com'
        assert cards[0]['email'] == 'john@example.com'

    def test_multiple_hcards(self):
        """Test extracting multiple h-cards."""
        html = """
        <div class="h-card">
            <span class="p-name">Alice</span>
        </div>
        <div class="h-card">
            <span class="p-name">Bob</span>
        </div>
        """

        cards = meta_oxide.extract_hcard(html)

        assert len(cards) == 2
        assert cards[0]['name'] == 'Alice'
        assert cards[1]['name'] == 'Bob'

    def test_empty_html(self):
        """Test with HTML containing no h-cards."""
        html = "<div>No microformats here</div>"

        cards = meta_oxide.extract_hcard(html)

        assert len(cards) == 0


@pytest.mark.skipif(not PACKAGE_AVAILABLE, reason="Package not built yet")
class TestHEntry:
    """Test h-entry extraction."""

    def test_basic_hentry(self):
        """Test extracting a basic h-entry."""
        html = """
        <article class="h-entry">
            <h1 class="p-name">My Blog Post</h1>
            <time class="dt-published" datetime="2024-01-01">January 1, 2024</time>
            <div class="e-content">This is the content.</div>
        </article>
        """

        entries = meta_oxide.extract_hentry(html)

        assert len(entries) == 1
        assert entries[0]['name'] == 'My Blog Post'
        assert entries[0]['published'] == '2024-01-01'

    def test_hentry_with_categories(self):
        """Test h-entry with multiple categories."""
        html = """
        <article class="h-entry">
            <h1 class="p-name">Test Post</h1>
            <a class="p-category" href="/tag/rust">Rust</a>
            <a class="p-category" href="/tag/python">Python</a>
        </article>
        """

        entries = meta_oxide.extract_hentry(html)

        assert len(entries) == 1
        assert 'Rust' in entries[0]['category']
        assert 'Python' in entries[0]['category']


@pytest.mark.skipif(not PACKAGE_AVAILABLE, reason="Package not built yet")
class TestHEvent:
    """Test h-event extraction."""

    def test_basic_hevent(self):
        """Test extracting a basic h-event."""
        html = """
        <div class="h-event">
            <h1 class="p-name">Conference 2024</h1>
            <time class="dt-start" datetime="2024-05-15T09:00">May 15, 2024</time>
            <span class="p-location">Convention Center</span>
        </div>
        """

        events = meta_oxide.extract_hevent(html)

        assert len(events) == 1
        assert events[0]['name'] == 'Conference 2024'
        assert events[0]['start'] == '2024-05-15T09:00'
        assert events[0]['location'] == 'Convention Center'


@pytest.mark.skipif(not PACKAGE_AVAILABLE, reason="Package not built yet")
class TestExtractAll:
    """Test extracting all microformats."""

    def test_extract_microformats(self):
        """Test extracting all microformats at once."""
        html = """
        <html>
        <body>
            <div class="h-card">
                <span class="p-name">Alice</span>
            </div>
            <article class="h-entry">
                <h1 class="p-name">Post</h1>
            </article>
        </body>
        </html>
        """

        result = meta_oxide.extract_microformats(html)

        assert 'h-card' in result
        assert 'h-entry' in result
        assert len(result['h-card']) == 1
        assert len(result['h-entry']) == 1


@pytest.mark.skipif(not PACKAGE_AVAILABLE, reason="Package not built yet")
class TestURLResolution:
    """Test URL resolution."""

    def test_url_resolution(self):
        """Test that relative URLs are resolved."""
        html = """
        <div class="h-card">
            <a class="u-url" href="/about">About</a>
        </div>
        """

        cards = meta_oxide.extract_hcard(html, base_url="https://example.com")

        assert cards[0]['url'] == 'https://example.com/about'

    def test_absolute_url_unchanged(self):
        """Test that absolute URLs remain unchanged."""
        html = """
        <div class="h-card">
            <a class="u-url" href="https://other.com/page">Page</a>
        </div>
        """

        cards = meta_oxide.extract_hcard(html, base_url="https://example.com")

        assert cards[0]['url'] == 'https://other.com/page'


@pytest.mark.skipif(not PACKAGE_AVAILABLE, reason="Package not built yet")
def test_version():
    """Test that version is available."""
    assert hasattr(meta_oxide, '__version__')
    assert isinstance(meta_oxide.__version__, str)
