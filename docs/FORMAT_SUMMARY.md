# Quick Format Reference

One-page reference for all structured data formats supported (or planned) by MetaOxide.

## At a Glance

| Format | Where | What | Adoption | Status |
|--------|-------|------|----------|--------|
| **Meta Tags** | `<head>` meta | Basic metadata | 100% | 🔜 Planned |
| **Open Graph** | `<head>` meta | Social sharing | 60%+ | 🔜 Planned |
| **Twitter Cards** | `<head>` meta | Twitter/X sharing | 45% | 🔜 Planned |
| **JSON-LD** | `<script>` JSON | SEO, Rich Results, AI | 41% ↗️ | 🔜 Planned |
| **Microdata** | `<body>` attrs | SEO (older) | 26% ↘️ | 🔜 Planned |
| **Microformats** | `<body>` classes | IndieWeb, semantic | 5-10% | ✅ In Progress |
| **RDFa** | `<body>` attrs | Semantic web | <10% | 🔜 Planned |

---

## Example: Complete Modern Page

```html
<!DOCTYPE html>
<html>
<head>
  <!-- 1. STANDARD META TAGS (100% of sites) -->
  <title>Amazing Article About Rust</title>
  <meta name="description" content="Learn about Rust programming language">
  <meta name="author" content="Jane Doe">
  <link rel="canonical" href="https://example.com/rust-article">

  <!-- 2. OPEN GRAPH (60%+ of sites) - Facebook, LinkedIn -->
  <meta property="og:title" content="Amazing Article About Rust">
  <meta property="og:description" content="Learn about Rust programming">
  <meta property="og:image" content="https://example.com/rust-hero.jpg">
  <meta property="og:url" content="https://example.com/rust-article">
  <meta property="og:type" content="article">

  <!-- 3. TWITTER CARDS (45% of sites) - Twitter/X -->
  <meta name="twitter:card" content="summary_large_image">
  <meta name="twitter:title" content="Amazing Article About Rust">
  <meta name="twitter:description" content="Learn about Rust programming">
  <meta name="twitter:image" content="https://example.com/rust-hero.jpg">
  <meta name="twitter:creator" content="@janedoe">

  <!-- 4. JSON-LD (41% of sites) - Google, AI, SEO -->
  <script type="application/ld+json">
  {
    "@context": "https://schema.org",
    "@type": "Article",
    "headline": "Amazing Article About Rust",
    "description": "Learn about Rust programming language",
    "image": "https://example.com/rust-hero.jpg",
    "datePublished": "2024-01-15T10:00:00Z",
    "author": {
      "@type": "Person",
      "name": "Jane Doe",
      "url": "https://example.com/authors/jane"
    },
    "publisher": {
      "@type": "Organization",
      "name": "Tech Blog",
      "logo": {
        "@type": "ImageObject",
        "url": "https://example.com/logo.png"
      }
    }
  }
  </script>
</head>

<body>
  <!-- 5. MICROFORMATS (5-10% of sites) - IndieWeb -->
  <article class="h-entry">
    <h1 class="p-name">Amazing Article About Rust</h1>

    <div class="p-author h-card">
      <img class="u-photo" src="jane.jpg" alt="">
      <a class="p-name u-url" href="https://example.com/authors/jane">Jane Doe</a>
    </div>

    <time class="dt-published" datetime="2024-01-15T10:00:00Z">
      January 15, 2024
    </time>

    <div class="e-content">
      <p>Rust is an amazing programming language...</p>
    </div>

    <a class="p-category" href="/tag/rust">Rust</a>
    <a class="p-category" href="/tag/programming">Programming</a>
  </article>

  <!-- 6. MICRODATA (26% of sites, declining) - SEO -->
  <div itemscope itemtype="https://schema.org/Article">
    <h1 itemprop="headline">Amazing Article About Rust</h1>
    <span itemprop="author" itemscope itemtype="https://schema.org/Person">
      <span itemprop="name">Jane Doe</span>
    </span>
    <time itemprop="datePublished" datetime="2024-01-15">Jan 15, 2024</time>
  </div>
</body>
</html>
```

---

## What MetaOxide Extracts

### Input
Any HTML page with structured data

### Output
Unified JSON with all formats:

