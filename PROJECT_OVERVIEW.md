# MetaOxide - Project Overview

## What Is This?

**MetaOxide** is a comprehensive **structured data extraction library** written in Rust with Python bindings. It extracts **ALL types of metadata** from web pages, not just microformats.

### The Vision

> **One library to extract them all** - Microformats, Open Graph, Twitter Cards, Schema.org JSON-LD, Microdata, RDFa, and standard meta tags.

---

## Why MetaOxide?

### The Problem

Modern web pages contain metadata in **many different formats**:

```
🌐 Example.com Article
├── Standard Meta Tags (title, description, keywords)
├── Open Graph (Facebook sharing)
├── Twitter Cards (Twitter/X sharing)
├── JSON-LD (Google Rich Results, AI training)
├── Microformats (IndieWeb, semantic web)
├── Microdata (older SEO)
└── RDFa (government, academic)
```

Extracting all of this currently requires:
- Multiple libraries (Python: extruct, mf2py, beautifulsoup)
- Slow performance (Python parsing)
- Complex integration
- Inconsistent APIs

### The Solution

**MetaOxide** provides:
- ✅ **One library** for all formats
- ✅ **Blazing fast** (Rust performance)
- ✅ **Simple API** (Python + Rust)
- ✅ **Type-safe** (Rust types)
- ✅ **Complete** (all major formats)

---

## Supported Formats

### Current Status (v0.1.0) ✅

**Microformats2** (Basic Implementation)
- ✅ h-card - Contact information
- ✅ h-entry - Blog posts, articles
- ✅ h-event - Events

### Coming Soon (Roadmap)

#### Phase 2: Social Media Meta 🎯
**Priority: HIGH** (Used by 60%+ of websites)

1. **Open Graph Protocol** (Facebook, LinkedIn)
   - Basic: title, type, image, url, description
   - Article-specific: author, published_time, section
   - Video, Audio, Book properties

2. **Twitter Cards** (Twitter/X)
   - summary, summary_large_image, app, player
   - title, description, image, creator

3. **Standard HTML Meta Tags**
   - description, keywords, author, canonical
   - Favicons, RSS feeds, viewport

#### Phase 3: Schema.org & SEO 🚀
**Priority: HIGHEST** (41% adoption, growing)

4. **JSON-LD** (Google Rich Results, AI Training)
   - Article, NewsArticle, BlogPosting
   - Product, Offer, Review, Rating
   - Recipe, Event, LocalBusiness
   - Person, Organization
   - BreadcrumbList, FAQPage
   - And 100+ more Schema.org types

5. **Microdata** (HTML5 inline structured data)
   - Same Schema.org vocabulary as JSON-LD
   - Inline in HTML (declining, but still 26% adoption)

#### Phase 4: Completeness

6. **More Microformats**
   - h-feed, h-review, h-product
   - h-recipe, h-adr, h-geo
   - h-resume, h-review-aggregate

7. **RDFa** (Semantic web)
   - Government sites, academic institutions

8. **Other Formats**
   - Dublin Core (libraries, archives)
   - oEmbed (embedded content)

---

## The API (Planned)

### Python

```python
import meta_oxide

html = fetch_webpage("https://example.com/article")

# Extract EVERYTHING at once
data = meta_oxide.extract_all(html, base_url="https://example.com")

# Returns unified structure:
{
    "meta": {
        "title": "Article Title",
        "description": "...",
        "canonical": "https://example.com/article"
    },
    "opengraph": {
        "title": "Article Title",
        "type": "article",
        "image": "https://example.com/hero.jpg"
    },
    "twitter": {
        "card": "summary_large_image",
        "title": "Article Title"
    },
    "jsonld": [
        {
            "@type": "Article",
            "headline": "Article Title",
            "author": {...}
        }
    ],
    "microformats": {
        "h-entry": [...],
        "h-card": [...]
    }
}

# Or extract specific formats
og_data = meta_oxide.extract_opengraph(html)
twitter_data = meta_oxide.extract_twitter(html)
jsonld_data = meta_oxide.extract_jsonld(html)
```

