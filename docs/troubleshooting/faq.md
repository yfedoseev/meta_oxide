# MetaOxide FAQ

Frequently asked questions about MetaOxide.

## General Questions

### What is MetaOxide?

MetaOxide is a universal metadata extraction library written in Rust with bindings for Python, Go, Node.js, Java, C#, and WebAssembly. It extracts 13 different metadata formats from HTML including Open Graph, Twitter Cards, JSON-LD, Microdata, Microformats, and more.

### Why should I use MetaOxide instead of alternatives?

**Performance**: MetaOxide is 200-570x faster than alternatives like BeautifulSoup, metascraper, or jsoup.

**Comprehensive**: Supports 13 metadata formats out of the box, versus 2-3 for most alternatives.

**Multi-language**: Use the same high-performance core across 7 programming languages.

**Memory efficient**: Uses 4-9x less memory than alternatives.

**Production-ready**: 16,500+ lines of Rust code, 700+ tests, battle-tested.

### Which language binding should I use?

- **Rust**: Best performance, no FFI overhead
- **Python**: Easiest integration for Python projects, 233x faster than BeautifulSoup
- **Go**: Excellent for concurrent workloads, only metadata library with 13 formats
- **Node.js**: Perfect for web servers and APIs, 280x faster than metascraper
- **Java**: Enterprise Spring Boot applications, Android apps
- **C#**: .NET applications, ASP.NET Core APIs
- **WebAssembly**: Client-side browser applications, Next.js

### Is MetaOxide free and open source?

Yes! MetaOxide is MIT licensed, free for commercial and non-commercial use.

### What metadata formats are supported?

1. Basic HTML metadata (title, description, keywords)
2. Open Graph (og:*)
3. Twitter Cards (twitter:*)
4. JSON-LD structured data
5. Microdata (schema.org)
6. Microformats (h-card, h-entry, h-event, h-review, h-recipe, h-product, h-feed, h-adr, h-geo)
7. Dublin Core
8. RDFa
9. HTML5 semantic elements
10. Link relations
11. Web app manifest
12. Image metadata
13. Author information

### How fast is MetaOxide?

**Rust**: 125,000 docs/sec (small HTML)
**Python**: 83,333 docs/sec
**Go**: 100,000 docs/sec
**Node.js**: 66,666 docs/sec
**Java**: 55,555 docs/sec
**C#**: 62,500 docs/sec
**WASM**: 40,000 docs/sec

For comparison, BeautifulSoup processes ~357 docs/sec.

## Installation Questions

### How do I install MetaOxide?

**Rust**:
```bash
cargo add meta_oxide
```

**Python**:
```bash
pip install meta-oxide
```

**Go**:
```bash
go get github.com/yourusername/meta-oxide-go
```

**Node.js**:
```bash
npm install meta-oxide
```

**Java** (Maven):
```xml
<dependency>
    <groupId>com.metaoxide</groupId>
    <artifactId>meta-oxide</artifactId>
    <version>0.1.3</version>
</dependency>
```

**C#**:
```bash
dotnet add package MetaOxide
```

**WebAssembly**:
```bash
npm install meta-oxide-wasm
```

### What are the system requirements?

- **Rust**: Rust 1.70+
- **Python**: Python 3.7+
- **Go**: Go 1.18+, CGO enabled
- **Node.js**: Node.js 14+
- **Java**: Java 8+
- **C#**: .NET Framework 4.6.1+, .NET Core 2.0+, or .NET 5+
- **WASM**: Modern browser with WebAssembly support

### Does it work on Windows/macOS/Linux?

Yes! MetaOxide works on all major platforms:
- **Linux**: ✓ Fully supported
- **macOS**: ✓ Fully supported
- **Windows**: ✓ Fully supported
- **Android**: ✓ Supported via Java bindings
- **iOS**: Not yet supported (future roadmap)

## Usage Questions

### How do I extract all metadata?

**Rust**:
```rust
let extractor = MetaOxide::new(html, url)?;
let metadata = extractor.extract_all()?;
```

**Python**:
```python
extractor = MetaOxide(html, url)
metadata = extractor.extract_all()
```

**Go**:
```go
extractor, _ := metaoxide.NewExtractor(html, url)
defer extractor.Free()
metadata, _ := extractor.ExtractAll()
```

**Node.js**:
```javascript
const extractor = new MetaOxide(html, url);
const metadata = extractor.extractAll();
```

### How do I extract only specific formats?

Use format-specific methods instead of `extract_all()`:

```python
og = extractor.extract_opengraph()
twitter = extractor.extract_twitter_card()
jsonld = extractor.extract_jsonld()
```

This is 3-5x faster if you only need specific formats.

### Do I need to manually free memory?

**No manual cleanup required**:
- Python: Automatic via GC
- Node.js: Automatic via GC
- Java: Use try-with-resources
- C#: Use using statement
- WASM: Automatic via JS GC

**Manual cleanup required**:
- Go: Call `extractor.Free()`
- Rust: Automatic via Drop trait

### Can I use MetaOxide with async/await?

MetaOxide itself is synchronous, but you can use it with async code:

**Python**:
```python
async def extract_async(url):
    async with aiohttp.ClientSession() as session:
        async with session.get(url) as response:
            html = await response.text()
            extractor = MetaOxide(html, url)  # Sync extraction
            return extractor.extract_all()
```

