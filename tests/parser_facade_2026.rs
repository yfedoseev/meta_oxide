//! End-to-end integration tests for the 2026 format additions wired
//! through `MetaParser` / `MetaGraph`.

use meta_oxide::{FormatMask, MetaParser};

#[test]
fn frames_and_app_links_surface_via_facade() {
    let html = r#"
        <!doctype html><html><head>
        <meta name="fc:frame" content="vNext">
        <meta name="fc:frame:image" content="https://img.example/c.png">
        <meta name="fc:frame:button:1" content="Mint">
        <meta property="al:ios:url" content="app://deep">
        <meta property="al:ios:app_store_id" content="777">
        <meta name="apple-itunes-app" content="app-id=777, app-argument=https://p/1">
        </head></html>
    "#;
    let graph = MetaParser::new().parse(html).unwrap();
    assert!(graph.frames.has_farcaster);
    assert_eq!(graph.frames.buttons.len(), 1);
    assert_eq!(graph.app_links.ios.as_ref().unwrap().app_store_id.as_deref(), Some("777"));
    assert_eq!(graph.app_links.apple_itunes_app.as_ref().unwrap().app_id.as_deref(), Some("777"));
}

#[test]
fn verification_ai_and_c2pa_populate() {
    let html = r#"
        <meta name="google-site-verification" content="GV1">
        <meta name="facebook-domain-verification" content="FBV1">
        <meta name="robots" content="noai, noimageai">
        <meta name="GPTBot" content="noai">
        <meta name="c2pa:claim_generator" content="Adobe Photoshop 26">
        <meta name="c2pa:ai_action" content="ai-generated">
        <link rel="c2pa-manifest" href="https://ex/m.c2pa">
    "#;
    let graph = MetaParser::new().parse(html).unwrap();
    assert_eq!(graph.verifications.google.as_deref(), Some("GV1"));
    assert_eq!(graph.verifications.facebook.as_deref(), Some("FBV1"));
    assert!(graph.ai_directives.noai);
    assert!(graph.ai_directives.noimageai);
    assert!(graph.ai_directives.per_bot.contains_key("gptbot"));
    assert_eq!(graph.c2pa.claim_generator.as_deref(), Some("Adobe Photoshop 26"));
    assert_eq!(graph.c2pa.manifest_url.as_deref(), Some("https://ex/m.c2pa"));
}

#[test]
fn activitypub_and_speakable() {
    let html = r#"
        <link rel="alternate" type="application/activity+json" href="/users/alice">
        <script type="application/ld+json">
        {"@type": "NewsArticle", "speakable": {"cssSelector": [".lede"]}}
        </script>
    "#;
    let graph = MetaParser::new().with_base_url("https://ex").parse(html).unwrap();
    assert_eq!(graph.activitypub.alternate_url.as_deref(), Some("https://ex/users/alice"));
    assert_eq!(graph.speakable.len(), 1);
    assert_eq!(graph.speakable[0].css_selectors, vec![".lede".to_string()]);
}

#[test]
fn format_mask_opt_out_excludes_2026_fields() {
    let html =
        r#"<meta name="fc:frame" content="vNext"><meta name="al:ios:url" content="app://d">"#;
    let graph = MetaParser::new()
        .with_heuristics(false)
        .with_formats(FormatMask::META)
        .parse(html)
        .unwrap();
    assert!(graph.frames.is_empty());
    assert!(graph.app_links.is_empty());
}

#[test]
fn speakable_requires_jsonld_and_speakable_masks() {
    let html = r#"<script type="application/ld+json">{"@type":"A","speakable":"x"}</script>"#;
    // With JSON_LD but without SPEAKABLE we still get json_ld but not speakable.
    let graph = MetaParser::new()
        .with_heuristics(false)
        .with_formats(FormatMask::JSON_LD)
        .parse(html)
        .unwrap();
    assert_eq!(graph.json_ld.len(), 1);
    assert!(graph.speakable.is_empty());
}
