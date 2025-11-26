"""Tests for microformat type conversions"""

import meta_oxide


def test_hcard_all_fields():
    """Test HCard with all possible fields"""
    html = """
        <div class="h-card">
            <span class="p-name">John Doe</span>
            <a class="u-url" href="https://example.com">Website</a>
            <img class="u-photo" src="https://example.com/photo.jpg" alt="Photo">
            <a class="u-email" href="mailto:john@example.com">Email</a>
            <span class="p-tel">555-1234</span>
            <p class="p-note">Software developer</p>
            <span class="p-org">Acme Corp</span>
        </div>
    """

    cards = meta_oxide.extract_hcard(html)

    assert len(cards) == 1
    card = cards[0]

    assert isinstance(card, dict)
    assert card["name"] == "John Doe"
    assert card["url"] == "https://example.com"
    assert card["photo"] == "https://example.com/photo.jpg"
    assert card["email"] == "john@example.com"
    assert card["tel"] == "555-1234"
    assert card["note"] == "Software developer"
    assert card["org"] == "Acme Corp"


def test_hentry_all_fields():
    """Test HEntry with all possible fields"""
    html = """
        <article class="h-entry">
            <h1 class="p-name">Blog Post Title</h1>
            <time class="dt-published" datetime="2024-01-15T10:00:00Z">Jan 15</time>
            <time class="dt-updated" datetime="2024-01-16T12:00:00Z">Jan 16</time>
            <div class="p-author h-card">
                <span class="p-name">Author Name</span>
            </div>
            <div class="e-content">
                <p>Rich HTML content here</p>
            </div>
            <p class="p-summary">Short summary</p>
            <a class="p-category" href="/tag/rust">rust</a>
            <a class="p-category" href="/tag/python">python</a>
            <a class="u-url" href="https://example.com/post">Permalink</a>
        </article>
    """

    entries = meta_oxide.extract_hentry(html)

    assert len(entries) == 1
    entry = entries[0]

    assert isinstance(entry, dict)
    assert entry["name"] == "Blog Post Title"
    assert entry["published"] == "2024-01-15T10:00:00Z"
    assert entry["updated"] == "2024-01-16T12:00:00Z"
    assert isinstance(entry["author"], dict)
    assert entry["author"]["name"] == "Author Name"
    assert "content" in entry
    assert "summary" in entry
    assert "category" in entry
    assert isinstance(entry["category"], list)
    assert entry["url"] == "https://example.com/post"


def test_hevent_all_fields():
    """Test HEvent with all possible fields"""
    html = """
        <div class="h-event">
            <h1 class="p-name">Conference 2024</h1>
            <time class="dt-start" datetime="2024-09-10T09:00:00-07:00">Sept 10, 9am</time>
            <time class="dt-end" datetime="2024-09-12T17:00:00-07:00">Sept 12, 5pm</time>
            <p class="p-location">San Francisco, CA</p>
            <p class="p-summary">Annual tech conference</p>
            <div class="e-description">
                <p>Full event description with HTML</p>
            </div>
            <a class="u-url" href="https://example.com/event">Event page</a>
        </div>
    """

    events = meta_oxide.extract_hevent(html)

    assert len(events) == 1
    event = events[0]

    assert isinstance(event, dict)
    assert event["name"] == "Conference 2024"
    assert event["start"] == "2024-09-10T09:00:00-07:00"
    assert event["end"] == "2024-09-12T17:00:00-07:00"
    assert event["location"] == "San Francisco, CA"
    assert "summary" in event
    assert "description" in event
    assert event["url"] == "https://example.com/event"


def test_hcard_minimal():
    """Test HCard with only required fields"""
    html = '<div class="h-card"><span class="p-name">Name</span></div>'
    cards = meta_oxide.extract_hcard(html)

    assert len(cards) == 1
    card = cards[0]
    assert card["name"] == "Name"
    # Optional fields should not crash


def test_multiple_microformats():
    """Test multiple microformats in same document"""
    html = """
        <div class="h-card">
            <span class="p-name">Person 1</span>
        </div>
        <div class="h-card">
            <span class="p-name">Person 2</span>
        </div>
        <article class="h-entry">
            <h1 class="p-name">Post</h1>
        </article>
        <div class="h-event">
            <span class="p-name">Event</span>
        </div>
    """

    cards = meta_oxide.extract_hcard(html)
    entries = meta_oxide.extract_hentry(html)
    events = meta_oxide.extract_hevent(html)

    assert len(cards) == 2
    assert len(entries) == 1
    assert len(events) == 1