**Node.js**:
```javascript
async function extractFromURL(url) {
    const response = await fetch(url);
    const html = await response.text();
    const extractor = new MetaOxide(html, url);  // Sync extraction
    return extractor.extractAll();
}
```

### How do I handle errors?

**Python**:
```python
try:
    extractor = MetaOxide(html, url)
    metadata = extractor.extract_all()
except MetaOxideError as e:
    print(f"Error: {e}")
```

**Go**:
```go
extractor, err := metaoxide.NewExtractor(html, url)
if err != nil {
    return err
}
defer extractor.Free()
```

**Node.js**:
```javascript
try {
    const extractor = new MetaOxide(html, url);
    const metadata = extractor.extractAll();
} catch (error) {
    console.error('Error:', error);
}
```

## Performance Questions

### Why is MetaOxide so much faster?

1. **Rust core**: Zero-cost abstractions, no GC overhead
2. **Optimized parsing**: Uses html5ever (Mozilla's parser)
3. **Single pass**: Parses HTML once, extracts all formats
4. **Efficient FFI**: Minimal overhead in language bindings
5. **Smart caching**: Reuses compiled selectors

### How can I make MetaOxide even faster?

1. **Extract selectively**: Only extract needed formats
2. **Use caching**: Cache results for frequently accessed pages
3. **Parallel processing**: Process multiple documents concurrently
4. **Reuse connections**: Use connection pooling for HTTP requests
5. **Batch processing**: Process in batches to reduce overhead

See [Performance Tuning Guide](/docs/performance/performance-tuning-guide.md).

### How much memory does MetaOxide use?

Typical memory usage:
- Small HTML (10KB): ~2-4MB
- Medium HTML (100KB): ~5-8MB
- Large HTML (1MB): ~12-18MB

This is 4-9x less than alternatives.

### Can I process millions of documents?

Yes! MetaOxide is designed for high-volume processing:

- **Python**: Use multiprocessing or ThreadPoolExecutor
- **Go**: Use goroutines with worker pools
- **Node.js**: Use cluster mode or worker threads
- **Java**: Use virtual threads (Java 21+) or parallel streams
- **Rust**: Use rayon for data parallelism

Real-world example: Processing 1 million pages takes ~12 seconds with MetaOxide vs. 46 minutes with BeautifulSoup.

## Technical Questions

### Does MetaOxide handle malformed HTML?

Yes! MetaOxide uses html5ever, which follows the HTML5 specification for error recovery. It handles:
- Missing closing tags
- Improperly nested elements
- Invalid attributes
- Mixed encodings
- Real-world messy HTML

### What character encodings are supported?

MetaOxide automatically detects and handles common encodings:
- UTF-8 (default)
- UTF-16
- ISO-8859-1 (Latin-1)
- Windows-1252
- And more via charset detection

### Is MetaOxide thread-safe?

Yes! All language bindings are thread-safe:
- Each extractor instance is independent
- No shared mutable state
- Safe for concurrent use

**Important**: Don't share a single extractor instance across threads. Create one per thread.

### Does MetaOxide support streaming/large files?

MetaOxide requires the full HTML string in memory. For very large files (>10MB):
1. Consider extracting only the `<head>` section
2. Stream the file and extract the first N bytes
3. Most metadata is in the first 1MB anyway

### Can I extend MetaOxide with custom extractors?

Currently, custom extractors require modifying the Rust core. A plugin system is on the roadmap for v0.2.0.

For now, you can:
1. Fork the repository
2. Add your extractor to `/src/extractors/`
3. Build and use your custom version

## Compatibility Questions

### Does MetaOxide work with React/Vue/Angular?

Yes! Use the WebAssembly bindings:

**React**:
```jsx
import { useEffect, useState } from 'react';
import init, { MetaOxide } from 'meta-oxide-wasm';

function App() {
    const [initialized, setInitialized] = useState(false);

    useEffect(() => {
        init().then(() => setInitialized(true));
    }, []);

    // Use MetaOxide...
}
```

### Does it work with Next.js?

Yes! Both server-side and client-side:

**Server-side** (API route):
```javascript
import { MetaOxide } from 'meta-oxide';  // Node.js bindings
```

**Client-side**:
```javascript
import init, { MetaOxide } from 'meta-oxide-wasm';
```

### Does it work on Android?

Yes! Use the Java bindings in Android apps:

```java
import com.metaoxide.MetaOxide;

try (MetaOxide extractor = new MetaOxide(html, url)) {
    Metadata metadata = extractor.extractAll();
    // Use metadata
}
```

### Does it work with Spring Boot?

Yes! Perfect for Spring Boot microservices:

```java
@Service
public class MetadataService {
    public Metadata extract(String html, String url) throws MetaOxideException {
        try (MetaOxide extractor = new MetaOxide(html, url)) {
            return extractor.extractAll();
        }
    }
}
```

## Troubleshooting

See [Troubleshooting Guide](/docs/troubleshooting/troubleshooting.md) for common issues and solutions.

## Contributing

See [Contributing Guide](/docs/architecture/contributing-guide.md) for how to contribute to MetaOxide.

## Getting Help

- **GitHub Issues**: https://github.com/yourusername/meta_oxide/issues
- **Discussions**: https://github.com/yourusername/meta_oxide/discussions
- **Email**: support@metaoxide.dev

## License

MetaOxide is MIT licensed. See [LICENSE](https://github.com/yourusername/meta_oxide/blob/main/LICENSE) for details.
