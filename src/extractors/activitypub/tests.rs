use super::*;

#[test]
fn discovers_alternate_activity_json() {
    let html = r#"
        <link rel="alternate" type="application/activity+json" href="/users/alice">
        <meta name="fediverse:creator" content="@alice@mastodon.example">
        <a rel="me" href="https://mastodon.example/@alice">me</a>
    "#;
    let d = extract(html, Some("https://alice.example")).unwrap();
    assert_eq!(d.alternate_url.as_deref(), Some("https://alice.example/users/alice"));
    assert_eq!(d.fediverse_creator.as_deref(), Some("@alice@mastodon.example"));
    assert_eq!(d.rel_me.len(), 1);
    assert!(d.rel_me[0].ends_with("@alice"));
}

#[test]
fn parse_actor_person() {
    let json = r#"{
      "@context": "https://www.w3.org/ns/activitystreams",
      "type": "Person",
      "id": "https://mastodon.example/users/alice",
      "preferredUsername": "alice",
      "name": "Alice",
      "summary": "<p>bio</p>",
      "inbox": "https://mastodon.example/users/alice/inbox",
      "outbox": "https://mastodon.example/users/alice/outbox",
      "followers": "https://mastodon.example/users/alice/followers",
      "following": "https://mastodon.example/users/alice/following",
      "url": "https://mastodon.example/@alice",
      "icon": { "type": "Image", "url": "https://cdn.example/avatar.png" }
    }"#;
    let a = parse_as2_actor(json).unwrap();
    assert_eq!(a.kind.as_deref(), Some("Person"));
    assert_eq!(a.preferred_username.as_deref(), Some("alice"));
    assert_eq!(a.inbox.as_deref(), Some("https://mastodon.example/users/alice/inbox"));
    assert_eq!(a.icon.as_deref(), Some("https://cdn.example/avatar.png"));
}

#[test]
fn parse_object_note() {
    let json = r#"{
      "@context": "https://www.w3.org/ns/activitystreams",
      "type": "Note",
      "id": "https://example/statuses/1",
      "attributedTo": "https://example/users/alice",
      "content": "<p>hello</p>",
      "published": "2026-04-18T10:00:00Z",
      "url": "https://example/@alice/1"
    }"#;
    let o = parse_as2_object(json).unwrap();
    assert_eq!(o.kind.as_deref(), Some("Note"));
    assert_eq!(o.attributed_to.as_deref(), Some("https://example/users/alice"));
    assert_eq!(o.published.as_deref(), Some("2026-04-18T10:00:00Z"));
}

#[test]
fn parse_invalid_json_errors() {
    assert!(parse_as2_actor("not json").is_err());
}

#[test]
fn empty_discovery_on_plain_html() {
    assert!(extract("<html><head></head></html>", None).unwrap().is_empty());
}
