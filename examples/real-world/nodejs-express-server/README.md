# MetaOxide Express.js Server

A REST API server demonstrating how to build a metadata extraction service using MetaOxide in Node.js with Express.

## What This Example Shows

This Express server demonstrates:
- Using MetaOxide Node.js bindings
- Building REST APIs with Express.js
- TypeScript support and type safety
- Async/await patterns
- Request validation and error handling
- CORS and middleware
- Performance optimization
- Deployment strategies

## Prerequisites

- Node.js 18+ ([Install](https://nodejs.org/))
- npm (comes with Node.js)
- TypeScript knowledge (optional)

## Installation & Setup

```bash
# Install dependencies
npm install

# Build (if using TypeScript)
npm run build

# Start server
npm start

# Development mode with auto-reload
npm run dev
```

## Running the Server

```bash
# Start the server (listens on port 3000)
npm start

# Or with environment variables
PORT=8080 npm start

# Server running at http://localhost:3000
```

## API Endpoints

### POST /api/extract

Extract all metadata from provided HTML.

**Request:**
```bash
curl -X POST http://localhost:3000/api/extract \
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
    },
    ...
  }
}
```

### POST /api/extract/:format

Extract specific metadata format.

**Formats:** `meta`, `open-graph`, `twitter`, `json-ld`, `microdata`, `microformats`, `rdfa`, `dublin-core`, `manifest`, `oembed`, `rel-links`

```bash
curl -X POST http://localhost:3000/api/extract/json-ld \
  -H "Content-Type: application/json" \
  -d '{"html": "...", "baseUrl": "https://example.com"}'
```

### GET /api/health

Health check endpoint.

```bash
curl http://localhost:3000/api/health
```

Response:
```json
{
  "status": "healthy",
  "version": "0.1.3",
  "uptime": 123.45
}
```

## Usage Examples

### Node.js with Fetch

```javascript
const response = await fetch('http://localhost:3000/api/extract', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    html: '<html><head><title>Test</title></head></html>',
    baseUrl: 'https://example.com'
  })
});

const result = await response.json();
console.log(result.data);
```

### TypeScript with Axios

```typescript
import axios from 'axios';

interface ExtractionRequest {
  html: string;
  baseUrl: string;
}

const response = await axios.post<any>('http://localhost:3000/api/extract', {
  html: '<html>...</html>',
  baseUrl: 'https://example.com'
});

console.log(response.data.data);
```

### Using the Included Client

```bash
node client.js --url https://github.com --format open-graph
```

## How It Works

### Main Components

1. **Express App** (`app.ts` or `app.js`)
   - Sets up routes and middleware
   - Request handling
   - Error handling

2. **MetaOxide Integration**
   - Imports native binding
   - Creates extractor per request
   - Returns results as JSON

3. **Middleware**
   - Body parser for JSON requests
   - CORS handling
   - Error catching

### Key Code Pattern

```typescript
import { extractAll } from '@yfedoseev/meta-oxide';

app.post('/api/extract', async (req, res) => {
  try {
    const { html, baseUrl } = req.body;

    // Validate input
    if (!html) {
      return res.status(400).json({ error: 'Missing html field' });
    }

    // Extract metadata
    const result = await extractAll(html, baseUrl || '');

    res.json({ success: true, data: result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});
```

## Performance

Expected performance on standard hardware:

- **Single extraction**: <10ms
- **10 concurrent requests**: <50ms
- **100 requests/second**: Easily handled

**Load testing with autocannon:**
```bash
npm install -g autocannon

# Run 10 concurrent connections for 10 seconds
autocannon http://localhost:3000/api/extract -c 10 -d 10
```

## Deployment

### Heroku

```bash
# Create Procfile
echo "web: npm start" > Procfile

# Deploy
heroku create metaoxide-api
git push heroku main
```

### Docker

```dockerfile
FROM node:18-alpine
WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production
COPY . .
EXPOSE 3000
CMD ["npm", "start"]
```

Build and run:
```bash
docker build -t metaoxide-api .
docker run -p 3000:3000 metaoxide-api
```

### Vercel

1. Install Vercel CLI: `npm i -g vercel`
2. Deploy: `vercel deploy`

For serverless function:
```javascript
import { extractAll } from '@yfedoseev/meta-oxide';

export default async (req, res) => {
  const { html, baseUrl } = req.body;
  const result = await extractAll(html, baseUrl);
  res.json(result);
};
```

## TypeScript Support

The example includes full TypeScript support:

```bash
# Build
npm run build

# Run compiled JavaScript
npm start

# Watch for changes
npm run dev
```

TypeScript configuration includes:
- Strict mode enabled
- ES2020 target
- Module resolution: node

## Testing

Run tests:
```bash
npm test

# Watch mode
npm run test:watch

# With coverage
npm run test:coverage
```

Example test:
```typescript
describe('POST /api/extract', () => {
  it('should extract metadata from HTML', async () => {
    const response = await request(app)
      .post('/api/extract')
      .send({
        html: '<html><head><title>Test</title></head></html>',
        baseUrl: 'https://example.com'
      });

    expect(response.status).toBe(200);
    expect(response.body.success).toBe(true);
    expect(response.body.data.meta.title).toBe('Test');
  });
});
```

## Middleware Examples

### Rate Limiting

```bash
npm install express-rate-limit
```

```typescript
import rateLimit from 'express-rate-limit';

const limiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100 // limit each IP to 100 requests per windowMs
});

app.post('/api/extract', limiter, async (req, res) => {
  // ...
});
```

### Logging

```bash
npm install morgan
```

```typescript
import morgan from 'morgan';

app.use(morgan('combined'));
```

### Authentication

```bash
npm install jsonwebtoken
```

```typescript
function verifyToken(req: Request, res: Response, next: NextFunction) {
  const token = req.headers.authorization?.split(' ')[1];
  if (!token) return res.status(401).json({ error: 'No token' });

  jwt.verify(token, process.env.JWT_SECRET!, (err) => {
    if (err) return res.status(403).json({ error: 'Invalid token' });
    next();
  });
}

app.post('/api/extract', verifyToken, async (req, res) => {
  // Protected route
});
```

## Learning Resources

- [MetaOxide Getting Started (Node.js)](../../../docs/getting-started/getting-started-nodejs.md)
- [Node.js API Reference](../../../docs/api/api-reference-nodejs.md)
- [Express.js Documentation](https://expressjs.com/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)

## Environment Variables

```bash
PORT=3000              # Server port (default: 3000)
NODE_ENV=production    # Environment (development/production)
LOG_LEVEL=info         # Logging level
```

## Troubleshooting

### Port Already in Use
```bash
# Use different port
PORT=3001 npm start

# Or kill the process
lsof -ti:3000 | xargs kill -9
```

### Module Not Found
```bash
# Reinstall dependencies
rm -rf node_modules package-lock.json
npm install
```

### Import Issues with Native Module
```bash
# Rebuild native module
npm rebuild

# Or reinstall
npm install --save @yfedoseev/meta-oxide
```

## License

This example is licensed under the same dual license as MetaOxide: **MIT OR Apache-2.0**

---

**Questions?** Check the main [MetaOxide documentation](../../../README.md) or open an issue.
