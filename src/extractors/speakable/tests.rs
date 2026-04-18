use super::*;

fn parse(json: &str) -> Vec<JsonLdObject> {
    let val: serde_json::Value = serde_json::from_str(json).unwrap();
    match val {
        serde_json::Value::Array(items) => {
            items.into_iter().map(|v| serde_json::from_value::<JsonLdObject>(v).unwrap()).collect()
        }
        v => vec![serde_json::from_value::<JsonLdObject>(v).unwrap()],
    }
}

#[test]
fn extracts_inline_css_selectors() {
    let objs = parse(
        r#"{
            "@type": "NewsArticle",
            "speakable": {
                "@type": "SpeakableSpecification",
                "cssSelector": [".summary", "h1"]
            }
        }"#,
    );
    let result = collect(&objs);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].css_selectors, vec![".summary".to_string(), "h1".to_string()]);
}

#[test]
fn extracts_xpath_and_url_variants() {
    let objs = parse(
        r#"[
            { "@type": "Article", "speakable": { "xpath": ["/html/body/article/h1"] }},
            { "@type": "Article", "speakable": "https://example.com/p#intro" }
        ]"#,
    );
    let result = collect(&objs);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].xpaths, vec!["/html/body/article/h1".to_string()]);
    assert_eq!(result[1].urls, vec!["https://example.com/p#intro".to_string()]);
}

#[test]
fn walks_graph_recursively() {
    let objs = parse(
        r#"{
            "@graph": [
                { "@type": "WebPage", "mainEntity": {
                    "@type": "Article",
                    "speakable": { "cssSelector": ".lede" }
                }}
            ]
        }"#,
    );
    let result = collect(&objs);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].css_selectors, vec![".lede".to_string()]);
}

#[test]
fn empty_without_speakable() {
    let objs = parse(r#"{"@type": "Article", "headline": "x"}"#);
    assert!(collect(&objs).is_empty());
}
