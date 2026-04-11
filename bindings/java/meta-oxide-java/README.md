# MetaOxide Java Bindings

[![Maven Central](https://img.shields.io/maven-central/v/io.github.yfedoseev/meta-oxide.svg)](https://search.maven.org/artifact/io.github.yfedoseev/meta-oxide)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](../../LICENSE)
[![Java](https://img.shields.io/badge/Java-8%2B-orange.svg)](https://www.oracle.com/java/)

Fast, comprehensive metadata extraction for Java - powered by Rust performance with a simple Java API.

## Features

- **Fast**: Native Rust performance (10-100x faster than pure-Java parsers)
- **Comprehensive**: Extracts 13 metadata formats including Open Graph, Twitter Cards, JSON-LD, Microdata, RDFa, Microformats, and more
- **Easy to use**: Simple, idiomatic Java API
- **Thread-safe**: Concurrent extraction supported
- **Zero external dependencies**: Self-contained native library
- **Cross-platform**: Works on Linux, macOS, Windows
- **Java 8+**: Compatible with Java 8 through 21+
- **Android Ready**: Works on Android API 21+

## Supported Formats

MetaOxide extracts **13 metadata formats**:

| Format | Adoption | Use Case |
|--------|----------|----------|
| **Standard Meta Tags** | 100% | Title, description, keywords, canonical |
| **Open Graph** | 60%+ | Facebook, LinkedIn, WhatsApp previews |
| **Twitter Cards** | 45% | Twitter/X link previews |
| **JSON-LD** | 41% ↗️ | Google Rich Results, Schema.org, AI training |
| **Microdata** | 26% | E-commerce, recipes, reviews |
| **RDFa** | 62%* | Semantic web, government sites |
| **Microformats** | 5-10% | IndieWeb (h-card, h-entry, h-event, etc.) |
| **Web App Manifest** | Growing | Progressive Web Apps |
| **Dublin Core** | <5% | Academic, library metadata |
| **oEmbed** | Moderate | YouTube, Twitter embeds |
| **rel-* Links** | Common | Canonical, alternate, feed |

_*62% adoption on desktop pages (HTTP Archive 2024)_

## Installation

### Maven

Add to your `pom.xml`:

```xml
<dependency>
    <groupId>io.github.yfedoseev</groupId>
    <artifactId>meta-oxide</artifactId>
    <version>0.1.3</version>
</dependency>
```

### Gradle

Add to your `build.gradle`:

```groovy
dependencies {
    implementation 'io.github.yfedoseev:meta-oxide:0.1.3'
}
```

### Android

Add to your `build.gradle` (app module):

```groovy
dependencies {
    implementation 'io.github.yfedoseev:meta-oxide:0.1.3'
}
```

**Note**: Native libraries for Android architectures (armeabi-v7a, arm64-v8a, x86, x86_64) are included.

## Quick Start

### Extract All Metadata (Recommended!)

The most efficient way to extract metadata - parses HTML only once:

```java
import io.github.yfedoseev.metaoxide.Extractor;
import io.github.yfedoseev.metaoxide.ExtractionResult;

String html = """
    <html>
    <head>
        <title>Amazing Article</title>
        <meta property="og:title" content="Amazing Article">
        <meta name="twitter:card" content="summary_large_image">
        <script type="application/ld+json">
        {
            "@context": "https://schema.org",
            "@type": "Article",
            "headline": "Amazing Article"
        }
        </script>
    </head>
    </html>
    """;

ExtractionResult result = Extractor.extractAll(html, "https://example.com");

// Access all formats
System.out.println("Title: " + result.meta.get("title"));
System.out.println("OG Title: " + result.openGraph.get("title"));
System.out.println("Twitter Card: " + result.twitter.get("card"));
System.out.println("JSON-LD: " + result.jsonLd.get(0));
```

### Extract Individual Formats

Extract specific formats when you only need certain metadata:

```java
import java.util.Map;
import java.util.List;

// Standard meta tags
Map<String, Object> meta = Extractor.extractMeta(html, "https://example.com");
System.out.println("Title: " + meta.get("title"));
System.out.println("Description: " + meta.get("description"));

// Open Graph
Map<String, Object> og = Extractor.extractOpenGraph(html, "https://example.com");
System.out.println("OG Title: " + og.get("title"));
System.out.println("OG Image: " + og.get("image"));

// Twitter Cards
Map<String, Object> twitter = Extractor.extractTwitter(html, "https://example.com");
System.out.println("Card Type: " + twitter.get("card"));

// JSON-LD (Schema.org)
List<Object> jsonLd = Extractor.extractJsonLd(html, "https://example.com");
for (Object item : jsonLd) {
    Map<String, Object> obj = (Map<String, Object>) item;
    System.out.println("Type: " + obj.get("@type"));
}

// Microdata
List<Object> microdata = Extractor.extractMicrodata(html, "https://example.com");

// Microformats (h-card, h-entry, h-event, etc.)
Map<String, Object> microformats = Extractor.extractMicroformats(html, "https://example.com");

// RDFa
List<Object> rdfa = Extractor.extractRdfa(html, "https://example.com");

// Dublin Core
Map<String, Object> dublinCore = Extractor.extractDublinCore(html);

// Web App Manifest
Map<String, Object> manifest = Extractor.extractManifest(html, "https://example.com");

// oEmbed
Map<String, Object> oembed = Extractor.extractOembed(html, "https://example.com");

// rel-* links
Map<String, Object> relLinks = Extractor.extractRelLinks(html, "https://example.com");
```

## Examples

### Extract from URL

```java
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;

String url = "https://example.com/article";

// Fetch HTML
HttpClient client = HttpClient.newHttpClient();
HttpRequest request = HttpRequest.newBuilder()
    .uri(URI.create(url))
    .build();
HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());

// Extract metadata
ExtractionResult result = Extractor.extractAll(response.body(), url);
System.out.println("Title: " + result.meta.get("title"));
```

### Blog Post Extraction

```java
String html = """
    <article class="h-entry">
        <h1 class="p-name">Getting Started with Rust</h1>
        <time class="dt-published" datetime="2024-01-15">January 15, 2024</time>
        <div class="p-author h-card">
            <span class="p-name">John Smith</span>
        </div>
        <div class="e-content">
            <p>Rust is a systems programming language...</p>
        </div>
    </article>
    """;

Map<String, Object> microformats = Extractor.extractMicroformats(html, "https://example.com");

if (microformats.containsKey("h-entry")) {
    List<Map<String, Object>> entries = (List<Map<String, Object>>) microformats.get("h-entry");
    for (Map<String, Object> entry : entries) {
        System.out.println("Title: " + entry.get("name"));
        System.out.println("Published: " + entry.get("published"));
    }
}
```

### E-commerce Product Extraction

```java
String html = """
    <div itemscope itemtype="https://schema.org/Product">
        <span itemprop="name">Awesome Widget</span>
        <span itemprop="brand">ACME Corp</span>
        <span itemprop="price">29.99</span>
        <div itemprop="aggregateRating" itemscope itemtype="https://schema.org/AggregateRating">
            <span itemprop="ratingValue">4.5</span>
            <span itemprop="reviewCount">127</span>
        </div>
    </div>
    """;

List<Object> microdata = Extractor.extractMicrodata(html, "https://example.com");

for (Object item : microdata) {
    Map<String, Object> product = (Map<String, Object>) item;
    System.out.println("Type: " + product.get("type"));

    Map<String, Object> props = (Map<String, Object>) product.get("properties");
    System.out.println("Name: " + props.get("name"));
    System.out.println("Price: " + props.get("price"));
}
```

### Social Media Preview

```java
String html = """
    <head>
        <meta property="og:title" content="10 Tips for Better Code">
        <meta property="og:description" content="Learn how to write cleaner code">
        <meta property="og:image" content="https://example.com/preview.jpg">
        <meta property="og:url" content="https://example.com/article">

        <meta name="twitter:card" content="summary_large_image">
        <meta name="twitter:title" content="10 Tips for Better Code">
    </head>
    """;

ExtractionResult result = Extractor.extractAll(html, "https://example.com");

// Generate social media preview
System.out.println("Preview Title: " + result.openGraph.get("title"));
System.out.println("Preview Image: " + result.openGraph.get("image"));
System.out.println("Twitter Card: " + result.twitter.get("card"));
```

### Error Handling

```java
import io.github.yfedoseev.metaoxide.MetaOxideException;

try {
    ExtractionResult result = Extractor.extractAll(html, "https://example.com");
    // Process result
} catch (MetaOxideException e) {
    System.err.println("Error extracting metadata: " + e.getMessage());
    System.err.println("Error code: " + e.getErrorCode());
    e.printStackTrace();
}
```

### Thread Safety

All methods are thread-safe and can be called concurrently:

```java
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

ExecutorService executor = Executors.newFixedThreadPool(10);

List<String> urls = Arrays.asList(
    "https://example1.com",
    "https://example2.com",
    "https://example3.com"
);

for (String url : urls) {
    executor.submit(() -> {
        try {
            String html = fetchHtml(url);
            ExtractionResult result = Extractor.extractAll(html, url);
            System.out.println(url + ": " + result.meta.get("title"));
        } catch (Exception e) {
            e.printStackTrace();
        }
    });
}

executor.shutdown();
```

## API Reference

### Extractor Class

Main API class with static methods for extraction.

#### extractAll(String html, String baseUrl)

Extract all metadata formats at once (most efficient).

**Parameters:**
- `html` (String) - HTML content (required)
- `baseUrl` (String) - Base URL for resolving relative URLs (optional)

**Returns:** `ExtractionResult` containing all metadata formats

**Throws:** `MetaOxideException` on error

---

#### extractMeta(String html, String baseUrl)

Extract standard HTML meta tags.

**Returns:** `Map<String, Object>` with meta tag properties

---

#### extractOpenGraph(String html, String baseUrl)

Extract Open Graph Protocol metadata.

**Returns:** `Map<String, Object>` with Open Graph properties

---

#### extractTwitter(String html, String baseUrl)

Extract Twitter Cards metadata.

**Returns:** `Map<String, Object>` with Twitter Card properties

---

#### extractJsonLd(String html, String baseUrl)

Extract JSON-LD structured data.

**Returns:** `List<Object>` of JSON-LD objects

---

#### extractMicrodata(String html, String baseUrl)

Extract Microdata.

**Returns:** `List<Object>` of Microdata items

---

#### extractMicroformats(String html, String baseUrl)

Extract Microformats (h-card, h-entry, h-event, etc.).

**Returns:** `Map<String, Object>` of Microformats by type

---

#### extractRdfa(String html, String baseUrl)

Extract RDFa structured data.

**Returns:** `List<Object>` of RDFa items

---

#### extractDublinCore(String html)

Extract Dublin Core metadata.

**Returns:** `Map<String, Object>` with Dublin Core properties

---

#### extractManifest(String html, String baseUrl)

Extract Web App Manifest link.

**Returns:** `Map<String, Object>` with manifest information

---

#### parseManifest(String manifestJson, String baseUrl)

Parse Web App Manifest JSON content.

**Parameters:**
- `manifestJson` (String) - Manifest JSON content
- `baseUrl` (String) - Base URL for resolving relative URLs

**Returns:** `Map<String, Object>` with parsed manifest properties

---

#### extractOembed(String html, String baseUrl)

Extract oEmbed endpoint discovery.

**Returns:** `Map<String, Object>` with oEmbed information

---

#### extractRelLinks(String html, String baseUrl)

Extract rel-* link relationships.

**Returns:** `Map<String, Object>` of link relationships by rel type

---

#### getVersion()

Get the MetaOxide library version.

**Returns:** `String` version (e.g., "0.1.0")

---

### ExtractionResult Class

Container for all extracted metadata.

**Fields:**
- `meta` (Map<String, Object>) - Standard HTML meta tags
- `openGraph` (Map<String, Object>) - Open Graph metadata
- `twitter` (Map<String, Object>) - Twitter Cards metadata
- `jsonLd` (List<Object>) - JSON-LD structured data
- `microdata` (List<Object>) - Microdata items
- `microformats` (Map<String, Object>) - Microformats by type
- `rdfa` (List<Object>) - RDFa items
- `dublinCore` (Map<String, Object>) - Dublin Core metadata
- `manifest` (Map<String, Object>) - Web App Manifest
- `oembed` (Map<String, Object>) - oEmbed information
- `relLinks` (Map<String, Object>) - rel-* link relationships

---

### MetaOxideException Class

Exception thrown on extraction errors.

**Methods:**
- `getErrorCode()` - Get error code from native library
- `getMessage()` - Get error message
- `toString()` - String representation including error code

## Performance

MetaOxide is built with Rust for maximum performance:

| HTML Size | Parse + Extract Time | Throughput |
|-----------|---------------------|------------|
| 10 KB     | ~60 µs              | 166 MB/s   |
| 100 KB    | ~600 µs             | 166 MB/s   |
| 1 MB      | ~6 ms               | 166 MB/s   |

*Benchmarks on Intel i7 @ 3.5 GHz*

**10-100x faster** than pure-Java HTML parsers like Jsoup for metadata extraction!

## Building from Source

### Prerequisites

- Java 8 or higher
- Maven 3.6+
- Rust 1.70+ (for building native library)
- C compiler (GCC or Clang)

### Build Steps

```bash
# Clone the repository
git clone https://github.com/yfedoseev/meta-oxide.git
cd meta-oxide/bindings/java/meta-oxide-java

# Build native library (in root directory)
cd ../../../
cargo build --release --features c-api

# Copy native library to resources
mkdir -p bindings/java/meta-oxide-java/src/main/resources/native/linux-x86_64
cp target/release/libmeta_oxide.so bindings/java/meta-oxide-java/src/main/resources/native/linux-x86_64/

# Build Java library
cd bindings/java/meta-oxide-java
mvn clean install

# Run tests
mvn test
```

### Cross-Platform Builds

To build for multiple platforms, use cross-compilation or build on each platform:

**Linux x86_64:**
```bash
cargo build --release --features c-api --target x86_64-unknown-linux-gnu
```

**macOS x86_64:**
```bash
cargo build --release --features c-api --target x86_64-apple-darwin
```

**macOS ARM64:**
```bash
cargo build --release --features c-api --target aarch64-apple-darwin
```

**Windows x86_64:**
```bash
cargo build --release --features c-api --target x86_64-pc-windows-msvc
```

## Android Usage

### Setup

Add to your `build.gradle`:

```groovy
android {
    // ...

    packagingOptions {
        pickFirst 'lib/armeabi-v7a/libmeta_oxide.so'
        pickFirst 'lib/arm64-v8a/libmeta_oxide.so'
        pickFirst 'lib/x86/libmeta_oxide.so'
        pickFirst 'lib/x86_64/libmeta_oxide.so'
    }
}

dependencies {
    implementation 'io.github.yfedoseev:meta-oxide:0.1.3'
}
```

### Example Android Activity

```java
import android.os.Bundle;
import androidx.appcompat.app.AppCompatActivity;
import io.github.yfedoseev.metaoxide.Extractor;
import io.github.yfedoseev.metaoxide.ExtractionResult;

public class MainActivity extends AppCompatActivity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        // Extract metadata in background thread
        new Thread(() -> {
            try {
                String html = fetchHtml("https://example.com");
                ExtractionResult result = Extractor.extractAll(html, "https://example.com");

                runOnUiThread(() -> {
                    // Update UI with results
                    titleTextView.setText(result.meta.get("title").toString());
                });
            } catch (Exception e) {
                e.printStackTrace();
            }
        }).start();
    }
}
```

## Publishing to Maven Central

This package is published to Maven Central. To publish a new version:

1. Configure your `~/.m2/settings.xml` with OSSRH credentials
2. Build and sign the artifacts:

```bash
mvn clean deploy -P release
```

3. The artifacts will be automatically staged and released to Maven Central

## Comparison with Other Libraries

| Feature | MetaOxide | Jsoup | Apache Tika |
|---------|-----------|-------|-------------|
| Speed | ⚡ 10-100x faster | Moderate | Slow |
| Formats | 13 formats | Manual parsing | Limited |
| Open Graph | ✅ | Manual | ❌ |
| JSON-LD | ✅ | Manual | ❌ |
| Microformats | ✅ 9 types | ❌ | ❌ |
| RDFa | ✅ | ❌ | ❌ |
| Thread-safe | ✅ | ❌ | ✅ |
| Dependencies | Zero | Many | Many |

## Contributing

Contributions are welcome! See the main repository for contribution guidelines.

## License

MetaOxide is dual-licensed under:

- MIT License ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license for your use.

## Links

- **Main Repository**: https://github.com/yfedoseev/meta-oxide
- **Maven Central**: https://search.maven.org/artifact/io.github.yfedoseev/meta-oxide
- **Documentation**: https://github.com/yfedoseev/meta-oxide/tree/main/docs
- **Issue Tracker**: https://github.com/yfedoseev/meta-oxide/issues

## Support

- Open an issue on GitHub
- Check the documentation
- See examples in the `examples/` directory

---

Made with ❤️ and Rust
