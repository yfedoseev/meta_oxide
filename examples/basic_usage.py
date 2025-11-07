"""
Basic usage examples for MetaOxide.

Run this after installing the package:
    pip install meta-oxide
    # or for development:
    maturin develop

Then run:
    python examples/basic_usage.py
"""

import meta_oxide


def example_hcard():
    """Extract h-card (contact information) from HTML."""
    print("=" * 60)
    print("Example 1: Extracting h-card (Contact Information)")
    print("=" * 60)

    html = """
    <div class="h-card">
        <img class="u-photo" src="https://example.com/photo.jpg" alt="Profile">
        <a class="p-name u-url" href="https://jane.example.com">Jane Doe</a>
        <a class="u-email" href="mailto:jane@example.com">jane@example.com</a>
        <span class="p-tel">+1-555-0123</span>
        <span class="p-org">Example Corp</span>
        <p class="p-note">Software engineer passionate about open source.</p>
    </div>
    """

    cards = meta_oxide.extract_hcard(html)

    for i, card in enumerate(cards, 1):
        print(f"\nCard {i}:")
        print(f"  Name: {card.get('name')}")
        print(f"  URL: {card.get('url')}")
        print(f"  Email: {card.get('email')}")
        print(f"  Phone: {card.get('tel')}")
        print(f"  Organization: {card.get('org')}")
        print(f"  Bio: {card.get('note')}")
        print(f"  Photo: {card.get('photo')}")


def example_hentry():
    """Extract h-entry (blog post) from HTML."""
    print("\n" + "=" * 60)
    print("Example 2: Extracting h-entry (Blog Post)")
    print("=" * 60)

    html = """
    <article class="h-entry">
        <h1 class="p-name">Getting Started with Microformats</h1>
        <p class="p-summary">
            Learn how to add structured data to your website using microformats.
        </p>
        <time class="dt-published" datetime="2024-01-15T10:00:00Z">
            January 15, 2024
        </time>
        <time class="dt-updated" datetime="2024-01-16T14:30:00Z">
            Updated: January 16, 2024
        </time>
        <div class="p-author h-card">
            <span class="p-name">John Smith</span>
            <a class="u-url" href="https://john.example.com">john.example.com</a>
        </div>
        <div class="e-content">
            <p>Microformats are a simple way to add semantic markup to HTML...</p>
            <p>They enable machines to understand the content better.</p>
        </div>
        <a class="p-category" href="/tag/web">Web Development</a>
        <a class="p-category" href="/tag/microformats">Microformats</a>
        <a class="u-url" href="/posts/microformats-intro">Permalink</a>
    </article>
    """

    entries = meta_oxide.extract_hentry(html)

    for i, entry in enumerate(entries, 1):
        print(f"\nEntry {i}:")
        print(f"  Title: {entry.get('name')}")
        print(f"  Summary: {entry.get('summary')}")
        print(f"  Published: {entry.get('published')}")
        print(f"  Updated: {entry.get('updated')}")
        print(f"  URL: {entry.get('url')}")

        author = entry.get('author')
        if author:
            print(f"  Author: {author.get('name')} ({author.get('url')})")

        categories = entry.get('category', [])
        if categories:
            print(f"  Categories: {', '.join(categories)}")

        content = entry.get('content', '')
        if content:
            # Show first 100 characters of content
            preview = content[:100] + "..." if len(content) > 100 else content
            print(f"  Content: {preview}")


def example_hevent():
    """Extract h-event (event information) from HTML."""
    print("\n" + "=" * 60)
    print("Example 3: Extracting h-event (Event)")
    print("=" * 60)

    html = """
    <div class="h-event">
        <h1 class="p-name">RustConf 2024</h1>
        <p class="p-summary">
            The official Rust programming language conference
        </p>
        <time class="dt-start" datetime="2024-09-10T09:00:00-07:00">
            September 10, 2024 at 9:00 AM PDT
        </time>
        <time class="dt-end" datetime="2024-09-12T17:00:00-07:00">
            September 12, 2024 at 5:00 PM PDT
        </time>
        <p class="p-location">Montreal Convention Center, Montreal, Canada</p>
        <a class="u-url" href="https://rustconf.com">Event Website</a>
        <div class="p-description">
            Join the Rust community for three days of talks, workshops, and networking.
        </div>
    </div>
    """

    events = meta_oxide.extract_hevent(html)

    for i, event in enumerate(events, 1):
        print(f"\nEvent {i}:")
        print(f"  Name: {event.get('name')}")
        print(f"  Summary: {event.get('summary')}")
        print(f"  Start: {event.get('start')}")
        print(f"  End: {event.get('end')}")
        print(f"  Location: {event.get('location')}")
        print(f"  URL: {event.get('url')}")
        print(f"  Description: {event.get('description')}")


def example_extract_all():
    """Extract all microformats at once."""
    print("\n" + "=" * 60)
    print("Example 4: Extracting All Microformats")
    print("=" * 60)

    html = """
    <html>
    <body>
        <div class="h-card">
            <span class="p-name">Alice Johnson</span>
            <a class="u-email" href="mailto:alice@example.com">Email</a>
        </div>

        <article class="h-entry">
            <h1 class="p-name">My First Blog Post</h1>
            <time class="dt-published" datetime="2024-01-01">Jan 1, 2024</time>
        </article>

        <div class="h-event">
            <h2 class="p-name">Meetup</h2>
            <time class="dt-start" datetime="2024-02-15T18:00">Feb 15, 2024</time>
        </div>
    </body>
    </html>
    """

    all_microformats = meta_oxide.extract_microformats(html)

    print(f"\nFound microformats:")
    for mf_type, items in all_microformats.items():
        print(f"  {mf_type}: {len(items)} item(s)")

    # Access specific types
    if 'h-card' in all_microformats:
        print(f"\n  First h-card name: {all_microformats['h-card'][0].get('name')}")

    if 'h-entry' in all_microformats:
        print(f"  First h-entry title: {all_microformats['h-entry'][0].get('name')}")

    if 'h-event' in all_microformats:
        print(f"  First h-event name: {all_microformats['h-event'][0].get('name')}")


def example_url_resolution():
    """Demonstrate URL resolution."""
    print("\n" + "=" * 60)
    print("Example 5: URL Resolution")
    print("=" * 60)

    html = """
    <div class="h-card">
        <span class="p-name">Bob Wilson</span>
        <a class="u-url" href="/about">About Page</a>
        <img class="u-photo" src="/images/photo.jpg" alt="Photo">
    </div>
    """

    base_url = "https://example.com/users/bob"

    cards = meta_oxide.extract_hcard(html, base_url=base_url)

    print(f"\nBase URL: {base_url}")
    print(f"\nWithout URL resolution:")
    print(f"  URL would be: /about")
    print(f"  Photo would be: /images/photo.jpg")

    print(f"\nWith URL resolution:")
    print(f"  URL: {cards[0].get('url')}")
    print(f"  Photo: {cards[0].get('photo')}")


if __name__ == "__main__":
    try:
        example_hcard()
        example_hentry()
        example_hevent()
        example_extract_all()
        example_url_resolution()

        print("\n" + "=" * 60)
        print("All examples completed successfully!")
        print("=" * 60)

    except Exception as e:
        print(f"\nError: {e}")
        print("\nMake sure you've installed meta_oxide:")
        print("  pip install meta-oxide")
        print("  # or for development:")
        print("  maturin develop")
