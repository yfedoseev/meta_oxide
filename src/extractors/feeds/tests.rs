use super::*;
use crate::types::feeds::FeedKind;

#[test]
fn parses_rss_2_basic() {
    let xml = r#"<?xml version="1.0"?>
<rss version="2.0">
  <channel>
    <title>Example Feed</title>
    <link>https://example.com/</link>
    <description>An RSS feed.</description>
    <language>en-us</language>
    <lastBuildDate>Sat, 18 Apr 2026 10:00:00 +0000</lastBuildDate>
    <item>
      <title>First post</title>
      <link>https://example.com/1</link>
      <guid>https://example.com/1</guid>
      <description>Hello &amp; welcome.</description>
      <pubDate>Sat, 18 Apr 2026 09:30:00 +0000</pubDate>
      <category>Intro</category>
      <enclosure url="https://example.com/1.mp3" type="audio/mpeg" length="1234"/>
    </item>
  </channel>
</rss>"#;
    let feed = parse_rss(xml).unwrap();
    assert_eq!(feed.kind, FeedKind::Rss);
    assert_eq!(feed.title.as_deref(), Some("Example Feed"));
    assert_eq!(feed.home_page_url.as_deref(), Some("https://example.com/"));
    assert_eq!(feed.language.as_deref(), Some("en-us"));
    assert_eq!(feed.items.len(), 1);
    let it = &feed.items[0];
    assert_eq!(it.title.as_deref(), Some("First post"));
    assert_eq!(it.summary.as_deref(), Some("Hello & welcome."));
    assert_eq!(it.tags, vec!["Intro".to_string()]);
    assert_eq!(it.enclosure_url.as_deref(), Some("https://example.com/1.mp3"));
    assert_eq!(it.enclosure_type.as_deref(), Some("audio/mpeg"));
}

#[test]
fn parses_atom_basic() {
    let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>Atom Feed</title>
  <subtitle>notes</subtitle>
  <link href="https://example.com/" rel="alternate"/>
  <link href="https://example.com/feed.atom" rel="self"/>
  <updated>2026-04-18T10:00:00Z</updated>
  <author><name>Alice</name><email>alice@example.com</email></author>
  <entry>
    <id>tag:example.com,2026:1</id>
    <title>Post</title>
    <link href="https://example.com/1" rel="alternate"/>
    <summary>brief</summary>
    <content type="html">&lt;p&gt;full&lt;/p&gt;</content>
    <published>2026-04-18T09:30:00Z</published>
    <updated>2026-04-18T09:35:00Z</updated>
    <category term="blog"/>
  </entry>
</feed>"#;
    let feed = parse_atom(xml).unwrap();
    assert_eq!(feed.kind, FeedKind::Atom);
    assert_eq!(feed.title.as_deref(), Some("Atom Feed"));
    assert_eq!(feed.description.as_deref(), Some("notes"));
    assert_eq!(feed.home_page_url.as_deref(), Some("https://example.com/"));
    assert_eq!(feed.feed_url.as_deref(), Some("https://example.com/feed.atom"));
    assert_eq!(feed.authors.len(), 1);
    assert_eq!(feed.authors[0].name.as_deref(), Some("Alice"));
    assert_eq!(feed.items.len(), 1);
    let it = &feed.items[0];
    assert_eq!(it.title.as_deref(), Some("Post"));
    assert_eq!(it.url.as_deref(), Some("https://example.com/1"));
    assert!(it.content_html.as_deref().unwrap().contains("full"));
    assert_eq!(it.tags, vec!["blog".to_string()]);
}

#[test]
fn parses_json_feed_1_1() {
    let json = r#"{
        "version": "https://jsonfeed.org/version/1.1",
        "title": "JSON Feed Example",
        "home_page_url": "https://example.com/",
        "feed_url": "https://example.com/feed.json",
        "language": "en",
        "authors": [{"name": "Alice", "url": "https://example.com/alice"}],
        "items": [{
            "id": "1",
            "title": "Item One",
            "url": "https://example.com/1",
            "content_html": "<p>hi</p>",
            "date_published": "2026-04-18T09:00:00Z",
            "tags": ["news", "tech"],
            "attachments": [{"url": "https://example.com/1.mp3", "mime_type": "audio/mpeg"}]
        }]
    }"#;
    let feed = parse_json_feed(json).unwrap();
    assert_eq!(feed.kind, FeedKind::JsonFeed);
    assert_eq!(feed.title.as_deref(), Some("JSON Feed Example"));
    assert_eq!(feed.authors[0].name.as_deref(), Some("Alice"));
    let it = &feed.items[0];
    assert_eq!(it.tags, vec!["news".to_string(), "tech".to_string()]);
    assert_eq!(it.enclosure_url.as_deref(), Some("https://example.com/1.mp3"));
}

#[test]
fn sniff_dispatches_correctly() {
    let rss_feed = parse(
        b"<?xml version=\"1.0\"?><rss version=\"2.0\"><channel><title>x</title></channel></rss>",
    )
    .unwrap();
    assert_eq!(rss_feed.kind, FeedKind::Rss);

    let atom_feed = parse(b"<?xml version=\"1.0\"?><feed xmlns=\"http://www.w3.org/2005/Atom\"><title>x</title></feed>").unwrap();
    assert_eq!(atom_feed.kind, FeedKind::Atom);

    let json_feed = parse(br#"{"version":"1.1","title":"x"}"#).unwrap();
    assert_eq!(json_feed.kind, FeedKind::JsonFeed);
}

#[test]
fn empty_payload_errors() {
    assert!(parse(b"").is_err());
    assert!(parse(b"   ").is_err());
}

#[test]
fn garbage_errors_not_panics() {
    assert!(parse(b"not a feed").is_err());
    assert!(parse_rss("<rss><broken").is_err() || parse_rss("<rss><broken").is_ok());
}