### Rust

```rust
use meta_oxide::extract_all;

let html = r#"<html>...</html>"#;
let data = extract_all(html, Some("https://example.com"))?;

println!("Open Graph title: {:?}", data.opengraph.title);
println!("JSON-LD data: {:?}", data.jsonld);
```

---

## Use Cases

### 1. **Web Scraping & Data Mining**
Extract all metadata from websites for analysis, research, or archiving.

### 2. **SEO Tools**
Build SEO analyzers that check for proper Open Graph, Twitter Cards, JSON-LD implementation.

### 3. **Social Media Tools**
Preview how links will appear when shared on Facebook, Twitter, LinkedIn, etc.

### 4. **Content Management Systems**
Validate and extract structured data from CMS-generated pages.

### 5. **AI/LLM Training Data**
Extract structured data for training machine learning models.

### 6. **Blog Aggregators**
Collect and aggregate content from multiple blogs using h-entry and JSON-LD.

### 7. **IndieWeb Applications**
Build decentralized social web applications using microformats.

### 8. **E-commerce Analytics**
Extract product data, reviews, prices from e-commerce sites.

### 9. **Event Aggregators**
Collect event information from multiple sources.

### 10. **Link Preview Services**
Build services that generate rich link previews (like Slack, Discord).

---

## Current Project Structure

```
meta_oxide/
├── src/                          # Rust source code
│   ├── lib.rs                   # Python bindings (PyO3)
│   ├── parser.rs                # HTML parser
│   ├── types.rs                 # Data structures
│   ├── errors.rs                # Error handling
│   └── extractors/              # Format extractors
│       ├── hcard.rs            # ✅ h-card
│       ├── hentry.rs           # ✅ h-entry
│       ├── hevent.rs           # ✅ h-event
│       ├── opengraph.rs        # 🔜 Open Graph
│       ├── twitter.rs          # 🔜 Twitter Cards
│       ├── jsonld.rs           # 🔜 JSON-LD
│       ├── microdata.rs        # 🔜 Microdata
│       └── ...
│
├── docs/                        # Documentation
│   ├── getting-started.md      # Installation & basics
│   ├── api-reference.md        # API docs
│   ├── FORMATS.md              # Complete format guide
│   ├── FORMAT_SUMMARY.md       # Quick reference
│   ├── ROADMAP.md              # Development roadmap
│   ├── examples.md             # Usage examples
│   ├── architecture.md         # System design
│   └── development.md          # Contributing guide
│
├── examples/                    # Example code
│   └── basic_usage.py          # Python examples
│
├── python/tests/                # Python tests
│   └── test_basic.py
│
├── Cargo.toml                   # Rust dependencies
├── pyproject.toml               # Python package config
├── README.md                    # Project overview
└── CHANGELOG.md                 # Version history
```

---

## Technology Stack

### Core Technologies
- **Rust** 1.70+ - Core library, performance
- **PyO3** 0.22+ - Python bindings
- **scraper** 0.20+ - HTML parsing
- **serde** 1.0+ - JSON serialization

### For Users
- **Python** 3.8+ - Easy Python API
- **maturin** - Build Python packages from Rust

---

## Performance Targets

| HTML Size | Target Parse Time | Status |
|-----------|------------------|--------|
| 10 KB     | < 100 µs         | ⏳ Testing |
| 100 KB    | < 1 ms           | ⏳ Testing |
| 1 MB      | < 10 ms          | ⏳ Testing |

**Why fast matters:**
- Scrape millions of pages efficiently
- Real-time link preview generation
- Large-scale data mining
- Browser extensions

---

## Competitive Landscape

### Existing Python Libraries

**extruct**
- ✅ Supports multiple formats
- ❌ Slow (pure Python)
- ❌ Heavy dependencies

**mf2py**
- ✅ Good microformats support
- ❌ Microformats only
- ❌ Python performance

**beautifulsoup + custom**
- ✅ Flexible
- ❌ Manual implementation
- ❌ No standard API

