# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- h-feed support
- h-review support
- h-product support
- Streaming parser for large documents
- Custom extractor plugins
- Validation utilities

## [0.1.0] - 2024-01-XX

### Added
- Initial release
- Core microformats parser
- h-card extractor with full property support
- h-entry extractor with author and category support
- h-event extractor with date/time handling
- Python bindings via PyO3
- URL resolution for relative links
- Comprehensive documentation
- Example scripts
- Test suite for Rust and Python
- Support for nested microformats
- Error handling with detailed messages

### Features
- Fast HTML parsing using scraper crate
- Type-safe Rust API
- Simple Python API
- Zero-copy parsing where possible
- Support for all microformat property types (p-, u-, dt-, e-)
- Thread-safe implementation

### Documentation
- Getting Started guide
- Architecture documentation
- Complete API reference
- Practical examples
- Development guide

### Performance
- Optimized for speed with Rust
- Release builds with LTO
- Minimal memory footprint

[Unreleased]: https://github.com/yourusername/meta_oxide/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/meta_oxide/releases/tag/v0.1.0
