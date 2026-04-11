# Getting Started with MetaOxide (Java)

Welcome to MetaOxide! This guide will help you get started with the Java bindings for MetaOxide in just 5 minutes.

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Basic Extraction](#basic-extraction)
- [Android Usage](#android-usage)
- [Next Steps](#next-steps)

## Installation

### Maven

Add to your `pom.xml`:

```xml
<dependency>
    <groupId>com.metaoxide</groupId>
    <artifactId>meta-oxide</artifactId>
    <version>0.1.3</version>
</dependency>
```

### Gradle

Add to your `build.gradle`:

```gradle
dependencies {
    implementation 'com.metaoxide:meta-oxide:0.1.3'
}
```

### Requirements

- Java 8+
- Works on Linux, macOS, Windows, and Android

## Quick Start

Here's a minimal example to extract metadata from HTML:

```java
import com.metaoxide.MetaOxide;
import com.metaoxide.Metadata;

public class QuickStart {
    public static void main(String[] args) {
        String html = """
            <!DOCTYPE html>
            <html>
            <head>
                <title>My Page</title>
                <meta name="description" content="A great page">
                <meta property="og:title" content="My Page">
            </head>
            <body>Hello World</body>
            </html>
        """;

        try (MetaOxide extractor = new MetaOxide(html, "https://example.com")) {
            Metadata metadata = extractor.extractAll();

            System.out.println("Title: " + metadata.get("title"));
            System.out.println("Description: " + metadata.get("description"));
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
```

**Important**: MetaOxide implements `AutoCloseable`, so use try-with-resources to ensure proper cleanup.

## Basic Extraction

MetaOxide supports 13 metadata formats. Here's how to extract specific formats:

### Extract Open Graph Data

```java
import com.metaoxide.MetaOxide;
import com.metaoxide.OpenGraphData;

public class OpenGraphExample {
    public static OpenGraphData extractOpenGraph(String html) {
        try (MetaOxide extractor = new MetaOxide(html, "https://example.com")) {
            OpenGraphData ogData = extractor.extractOpenGraph();

            if (ogData != null) {
                System.out.println("OG Title: " + ogData.getTitle());
                System.out.println("OG Type: " + ogData.getType());
                System.out.println("OG Image: " + ogData.getImage());
            }

            return ogData;
        } catch (Exception e) {
            e.printStackTrace();
            return null;
        }
    }
}
```

### Extract Twitter Cards

```java
import com.metaoxide.TwitterCardData;

public class TwitterExample {
    public static TwitterCardData extractTwitter(String html) {
        try (MetaOxide extractor = new MetaOxide(html, "https://example.com")) {
            TwitterCardData twitterData = extractor.extractTwitterCard();

            if (twitterData != null) {
                System.out.println("Card Type: " + twitterData.getCard());
                System.out.println("Title: " + twitterData.getTitle());
            }

            return twitterData;
        } catch (Exception e) {
            e.printStackTrace();
            return null;
        }
    }
}
```

### Extract JSON-LD Structured Data

```java
import com.metaoxide.JSONLDData;
import com.google.gson.Gson;
import com.google.gson.GsonBuilder;

public class JSONLDExample {
    public static List<JSONLDData> extractJSONLD(String html) {
        try (MetaOxide extractor = new MetaOxide(html, "https://example.com")) {
            List<JSONLDData> jsonldData = extractor.extractJSONLD();

            if (jsonldData != null) {
                Gson gson = new GsonBuilder().setPrettyPrinting().create();
                System.out.println(gson.toJson(jsonldData));
            }

            return jsonldData;
        } catch (Exception e) {
            e.printStackTrace();
            return null;
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

public class URLExample {
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

    public static void main(String[] args) {
        try {
            Metadata metadata = extractFromURL("https://example.com");
            System.out.println("Title: " + metadata.get("title"));
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
```

### Parallel Processing with Streams

```java
import java.util.List;
import java.util.concurrent.CompletableFuture;
import java.util.stream.Collectors;

public class ParallelExample {
    public static List<Metadata> extractMultipleURLs(List<String> urls) {
        return urls.parallelStream()
                .map(url -> {
                    try {
                        return extractFromURL(url);
                    } catch (Exception e) {
                        System.err.println("Failed to extract from " + url);
                        return null;
                    }
                })
                .filter(metadata -> metadata != null)
                .collect(Collectors.toList());
    }

    public static void main(String[] args) {
        List<String> urls = List.of(
                "https://example.com",
                "https://example.org",
                "https://example.net"
        );

        List<Metadata> results = extractMultipleURLs(urls);
        results.forEach(metadata ->
                System.out.println("Title: " + metadata.get("title"))
        );
    }
}
```

## Android Usage

MetaOxide works seamlessly on Android:

### Add to Android Project

In your `build.gradle` (app level):

```gradle
dependencies {
    implementation 'com.metaoxide:meta-oxide:0.1.3'
}
```

### Extract in Activity

```java
import android.os.AsyncTask;
import com.metaoxide.MetaOxide;
import com.metaoxide.Metadata;

public class MainActivity extends AppCompatActivity {
    private void extractMetadata(String url) {
        new AsyncTask<String, Void, Metadata>() {
            @Override
            protected Metadata doInBackground(String... urls) {
                try {
                    return extractFromURL(urls[0]);
                } catch (Exception e) {
                    e.printStackTrace();
                    return null;
                }
            }

            @Override
            protected void onPostExecute(Metadata metadata) {
                if (metadata != null) {
                    String title = (String) metadata.get("title");
                    // Update UI with metadata
                }
            }
        }.execute(url);
    }
}
```

### Using Kotlin Coroutines (Android)

```kotlin
import com.metaoxide.MetaOxide
import kotlinx.coroutines.*

class MainActivity : AppCompatActivity() {
    private fun extractMetadata(url: String) {
        lifecycleScope.launch {
            val metadata = withContext(Dispatchers.IO) {
                extractFromURL(url)
            }

            // Update UI with metadata
            titleTextView.text = metadata?.get("title") as? String
        }
    }
}
```

## Error Handling

Handle errors appropriately:

```java
import com.metaoxide.MetaOxide;
import com.metaoxide.MetaOxideException;

public class ErrorHandlingExample {
    public static Metadata safeExtraction(String html, String url) {
        try (MetaOxide extractor = new MetaOxide(html, url)) {
            Metadata metadata = extractor.extractAll();
            System.out.println("Extracted " + metadata.size() + " fields");
            return metadata;
        } catch (MetaOxideException e) {
            System.err.println("Extraction failed: " + e.getMessage());
            return new Metadata();
        } catch (Exception e) {
            System.err.println("Unexpected error: " + e.getMessage());
            return new Metadata();
        }
    }
}
```

Common exceptions:
- `ParseException`: Invalid HTML structure
- `URLException`: Invalid base URL
- `ExtractionException`: Failed to extract specific metadata format

## Next Steps

Now that you've got the basics, explore more:

1. **[Complete API Reference](/docs/api/api-reference-java.md)** - Learn about all available methods
2. **[Real-World Examples](/examples/real-world/java-spring-service/)** - See a complete Spring Boot service
3. **[Integration Guides](/docs/integrations/spring-boot-integration.md)** - Use with Spring Boot

### All Supported Formats

MetaOxide extracts these 13 metadata formats:

- Basic HTML metadata (title, description, keywords)
- Open Graph (og:*)
- Twitter Cards (twitter:*)
- JSON-LD structured data
- Microdata (schema.org)
- Microformats (h-card, h-entry, h-event)
- Dublin Core
- RDFA
- HTML5 semantic elements
- Link relations
- Image metadata
- Author information

### Spring Boot Example

```java
import org.springframework.web.bind.annotation.*;
import com.metaoxide.MetaOxide;
import com.metaoxide.Metadata;

@RestController
@RequestMapping("/api")
public class MetadataController {
    @GetMapping("/extract")
    public ResponseEntity<Metadata> extractMetadata(@RequestParam String url) {
        try {
            Metadata metadata = extractFromURL(url);
            return ResponseEntity.ok(metadata);
        } catch (Exception e) {
            return ResponseEntity.status(500).build();
        }
    }
}
```

### Learn More

- [Spring Boot Integration](/docs/integrations/spring-boot-integration.md)
- [Performance Tuning](/docs/performance/performance-tuning-guide.md)
- [Architecture Overview](/docs/architecture/architecture-overview.md)

Happy extracting! 🚀