### MetaOxide Advantages

| Feature | extruct | mf2py | BeautifulSoup | MetaOxide |
|---------|---------|-------|---------------|-----------|
| **Speed** | Slow | Slow | Slow | ⚡ Blazing Fast |
| **All Formats** | ✅ | ❌ | Manual | ✅ |
| **Type-Safe** | ❌ | ❌ | ❌ | ✅ |
| **Easy API** | ✅ | ✅ | ❌ | ✅ |
| **Python + Rust** | ❌ | ❌ | ❌ | ✅ |
| **Maintained** | ⚠️ | ⚠️ | ✅ | ✅ |

---

## Development Roadmap

### Q1 2024: Foundation ✅
- [x] Project structure
- [x] Basic microformats (h-card, h-entry, h-event)
- [x] Python bindings
- [x] Documentation

### Q2 2024: Social Media 🎯
- [ ] Open Graph Protocol
- [ ] Twitter Cards
- [ ] Standard meta tags
- [ ] More microformats (h-feed, h-review, h-product)

### Q3 2024: Schema.org & SEO 🚀
- [ ] JSON-LD parser
- [ ] Common Schema.org types
- [ ] Validation
- [ ] E-commerce support

### Q4 2024: Completeness
- [ ] Microdata
- [ ] RDFa
- [ ] All microformats
- [ ] All Schema.org types

### 2025: Advanced Features
- [ ] CLI tool
- [ ] Browser extension
- [ ] VS Code extension
- [ ] Validation tools
- [ ] Conversion utilities

---

## Getting Started

### Installation (Future)

```bash
# Python
pip install meta-oxide

# Rust
cargo add meta_oxide
```

### Build from Source (Now)

```bash
# Clone repository
git clone https://github.com/yourusername/meta_oxide.git
cd meta_oxide

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build Rust library
cargo build
cargo test

# Build Python package
pip install maturin
maturin develop

# Run examples
python examples/basic_usage.py
```

---

## Documentation

Start here based on your role:

### I'm a User
1. **[Getting Started](docs/getting-started.md)** - Installation and basics
2. **[FORMAT_SUMMARY](docs/FORMAT_SUMMARY.md)** - Quick reference
3. **[Examples](docs/examples.md)** - Usage examples

### I'm a Developer
1. **[ROADMAP](docs/ROADMAP.md)** - What we're building
2. **[FORMATS](docs/FORMATS.md)** - All format details
3. **[Development Guide](docs/development.md)** - How to contribute
4. **[Architecture](docs/architecture.md)** - How it works

### I Want to Contribute
1. **[ROADMAP](docs/ROADMAP.md)** - Pick a format to implement
2. **[Development Guide](docs/development.md)** - Setup and guidelines
3. Open an issue or PR on GitHub

---

## Why "MetaOxide"?

- **Meta** - Metadata, all types of structured data
- **Oxide** - Rust (iron oxide), performance and safety
- **MetaOxide** - Comprehensive metadata extraction in Rust

---

## Status

**Current Version**: 0.1.0-alpha
**Status**: Early development, microformats partially implemented
**Next Milestone**: Open Graph + Twitter Cards support

---

## Contributing

Contributions are welcome! This is an ambitious project and we need help:

1. **Implement formats** - Pick a format from ROADMAP.md
2. **Write tests** - Improve test coverage
3. **Documentation** - Improve guides and examples
4. **Performance** - Optimize parsing and extraction
5. **Use cases** - Share how you're using MetaOxide

See [Development Guide](docs/development.md) for details.

---

## License

Dual-licensed under MIT or Apache-2.0 (your choice).

---

## Links

- **Repository**: https://github.com/yourusername/meta_oxide
- **Documentation**: [docs/](docs/)
- **Roadmap**: [docs/ROADMAP.md](docs/ROADMAP.md)
- **Formats Guide**: [docs/FORMATS.md](docs/FORMATS.md)

---

**Built with ❤️ and 🦀 Rust**
