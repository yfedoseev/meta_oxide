# MetaOxide Spring Boot Service

An enterprise REST API demonstrating how to build a metadata extraction service using MetaOxide in Java with Spring Boot.

## What This Example Shows

This Spring Boot application demonstrates:
- Using MetaOxide Java bindings
- Building enterprise REST APIs
- Spring Boot dependency injection
- Request/response handling
- Exception mapping
- Async processing
- Logging and monitoring
- Deployment on application servers

## Prerequisites

- Java 8+ ([Install](https://openjdk.org/install/))
- Maven 3.6+ ([Install](https://maven.apache.org/install.html))
- Spring Boot 2.6+ or 3.0+

## Installation & Setup

```bash
# Build the project
mvn clean install

# Run with Spring Boot
mvn spring-boot:run

# Or run the generated JAR
java -jar target/metaoxide-spring-*.jar
```

## Running the Service

```bash
# Start the application
mvn spring-boot:run

# Server runs at http://localhost:8080
```

## API Endpoints

### POST /api/extract

Extract all metadata from provided HTML.

**Request:**
```bash
curl -X POST http://localhost:8080/api/extract \
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
      "og:title": "Example",
      ...
    }
  }
}
```

### POST /api/extract/{format}

Extract specific metadata format.

**Formats:** `meta`, `open-graph`, `twitter`, `json-ld`, `microdata`, `microformats`, `rdfa`, `dublin-core`, `manifest`, `oembed`, `rel-links`

```bash
curl -X POST http://localhost:8080/api/extract/json-ld \
  -H "Content-Type: application/json" \
  -d '{"html": "...", "baseUrl": "https://example.com"}'
```

### GET /actuator/health

Spring Boot health check.

```bash
curl http://localhost:8080/actuator/health
```

## Usage Examples

### Java with RestTemplate

```java
RestTemplate restTemplate = new RestTemplate();

Map<String, String> request = new HashMap<>();
request.put("html", "<html><head><title>Test</title></head></html>");
request.put("baseUrl", "https://example.com");

ResponseEntity<Map> response = restTemplate.postForEntity(
    "http://localhost:8080/api/extract",
    request,
    Map.class
);

System.out.println(response.getBody());
```

### Spring Boot Service

```java
@Service
public class MetadataExtractionService {

    public Map<String, Object> extractMetadata(String html, String baseUrl) {
        Extractor extractor = new Extractor(html, baseUrl);
        return extractor.extractAll();
    }
}
```

## How It Works

### Main Components

1. **Controller** (`MetadataController.java`)
   - REST endpoints
   - Request handling
   - Response formatting

2. **Service** (`MetadataExtractionService.java`)
   - Business logic
   - MetaOxide integration
   - Error handling

3. **Models**
   - Request objects (ExtractionRequest)
   - Response objects (ExtractionResponse)

### Key Code Pattern

```java
@RestController
@RequestMapping("/api")
public class MetadataController {

    @Autowired
    private MetadataExtractionService service;

    @PostMapping("/extract")
    public ResponseEntity<Map<String, Object>> extractAll(
            @RequestBody ExtractionRequest request) {

        Map<String, Object> result = service.extractMetadata(
            request.getHtml(),
            request.getBaseUrl()
        );

        return ResponseEntity.ok(result);
    }
}
```

## Configuration

### application.yml

```yaml
spring:
  application:
    name: metaoxide-api
  jpa:
    database-platform: org.hibernate.dialect.H2Dialect
server:
  port: 8080
  servlet:
    context-path: /

management:
  endpoints:
    web:
      exposure:
        include: health,metrics
  metrics:
    enable:
      jvm: true
      process: true
```

## Performance

Expected performance on standard hardware:

- **Single extraction**: <20ms
- **10 concurrent requests**: <100ms
- **100 requests/second**: Easily handled with 4 threads

**Performance tuning:**
```yaml
server:
  tomcat:
    threads:
      max: 200
      min-spare: 10
    max-connections: 10000
```

## Deployment

### Executable JAR

```bash
# Build
mvn clean package

# Run
java -jar target/metaoxide-spring-*.jar

# With custom port
java -Dserver.port=8081 -jar target/metaoxide-spring-*.jar
```

### Docker Deployment

```dockerfile
FROM maven:3.8-eclipse-temurin-17 as builder
WORKDIR /app
COPY . .
RUN mvn clean install -DskipTests

FROM eclipse-temurin:17-jre-alpine
COPY --from=builder /app/target/*.jar app.jar
EXPOSE 8080
ENTRYPOINT ["java", "-jar", "/app.jar"]
```

Build and run:
```bash
docker build -t metaoxide-api .
docker run -p 8080:8080 metaoxide-api
```

### Application Server (Tomcat, Jetty)

```bash
# Build WAR
mvn clean package -Pwar

# Deploy to Tomcat
cp target/*.war /opt/tomcat/webapps/
```

## Testing

Run tests:
```bash
mvn test

# With coverage
mvn clean test jacoco:report

# View report
open target/site/jacoco/index.html
```

Example test:
```java
@SpringBootTest
@AutoConfigureMockMvc
public class MetadataControllerTest {

    @Autowired
    private MockMvc mockMvc;

    @Test
    public void testExtractAll() throws Exception {
        String html = "<html><head><title>Test</title></head></html>";

        mockMvc.perform(post("/api/extract")
            .contentType(MediaType.APPLICATION_JSON)
            .content("{\"html\": \"" + html + "\"}"))
            .andExpect(status().isOk())
            .andExpect(jsonPath("$.success").value(true));
    }
}
```

## Android Integration

Use MetaOxide in Android applications:

```gradle
dependencies {
    implementation 'io.github.yfedoseev:meta-oxide:0.1.3'
}
```

```java
// In Android Activity
Extractor extractor = new Extractor(htmlContent, baseUrl);
Map<String, Object> result = extractor.extractAll();

// Display results
textView.setText(result.toString());
```

## Logging

Configure logging in `application.yml`:

```yaml
logging:
  level:
    root: INFO
    com.example.metaoxide: DEBUG
  pattern:
    console: "%d{HH:mm:ss.SSS} [%thread] %-5level %logger{36} - %msg%n"
```

## Monitoring with Actuator

Spring Boot Actuator provides metrics:

```bash
# Get metrics
curl http://localhost:8080/actuator/metrics

# View specific metric
curl http://localhost:8080/actuator/metrics/http.server.requests
```

Add custom metrics:

```java
@Component
public class ExtractionMetrics {
    private final AtomicInteger extractionCount = new AtomicInteger(0);

    public void recordExtraction() {
        extractionCount.incrementAndGet();
    }
}
```

## Exception Handling

Global exception handling:

```java
@ControllerAdvice
public class GlobalExceptionHandler {

    @ExceptionHandler(IllegalArgumentException.class)
    public ResponseEntity<ErrorResponse> handleIllegalArgument(
            IllegalArgumentException e) {
        return ResponseEntity.badRequest()
            .body(new ErrorResponse("Invalid input: " + e.getMessage()));
    }

    @ExceptionHandler(Exception.class)
    public ResponseEntity<ErrorResponse> handleGeneral(Exception e) {
        return ResponseEntity.internalServerError()
            .body(new ErrorResponse("Internal server error"));
    }
}
```

## Learning Resources

- [MetaOxide Getting Started (Java)](../../../docs/getting-started/getting-started-java.md)
- [Java API Reference](../../../docs/api/api-reference-java.md)
- [Spring Boot Documentation](https://spring.io/projects/spring-boot)
- [Spring REST Guide](https://spring.io/guides/gs/rest-service/)

## Troubleshooting

### Port Already in Use
```bash
# Use different port
mvn spring-boot:run -Dspring-boot.run.arguments="--server.port=8081"
```

### Native Library Not Found
```bash
# Ensure MetaOxide dependency is installed
mvn dependency:tree | grep meta-oxide
```

### Out of Memory
```bash
# Increase heap size
export MAVEN_OPTS="-Xmx1024m"
mvn clean install
```

## License

This example is licensed under the same dual license as MetaOxide: **MIT OR Apache-2.0**

---

**Questions?** Check the main [MetaOxide documentation](../../../README.md) or open an issue.
