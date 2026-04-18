use super::*;

#[test]
fn extracts_farcaster_frame_basic() {
    let html = r#"
        <meta name="fc:frame" content="vNext">
        <meta name="fc:frame:image" content="https://img.example/cast.png">
        <meta name="fc:frame:post_url" content="https://frame.example/post">
        <meta name="fc:frame:button:1" content="Like">
        <meta name="fc:frame:button:2" content="Open">
        <meta name="fc:frame:button:2:action" content="link">
        <meta name="fc:frame:button:2:target" content="https://target.example">
    "#;
    let frame = extract(html, None).unwrap();
    assert!(frame.has_farcaster);
    assert!(!frame.has_open_frames);
    assert_eq!(frame.version.as_deref(), Some("vNext"));
    assert_eq!(frame.image.as_deref(), Some("https://img.example/cast.png"));
    assert_eq!(frame.post_url.as_deref(), Some("https://frame.example/post"));
    assert_eq!(frame.buttons.len(), 2);
    assert_eq!(frame.buttons[0].label.as_deref(), Some("Like"));
    assert_eq!(frame.buttons[1].action.as_deref(), Some("link"));
    assert_eq!(frame.buttons[1].target.as_deref(), Some("https://target.example/"));
}

#[test]
fn extracts_open_frames_basic() {
    let html = r#"
        <meta property="of:version" content="vNext">
        <meta property="of:image" content="/img.png">
        <meta property="of:button:1" content="Vote">
        <meta property="of:accepts:xmtp" content="2024-02-09">
        <meta property="of:accepts:lens" content="1.0">
    "#;
    let frame = extract(html, Some("https://host.example")).unwrap();
    assert!(frame.has_open_frames);
    assert!(!frame.has_farcaster);
    assert_eq!(frame.image.as_deref(), Some("https://host.example/img.png"));
    assert_eq!(frame.accepts.len(), 2);
    assert_eq!(frame.buttons.len(), 1);
    assert_eq!(frame.buttons[0].label.as_deref(), Some("Vote"));
}

#[test]
fn empty_when_no_frame_tags() {
    let html = r#"<meta name="description" content="nope">"#;
    let frame = extract(html, None).unwrap();
    assert!(frame.is_empty());
}

#[test]
fn ignores_out_of_range_button_indices() {
    let html = r#"<meta name="fc:frame:button:99" content="Nope">"#;
    let frame = extract(html, None).unwrap();
    assert!(frame.buttons.is_empty());
}

#[test]
fn refresh_period_parses() {
    let html = r#"<meta property="of:refresh_period" content="60">"#;
    let frame = extract(html, None).unwrap();
    assert_eq!(frame.refresh_period, Some(60));
}
