# Development Guide

Guide for contributing to and developing MetaOxide.

## Table of Contents

- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Building](#building)
- [Testing](#testing)
- [Adding New Microformats](#adding-new-microformats)
- [Code Style](#code-style)
- [Contributing](#contributing)
- [Release Process](#release-process)

---

## Development Setup

### Prerequisites

1. **Rust Toolchain**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   rustup update
   ```

2. **Python Development Environment**
   ```bash
   # Install Python 3.8+
   python3 --version

   # Create virtual environment
   python3 -m venv venv
   source venv/bin/activate  # On Windows: venv\Scripts\activate
   ```

3. **Maturin** (for Python bindings)
   ```bash
   pip install maturin
   ```

4. **Development Tools**
   ```bash
   # Rust formatter and linter
   rustup component add rustfmt clippy

   # Python tools
   pip install pytest black mypy ruff
   ```

### Clone and Setup

```bash
# Clone repository
git clone https://github.com/yourusername/meta_oxide.git
cd meta_oxide

# Install development dependencies
pip install -r requirements-dev.txt

# Build in development mode
maturin develop
```

---

## Project Structure

```
meta_oxide/
├── Cargo.toml              # Rust package configuration
├── pyproject.toml          # Python package configuration
├── README.md               # Project overview
├── LICENSE                 # License file
├── src/                    # Rust source code
│   ├── lib.rs              # Python bindings (PyO3)
│   ├── parser.rs           # HTML parser
│   ├── types.rs            # Data structures
│   ├── errors.rs           # Error types
│   └── extractors/         # Microformat extractors
│       ├── mod.rs          # Extractor module
│       ├── hcard.rs        # h-card extractor
│       ├── hentry.rs       # h-entry extractor
│       └── hevent.rs       # h-event extractor
├── tests/                  # Integration tests
│   ├── test_hcard.rs
│   ├── test_hentry.rs
│   └── test_hevent.rs
├── python/                 # Python-specific code
│   └── tests/              # Python tests
│       ├── test_hcard.py
│       ├── test_hentry.py
│       └── test_hevent.py
├── docs/                   # Documentation
│   ├── getting-started.md
│   ├── architecture.md
│   ├── api-reference.md
│   ├── examples.md
│   └── development.md
├── examples/               # Example code
│   ├── basic_usage.py
│   ├── rust_example.rs
│   └── web_scraper.py
└── benches/                # Benchmarks
    └── extraction_bench.rs
```

---

## Building

### Build Rust Library

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Build and run tests
cargo test

# Build documentation
cargo doc --open
```

### Build Python Package

```bash
# Development build (with debug symbols)
maturin develop

# Release build
maturin build --release

# Build wheel
maturin build --release --out dist/

# Install from wheel
pip install dist/meta_oxide-*.whl
```

---

## Testing

### Rust Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_extract_hcard

# Run tests with output
cargo test -- --nocapture

# Run tests in specific module
cargo test extractors::hcard

# Run integration tests
cargo test --test '*'
```

### Python Tests

```bash
# Make sure package is built
maturin develop

# Run pytest
pytest python/tests/

# Run with coverage
pytest --cov=meta_oxide python/tests/

# Run specific test
pytest python/tests/test_hcard.py::test_basic_hcard
```

### Test Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage/

# Open report
open coverage/index.html
```

---

## Adding New Microformats

### Step 1: Define the Type

Add a new struct in `src/types.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HReview {
    pub name: Option<String>,
    pub rating: Option<String>,
    pub item: Option<String>,
    pub summary: Option<String>,
    pub reviewer: Option<Box<HCard>>,
    pub additional_properties: HashMap<String, Vec<String>>,
}

impl HReview {
    pub fn to_py_dict(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);

        if let Some(name) = &self.name {
            dict.set_item("name", name).unwrap();
        }
        if let Some(rating) = &self.rating {
            dict.set_item("rating", rating).unwrap();
        }
        // ... add other fields

        dict.into()
    }
}
```

### Step 2: Create Extractor

Create `src/extractors/hreview.rs`:

```rust
use crate::errors::{MicroformatError, Result};
use crate::types::HReview;
use scraper::{Html, Selector};
use std::collections::HashMap;

pub fn extract(html: &str, _base_url: Option<&str>) -> Result<Vec<HReview>> {
    let document = Html::parse_document(html);
    let mut reviews = Vec::new();

    let selector = Selector::parse(".h-review")
        .map_err(|e| MicroformatError::ParseError(e.to_string()))?;

    for element in document.select(&selector) {
        let mut review = HReview {
            name: None,
            rating: None,
            item: None,
            summary: None,
            reviewer: None,
            additional_properties: HashMap::new(),
        };

        // Extract properties
        if let Ok(name_sel) = Selector::parse(".p-name") {
            if let Some(name_elem) = element.select(&name_sel).next() {
                review.name = Some(name_elem.text().collect::<String>().trim().to_string());
            }
        }

        // ... extract other properties

        reviews.push(review);
    }

    Ok(reviews)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hreview() {
        let html = r#"
            <div class="h-review">
                <span class="p-name">Great Product</span>
                <span class="p-rating">5</span>
            </div>
        "#;

        let reviews = extract(html, None).unwrap();
        assert_eq!(reviews.len(), 1);
        assert_eq!(reviews[0].name, Some("Great Product".to_string()));
        assert_eq!(reviews[0].rating, Some("5".to_string()));
    }
}
```

### Step 3: Update Module

Add to `src/extractors/mod.rs`:

```rust
pub mod hreview;
pub use hreview::extract as extract_hreview;
```

### Step 4: Add Python Binding

Add to `src/lib.rs`:

```rust
#[pyfunction]
fn extract_hreview(html: &str, base_url: Option<&str>) -> PyResult<Vec<PyObject>> {
    Python::with_gil(|py| {
        let reviews = extractors::hreview::extract(html, base_url)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        Ok(reviews.iter().map(|review| review.to_py_dict(py).into()).collect())
    })
}

#[pymodule]
fn meta_oxide(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(extract_microformats, m)?)?;
    m.add_function(wrap_pyfunction!(extract_hcard, m)?)?;
    m.add_function(wrap_pyfunction!(extract_hentry, m)?)?;
    m.add_function(wrap_pyfunction!(extract_hevent, m)?)?;
    m.add_function(wrap_pyfunction!(extract_hreview, m)?)?;  // Add this
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
```

### Step 5: Add Tests

Create `python/tests/test_hreview.py`:

```python
import meta_oxide

def test_extract_hreview():
    html = """
    <div class="h-review">
        <span class="p-name">Great Product</span>
        <span class="p-rating">5</span>
    </div>
    """

    reviews = meta_oxide.extract_hreview(html)
    assert len(reviews) == 1
    assert reviews[0]['name'] == 'Great Product'
    assert reviews[0]['rating'] == '5'
```

### Step 6: Update Documentation

1. Add entry to `docs/api-reference.md`
2. Add example to `docs/examples.md`
3. Update README.md supported formats list

---

## Code Style

### Rust

Follow Rust conventions and use the formatter:

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy -- -D warnings

# Fix clippy suggestions
cargo clippy --fix
```

**Style Guidelines:**
- Use 4 spaces for indentation
- Maximum line length: 100 characters
- Use descriptive variable names
- Add documentation comments for public APIs
- Use `Result<T, E>` for functions that can fail

**Example:**
```rust
/// Extract h-card microformats from HTML
///
/// # Arguments
///
/// * `html` - HTML string to parse
/// * `base_url` - Optional base URL for resolving relative URLs
///
/// # Returns
///
/// Vector of extracted h-cards
///
/// # Errors
///
/// Returns `MicroformatError` if parsing fails
pub fn extract(html: &str, base_url: Option<&str>) -> Result<Vec<HCard>> {
    // Implementation
}
```

### Python

Follow PEP 8 and use the formatter:

```bash
# Format with black
black python/

# Lint with ruff
ruff check python/

# Type check with mypy
mypy python/
```

---

## Benchmarking

### Create Benchmark

Create `benches/extraction_bench.rs`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use meta_oxide::extractors::extract_hcard;

fn bench_hcard_extraction(c: &mut Criterion) {
    let html = r#"
        <div class="h-card">
            <span class="p-name">John Doe</span>
            <a class="u-url" href="https://example.com">Website</a>
        </div>
    "#;

    c.bench_function("extract_hcard", |b| {
        b.iter(|| extract_hcard(black_box(html), None))
    });
}

criterion_group!(benches, bench_hcard_extraction);
criterion_main!(benches);
```

### Run Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench extract_hcard

# Generate HTML report
cargo bench -- --save-baseline my_baseline
```

---

## Contributing

### Workflow

1. **Fork the repository**
   ```bash
   # On GitHub, click "Fork"
   git clone https://github.com/yourusername/meta_oxide.git
   ```

2. **Create a branch**
   ```bash
   git checkout -b feature/my-new-feature
   ```

3. **Make changes**
   - Write code
   - Add tests
   - Update documentation

4. **Run tests and linters**
   ```bash
   cargo test
   cargo fmt
   cargo clippy
   pytest python/tests/
   ```

5. **Commit changes**
   ```bash
   git add .
   git commit -m "Add new feature: description"
   ```

6. **Push and create PR**
   ```bash
   git push origin feature/my-new-feature
   # Create pull request on GitHub
   ```

### Commit Message Format

Use conventional commits:

```
type(scope): short description

Longer description if needed

Fixes #123
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Test changes
- `refactor`: Code refactoring
- `perf`: Performance improvement
- `chore`: Maintenance tasks

**Examples:**
```
feat(extractors): add h-review support
fix(parser): handle nested microformats correctly
docs(api): update h-card examples
test(hentry): add test for author extraction
```

---

## Release Process

### Version Bumping

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag

```bash
# Bump version
# Edit Cargo.toml: version = "0.2.0"

# Update changelog
# Edit CHANGELOG.md

# Commit
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 0.2.0"

# Tag
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

### Publishing to PyPI

```bash
# Build wheels for multiple platforms
maturin build --release

# Publish to PyPI
maturin publish --username __token__ --password $PYPI_TOKEN
```

### Publishing to crates.io

```bash
# Login
cargo login

# Publish
cargo publish
```

---

## Debugging

### Debug Rust Code

```bash
# Build with debug symbols
cargo build

# Run with debugger (lldb on macOS, gdb on Linux)
rust-lldb target/debug/meta_oxide
```

### Debug Python Bindings

```python
import meta_oxide
import pdb

html = "..."

# Set breakpoint
pdb.set_trace()
result = meta_oxide.extract_hcard(html)
```

### Enable Logging

Add to `Cargo.toml`:

```toml
[dependencies]
env_logger = "0.11"
log = "0.4"
```

Use in code:

```rust
use log::{debug, info, warn, error};

info!("Parsing HTML document");
debug!("Found {} elements", elements.len());
```

Run with logging:

```bash
RUST_LOG=debug cargo test
```

---

## IDE Setup

### VS Code

Recommended extensions:
- rust-analyzer
- Python
- Even Better TOML
- CodeLLDB (for debugging)

Settings (`.vscode/settings.json`):

```json
{
    "rust-analyzer.check.command": "clippy",
    "python.linting.enabled": true,
    "python.linting.ruffEnabled": true,
    "python.formatting.provider": "black",
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer",
        "editor.formatOnSave": true
    }
}
```

### PyCharm / IntelliJ IDEA

1. Install Rust plugin
2. Configure Python interpreter
3. Enable "Format on save"

---

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [PyO3 Guide](https://pyo3.rs/)
- [Microformats Wiki](http://microformats.org/wiki/)
- [scraper Documentation](https://docs.rs/scraper/)

---

## Getting Help

- Open an issue on GitHub
- Join discussions
- Check existing documentation
- Read the code (it's well-commented!)
