# API Reference

Complete API reference for MetaOxide.

## Python API

### Functions

#### `extract_microformats(html, base_url=None)`

Extract all microformats from HTML content.

**Parameters:**
- `html` (str): HTML content to parse
- `base_url` (str, optional): Base URL for resolving relative URLs

**Returns:**
- `dict[str, list[dict]]`: Dictionary mapping microformat types to lists of items

**Example:**
```python
result = meta_oxide.extract_microformats(html)
# Returns: {
#     'h-card': [{...}],
#     'h-entry': [{...}],
#     'h-event': [{...}]
# }
```

**Raises:**
- `ValueError`: If HTML parsing fails

---

#### `extract_hcard(html, base_url=None)`

Extract h-card microformats (contact information) from HTML.

**Parameters:**
- `html` (str): HTML content to parse
- `base_url` (str, optional): Base URL for resolving relative URLs

**Returns:**
- `list[dict]`: List of h-card dictionaries

**H-Card Properties:**
- `name` (str): Full name
- `url` (str): Website URL
- `photo` (str): Photo URL
- `email` (str): Email address
- `tel` (str): Telephone number
- `note` (str): Biographical note
- `org` (str): Organization name

**Example:**
```python
cards = meta_oxide.extract_hcard(html)
for card in cards:
    print(f"Name: {card['name']}")
    print(f"Email: {card['email']}")
```

**Raises:**
- `ValueError`: If HTML parsing fails

---

#### `extract_hentry(html, base_url=None)`

Extract h-entry microformats (blog posts, articles) from HTML.

**Parameters:**
- `html` (str): HTML content to parse
- `base_url` (str, optional): Base URL for resolving relative URLs

**Returns:**
- `list[dict]`: List of h-entry dictionaries

**H-Entry Properties:**
- `name` (str): Title of the entry
- `summary` (str): Short summary
- `content` (str): Full content (HTML)
- `published` (str): Publication date/time
- `updated` (str): Last update date/time
- `author` (dict): Author h-card
- `url` (str): Permalink URL
- `category` (list[str]): List of categories/tags

**Example:**
```python
entries = meta_oxide.extract_hentry(html)
for entry in entries:
    print(f"Title: {entry['name']}")
    print(f"Published: {entry['published']}")
    print(f"Categories: {', '.join(entry['category'])}")
```

**Raises:**
- `ValueError`: If HTML parsing fails

---

#### `extract_hevent(html, base_url=None)`

Extract h-event microformats (events) from HTML.

**Parameters:**
- `html` (str): HTML content to parse
- `base_url` (str, optional): Base URL for resolving relative URLs

**Returns:**
- `list[dict]`: List of h-event dictionaries

**H-Event Properties:**
- `name` (str): Event name
- `summary` (str): Short summary
- `start` (str): Start date/time
- `end` (str): End date/time
- `location` (str): Event location
- `url` (str): Event URL
- `description` (str): Full description

**Example:**
```python
events = meta_oxide.extract_hevent(html)
for event in events:
    print(f"Event: {event['name']}")
    print(f"When: {event['start']} to {event['end']}")
    print(f"Where: {event['location']}")
```

**Raises:**
- `ValueError`: If HTML parsing fails

---

## Rust API

### Modules

#### `meta_oxide::extractors`

Contains extractor functions for different microformat types.

**Functions:**
- `extract_hcard(html: &str, base_url: Option<&str>) -> Result<Vec<HCard>>`
- `extract_hentry(html: &str, base_url: Option<&str>) -> Result<Vec<HEntry>>`
- `extract_hevent(html: &str, base_url: Option<&str>) -> Result<Vec<HEvent>>`

---

#### `meta_oxide::types`

Data structures for microformats.

**Structs:**

##### `HCard`

```rust
pub struct HCard {
    pub name: Option<String>,
    pub url: Option<String>,
    pub photo: Option<String>,
    pub email: Option<String>,
    pub tel: Option<String>,
    pub note: Option<String>,
    pub org: Option<String>,
    pub additional_properties: HashMap<String, Vec<String>>,
}
```

##### `HEntry`

```rust
pub struct HEntry {
    pub name: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub published: Option<String>,
    pub updated: Option<String>,
    pub author: Option<Box<HCard>>,
    pub url: Option<String>,
    pub category: Vec<String>,
    pub additional_properties: HashMap<String, Vec<String>>,
}
```

##### `HEvent`

```rust
pub struct HEvent {
    pub name: Option<String>,
    pub summary: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub additional_properties: HashMap<String, Vec<String>>,
}
```

##### `MicroformatItem`

Generic microformat representation:

```rust
pub struct MicroformatItem {
    pub type_: Vec<String>,
    pub properties: HashMap<String, Vec<PropertyValue>>,
    pub children: Option<Vec<MicroformatItem>>,
}
```

##### `PropertyValue`

```rust
pub enum PropertyValue {
    Text(String),
    Url(String),
    Nested(Box<MicroformatItem>),
}
```

---

#### `meta_oxide::errors`

Error types for the library.

##### `MicroformatError`

```rust
pub enum MicroformatError {
    ParseError(String),
    InvalidUrl(url::ParseError),
    MissingProperty(String),
    InvalidStructure(String),
    ExtractionFailed(String),
}
```

