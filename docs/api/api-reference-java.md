# MetaOxide Java API Reference

Complete API documentation for the Java library.

## Table of Contents

- [Installation](#installation)
- [Package Structure](#package-structure)
- [Classes](#classes)
- [Methods](#methods)
- [Data Types](#data-types)
- [Exceptions](#exceptions)
- [Examples](#examples)

## Installation

### Maven

```xml
<dependency>
    <groupId>com.metaoxide</groupId>
    <artifactId>meta-oxide</artifactId>
    <version>0.1.3</version>
</dependency>
```

### Gradle

```gradle
dependencies {
    implementation 'com.metaoxide:meta-oxide:0.1.3'
}
```

## Package Structure

```java
package com.metaoxide;

import com.metaoxide.MetaOxide;
import com.metaoxide.Metadata;
import com.metaoxide.OpenGraphData;
import com.metaoxide.TwitterCardData;
import com.metaoxide.MetaOxideException;
```

## Classes

### `MetaOxide`

Main class for extracting metadata from HTML. Implements `AutoCloseable`.

#### Constructor

```java
public MetaOxide(String html, String baseUrl) throws MetaOxideException
```

**Parameters:**
- `html` (String) - HTML content to parse
- `baseUrl` (String) - Base URL for resolving relative URLs

**Throws:**
- `MetaOxideException` - If HTML parsing fails

**Example:**
```java
try (MetaOxide extractor = new MetaOxide(html, "https://example.com")) {
    // Use extractor
}
```

**Important:** Use try-with-resources to ensure proper resource cleanup.

## Methods

### `extractAll()`

Extract all metadata formats.

```java
public Metadata extractAll() throws MetaOxideException
```

**Returns:** `Metadata` - All extracted metadata

**Throws:** `MetaOxideException` - If extraction fails

**Example:**
```java
try (MetaOxide extractor = new MetaOxide(html, url)) {
    Metadata metadata = extractor.extractAll();
    System.out.println("Title: " + metadata.get("title"));
}
```

### `extractBasicMeta()`

Extract basic HTML metadata.

```java
public Metadata extractBasicMeta() throws MetaOxideException
```

**Returns:** `Metadata` - Basic HTML metadata

**Example:**
```java
Metadata basic = extractor.extractBasicMeta();
System.out.println("Title: " + basic.get("title"));
System.out.println("Description: " + basic.get("description"));
```

### `extractOpenGraph()`

Extract Open Graph metadata.

```java
public OpenGraphData extractOpenGraph() throws MetaOxideException
```

**Returns:** `OpenGraphData` or `null` if not present

**Example:**
```java
OpenGraphData og = extractor.extractOpenGraph();
if (og != null) {
    System.out.println("OG Title: " + og.getTitle());
    System.out.println("OG Image: " + og.getImage());
}
```

### `extractTwitterCard()`

Extract Twitter Card metadata.

```java
public TwitterCardData extractTwitterCard() throws MetaOxideException
```

**Returns:** `TwitterCardData` or `null` if not present

**Example:**
```java
TwitterCardData twitter = extractor.extractTwitterCard();
if (twitter != null) {
    System.out.println("Card Type: " + twitter.getCard());
}
```

### `extractJSONLD()`

Extract JSON-LD structured data.

```java
public List<JSONLDData> extractJSONLD() throws MetaOxideException
```

**Returns:** `List<JSONLDData>` - List of JSON-LD objects

**Example:**
```java
List<JSONLDData> jsonld = extractor.extractJSONLD();
for (JSONLDData item : jsonld) {
    System.out.println("Type: " + item.getType());
}
```

### `extractMicrodata()`

Extract Microdata items.

```java
public List<MicrodataItem> extractMicrodata() throws MetaOxideException
```

**Returns:** `List<MicrodataItem>` - List of Microdata items

**Example:**
```java
List<MicrodataItem> microdata = extractor.extractMicrodata();
for (MicrodataItem item : microdata) {
    System.out.println("Type: " + item.getType());
}
```

### `extractMicroformats()`

Extract Microformats data.

```java
public Map<String, List<Metadata>> extractMicroformats() throws MetaOxideException
```

**Returns:** `Map<String, List<Metadata>>` - Microformats grouped by type

**Example:**
```java
Map<String, List<Metadata>> mf = extractor.extractMicroformats();
if (mf.containsKey("h-card")) {
    for (Metadata card : mf.get("h-card")) {
        System.out.println("Name: " + card.get("name"));
    }
}
```

### `extractDublinCore()`

Extract Dublin Core metadata.

```java
public DublinCoreData extractDublinCore() throws MetaOxideException
```

**Returns:** `DublinCoreData` or `null` if not present

### `extractRelLinks()`

Extract link relations.

```java
public Map<String, List<Link>> extractRelLinks() throws MetaOxideException
```

**Returns:** `Map<String, List<Link>>` - Links grouped by rel type

**Example:**
```java
Map<String, List<Link>> links = extractor.extractRelLinks();
if (links.containsKey("canonical")) {
    System.out.println("Canonical: " + links.get("canonical").get(0).getHref());
}
```

### `close()`

Close and release native resources. Automatically called by try-with-resources.

```java
public void close()
```

## Data Types

### `Metadata`

Generic metadata container extending `HashMap<String, Object>`.

```java
public class Metadata extends HashMap<String, Object> {
    // Convenience methods
    public String getString(String key);
    public Integer getInt(String key);
    public List<String> getStringList(String key);
}
```

### `OpenGraphData`

Open Graph metadata.

```java
public class OpenGraphData {
    private String title;
    private String type;
    private String image;
    private String url;
    private String description;
    private String siteName;
    private String locale;

    // Getters
    public String getTitle() { return title; }
    public String getType() { return type; }
    public String getImage() { return image; }
    public String getUrl() { return url; }
    public String getDescription() { return description; }
    public String getSiteName() { return siteName; }
    public String getLocale() { return locale; }
}
```

### `TwitterCardData`

Twitter Card metadata.

```java
public class TwitterCardData {
    private String card;
    private String site;
    private String creator;
    private String title;
    private String description;
    private String image;
    private String imageAlt;

    // Getters
    public String getCard() { return card; }
    public String getSite() { return site; }
    public String getCreator() { return creator; }
    public String getTitle() { return title; }
    public String getDescription() { return description; }
    public String getImage() { return image; }
    public String getImageAlt() { return imageAlt; }
}
```

### `MicrodataItem`

Microdata item.

```java
public class MicrodataItem {
    private List<String> type;
    private Map<String, List<Object>> properties;
    private String id;

    // Getters
    public List<String> getType() { return type; }
    public Map<String, List<Object>> getProperties() { return properties; }
    public String getId() { return id; }
}
```

### `Link`

Link element.

```java
public class Link {
    private String href;
    private String rel;
    private String media;
    private String title;
    private String type;
    private String hreflang;

    // Getters
    public String getHref() { return href; }
    public String getRel() { return rel; }
    // ... other getters
}
```

## Exceptions

### `MetaOxideException`

Base exception for MetaOxide errors.

```java
public class MetaOxideException extends Exception {
    public MetaOxideException(String message) {
        super(message);
    }

    public MetaOxideException(String message, Throwable cause) {
        super(message, cause);
    }
}
```

### Exception Handling

```java
try (MetaOxide extractor = new MetaOxide(html, url)) {
    Metadata metadata = extractor.extractAll();
    // Use metadata
} catch (MetaOxideException e) {
    System.err.println("Extraction failed: " + e.getMessage());
} catch (Exception e) {
    System.err.println("Unexpected error: " + e.getMessage());
}
```

## Examples

### Basic Usage

```java
import com.metaoxide.MetaOxide;
import com.metaoxide.Metadata;

public class Example {
    public static void main(String[] args) {
        String html = "<!DOCTYPE html>...";

        try (MetaOxide extractor = new MetaOxide(html, "https://example.com")) {
            Metadata metadata = extractor.extractAll();
            System.out.println("Title: " + metadata.get("title"));
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
```

### Extract from URL

```java
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;

public class URLExtractor {
    public static Metadata extractFromURL(String url) throws Exception {
        HttpClient client = HttpClient.newHttpClient();
        HttpRequest request = HttpRequest.newBuilder()
                .uri(URI.create(url))
                .build();

        HttpResponse<String> response = client.send(request,
                HttpResponse.BodyHandlers.ofString());

        try (MetaOxide extractor = new MetaOxide(response.body(), url)) {
            return extractor.extractAll();
        }
    }
}
```

### Parallel Processing

```java
import java.util.List;
import java.util.stream.Collectors;

public class ParallelExtractor {
    public static List<Metadata> extractMultiple(List<String> urls) {
        return urls.parallelStream()
                .map(url -> {
                    try {
                        return extractFromURL(url);
                    } catch (Exception e) {
                        System.err.println("Failed: " + url);
                        return null;
                    }
                })
                .filter(m -> m != null)
                .collect(Collectors.toList());
    }
}
```

### Spring Boot Service

```java
import org.springframework.stereotype.Service;
import com.metaoxide.MetaOxide;
import com.metaoxide.Metadata;

@Service
public class MetadataService {
    public Metadata extract(String html, String url) throws MetaOxideException {
        try (MetaOxide extractor = new MetaOxide(html, url)) {
            return extractor.extractAll();
        }
    }
}
```

### Android Usage

```java
import android.os.AsyncTask;
import com.metaoxide.MetaOxide;
import com.metaoxide.Metadata;

public class MetadataTask extends AsyncTask<String, Void, Metadata> {
    @Override
    protected Metadata doInBackground(String... params) {
        String url = params[0];
        try {
            String html = fetchHTML(url);
            try (MetaOxide extractor = new MetaOxide(html, url)) {
                return extractor.extractAll();
            }
        } catch (Exception e) {
            e.printStackTrace();
            return null;
        }
    }

    @Override
    protected void onPostExecute(Metadata metadata) {
        if (metadata != null) {
            // Update UI
        }
    }
}
```

### Kotlin Usage

```kotlin
import com.metaoxide.MetaOxide

fun extractMetadata(html: String, url: String): Metadata? {
    return try {
        MetaOxide(html, url).use { extractor ->
            extractor.extractAll()
        }
    } catch (e: Exception) {
        null
    }
}

// Usage
val metadata = extractMetadata(html, "https://example.com")
metadata?.let {
    println("Title: ${it["title"]}")
}
```

## Thread Safety

MetaOxide is thread-safe. Each thread should create its own instance.

```java
// Safe: Each thread has its own extractor
ExecutorService executor = Executors.newFixedThreadPool(4);
urls.forEach(url -> {
    executor.submit(() -> {
        try (MetaOxide extractor = new MetaOxide(html, url)) {
            // Safe
        }
    });
});
```

## Performance Tips

1. **Use try-with-resources**: Ensures proper cleanup
2. **Parallel Streams**: Process multiple URLs concurrently
3. **Selective Extraction**: Extract only needed formats
4. **Connection Pooling**: Reuse HTTP client instances

## See Also

- [Getting Started Guide](/docs/getting-started/getting-started-java.md)
- [Spring Boot Integration](/docs/integrations/spring-boot-integration.md)
- [Examples](/examples/real-world/java-spring-service/)
