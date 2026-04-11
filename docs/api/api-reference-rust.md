# MetaOxide Rust API Reference

Complete API documentation for the Rust crate.

## Table of Contents

- [Core Types](#core-types)
- [Extraction Functions](#extraction-functions)
- [Metadata Formats](#metadata-formats)
- [Error Handling](#error-handling)
- [Advanced Usage](#advanced-usage)

## Core Types

### `MetaOxide`

The main struct for extracting metadata from HTML.

```rust
pub struct MetaOxide {
    // Internal fields are private
}
```

#### Methods

##### `new(html: &str, base_url: &str) -> Result<Self>`

Creates a new MetaOxide instance from HTML content.

**Parameters:**
- `html: &str` - The HTML content to parse
- `base_url: &str` - Base URL for resolving relative URLs

**Returns:**
- `Result<Self>` - The extractor instance or an error

**Example:**
```rust
use meta_oxide::MetaOxide;

let html = r#"<!DOCTYPE html><html>...</html>"#;
let extractor = MetaOxide::new(html, "https://example.com")?;
```

##### `extract_all(&self) -> Result<HashMap<String, Value>>`

Extracts all metadata formats at once.

**Returns:**
- `Result<HashMap<String, Value>>` - All extracted metadata

**Example:**
```rust
let metadata = extractor.extract_all()?;
println!("{:#?}", metadata);
```

##### `extract_basic_meta(&self) -> Result<BasicMeta>`

Extracts basic HTML metadata (title, description, etc.).

**Returns:**
- `Result<BasicMeta>` - Basic HTML metadata

**Example:**
```rust
let basic = extractor.extract_basic_meta()?;
println!("Title: {:?}", basic.title);
println!("Description: {:?}", basic.description);
```

##### `extract_opengraph(&self) -> Result<Option<OpenGraph>>`

Extracts Open Graph metadata.

**Returns:**
- `Result<Option<OpenGraph>>` - Open Graph data if present

**Example:**
```rust
if let Some(og) = extractor.extract_opengraph()? {
    println!("OG Title: {:?}", og.title);
    println!("OG Image: {:?}", og.image);
}
```

##### `extract_twitter_card(&self) -> Result<Option<TwitterCard>>`

Extracts Twitter Card metadata.

**Returns:**
- `Result<Option<TwitterCard>>` - Twitter Card data if present

**Example:**
```rust
if let Some(twitter) = extractor.extract_twitter_card()? {
    println!("Card Type: {:?}", twitter.card_type);
}
```

##### `extract_jsonld(&self) -> Result<Vec<JsonValue>>`

Extracts JSON-LD structured data.

**Returns:**
- `Result<Vec<JsonValue>>` - Array of JSON-LD objects

**Example:**
```rust
let jsonld = extractor.extract_jsonld()?;
for item in jsonld {
    println!("{:#?}", item);
}
```

##### `extract_microdata(&self) -> Result<Vec<MicrodataItem>>`

Extracts Microdata (schema.org) structured data.

**Returns:**
- `Result<Vec<MicrodataItem>>` - Array of Microdata items

**Example:**
```rust
let microdata = extractor.extract_microdata()?;
```

##### `extract_microformats(&self) -> Result<MicroformatsData>`

Extracts all Microformats data.

**Returns:**
- `Result<MicroformatsData>` - All Microformats found

**Example:**
```rust
let mf = extractor.extract_microformats()?;
```

##### `extract_dublin_core(&self) -> Result<Option<DublinCore>>`

Extracts Dublin Core metadata.

**Returns:**
- `Result<Option<DublinCore>>` - Dublin Core data if present

**Example:**
```rust
if let Some(dc) = extractor.extract_dublin_core()? {
    println!("DC Title: {:?}", dc.title);
}
```

##### `extract_rdfa(&self) -> Result<Vec<RdfaTriple>>`

Extracts RDFa triples.

**Returns:**
- `Result<Vec<RdfaTriple>>` - Array of RDFa triples

**Example:**
```rust
let rdfa = extractor.extract_rdfa()?;
```

##### `extract_rel_links(&self) -> Result<HashMap<String, Vec<Link>>>`

Extracts link relations.

**Returns:**
- `Result<HashMap<String, Vec<Link>>>` - Links grouped by rel type

**Example:**
```rust
let links = extractor.extract_rel_links()?;
if let Some(canonical) = links.get("canonical") {
    println!("Canonical URL: {}", canonical[0].href);
}
```

##### `extract_manifest(&self) -> Result<Option<WebManifest>>`

Extracts web app manifest data.

**Returns:**
- `Result<Option<WebManifest>>` - Manifest data if present

**Example:**
```rust
if let Some(manifest) = extractor.extract_manifest()? {
    println!("App Name: {:?}", manifest.name);
}
```

## Metadata Formats

### `BasicMeta`

Basic HTML metadata structure.

```rust
pub struct BasicMeta {
    pub title: Option<String>,
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub author: Option<String>,
    pub canonical: Option<String>,
    pub charset: Option<String>,
    pub viewport: Option<String>,
    pub language: Option<String>,
    pub robots: Option<String>,
}
```

### `OpenGraph`

Open Graph protocol metadata.

```rust
pub struct OpenGraph {
    pub title: Option<String>,
    pub og_type: Option<String>,
    pub image: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub site_name: Option<String>,
    pub locale: Option<String>,
    pub audio: Option<String>,
    pub video: Option<String>,
    pub additional: HashMap<String, String>,
}
```

### `TwitterCard`

Twitter Card metadata.

```rust
pub struct TwitterCard {
    pub card_type: Option<String>,
    pub site: Option<String>,
    pub creator: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub image_alt: Option<String>,
    pub additional: HashMap<String, String>,
}
```

### `MicrodataItem`

Schema.org Microdata item.

```rust
pub struct MicrodataItem {
    pub item_type: Vec<String>,
    pub properties: HashMap<String, Vec<Value>>,
    pub id: Option<String>,
}
```

### `HCard`

Microformats h-card (person/organization).

```rust
pub struct HCard {
    pub name: Option<String>,
    pub url: Option<String>,
    pub photo: Option<String>,
    pub email: Option<String>,
    pub tel: Option<String>,
    pub org: Option<String>,
    pub adr: Option<HAdr>,
    pub additional: HashMap<String, Value>,
}
```

### `HEntry`

Microformats h-entry (blog post/article).

```rust
pub struct HEntry {
    pub name: Option<String>,
    pub author: Option<HCard>,
    pub published: Option<String>,
    pub updated: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub url: Option<String>,
    pub category: Vec<String>,
    pub syndication: Vec<String>,
}
```

### `HEvent`

Microformats h-event (calendar event).

```rust
pub struct HEvent {
    pub name: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub duration: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub location: Option<String>,
    pub category: Vec<String>,
}
```

### `DublinCore`

Dublin Core metadata.

```rust
pub struct DublinCore {
    pub title: Option<String>,
    pub creator: Option<String>,
    pub subject: Option<String>,
    pub description: Option<String>,
    pub publisher: Option<String>,
    pub contributor: Option<String>,
    pub date: Option<String>,
    pub dc_type: Option<String>,
    pub format: Option<String>,
    pub identifier: Option<String>,
    pub source: Option<String>,
    pub language: Option<String>,
    pub relation: Option<String>,
    pub coverage: Option<String>,
    pub rights: Option<String>,
}
```

### `Link`

Represents an HTML link element.

```rust
pub struct Link {
    pub href: String,
    pub rel: String,
    pub media: Option<String>,
    pub title: Option<String>,
    pub link_type: Option<String>,
    pub hreflang: Option<String>,
}
```

## Error Handling

### `MicroformatError`

Main error type for MetaOxide operations.

```rust
pub enum MicroformatError {
    ParseError(String),
    ExtractionError(String),
    UrlError(String),
    JsonError(String),
    IoError(String),
}
```

#### Variants

- `ParseError` - HTML parsing failed
- `ExtractionError` - Metadata extraction failed
- `UrlError` - URL parsing or resolution failed
- `JsonError` - JSON-LD parsing failed
- `IoError` - I/O operation failed

#### Implementation

```rust
impl std::fmt::Display for MicroformatError { /* ... */ }
impl std::error::Error for MicroformatError { /* ... */ }
```

### `Result<T>`

Type alias for Result with MicroformatError.

```rust
pub type Result<T> = std::result::Result<T, MicroformatError>;
```

## Advanced Usage

### Custom Extraction

Extract specific metadata format:

```rust
use meta_oxide::MetaOxide;

let extractor = MetaOxide::new(html, url)?;

// Extract only what you need
let og = extractor.extract_opengraph()?;
let twitter = extractor.extract_twitter_card()?;
```

### Parallel Extraction

Process multiple pages concurrently:

```rust
use rayon::prelude::*;

let urls = vec!["url1", "url2", "url3"];
let results: Vec<_> = urls.par_iter()
    .map(|url| {
        let html = fetch_html(url)?;
        let extractor = MetaOxide::new(&html, url)?;
        extractor.extract_all()
    })
    .collect();
```

### Error Recovery

Handle errors gracefully:

```rust
match MetaOxide::new(html, url) {
    Ok(extractor) => {
        match extractor.extract_all() {
            Ok(metadata) => println!("Success: {:#?}", metadata),
            Err(e) => eprintln!("Extraction failed: {}", e),
        }
    }
    Err(e) => eprintln!("Parse failed: {}", e),
}
```

### Selective Extraction

Extract only specific formats for performance:

```rust
let extractor = MetaOxide::new(html, url)?;

// Only extract social media metadata
let og = extractor.extract_opengraph()?;
let twitter = extractor.extract_twitter_card()?;

// Skip other formats if not needed
```

## Performance Considerations

### Memory Usage

- MetaOxide parses HTML once and keeps the DOM in memory
- Call `drop(extractor)` when done to free memory
- For batch processing, create and drop extractors in a loop

### CPU Usage

- Extraction is CPU-intensive for complex HTML
- Use parallel processing for multiple documents
- Extract only needed formats to reduce CPU time

### Caching

```rust
use std::collections::HashMap;

struct MetadataCache {
    cache: HashMap<String, HashMap<String, Value>>,
}

impl MetadataCache {
    fn get_or_extract(&mut self, url: &str, html: &str) -> Result<&HashMap<String, Value>> {
        if !self.cache.contains_key(url) {
            let extractor = MetaOxide::new(html, url)?;
            let metadata = extractor.extract_all()?;
            self.cache.insert(url.to_string(), metadata);
        }
        Ok(&self.cache[url])
    }
}
```

## Feature Flags

Enable specific features in `Cargo.toml`:

```toml
[dependencies]
meta_oxide = { version = "0.1.3", features = ["python"] }
```

Available features:
- `python` - Python bindings via PyO3
- `default` - Standard Rust library

## Thread Safety

MetaOxide is thread-safe:

```rust
use std::sync::Arc;
use std::thread;

let html = Arc::new(html_string);
let handles: Vec<_> = (0..4).map(|_| {
    let html = Arc::clone(&html);
    thread::spawn(move || {
        let extractor = MetaOxide::new(&html, url)?;
        extractor.extract_all()
    })
}).collect();

for handle in handles {
    let result = handle.join().unwrap();
    println!("{:?}", result);
}
```

## See Also

- [Getting Started Guide](/docs/getting-started/getting-started-rust.md)
- [Examples](/examples/real-world/rust-cli-tool/)
- [Architecture](/docs/architecture/architecture-overview.md)