**Methods:**
- `to_string() -> String`: Get error message

---

#### `meta_oxide::parser`

HTML parsing functionality.

##### `parse_html(html: &str, base_url: Option<&str>) -> Result<HashMap<String, Vec<MicroformatItem>>>`

Parse HTML and extract all microformats.

**Parameters:**
- `html`: HTML string to parse
- `base_url`: Optional base URL for resolving relative URLs

**Returns:**
- `Result<HashMap<String, Vec<MicroformatItem>>>`: Map of microformat types to items

**Example:**
```rust
use meta_oxide::parser::parse_html;

let html = "<div class='h-card'><span class='p-name'>John</span></div>";
let result = parse_html(html, None)?;

for (mf_type, items) in result {
    println!("Found {} items of type {}", items.len(), mf_type);
}
```

---

## Microformat Class Reference

### Root Classes (h-*)

- `h-card`: Contact information
- `h-entry`: Blog post or article
- `h-event`: Event
- `h-feed`: Feed of h-entry items
- `h-review`: Review
- `h-product`: Product information

### Property Classes

#### Text Properties (p-*)

Extract plain text content:
- `p-name`: Name/title
- `p-summary`: Summary
- `p-category`: Category/tag
- `p-tel`: Telephone number
- `p-note`: Note/description
- `p-org`: Organization
- `p-location`: Location

**Example:**
```html
<span class="p-name">Jane Doe</span>
```

#### URL Properties (u-*)

Extract URLs from href or src attributes:
- `u-url`: URL/permalink
- `u-photo`: Photo URL
- `u-email`: Email address (extracts from mailto:)
- `u-logo`: Logo URL

**Example:**
```html
<a class="u-url" href="https://example.com">Website</a>
<img class="u-photo" src="/photo.jpg">
```

#### DateTime Properties (dt-*)

Extract datetime values from datetime attribute or text:
- `dt-published`: Publication date
- `dt-updated`: Update date
- `dt-start`: Start date/time
- `dt-end`: End date/time

**Example:**
```html
<time class="dt-published" datetime="2024-01-01T12:00:00Z">
    January 1, 2024
</time>
```

#### Embedded HTML Properties (e-*)

Extract HTML content:
- `e-content`: HTML content
- `e-description`: HTML description

**Example:**
```html
<div class="e-content">
    <p>This is <strong>HTML</strong> content.</p>
</div>
```

---

## URL Resolution

When a `base_url` is provided, relative URLs are resolved to absolute URLs:

```python
html = '<a class="u-url" href="/about">About</a>'
result = meta_oxide.extract_hcard(html, base_url="https://example.com")
# result[0]['url'] == "https://example.com/about"
```

**URL Resolution Rules:**
1. Absolute URLs (http://, https://) are kept as-is
2. Protocol-relative URLs (//example.com) use base URL's protocol
3. Root-relative URLs (/path) are resolved against base domain
4. Relative URLs (path) are resolved against base URL

---

## Error Handling

### Python

All functions raise `ValueError` on parsing errors:

```python
try:
    result = meta_oxide.extract_hcard(invalid_html)
except ValueError as e:
    print(f"Parsing failed: {e}")
```

### Rust

Functions return `Result<T, MicroformatError>`:

```rust
use meta_oxide::extractors::extract_hcard;
use meta_oxide::MicroformatError;

match extract_hcard(html, None) {
    Ok(cards) => println!("Found {} cards", cards.len()),
    Err(MicroformatError::ParseError(msg)) => eprintln!("Parse error: {}", msg),
    Err(e) => eprintln!("Error: {}", e),
}
```

---

## Performance Characteristics

### Time Complexity

- **Parsing**: O(n) where n is the HTML size
- **Extraction**: O(m) where m is the number of elements
- **Overall**: O(n + m), typically dominated by parsing

### Memory Usage

- **HTML Parsing**: ~2-3x the input size (DOM tree)
- **Extraction**: ~1x the output size (microformat data)
- **Python Conversion**: Additional copy for FFI boundary

### Benchmarks

Typical performance on modern hardware:

| HTML Size | Parse Time | Extraction Time | Total Time |
|-----------|------------|-----------------|------------|
| 10 KB     | ~50 µs     | ~10 µs          | ~60 µs     |
| 100 KB    | ~500 µs    | ~100 µs         | ~600 µs    |
| 1 MB      | ~5 ms      | ~1 ms           | ~6 ms      |

*Benchmarks performed on Intel i7, 3.5 GHz*

---

## Thread Safety

### Rust

All types are `Send + Sync` and can be safely used across threads:

```rust
use std::thread;

let html = get_html();
let handle = thread::spawn(move || {
    extract_hcard(&html, None)
});

let cards = handle.join().unwrap()?;
```

### Python

Due to Python's GIL, only one thread executes Python code at a time. However, the Rust code releases the GIL during parsing, allowing other Python threads to run.

---

## Version Compatibility

### Python

- Minimum: Python 3.8
- Recommended: Python 3.10+
- Tested: Python 3.8, 3.9, 3.10, 3.11, 3.12

### Rust

- Minimum: Rust 1.70
- Recommended: Latest stable
- Edition: 2021
