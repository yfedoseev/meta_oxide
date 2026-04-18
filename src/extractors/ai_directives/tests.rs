use super::*;

#[test]
fn robots_noai_detected() {
    let html = r#"<meta name="robots" content="index, noai, noimageai">"#;
    let d = extract(html).unwrap();
    assert!(d.noai);
    assert!(d.noimageai);
    assert!(!d.noml);
    assert!(d.robots.iter().any(|t| t == "index"));
}

#[test]
fn per_bot_directives_surface() {
    let html = r#"
        <meta name="GPTBot" content="noai">
        <meta name="Anthropic-AI" content="noai, noindex">
        <meta name="google-extended" content="noai">
    "#;
    let d = extract(html).unwrap();
    assert_eq!(d.per_bot.get("gptbot").map(String::as_str), Some("noai"));
    assert!(d.per_bot.contains_key("anthropic-ai"));
    assert!(d.per_bot.contains_key("google-extended"));
    assert!(d.noai);
}

#[test]
fn generator_ai_heuristic() {
    let html = r#"<meta name="generator" content="GPT-4 article generator">"#;
    let d = extract(html).unwrap();
    assert!(d.generator_ai.is_some());
}

#[test]
fn human_generator_not_flagged() {
    let html = r#"<meta name="generator" content="Hugo 0.130.0">"#;
    let d = extract(html).unwrap();
    assert!(d.generator_ai.is_none());
}

#[test]
fn ai_generated_and_training_tags() {
    let html = r#"
        <meta name="ai-generated" content="partial">
        <meta name="ai-training" content="not-permitted">
    "#;
    let d = extract(html).unwrap();
    assert_eq!(d.ai_generated.as_deref(), Some("partial"));
    assert_eq!(d.ai_training.as_deref(), Some("not-permitted"));
}

#[test]
fn empty_page() {
    assert!(extract("<html></html>").unwrap().is_empty());
}
