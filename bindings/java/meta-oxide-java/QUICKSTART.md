# MetaOxide Java - Quick Start Guide

Get started with MetaOxide in 5 minutes!

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

## 5-Minute Tutorial

### 1. Extract All Metadata (Recommended!)

The most efficient way - parses HTML only once:

```java
import io.github.yfedoseev.metaoxide.Extractor;
import io.github.yfedoseev.metaoxide.ExtractionResult;

String html = """
    <html>
    <head>
        <title>My Page</title>
        <meta property="og:title" content="My Page">
        <meta name="twitter:card" content="summary">
    </head>
    </html>
    """;

ExtractionResult result = Extractor.extractAll(html, "https://example.com");

// Access metadata
System.out.println("Title: " + result.meta.get("title"));
System.out.println("OG Title: " + result.openGraph.get("title"));
System.out.println("Twitter Card: " + result.twitter.get("card"));
```

**Output:**
```
Title: My Page
OG Title: My Page
Twitter Card: summary
```

### 2. Extract Individual Formats

When you only need specific metadata:

```java
import java.util.Map;

// Standard meta tags
Map<String, Object> meta = Extractor.extractMeta(html, "https://example.com");
System.out.println("Title: " + meta.get("title"));

// Open Graph
Map<String, Object> og = Extractor.extractOpenGraph(html, "https://example.com");
System.out.println("OG Image: " + og.get("image"));

// Twitter Cards
Map<String, Object> twitter = Extractor.extractTwitter(html, "https://example.com");
System.out.println("Card: " + twitter.get("card"));
```

### 3. Extract from URL

Fetch HTML and extract metadata:

```java
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;

String url = "https://example.com";

HttpClient client = HttpClient.newHttpClient();
HttpRequest request = HttpRequest.newBuilder()
    .uri(URI.create(url))
    .build();

HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());

ExtractionResult result = Extractor.extractAll(response.body(), url);
System.out.println("Title: " + result.meta.get("title"));
```

### 4. Handle Errors

Proper error handling:

```java
import io.github.yfedoseev.metaoxide.MetaOxideException;

try {
    ExtractionResult result = Extractor.extractAll(html, "https://example.com");
    // Process result
} catch (MetaOxideException e) {
    System.err.println("Error: " + e.getMessage());
    System.err.println("Code: " + e.getErrorCode());
}
```

### 5. All 13 Formats

MetaOxide extracts 13 metadata formats:

```java
ExtractionResult result = Extractor.extractAll(html, "https://example.com");

// 1. Standard meta tags
result.meta.get("title");
result.meta.get("description");

// 2. Open Graph (Facebook, LinkedIn)
result.openGraph.get("title");
result.openGraph.get("image");

// 3. Twitter Cards
result.twitter.get("card");

// 4. JSON-LD (Schema.org)
result.jsonLd.get(0);

// 5. Microdata
result.microdata.get(0);

// 6. Microformats (h-card, h-entry, h-event, etc.)
result.microformats.get("h-card");

// 7. RDFa
result.rdfa.get(0);

// 8. Dublin Core
result.dublinCore.get("title");

// 9. Web App Manifest
result.manifest.get("href");

// 10. oEmbed
result.oembed.get("json");

// 11. rel-* links
result.relLinks.get("canonical");
```

## Common Use Cases

### Extract Blog Post

```java
String html = """
    <article class="h-entry">
        <h1 class="p-name">Blog Post Title</h1>
        <time class="dt-published" datetime="2024-01-15">Jan 15, 2024</time>
    </article>
    """;

Map<String, Object> microformats = Extractor.extractMicroformats(html, "https://example.com");
```

### Extract Product Data

```java
String html = """
    <div itemscope itemtype="https://schema.org/Product">
        <span itemprop="name">Widget</span>
        <span itemprop="price">$29.99</span>
    </div>
    """;

List<Object> microdata = Extractor.extractMicrodata(html, "https://example.com");
```

### Social Media Preview

```java
String html = """
    <meta property="og:title" content="Article Title">
    <meta property="og:image" content="https://example.com/image.jpg">
    <meta name="twitter:card" content="summary_large_image">
    """;

ExtractionResult result = Extractor.extractAll(html, "https://example.com");

// Generate preview
String title = (String) result.openGraph.get("title");
String image = (String) result.openGraph.get("image");
String card = (String) result.twitter.get("card");
```

## Thread Safety

All methods are thread-safe:

```java
ExecutorService executor = Executors.newFixedThreadPool(10);

List<String> urls = List.of(
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
```

## Android Usage

Add to your `build.gradle`:

```groovy
dependencies {
    implementation 'io.github.yfedoseev:meta-oxide:0.1.3'
}
```

Use in your Activity:

```java
public class MainActivity extends AppCompatActivity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        new Thread(() -> {
            try {
                String html = fetchHtml("https://example.com");
                ExtractionResult result = Extractor.extractAll(html, "https://example.com");

                runOnUiThread(() -> {
                    titleTextView.setText(result.meta.get("title").toString());
                });
            } catch (Exception e) {
                e.printStackTrace();
            }
        }).start();
    }
}
```

## Performance

MetaOxide is **10-100x faster** than pure-Java parsers:

| HTML Size | Time |
|-----------|------|
| 10 KB     | ~60 µs |
| 100 KB    | ~600 µs |
| 1 MB      | ~6 ms |

## Next Steps

- [Complete Documentation](README.md)
- [API Reference](README.md#api-reference)
- [More Examples](examples/)
- [GitHub Repository](https://github.com/yfedoseev/meta-oxide)

## Support

- GitHub Issues: https://github.com/yfedoseev/meta-oxide/issues
- Documentation: https://github.com/yfedoseev/meta-oxide/tree/main/docs

---

**You're ready!** Start extracting metadata with MetaOxide 🚀
