# Getting Started with MetaOxide

MetaOxide is a high-performance Rust library with Python bindings for extracting microformats data from HTML. It supports various microformat types including h-card, h-entry, h-event, and more.

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Prerequisites](#prerequisites)
- [Building from Source](#building-from-source)
- [Basic Usage](#basic-usage)
- [Next Steps](#next-steps)

## Installation

### For Python Users

Once published, you'll be able to install via pip:

```bash
pip install meta-oxide
```

### For Rust Users

Add to your `Cargo.toml`:

```toml
[dependencies]
meta_oxide = "0.1.0"
```

## Prerequisites

### Python Development

- Python 3.8 or higher
- pip (Python package manager)

### Rust Development

- Rust 1.70 or higher
- Cargo (Rust package manager)

### Building Python Bindings

For building the Python package from source:

- Python 3.8+
- Rust toolchain (install via [rustup](https://rustup.rs/))
- maturin (install via `pip install maturin`)

## Building from Source

### Clone the Repository

```bash
git clone https://github.com/yourusername/meta_oxide.git
cd meta_oxide
```

### Build Python Package

Using maturin for development:

```bash
# Install maturin if you haven't already
pip install maturin

# Build and install in development mode
maturin develop

# Or build a wheel
maturin build --release
```

### Build Rust Library

```bash
# Build the library
cargo build --release

# Run tests
cargo test

# Build documentation
cargo doc --open
```

## Basic Usage

### Python

```python
import meta_oxide

# Extract all microformats
html = """
<div class="h-card">
    <span class="p-name">Jane Doe</span>
    <a class="u-url" href="https://example.com">Website</a>
    <a class="u-email" href="mailto:jane@example.com">Email</a>
</div>
"""

# Extract all microformats at once
result = meta_oxide.extract_microformats(html)
print(result)

# Extract specific microformat types
cards = meta_oxide.extract_hcard(html)
print(f"Found {len(cards)} h-cards")

for card in cards:
    print(f"Name: {card.get('name')}")
    print(f"URL: {card.get('url')}")
    print(f"Email: {card.get('email')}")
```

### Rust

```rust
use meta_oxide::extractors::{extract_hcard, extract_hentry, extract_hevent};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = r#"
        <div class="h-card">
            <span class="p-name">Jane Doe</span>
            <a class="u-url" href="https://example.com">Website</a>
        </div>
    "#;

    let cards = extract_hcard(html, None)?;

    for card in cards {
        println!("Name: {:?}", card.name);
        println!("URL: {:?}", card.url);
    }

    Ok(())
}
```

## Supported Microformats

MetaOxide currently supports the following microformat types:

- **h-card**: Personal or organizational contact information
- **h-entry**: Blog posts, articles, and other content entries
- **h-event**: Events with dates, times, and locations
- More formats coming soon!

## Next Steps

- Read the [API Reference](api-reference.md) for detailed function documentation
- Check out [Examples](examples.md) for more usage patterns
- Learn about the [Architecture](architecture.md) to understand how it works
- See [Development Guide](development.md) to contribute to the project

## Common Issues

### Python Module Not Found

If you get `ModuleNotFoundError: No module named 'meta_oxide'`:

1. Make sure you've built and installed the package: `maturin develop`
2. Verify you're using the correct Python environment
3. Check that the build completed successfully

### Rust Compilation Errors

If you encounter compilation errors:

1. Update Rust: `rustup update`
2. Clean the build: `cargo clean`
3. Rebuild: `cargo build --release`

## Getting Help

- Check the [examples](examples.md) for common use cases
- Read the [API documentation](api-reference.md)
- Open an issue on GitHub
- Join our community discussions

## License

MetaOxide is dual-licensed under MIT or Apache-2.0.
