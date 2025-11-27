use crate::microformat_extractor;
use crate::types::HEntry;

microformat_extractor! {
    HEntry, ".h-entry" {
        name: text(".p-name"),
        summary: text(".p-summary"),
        content: html(".e-content"),
        published: date(".dt-published"),
        updated: date(".dt-updated"),
        url: url(".u-url"),
        category: multi_text(".p-category"),
        author: nested_hcard(".p-author.h-card"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hentry() {
        let html = r#"
            <article class="h-entry">
                <h1 class="p-name">My Blog Post</h1>
                <div class="e-content">This is the content of my blog post.</div>
                <time class="dt-published" datetime="2024-01-01">January 1, 2024</time>
                <a class="p-category" href="/tag/rust">Rust</a>
            </article>
        "#;

        let entries = extract(html, None).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, Some("My Blog Post".to_string()));
        assert_eq!(entries[0].published, Some("2024-01-01".to_string()));
        assert_eq!(entries[0].category, vec!["Rust"]);
    }

    #[test]
    fn test_hentry_with_all_properties() {
        let html = r#"
            <article class="h-entry">
                <h1 class="p-name">Complete Blog Post</h1>
                <time class="dt-published" datetime="2024-01-15T10:00:00Z">Jan 15</time>
                <time class="dt-updated" datetime="2024-01-16T12:00:00Z">Jan 16</time>
                <div class="p-author h-card">
                    <span class="p-name">Author Name</span>
                    <a class="u-url" href="https://author.example.com">Website</a>
                </div>
                <div class="e-content">
                    <p>This is the <strong>full content</strong> with HTML.</p>
                </div>
                <p class="p-summary">Short summary of the post</p>
                <a class="p-category" href="/rust">rust</a>
                <a class="p-category" href="/python">python</a>
                <a class="u-url" href="https://example.com/post">Permalink</a>
                <a class="u-syndication" href="https://twitter.com/user/status/123">Twitter</a>
                <span class="p-location">San Francisco</span>
            </article>
        "#;
        let entries = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(entries.len(), 1);
        let entry = &entries[0];
        assert_eq!(entry.name, Some("Complete Blog Post".to_string()));
        assert_eq!(entry.published, Some("2024-01-15T10:00:00Z".to_string()));
        assert_eq!(entry.updated, Some("2024-01-16T12:00:00Z".to_string()));
        assert!(entry.author.is_some());
        assert!(entry.content.is_some());
        assert!(entry.summary.is_some());
    }

    #[test]
    fn test_hentry_with_nested_author() {
        let html = r#"
            <article class="h-entry">
                <h1 class="p-name">Post Title</h1>
                <div class="p-author h-card">
                    <span class="p-name">Jane Doe</span>
                    <img class="u-photo" src="/jane.jpg" alt="Jane">
                    <a class="u-url" href="https://jane.example.com">Website</a>
                </div>
            </article>
        "#;
        let entries = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(entries.len(), 1);
        let entry = &entries[0];
        assert!(entry.author.is_some());
        let author = entry.author.as_ref().unwrap();
        assert_eq!(author.name, Some("Jane Doe".to_string()));
    }

    #[test]
    fn test_hentry_with_multiple_categories() {
        let html = r#"
            <article class="h-entry">
                <h1 class="p-name">Tagged Post</h1>
                <a class="p-category" href="/tag1">Tag 1</a>
                <a class="p-category" href="/tag2">Tag 2</a>
                <a class="p-category" href="/tag3">Tag 3</a>
                <a class="p-category" href="/tag4">Tag 4</a>
                <a class="p-category" href="/tag5">Tag 5</a>
            </article>
        "#;
        let entries = extract(html, None).unwrap();
        assert_eq!(entries.len(), 1);
        let entry = &entries[0];
        assert_eq!(entry.category.len(), 5);
    }

    #[test]
    fn test_hentry_with_html_content() {
        let html = r#"
            <article class="h-entry">
                <h1 class="p-name">Post with HTML</h1>
                <div class="e-content">
                    <h2>Section 1</h2>
                    <p>Paragraph with <strong>bold</strong> and <em>italic</em>.</p>
                    <ul>
                        <li>Item 1</li>
                        <li>Item 2</li>
                    </ul>
                    <pre><code>let x = 5;</code></pre>
                </div>
            </article>
        "#;
        let entries = extract(html, None).unwrap();
        assert_eq!(entries.len(), 1);
        let entry = &entries[0];
        assert!(entry.content.is_some());
        let content = entry.content.as_ref().unwrap();
        assert!(content.contains("<h2>"));
        assert!(content.contains("<strong>"));
        assert!(content.contains("<code>"));
    }

    #[test]
    fn test_hentry_minimal() {
        let html = r#"
            <article class="h-entry">
                <h1 class="p-name">Minimal Post</h1>
            </article>
        "#;
        let entries = extract(html, None).unwrap();
        assert_eq!(entries.len(), 1);
        let entry = &entries[0];
        assert_eq!(entry.name, Some("Minimal Post".to_string()));
        // Other properties should be None
    }

    #[test]
    fn test_hentry_with_relative_urls() {
        let html = r#"
            <article class="h-entry">
                <h1 class="p-name">Post</h1>
                <a class="u-url" href="/blog/post-1">Permalink</a>
                <a class="u-syndication" href="/syndication/twitter">Syndication</a>
            </article>
        "#;
        let entries = extract(html, Some("https://example.com")).unwrap();
        assert_eq!(entries.len(), 1);
        let entry = &entries[0];
        // URLs should be resolved against base_url
        assert_eq!(entry.url, Some("https://example.com/blog/post-1".to_string()));
    }

    #[test]
    fn test_multiple_hentries() {
        let html = r#"
            <article class="h-entry">
                <h1 class="p-name">Post 1</h1>
            </article>
            <article class="h-entry">
                <h1 class="p-name">Post 2</h1>
            </article>
            <article class="h-entry">
                <h1 class="p-name">Post 3</h1>
            </article>
        "#;
        let entries = extract(html, None).unwrap();
        assert_eq!(entries.len(), 3);
    }

    #[test]
    fn test_hentry_with_empty_content() {
        let html = r#"
            <article class="h-entry">
                <h1 class="p-name">Post</h1>
                <div class="e-content"></div>
            </article>
        "#;
        let entries = extract(html, None).unwrap();
        assert_eq!(entries.len(), 1);
        // Empty content should be handled gracefully
    }

    #[test]
    fn test_hentry_with_special_characters() {
        let html = r#"
            <article class="h-entry">
                <h1 class="p-name">Post with "quotes" & special chars</h1>
                <div class="e-content">
                    <p>Content with émojis 🎉 and ñ</p>
                </div>
            </article>
        "#;
        let entries = extract(html, None).unwrap();
        assert_eq!(entries.len(), 1);
        let entry = &entries[0];
        assert!(entry.name.as_ref().unwrap().contains("quotes"));
    }

    #[test]
    fn test_hentry_with_no_datetime() {
        let html = r#"
            <article class="h-entry">
                <h1 class="p-name">Undated Post</h1>
                <time class="dt-published">January 15, 2024</time>
            </article>
        "#;
        let entries = extract(html, None).unwrap();
        assert_eq!(entries.len(), 1);
        // Should extract text content even without datetime attribute
    }

    #[test]
    fn test_hentry_nested_in_div() {
        let html = r#"
            <div>
                <div>
                    <article class="h-entry">
                        <h1 class="p-name">Nested Entry</h1>
                    </article>
                </div>
            </div>
        "#;
        let entries = extract(html, None).unwrap();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn test_hentry_with_reply_context() {
        let html = r#"
            <article class="h-entry">
                <h1 class="p-name">Reply Post</h1>
                <a class="u-in-reply-to" href="https://other.example.com/post">In reply to</a>
            </article>
        "#;
        let entries = extract(html, None).unwrap();
        assert_eq!(entries.len(), 1);
        // in-reply-to should be captured
    }
}
