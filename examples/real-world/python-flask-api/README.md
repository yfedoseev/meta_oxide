# MetaOxide Flask API

A REST API demonstrating how to build a metadata extraction service using MetaOxide in Python with Flask.

## What This Example Shows

This Flask API demonstrates:
- Building a REST API with Flask
- Integrating MetaOxide Python bindings
- Handling HTTP requests and responses
- Input validation
- Error handling and status codes
- Extracting metadata from user-provided HTML
- CORS handling for cross-origin requests
- Deployment considerations

## Prerequisites

- Python 3.8+ ([Install](https://www.python.org/downloads/))
- pip (comes with Python)

## Installation & Setup

```bash
# Create virtual environment
python -m venv venv

# Activate it
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt
```

## Running the Server

```bash
# Start the Flask development server
python app.py

# Server will run at http://localhost:5000
```

For production, use a WSGI server:
```bash
# Install gunicorn
pip install gunicorn

# Run with gunicorn
gunicorn -w 4 -b 0.0.0.0:5000 app:app
```

## API Endpoints

### POST /extract

Extract all metadata from provided HTML.

**Request:**
```bash
curl -X POST http://localhost:5000/extract \
  -H "Content-Type: application/json" \
  -d '{
    "html": "<html><head>...</head></html>",
    "base_url": "https://example.com"
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
    },
    ...
  }
}
```

### POST /extract/:format

Extract specific metadata format.

**Supported formats:**
- `meta` - HTML Meta Tags
- `open-graph` - Open Graph
- `twitter` - Twitter Cards
- `json-ld` - JSON-LD
- `microdata` - Microdata
- `microformats` - Microformats
- `rdfa` - RDFa
- `dublin-core` - Dublin Core
- `manifest` - Web App Manifest
- `oembed` - oEmbed
- `rel-links` - Link Relations

**Request:**
```bash
curl -X POST http://localhost:5000/extract/open-graph \
  -H "Content-Type: application/json" \
  -d '{
    "html": "<html><head>...</head></html>",
    "base_url": "https://example.com"
  }'
```

**Response:**
```json
{
  "success": true,
  "data": {
    "og:title": "Example",
    "og:description": "..."
  }
}
```

### GET /health

Health check endpoint.

**Response:**
```json
{
  "status": "healthy",
  "version": "0.1.3"
}
```

## Usage Examples

### Python Requests Library

```python
import requests

# Extract all metadata
response = requests.post('http://localhost:5000/extract', json={
    'html': '<html><head><title>Test</title></head></html>',
    'base_url': 'https://example.com'
})

print(response.json()['data'])

# Extract specific format
response = requests.post('http://localhost:5000/extract/json-ld', json={
    'html': html_content,
    'base_url': 'https://example.com'
})

print(response.json()['data'])
```

### cURL

```bash
# Extract all metadata
curl -X POST http://localhost:5000/extract \
  -H "Content-Type: application/json" \
  -d '{"html": "<html>...</html>", "base_url": "https://example.com"}'

# Extract Open Graph only
curl -X POST http://localhost:5000/extract/open-graph \
  -H "Content-Type: application/json" \
  -d '{"html": "<html>...</html>", "base_url": "https://example.com"}'

# Pretty print response
curl -X POST http://localhost:5000/extract \
  -H "Content-Type: application/json" \
  -d '{"html": "<html>...</html>", "base_url": "https://example.com"}' | python -m json.tool
```

### JavaScript/Node.js

```javascript
const fetch = require('node-fetch');

// Extract metadata
const response = await fetch('http://localhost:5000/extract', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    html: '<html>...</html>',
    base_url: 'https://example.com'
  })
});

const result = await response.json();
console.log(result.data);
```

## Error Handling

The API returns appropriate HTTP status codes:

| Code | Meaning |
|------|---------|
| 200 | Success |
| 400 | Bad request (missing fields, invalid input) |
| 404 | Endpoint not found |
| 500 | Server error |

**Error Response:**
```json
{
  "success": false,
  "error": "Missing required field: html"
}
```

## How It Works

### Main Components

1. **Flask App** (`app.py`)
   - Sets up routes and request handlers
   - Input validation
   - Response formatting

2. **MetaOxide Integration**
   - Imports Python MetaOxide library
   - Creates extractor for each request
   - Handles extraction and error cases

3. **Request Handling**
   - Accepts JSON POST requests
   - Validates required fields
   - Returns JSON responses

### Key Code Pattern

```python
from meta_oxide import MetaOxide
from flask import Flask, request, jsonify

@app.route('/extract', methods=['POST'])
def extract():
    data = request.get_json()

    # Validate input
    if 'html' not in data:
        return jsonify({'error': 'Missing html field'}), 400

    # Extract metadata
    try:
        extractor = MetaOxide(data['html'], data.get('base_url', ''))
        result = extractor.extract_all()
        return jsonify({'success': True, 'data': result})
    except Exception as e:
        return jsonify({'error': str(e)}), 500
```

## Performance

Expected performance on standard hardware:

- **Single extraction**: <50ms
- **10 concurrent requests**: <100ms total
- **100 requests/second**: Easily handled with 4 workers

**Scaling tips:**
```bash
# Use multiple workers
gunicorn -w 8 app:app

# Use gevent for async
pip install gevent gunicorn[gevent]
gunicorn -w 1 -k gevent -b 0.0.0.0:5000 app:app
```

## Deployment

### Docker Deployment

```dockerfile
FROM python:3.11-slim

WORKDIR /app
COPY requirements.txt .
RUN pip install -r requirements.txt

COPY app.py .
EXPOSE 5000

CMD ["gunicorn", "-w", "4", "-b", "0.0.0.0:5000", "app:app"]
```

Build and run:
```bash
docker build -t metaoxide-api .
docker run -p 5000:5000 metaoxide-api
```

### Heroku Deployment

1. Create `Procfile`:
```
web: gunicorn app:app
```

2. Deploy:
```bash
heroku create metaoxide-api
git push heroku main
```

## Testing

Run tests:
```bash
python -m pytest tests/
```

Example test:
```python
def test_extract_endpoint():
    response = client.post('/extract', json={
        'html': '<html><head><title>Test</title></head></html>',
        'base_url': 'https://example.com'
    })

    assert response.status_code == 200
    assert response.json['success'] == True
    assert 'meta' in response.json['data']
```

## Monitoring & Logging

The app includes logging for debugging:

```python
import logging

logger = logging.getLogger(__name__)

@app.route('/extract', methods=['POST'])
def extract():
    logger.info(f"Extraction request received")
    # ...
```

## Rate Limiting (Optional)

Add rate limiting to prevent abuse:

```bash
pip install flask-limiter
```

```python
from flask_limiter import Limiter

limiter = Limiter(app)

@app.route('/extract', methods=['POST'])
@limiter.limit("60 per minute")
def extract():
    # ...
```

## Extensions

### Add URL Fetching
Fetch and extract metadata from URLs:

```bash
pip install requests
```

```python
@app.route('/extract-url', methods=['POST'])
def extract_from_url():
    url = request.json.get('url')
    response = requests.get(url)
    # Extract from response.text
```

### Add Webhook Support
Post results to a webhook URL:

```python
@app.route('/extract-async', methods=['POST'])
def extract_async():
    # Queue job
    # POST results to webhook_url when done
```

## Learning Resources

- [MetaOxide Getting Started (Python)](../../../docs/getting-started/getting-started-python.md)
- [Python API Reference](../../../docs/api/api-reference-python.md)
- [Flask Documentation](https://flask.palletsprojects.com/)
- [MetaOxide Documentation](../../../README.md)

## Troubleshooting

### Port Already in Use
```bash
# Use different port
python app.py --port 5001

# Or kill process using port 5000
lsof -ti:5000 | xargs kill -9
```

### Import Error
```bash
# Make sure MetaOxide is installed
pip install meta-oxide

# Check installation
python -c "from meta_oxide import MetaOxide; print('OK')"
```

## License

This example is licensed under the same dual license as MetaOxide: **MIT OR Apache-2.0**

---

**Questions?** Check the main [MetaOxide documentation](../../../README.md) or open an issue.
