# MetaOxide WASM Examples

This directory contains working examples for various JavaScript/TypeScript environments.

## 📋 Examples

### 1. Browser Example (`browser.html` + `browser.js`)

Interactive browser demo with a beautiful UI for extracting metadata.

**Features:**
- Extract from current page
- Extract from any URL
- Paste HTML directly
- Real-time results display
- Format statistics

**Run:**
```bash
# Serve with any static file server
npx serve .
# Open http://localhost:3000/browser.html
```

### 2. Node.js WASM Example (`node-wasm.js`)

Alternative to native Node.js binding using pure WASM.

**Usage:**
```bash
node node-wasm.js https://example.com
node node-wasm.js local-file.html
node node-wasm.js https://example.com --json
```

**Features:**
- Fetch from URLs
- Read local files
- Colored terminal output
- JSON export option

### 3. Deno Example (`deno.ts`)

Modern TypeScript runtime with full type safety.

**Usage:**
```bash
deno run --allow-net --allow-read deno.ts https://example.com
deno run --allow-read deno.ts local-file.html
deno run --allow-net --allow-write deno.ts https://example.com --json
```

**Install as command:**
```bash
deno install --allow-net --allow-read -n meta-extract deno.ts
meta-extract https://example.com
```

**Features:**
- Native TypeScript
- Secure by default
- Beautiful output formatting
- JSON export

### 4. Cloudflare Workers Example (`cloudflare-worker.ts`)

Deploy metadata extraction to Cloudflare's global edge network.

**Setup:**
```bash
npm install -g wrangler
```

**wrangler.toml:**
```toml
name = "meta-oxide-worker"
main = "cloudflare-worker.ts"
compatibility_date = "2024-01-01"

[build]
command = "npm install"
```

**Deploy:**
```bash
wrangler deploy
```

**API Endpoints:**
- `GET /` - API documentation
- `GET /extract?url=https://example.com` - Extract from URL
- `POST /extract` - Extract from HTML body

**Test locally:**
```bash
wrangler dev
curl "http://localhost:8787/extract?url=https://example.com"
```

### 5. Vercel Edge Functions Example (`vercel-edge.ts`)

Deploy to Vercel's edge network for global low-latency extraction.

**Setup:**

1. Create `api/metadata.ts` in your Vercel project:
```typescript
export { default } from '../bindings/wasm/examples/vercel-edge';
export { config } from '../bindings/wasm/examples/vercel-edge';
```

2. Add to `package.json`:
```json
{
  "dependencies": {
    "@yfedoseev/meta-oxide-wasm": "^0.1.3"
  }
}
```

3. Deploy:
```bash
vercel --prod
```

**API Endpoints:**
- `GET /api/metadata?url=https://example.com` - Extract from URL
- `POST /api/metadata` - Extract from HTML body (JSON or text)

**Test:**
```bash
curl "https://your-app.vercel.app/api/metadata?url=https://example.com"

curl -X POST "https://your-app.vercel.app/api/metadata" \
  -H "Content-Type: application/json" \
  -d '{"html": "<html>...</html>", "baseUrl": "https://example.com"}'
```

## 🚀 Quick Start

### Browser

```bash
# Build WASM first
cd ..
npm run build

# Serve examples
cd examples
npx serve .
```

Open http://localhost:3000/browser.html

### Node.js

```bash
node node-wasm.js https://github.com
```

### Deno

```bash
deno run --allow-net deno.ts https://github.com
```

## 💡 Use Cases

### Link Preview Service

```typescript
// Vercel Edge Function
export default async function handler(req: Request) {
    const url = new URL(req.url).searchParams.get('url');
    const html = await fetch(url).then(r => r.text());
    const meta = await extractAll(html, { baseUrl: url });

    return Response.json({
        title: meta.openGraph?.title,
        description: meta.openGraph?.description,
        image: meta.openGraph?.image,
    });
}
```

### SEO Analyzer

```typescript
// Node.js script
import { extractAll } from '@yfedoseev/meta-oxide-wasm';

const urls = ['https://site1.com', 'https://site2.com'];

for (const url of urls) {
    const html = await fetch(url).then(r => r.text());
    const meta = await extractAll(html, { baseUrl: url });

    console.log(`${url}:`);
    console.log(`  Description: ${meta.meta?.description ? '✓' : '✗'}`);
    console.log(`  Open Graph: ${meta.openGraph ? '✓' : '✗'}`);
    console.log(`  JSON-LD: ${meta.jsonLd?.items.length || 0} items`);
}
```

### Metadata Cache API

```typescript
// Cloudflare Workers with KV
export default {
    async fetch(request, env) {
        const url = new URL(request.url).searchParams.get('url');

        // Check cache
        let meta = await env.METADATA_CACHE.get(url, 'json');

        if (!meta) {
            // Extract and cache
            const html = await fetch(url).then(r => r.text());
            meta = await extractAll(html, { baseUrl: url });
            await env.METADATA_CACHE.put(url, JSON.stringify(meta), {
                expirationTtl: 3600 // 1 hour
            });
        }

        return Response.json(meta);
    }
}
```

## 🧪 Testing Examples

Each example includes inline tests you can run:

```bash
# Browser - open browser.html and use the UI

# Node.js
node node-wasm.js https://github.com

# Deno
deno run --allow-net deno.ts https://github.com

# Cloudflare Workers
wrangler dev
# Then test with curl or browser

# Vercel Edge
vercel dev
# Then test with curl or browser
```

## 📚 Additional Resources

- [Main README](../README.md) - Full documentation
- [API Reference](../README.md#api-reference) - Complete API docs
- [GitHub Repository](https://github.com/yfedoseev/meta-oxide) - Source code

## 🐛 Issues

Found a bug? [Open an issue](https://github.com/yfedoseev/meta-oxide/issues)
