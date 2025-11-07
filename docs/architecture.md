# Architecture

This document describes the architecture and design decisions of MetaOxide.

## Overview

MetaOxide is designed as a high-performance Rust library with Python bindings for extracting microformats from HTML. It uses a modular architecture that separates concerns and makes it easy to add new microformat types.

## Component Architecture

```
┌─────────────────────────────────────────────┐
│           Python Interface (PyO3)           │
│  extract_microformats(), extract_hcard()   │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│              Core Library                   │
│  ┌───────────┐  ┌──────────┐  ┌──────────┐ │
│  │  Parser   │  │  Types   │  │  Errors  │ │
│  └─────┬─────┘  └──────────┘  └──────────┘ │
│        │                                    │
│  ┌─────▼─────────────────────────────────┐ │
│  │         Extractors Module             │ │
│  │  ┌────────┐ ┌─────────┐ ┌──────────┐ │ │
│  │  │ h-card │ │ h-entry │ │ h-event  │ │ │
│  │  └────────┘ └─────────┘ └──────────┘ │ │
│  └───────────────────────────────────────┘ │
└─────────────────────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│         External Dependencies               │
│   scraper, pyo3, serde, url, thiserror     │
└─────────────────────────────────────────────┘
```

## Core Components

### 1. Parser Module (`src/parser.rs`)

The parser module is responsible for:
- Parsing HTML using the `scraper` crate
- Identifying microformat root classes (h-*)
- Extracting properties (p-*, u-*, dt-*, e-*)
- Resolving relative URLs
- Building the microformat data structure

**Key Functions:**
- `parse_html()`: Main entry point for parsing
- `parse_microformat_item()`: Extracts a single microformat item
- `extract_properties()`: Recursively extracts properties
- `resolve_url()`: Resolves relative URLs against a base URL

### 2. Types Module (`src/types.rs`)

Defines the data structures used throughout the library:

**Generic Types:**
- `MicroformatItem`: Generic microformat representation
- `PropertyValue`: Enum for different property value types (text, URL, nested)

**Specific Microformat Types:**
- `HCard`: Contact information
- `HEntry`: Blog posts and articles
- `HEvent`: Events

All types implement:
- `Serialize`/`Deserialize` for JSON conversion
- `to_py_dict()` for Python interop via PyO3

### 3. Extractors Module (`src/extractors/`)

Specialized extractors for each microformat type:

- **hcard.rs**: Extracts h-card microformats (contact info)
- **hentry.rs**: Extracts h-entry microformats (blog posts)
- **hevent.rs**: Extracts h-event microformats (events)

Each extractor:
1. Uses CSS selectors to find relevant elements
2. Extracts specific properties for that microformat type
3. Handles nested microformats (e.g., author h-card in h-entry)
4. Returns strongly-typed Rust structs

### 4. Error Handling (`src/errors.rs`)

Centralized error handling using `thiserror`:

```rust
pub enum MicroformatError {
    ParseError(String),
    InvalidUrl(url::ParseError),
    MissingProperty(String),
    InvalidStructure(String),
    ExtractionFailed(String),
}
```

Errors are converted to Python exceptions when crossing the FFI boundary.

### 5. Python Interface (`src/lib.rs`)

PyO3 bindings that expose Rust functionality to Python:

- `extract_microformats()`: Extract all microformats
- `extract_hcard()`: Extract h-card only
- `extract_hentry()`: Extract h-entry only
- `extract_hevent()`: Extract h-event only

## Design Decisions

### Why Rust + PyO3?

1. **Performance**: Rust provides C-level performance for HTML parsing
2. **Safety**: Memory safety without garbage collection
3. **Python Integration**: PyO3 makes it easy to create Python packages
4. **Type Safety**: Strong typing catches errors at compile time

### Modular Extractor Design

Each microformat type has its own extractor module because:

1. **Separation of Concerns**: Each format has unique properties
2. **Maintainability**: Easy to add new formats without affecting existing ones
3. **Testing**: Can test each format independently
4. **Extensibility**: Users can add custom extractors

### CSS Selectors vs Manual Parsing

We use the `scraper` crate with CSS selectors because:

1. **Readability**: `.h-card` is clearer than manual DOM traversal
2. **Reliability**: Well-tested library used by many projects
3. **Standards-Compliant**: Follows CSS selector specifications
4. **Performance**: Optimized selector engine

### Property Value Types

The `PropertyValue` enum supports three types:

```rust
pub enum PropertyValue {
    Text(String),      // p-* properties
    Url(String),       // u-* properties
    Nested(Box<MicroformatItem>),  // nested microformats
}
```

This design:
- Preserves type information
- Enables proper URL resolution
- Supports nested structures
- Converts cleanly to Python types

## Data Flow

### Extraction Flow

1. **Input**: HTML string + optional base URL
2. **Parse**: HTML is parsed into a DOM tree
3. **Identify**: Find elements with microformat classes
4. **Extract**: For each root element:
   - Identify type classes (h-*)
   - Extract properties (p-*, u-*, dt-*, e-*)
   - Resolve relative URLs
   - Handle nested microformats
5. **Convert**: Convert to Python objects
6. **Return**: Python dictionaries/lists

### Example Flow Diagram

```
HTML Input
    │
    ▼
scraper::Html::parse_document()
    │
    ▼
Find .h-card elements
    │
    ▼
For each h-card:
  ├─ Extract .p-name → PropertyValue::Text
  ├─ Extract .u-url → PropertyValue::Url (resolved)
  ├─ Extract .u-email → PropertyValue::Url
  └─ Build HCard struct
    │
    ▼
Convert to PyDict
    │
    ▼
Return to Python
```

## Performance Considerations

### Optimization Strategies

1. **Single-Pass Parsing**: Parse HTML once, extract all formats
2. **Lazy Evaluation**: Only parse requested microformat types
3. **Zero-Copy Where Possible**: Use string slices instead of clones
4. **Release Optimizations**: LTO and optimization level 3

### Memory Management

- Rust's ownership system prevents memory leaks
- Python objects are managed by PyO3's GIL integration
- Large HTML documents are parsed with streaming where possible

## Testing Strategy

### Unit Tests

Each extractor has unit tests in the same file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hcard() {
        // Test implementation
    }
}
```

### Integration Tests

Located in `tests/` directory for end-to-end testing.

### Python Tests

Test Python bindings with pytest:

```python
def test_extract_hcard():
    result = meta_oxide.extract_hcard(html)
    assert len(result) == 1
```

## Future Enhancements

### Planned Features

1. **More Microformats**: h-feed, h-review, h-product
2. **Streaming Parser**: For very large documents
3. **Custom Extractors**: Allow users to define custom formats
4. **Validation**: Validate microformat structure
5. **Performance Metrics**: Built-in benchmarking

### Extensibility Points

1. **Custom Property Extractors**: Override default extraction logic
2. **Custom Validators**: Add validation rules
3. **Custom Converters**: Convert to different output formats
4. **Plugin System**: Load extractors dynamically

## Dependencies

### Core Dependencies

- **pyo3**: Python bindings and FFI
- **scraper**: HTML parsing with CSS selectors
- **serde**: Serialization framework
- **url**: URL parsing and resolution
- **thiserror**: Error handling

### Why These Dependencies?

- **scraper**: Best-in-class HTML parser for Rust
- **pyo3**: Official Python bindings for Rust
- **serde**: Industry-standard serialization
- **url**: WHATWG URL standard implementation

## Contributing

See [Development Guide](development.md) for information on:
- Setting up the development environment
- Adding new microformat types
- Running tests
- Submitting pull requests