```python
import meta_oxide

result = meta_oxide.extract_all(html, base_url="https://example.com")

# Returns:
{
  # Standard meta tags
  "meta": {
    "title": "Amazing Article About Rust",
    "description": "Learn about Rust programming language",
    "author": "Jane Doe",
    "canonical": "https://example.com/rust-article"
  },

  # Open Graph
  "opengraph": {
    "title": "Amazing Article About Rust",
    "type": "article",
    "image": "https://example.com/rust-hero.jpg",
    "url": "https://example.com/rust-article",
    "description": "Learn about Rust programming"
  },

  # Twitter Cards
  "twitter": {
    "card": "summary_large_image",
    "title": "Amazing Article About Rust",
    "description": "Learn about Rust programming",
    "image": "https://example.com/rust-hero.jpg",
    "creator": "@janedoe"
  },

  # JSON-LD (parsed from <script> tag)
  "jsonld": [
    {
      "@type": "Article",
      "headline": "Amazing Article About Rust",
      "author": {
        "@type": "Person",
        "name": "Jane Doe"
      },
      "datePublished": "2024-01-15T10:00:00Z"
    }
  ],

  # Microformats
  "microformats": {
    "h-entry": [
      {
        "type": ["h-entry"],
        "properties": {
          "name": ["Amazing Article About Rust"],
          "author": [{
            "type": ["h-card"],
            "properties": {
              "name": ["Jane Doe"],
              "url": ["https://example.com/authors/jane"],
              "photo": ["jane.jpg"]
            }
          }],
          "published": ["2024-01-15T10:00:00Z"],
          "category": ["Rust", "Programming"],
          "content": [...]
        }
      }
    ]
  },

  # Microdata (parsed from itemscope/itemprop)
  "microdata": [
    {
      "type": "https://schema.org/Article",
      "properties": {
        "headline": "Amazing Article About Rust",
        "author": {
          "type": "https://schema.org/Person",
          "properties": {
            "name": "Jane Doe"
          }
        },
        "datePublished": "2024-01-15"
      }
    }
  ]
}
```

---

## Use Case Matrix

| I want to... | Use this format | Why |
|--------------|----------------|-----|
| Share on Facebook | Open Graph | Controls preview appearance |
| Share on Twitter/X | Twitter Cards | Controls tweet card |
| Rank in Google | JSON-LD | Rich Results eligibility |
| Support AI/LLMs | JSON-LD + Microformats | Training data |
| Sell products online | JSON-LD Product | Shopping results |
| Show recipes | JSON-LD Recipe | Recipe cards |
| List local business | JSON-LD LocalBusiness | Map listings |
| Build IndieWeb site | Microformats | Decentralized social |
| Basic SEO | Standard Meta | Foundation |

---

## Implementation Priority

### Phase 1: Foundation (Current) ✅
```
├── Microformats (h-card, h-entry, h-event)
├── Basic parser
└── Python bindings
```

### Phase 2: Social (Next) 🎯
```
├── Open Graph Protocol
├── Twitter Cards
├── Standard Meta Tags
└── More microformats (h-feed, h-review, h-product)
```

### Phase 3: SEO & AI (High Priority) 🚀
```
├── JSON-LD parser
├── Common Schema.org types (Article, Product, Person, Org)
├── Validation
└── Rich Results support
```

### Phase 4: Completeness
```
├── Microdata
├── RDFa
├── All microformats (h-recipe, h-adr, h-geo, etc.)
└── All Schema.org types
```

---

## Quick Reference: Property Prefixes

### Microformats
- `p-*` = Plain text (name, summary)
- `u-*` = URL (url, photo, email)
- `dt-*` = DateTime (published, updated)
- `e-*` = Embedded HTML (content)

### Microdata
- `itemscope` = Start of item
- `itemtype` = Type of item
- `itemprop` = Property name

### RDFa
- `vocab` = Vocabulary
- `typeof` = Type
- `property` = Property name

### Open Graph
- `property="og:*"` = Open Graph property

### Twitter Cards
- `name="twitter:*"` = Twitter property

### JSON-LD
- `@context` = Vocabulary
- `@type` = Type
- `property: value` = Standard JSON

---

## Testing Tools

| Format | Testing Tool | URL |
|--------|-------------|-----|
| JSON-LD | Google Rich Results Test | https://search.google.com/test/rich-results |
| JSON-LD | Schema Validator | https://validator.schema.org/ |
| Open Graph | Facebook Debugger | https://developers.facebook.com/tools/debug/ |
| Twitter Cards | Twitter Validator | https://cards-dev.twitter.com/validator |
| Microformats | Microformats Parser | https://microformats.io/ |
| All | MetaOxide (soon!) | `meta_oxide.extract_all()` |

---

## Common Mistakes to Avoid

### ❌ Wrong
```html
<!-- Missing required properties -->
<meta property="og:title" content="My Page">
<!-- No og:type, og:image, og:url -->

<!-- Invalid JSON-LD -->
<script type="application/ld+json">
{
  "@type": "Article"
  <!-- Missing @context -->
}
</script>

<!-- Mixed up prefixes -->
<div class="h-card">
  <span class="p-url">https://example.com</span>
  <!-- URLs should use u-*, not p-* -->
</div>
```

### ✅ Correct
```html
<!-- Complete Open Graph -->
<meta property="og:title" content="My Page">
<meta property="og:type" content="website">
<meta property="og:image" content="https://example.com/image.jpg">
<meta property="og:url" content="https://example.com">

<!-- Valid JSON-LD -->
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Article",
  "headline": "My Article"
}
</script>

<!-- Correct prefixes -->
<div class="h-card">
  <a class="u-url" href="https://example.com">Website</a>
</div>
```

---

## Resources

- **Full Format Guide**: [FORMATS.md](FORMATS.md)
- **Roadmap**: [ROADMAP.md](ROADMAP.md)
- **API Reference**: [api-reference.md](api-reference.md)
- **Examples**: [examples.md](examples.md)
