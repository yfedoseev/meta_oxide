# Getting Started with MetaOxide (Rust)

Welcome to MetaOxide! This guide will help you get started with the Rust version of MetaOxide in just 5 minutes.

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Basic Extraction](#basic-extraction)
- [Error Handling](#error-handling)
- [Next Steps](#next-steps)

## Installation

Add MetaOxide to your `Cargo.toml`:

```toml
[dependencies]
meta_oxide = "0.1.3"
```

Or use cargo add:

```bash
cargo add meta_oxide
```

## Quick Start

Here's a minimal example to extract metadata from HTML:

```rust
use meta_oxide::MetaOxide;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>My Page</title>
            <meta name="description" content="A great page">
            <meta property="og:title" content="My Page">
        </head>
        <body>Hello World</body>
        </html>
    "#;

    let extractor = MetaOxide::new(html, "https://example.com")?;
    let metadata = extractor.extract_all()?;

    println!("Title: {:?}", metadata.get("title"));
    println!("Description: {:?}", metadata.get("description"));

    Ok(())
}
```

## Basic Extraction

MetaOxide supports 13 metadata formats. Here's how to extract specific formats:

### Extract Open Graph Data

```rust
use meta_oxide::MetaOxide;

fn extract_opengraph(html: &str) -> Result<(), Box<dyn std::error::Error>> {
    let extractor = MetaOxide::new(html, "https://example.com")?;

    if let Some(og_data) = extractor.extract_opengraph()? {
        println!("OG Title: {:?}", og_data.get("title"));
        println!("OG Type: {:?}", og_data.get("type"));
        println!("OG Image: {:?}", og_data.get("image"));
    }

    Ok(())
}
```

### Extract Twitter Cards

```rust
use meta_oxide::MetaOxide;

fn extract_twitter(html: &str) -> Result<(), Box<dyn std::error::Error>> {
    let extractor = MetaOxide::new(html, "https://example.com")?;

    if let Some(twitter_data) = extractor.extract_twitter_card()? {
        println!("Card Type: {:?}", twitter_data.get("card"));
        println!("Title: {:?}", twitter_data.get("title"));
    }

    Ok(())
}
```

### Extract JSON-LD Structured Data

```rust
use meta_oxide::MetaOxide;

fn extract_jsonld(html: &str) -> Result<(), Box<dyn std::error::Error>> {
    let extractor = MetaOxide::new(html, "https://example.com")?;

    if let Some(jsonld_data) = extractor.extract_jsonld()? {
        println!("JSON-LD: {:#?}", jsonld_data);
    }

    Ok(())
}
```

## Error Handling

MetaOxide uses Result types for error handling. Here's how to handle errors properly:

```rust
use meta_oxide::{MetaOxide, MetaOxideError};

fn safe_extraction(html: &str, url: &str) -> Result<(), MetaOxideError> {
    match MetaOxide::new(html, url) {
        Ok(extractor) => {
            match extractor.extract_all() {
                Ok(metadata) => {
                    println!("Extracted {} fields", metadata.len());
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Extraction failed: {}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create extractor: {}", e);
            Err(e)
        }
    }
}
```

Common errors:
- `ParseError`: Invalid HTML structure
- `UrlParseError`: Invalid base URL
- `ExtractionError`: Failed to extract specific metadata format

## Next Steps

Now that you've got the basics, explore more:

1. **[Complete API Reference](/docs/api/api-reference-rust.md)** - Learn about all available extractors
2. **[Real-World Examples](/examples/real-world/rust-cli-tool/)** - See a complete CLI tool implementation
3. **[Performance Tuning](/docs/performance/performance-tuning-guide.md)** - Optimize for your use case

### All Supported Formats

MetaOxide extracts these 13 metadata formats:

- Basic HTML metadata (title, description, keywords)
- Open Graph (og:*)
- Twitter Cards (twitter:*)
- JSON-LD structured data
- Microdata (schema.org)
- Microformats (h-card, h-entry, h-event)
- Dublin Core
- RDFA
- HTML5 semantic elements
- Link relations
- Image metadata
- Author information

### Learn More

- [Architecture Overview](/docs/architecture/architecture-overview.md)
- [Contributing Guide](/docs/architecture/contributing-guide.md)
- [FAQ](/docs/troubleshooting/faq.md)

Happy extracting! 🚀
