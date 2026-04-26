# MetaOxide ASP.NET Core API

An enterprise REST API demonstrating how to build a metadata extraction service using MetaOxide in C# with ASP.NET Core.

## What This Example Shows

This ASP.NET Core application demonstrates:
- Using MetaOxide C# bindings
- Building modern REST APIs with ASP.NET Core
- Dependency injection
- Async/await patterns
- Request validation
- Error handling
- Cross-platform compatibility (.NET 4.6+ to .NET 8)
- Deployment strategies

## Prerequisites

- .NET SDK 6.0+ ([Install](https://dotnet.microsoft.com/download))
- C# 9.0+ knowledge
- Visual Studio Code or Visual Studio (optional)

## Installation & Setup

```bash
# Build the project
dotnet build

# Run the application
dotnet run

# Or with specific port
dotnet run --urls "http://localhost:8080"
```

## Running the Service

```bash
# Start the API
dotnet run

# Application runs at https://localhost:5001 (HTTPS by default)
# Or http://localhost:5000 (HTTP if configured)
```

## API Endpoints

### POST /api/extract

Extract all metadata from provided HTML.

**Request:**
```bash
curl -X POST http://localhost:5000/api/extract \
  -H "Content-Type: application/json" \
  -d '{
    "html": "<html><head><title>Example</title></head></html>",
    "baseUrl": "https://example.com"
  }'
```

**Response:**
```json
{
  "success": true,
  "data": {
    "meta": {
      "title": "Example",
      "description": "..."
    },
    "openGraph": {
      "og:title": "Example"
    }
  }
}
```

### POST /api/extract/{format}

Extract specific metadata format.

**Formats:** `meta`, `open-graph`, `twitter`, `json-ld`, `microdata`, `microformats`, `rdfa`, `dublin-core`, `manifest`, `oembed`, `rel-links`

```bash
curl -X POST http://localhost:5000/api/extract/json-ld \
  -H "Content-Type: application/json" \
  -d '{"html": "...", "baseUrl": "https://example.com"}'
```

### GET /health

Health check endpoint.

```bash
curl http://localhost:5000/health
```

## Usage Examples

### C# with HttpClient

```csharp
var client = new HttpClient();

var request = new
{
    html = "<html><head><title>Test</title></head></html>",
    baseUrl = "https://example.com"
};

var json = JsonSerializer.Serialize(request);
var content = new StringContent(json, Encoding.UTF8, "application/json");

var response = await client.PostAsync(
    "http://localhost:5000/api/extract",
    content
);

var result = await response.Content.ReadAsStringAsync();
Console.WriteLine(result);
```

### ASP.NET Core Service

```csharp
[ApiController]
[Route("api")]
public class MetadataController : ControllerBase
{
    private readonly IMetadataExtractionService _service;

    public MetadataController(IMetadataExtractionService service)
    {
        _service = service;
    }

    [HttpPost("extract")]
    public async Task<ActionResult<ExtractResponse>> ExtractAll(
        ExtractionRequest request)
    {
        var result = await _service.ExtractMetadataAsync(
            request.Html,
            request.BaseUrl
        );

        return Ok(new ExtractResponse { Success = true, Data = result });
    }
}
```

## How It Works

### Main Components

1. **Controller** (`MetadataController.cs`)
   - REST endpoints
   - Request routing
   - Response formatting

2. **Service** (`IMetadataExtractionService.cs`)
   - Business logic
   - MetaOxide integration
   - Error handling

3. **Models**
   - `ExtractionRequest`
   - `ExtractionResponse`
   - Format-specific result types

### Key Code Pattern

```csharp
[ApiController]
[Route("api/[controller]")]
public class ExtractionController : ControllerBase
{
    [HttpPost]
    public IActionResult Extract([FromBody] ExtractionRequest request)
    {
        try
        {
            var extractor = new Extractor();
            var result = extractor.ExtractAll(request.Html, request.BaseUrl);

            return Ok(new { success = true, data = result });
        }
        catch (Exception ex)
        {
            return StatusCode(500, new { error = ex.Message });
        }
    }
}
```

## Configuration

### appsettings.json

```json
{
  "Logging": {
    "LogLevel": {
      "Default": "Information",
      "Microsoft": "Warning"
    }
  },
  "AllowedHosts": "*",
  "MetaOxide": {
    "MaxHtmlSize": 10485760,
    "TimeoutMs": 5000
  }
}
```

### Startup Configuration (Program.cs)

```csharp
var builder = WebApplication.CreateBuilder(args);

builder.Services.AddControllers();
builder.Services.AddScoped<IMetadataExtractionService, MetadataExtractionService>();
builder.Services.AddCors(options =>
{
    options.AddDefaultPolicy(builder =>
    {
        builder.AllowAnyOrigin().AllowAnyMethod().AllowAnyHeader();
    });
});

var app = builder.Build();
app.UseCors();
app.MapControllers();
app.Run();
```

## Performance

Expected performance on standard hardware:

- **Single extraction**: <15ms
- **10 concurrent requests**: <50ms
- **100 requests/second**: Easily handled

## Deployment

### Executable (.NET)

```bash
# Build self-contained executable
dotnet publish -c Release -o ./publish

# Run executable
./publish/MetaOxide.API
```

### Docker Deployment

```dockerfile
FROM mcr.microsoft.com/dotnet/sdk:8.0 as builder
WORKDIR /app
COPY . .
RUN dotnet publish -c Release -o out

FROM mcr.microsoft.com/dotnet/aspnet:8.0
WORKDIR /app
COPY --from=builder /app/out .
EXPOSE 5000
ENTRYPOINT ["dotnet", "MetaOxide.API.dll"]
```

Build and run:
```bash
docker build -t metaoxide-api .
docker run -p 5000:5000 metaoxide-api
```

### Azure App Service

```bash
# Publish to Azure
dotnet publish -c Release

# Or use Visual Studio
# Right-click project > Publish to Azure
```

### IIS Hosting

1. Build: `dotnet publish -c Release`
2. Copy published files to IIS wwwroot
3. Create Application Pool (.NET CLR version: No Managed Code)
4. Configure site in IIS Manager

## Testing

Run tests:
```bash
dotnet test

# With coverage
dotnet test /p:CollectCoverage=true
```

Example test:
```csharp
[Fact]
public async Task ExtractAll_WithValidHtml_ReturnsMetadata()
{
    // Arrange
    var request = new ExtractionRequest
    {
        Html = "<html><head><title>Test</title></head></html>",
        BaseUrl = "https://example.com"
    };

    // Act
    var response = await _client.PostAsJsonAsync("/api/extract", request);

    // Assert
    Assert.True(response.IsSuccessStatusCode);
}
```

## Async/Await Pattern

All endpoints support async operations:

```csharp
[HttpPost("extract-async")]
public async Task<IActionResult> ExtractAsync(ExtractionRequest request)
{
    var result = await Task.Run(() =>
        new Extractor().ExtractAll(request.Html, request.BaseUrl)
    );

    return Ok(new { success = true, data = result });
}
```

## Middleware & Filters

### Custom Exception Handling

```csharp
app.UseExceptionHandler(errorApp =>
{
    errorApp.Run(async context =>
    {
        context.Response.StatusCode = 500;
        await context.Response.WriteAsJsonAsync(new
        {
            error = "An error occurred processing your request."
        });
    });
});
```

### Logging Middleware

```csharp
app.Use(async (context, next) =>
{
    _logger.LogInformation("Request: {Method} {Path}",
        context.Request.Method,
        context.Request.Path);

    await next();

    _logger.LogInformation("Response: {StatusCode}",
        context.Response.StatusCode);
});
```

## .NET Framework Compatibility

This example works with:
- .NET Framework 4.6.2+
- .NET Standard 2.0+
- .NET Core 2.1+
- .NET 5, 6, 7, 8

Build for specific target:
```bash
dotnet build -f net462
dotnet build -f netstandard2.0
dotnet build -f net8.0
```

## Learning Resources

- [MetaOxide Getting Started (C#)](../../../docs/getting-started/getting-started-csharp.md)
- [C# API Reference](../../../docs/api/api-reference-csharp.md)
- [ASP.NET Core Documentation](https://learn.microsoft.com/en-us/aspnet/core/)
- [C# Async/Await Guide](https://learn.microsoft.com/en-us/dotnet/csharp/asynchronous-programming/)

## Troubleshooting

### Port Already in Use
```bash
# Use different port
dotnet run --urls "http://localhost:8081"
```

### SSL Certificate Issues
```bash
# Create dev certificate
dotnet dev-certs https --trust
```

### Missing Native Library
```bash
# Ensure MetaOxide is installed
dotnet add package MetaOxide
```

## Extensions

### Add OpenAPI/Swagger

```bash
dotnet add package Swashbuckle.AspNetCore
```

```csharp
builder.Services.AddSwaggerGen();
app.UseSwagger();
app.UseSwaggerUI();
```

Visit: `http://localhost:5000/swagger`

### Add Authentication

```csharp
builder.Services.AddAuthentication(JwtBearerDefaults.AuthenticationScheme)
    .AddJwtBearer(options => { /* config */ });

app.UseAuthentication();
app.UseAuthorization();
```

## License

This example is licensed under the same dual license as MetaOxide: **MIT OR Apache-2.0**

---

**Questions?** Check the main [MetaOxide documentation](../../../README.md) or open an issue.
