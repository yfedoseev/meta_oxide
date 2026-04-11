# Getting Started with MetaOxide (C#)

Welcome to MetaOxide! This guide will help you get started with the C# bindings for MetaOxide in just 5 minutes.

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Basic Extraction](#basic-extraction)
- [.NET Versions](#net-versions)
- [Next Steps](#next-steps)

## Installation

### NuGet Package Manager

```bash
dotnet add package MetaOxide
```

Or via NuGet Package Manager Console:

```powershell
Install-Package MetaOxide
```

### Package Reference

Add to your `.csproj` file:

```xml
<ItemGroup>
    <PackageReference Include="MetaOxide" Version="0.1.3" />
</ItemGroup>
```

### Requirements

- .NET Framework 4.6.1+ or .NET Core 2.0+ or .NET 5+
- Works on Windows, Linux, and macOS

## Quick Start

Here's a minimal example to extract metadata from HTML:

```csharp
using MetaOxide;
using System;

class Program
{
    static void Main()
    {
        string html = @"
            <!DOCTYPE html>
            <html>
            <head>
                <title>My Page</title>
                <meta name=""description"" content=""A great page"">
                <meta property=""og:title"" content=""My Page"">
            </head>
            <body>Hello World</body>
            </html>
        ";

        using var extractor = new MetaOxideExtractor(html, "https://example.com");
        var metadata = extractor.ExtractAll();

        Console.WriteLine($"Title: {metadata["title"]}");
        Console.WriteLine($"Description: {metadata["description"]}");
    }
}
```

**Important**: MetaOxide implements `IDisposable`, so use `using` statements to ensure proper cleanup.

## Basic Extraction

MetaOxide supports 13 metadata formats. Here's how to extract specific formats:

### Extract Open Graph Data

```csharp
using MetaOxide;
using System;
using System.Collections.Generic;

public class OpenGraphExample
{
    public static Dictionary<string, object> ExtractOpenGraph(string html)
    {
        using var extractor = new MetaOxideExtractor(html, "https://example.com");
        var ogData = extractor.ExtractOpenGraph();

        if (ogData != null)
        {
            Console.WriteLine($"OG Title: {ogData["title"]}");
            Console.WriteLine($"OG Type: {ogData["type"]}");
            Console.WriteLine($"OG Image: {ogData["image"]}");
        }

        return ogData;
    }
}
```

### Extract Twitter Cards

```csharp
public class TwitterExample
{
    public static Dictionary<string, object> ExtractTwitter(string html)
    {
        using var extractor = new MetaOxideExtractor(html, "https://example.com");
        var twitterData = extractor.ExtractTwitterCard();

        if (twitterData != null)
        {
            Console.WriteLine($"Card Type: {twitterData["card"]}");
            Console.WriteLine($"Title: {twitterData["title"]}");
        }

        return twitterData;
    }
}
```

### Extract JSON-LD Structured Data

```csharp
using System.Text.Json;

public class JSONLDExample
{
    public static List<object> ExtractJSONLD(string html)
    {
        using var extractor = new MetaOxideExtractor(html, "https://example.com");
        var jsonldData = extractor.ExtractJSONLD();

        if (jsonldData != null)
        {
            string json = JsonSerializer.Serialize(jsonldData,
                new JsonSerializerOptions { WriteIndented = true });
            Console.WriteLine(json);
        }

        return jsonldData;
    }
}
```

### Extract from URL

```csharp
using System.Net.Http;
using System.Threading.Tasks;

public class URLExample
{
    private static readonly HttpClient client = new HttpClient();

    public static async Task<Dictionary<string, object>> ExtractFromURLAsync(string url)
    {
        string html = await client.GetStringAsync(url);

        using var extractor = new MetaOxideExtractor(html, url);
        return extractor.ExtractAll();
    }

    public static async Task Main()
    {
        var metadata = await ExtractFromURLAsync("https://example.com");
        Console.WriteLine($"Title: {metadata["title"]}");
    }
}
```

### Async/Await Pattern

```csharp
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Threading.Tasks;

public class AsyncExample
{
    private static readonly HttpClient client = new HttpClient();

    public static async Task<List<Dictionary<string, object>>> ExtractMultipleURLsAsync(
        List<string> urls)
    {
        var tasks = urls.Select(async url =>
        {
            try
            {
                return await ExtractFromURLAsync(url);
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Failed to extract from {url}: {ex.Message}");
                return null;
            }
        });

        var results = await Task.WhenAll(tasks);
        return results.Where(r => r != null).ToList();
    }

    public static async Task Main()
    {
        var urls = new List<string>
        {
            "https://example.com",
            "https://example.org",
            "https://example.net"
        };

        var results = await ExtractMultipleURLsAsync(urls);

        for (int i = 0; i < results.Count; i++)
        {
            Console.WriteLine($"{urls[i]}: {results[i]["title"]}");
        }
    }
}
```

## .NET Versions

### .NET 6+ with Top-Level Statements

```csharp
using MetaOxide;

string html = """
<!DOCTYPE html>
<html>
<head>
    <title>My Page</title>
</head>
</html>
""";

using var extractor = new MetaOxideExtractor(html, "https://example.com");
var metadata = extractor.ExtractAll();

Console.WriteLine($"Title: {metadata["title"]}");
```

### .NET Framework 4.6.1+

```csharp
using MetaOxide;
using System;
using System.Collections.Generic;

namespace MetaOxideExample
{
    class Program
    {
        static void Main(string[] args)
        {
            string html = @"<!DOCTYPE html>...";

            using (var extractor = new MetaOxideExtractor(html, "https://example.com"))
            {
                Dictionary<string, object> metadata = extractor.ExtractAll();
                Console.WriteLine("Title: " + metadata["title"]);
            }
        }
    }
}
```

### ASP.NET Core Integration

```csharp
using Microsoft.AspNetCore.Mvc;
using MetaOxide;
using System.Net.Http;
using System.Threading.Tasks;

[ApiController]
[Route("api/[controller]")]
public class MetadataController : ControllerBase
{
    private readonly HttpClient _httpClient;

    public MetadataController(IHttpClientFactory httpClientFactory)
    {
        _httpClient = httpClientFactory.CreateClient();
    }

    [HttpGet]
    public async Task<IActionResult> Extract([FromQuery] string url)
    {
        if (string.IsNullOrEmpty(url))
        {
            return BadRequest("URL parameter is required");
        }

        try
        {
            string html = await _httpClient.GetStringAsync(url);

            using var extractor = new MetaOxideExtractor(html, url);
            var metadata = extractor.ExtractAll();

            return Ok(new { success = true, metadata });
        }
        catch (Exception ex)
        {
            return StatusCode(500, new { error = ex.Message });
        }
    }
}
```

### Dependency Injection

```csharp
using Microsoft.Extensions.DependencyInjection;

public class Startup
{
    public void ConfigureServices(IServiceCollection services)
    {
        services.AddHttpClient();
        services.AddSingleton<IMetadataExtractor, MetadataExtractorService>();
    }
}

public interface IMetadataExtractor
{
    Task<Dictionary<string, object>> ExtractAsync(string url);
}

public class MetadataExtractorService : IMetadataExtractor
{
    private readonly HttpClient _httpClient;

    public MetadataExtractorService(IHttpClientFactory httpClientFactory)
    {
        _httpClient = httpClientFactory.CreateClient();
    }

    public async Task<Dictionary<string, object>> ExtractAsync(string url)
    {
        string html = await _httpClient.GetStringAsync(url);

        using var extractor = new MetaOxideExtractor(html, url);
        return extractor.ExtractAll();
    }
}
```

## Error Handling

Handle errors appropriately:

```csharp
using MetaOxide;
using System;
using System.Collections.Generic;

public class ErrorHandlingExample
{
    public static Dictionary<string, object> SafeExtraction(string html, string url)
    {
        try
        {
            using var extractor = new MetaOxideExtractor(html, url);
            var metadata = extractor.ExtractAll();
            Console.WriteLine($"Extracted {metadata.Count} fields");
            return metadata;
        }
        catch (MetaOxideException ex)
        {
            Console.WriteLine($"Extraction failed: {ex.Message}");
            return new Dictionary<string, object>();
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Unexpected error: {ex.Message}");
            return new Dictionary<string, object>();
        }
    }
}
```

Common exceptions:
- `ParseException`: Invalid HTML structure
- `UrlException`: Invalid base URL
- `ExtractionException`: Failed to extract specific metadata format

## Next Steps

Now that you've got the basics, explore more:

1. **[Complete API Reference](/docs/api/api-reference-csharp.md)** - Learn about all available methods
2. **[Real-World Examples](/examples/real-world/csharp-aspnet-api/)** - See a complete ASP.NET Core API
3. **[Integration Guides](/docs/integrations/aspnetcore-integration.md)** - Use with ASP.NET Core

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

### Learn More

- [ASP.NET Core Integration](/docs/integrations/aspnetcore-integration.md)
- [Performance Tuning](/docs/performance/performance-tuning-guide.md)
- [Architecture Overview](/docs/architecture/architecture-overview.md)

Happy extracting! 🚀
