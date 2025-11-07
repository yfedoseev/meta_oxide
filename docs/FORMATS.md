# Structured Data Formats - Complete Guide

This document provides a comprehensive overview of all structured data formats that MetaOxide extracts from web pages.

## Table of Contents

- [Format Comparison](#format-comparison)
- [Microformats2](#microformats2)
- [Open Graph Protocol](#open-graph-protocol)
- [Twitter Cards](#twitter-cards)
- [Schema.org JSON-LD](#schemaorg-json-ld)
- [Microdata](#microdata)
- [RDFa](#rdfa)
- [Standard HTML Meta Tags](#standard-html-meta-tags)
- [Which Format Should I Use?](#which-format-should-i-use)

---

## Format Comparison

| Format | Adoption | Primary Use Case | Complexity | SEO Impact | Social Impact |
|--------|----------|-----------------|------------|------------|---------------|
| **Open Graph** | 60%+ | Facebook/social sharing | Low | Medium | High |
| **Twitter Cards** | 45% | Twitter/X sharing | Low | Low | High |
| **JSON-LD** | 41% | Google Rich Results, AI | Medium | Very High | Low |
| **Microdata** | 26% | SEO (older sites) | Medium | High | Low |
| **Microformats** | 5-10% | IndieWeb, semantic web | Low | Low | Low |
| **RDFa** | <10% | Government, academic | High | Medium | Low |
| **Meta Tags** | 100% | Basic metadata | Very Low | Medium | Low |

### When Each Format is Used

```
┌─────────────────────────────────────────────────────────────┐
│                    Modern Website (2024)                     │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  <head>                                                      │
│    <!-- Standard Meta (100% of sites) -->                   │
│    <meta name="description" content="...">                  │
│    <link rel="canonical" href="...">                        │
│                                                              │
│    <!-- Open Graph (60%+ of sites) -->                      │
│    <meta property="og:title" content="...">                 │
│    <meta property="og:image" content="...">                 │
│                                                              │
│    <!-- Twitter Cards (45% of sites) -->                    │
│    <meta name="twitter:card" content="...">                 │
│                                                              │
│    <!-- JSON-LD (41% of sites, growing fast) -->            │
│    <script type="application/ld+json">                      │
│    {                                                         │
│      "@context": "https://schema.org",                      │
│      "@type": "Article",                                    │
│      "headline": "..."                                       │
│    }                                                         │
│    </script>                                                 │
│  </head>                                                     │
│                                                              │
│  <body>                                                      │
│    <!-- Microformats2 (5-10% of sites) -->                  │
│    <article class="h-entry">                                │
│      <h1 class="p-name">Title</h1>                          │
│    </article>                                                │
│                                                              │
│    <!-- Microdata (26% of sites, declining) -->             │
│    <div itemscope itemtype="https://schema.org/Person">    │
│      <span itemprop="name">John</span>                      │
│    </div>                                                    │
│  </body>                                                     │
└─────────────────────────────────────────────────────────────┘
```

---

## Microformats2

**Adoption**: 5-10% of websites
**Primary Use**: IndieWeb, semantic HTML, decentralized social web
**Location**: HTML body (class attributes)

### What It Is

Microformats are simple conventions for embedding structured data in HTML using class names.

### Example

```html
<div class="h-card">
  <span class="p-name">Jane Doe</span>
  <a class="u-email" href="mailto:jane@example.com">Email</a>
  <img class="u-photo" src="photo.jpg">
  <p class="p-note">Software engineer</p>
</div>
```

### Supported Types (Current + Planned)

- ✅ **h-card** - People and organizations
- ✅ **h-entry** - Blog posts, articles, notes
- ✅ **h-event** - Events
- 🔜 **h-feed** - Feeds of entries
- 🔜 **h-review** - Reviews
- 🔜 **h-product** - Products
- 🔜 **h-recipe** - Recipes
- 🔜 **h-adr** - Addresses
- 🔜 **h-geo** - Geographic coordinates

### Property Prefixes

- `p-*` - Plain text properties (name, summary)
- `u-*` - URL properties (url, photo)
- `dt-*` - DateTime properties (published, updated)
- `e-*` - Embedded HTML properties (content)

### Best For

- Personal websites and blogs
- IndieWeb applications
- Decentralized social networks
- Semantic HTML markup

### Pros & Cons

**Pros:**
- Simple to implement
- Human-readable HTML
- No external schema required
- Works well for small sites

**Cons:**
- Lower adoption
- Not recognized by Google for Rich Results
- Limited social media support

---

## Open Graph Protocol

**Adoption**: 60%+ of websites
**Primary Use**: Facebook, LinkedIn, social sharing
**Location**: HTML head (meta tags with property attribute)

### What It Is

Created by Facebook, Open Graph turns web pages into rich objects in social graphs. When you share a link on Facebook/LinkedIn, this is what controls how it appears.

### Example

```html
<head>
  <!-- Basic -->
  <meta property="og:title" content="The Best Article Ever">
  <meta property="og:type" content="article">
  <meta property="og:url" content="https://example.com/article">
  <meta property="og:image" content="https://example.com/image.jpg">
  <meta property="og:description" content="This article is amazing...">
  <meta property="og:site_name" content="My Website">

  <!-- Article-specific -->
  <meta property="article:published_time" content="2024-01-01T12:00:00Z">
  <meta property="article:author" content="Jane Doe">
  <meta property="article:section" content="Technology">
  <meta property="article:tag" content="Rust">

  <!-- Image details -->
  <meta property="og:image:width" content="1200">
  <meta property="og:image:height" content="630">
  <meta property="og:image:alt" content="Article hero image">
</head>
```

### Supported Types

**Basic Types:**
- article
- website
- profile
- book
- music.song, music.album, music.playlist
- video.movie, video.tv_show, video.episode

**Properties by Type:**

**Article:**
```
article:published_time
article:modified_time
article:author
article:section
article:tag
```

**Profile:**
```
profile:first_name
profile:last_name
profile:username
profile:gender
```

**Video:**
```
video:url
video:secure_url
video:type
video:width
video:height
video:duration
```

### Best For

- Social media sharing
- Link previews on Facebook, LinkedIn, Slack
- Controlling how your content appears when shared
- Any content that might be shared socially

### Pros & Cons

**Pros:**
- Widely supported (60%+ of websites)
- Controls social sharing appearance
- Simple to implement
- Works across multiple platforms

**Cons:**
- No direct SEO benefit
- Duplicates other metadata
- Image size requirements (1200x630 recommended)

---

## Twitter Cards

**Adoption**: 45% of websites
**Primary Use**: Twitter/X link previews
**Location**: HTML head (meta tags with name attribute)

### What It Is

Twitter's metadata format for rich link previews. Similar to Open Graph but Twitter-specific.

### Example

```html
<head>
  <!-- Card type -->
  <meta name="twitter:card" content="summary_large_image">

  <!-- Basic info -->
  <meta name="twitter:title" content="The Best Article Ever">
  <meta name="twitter:description" content="This article is amazing...">
  <meta name="twitter:image" content="https://example.com/image.jpg">
  <meta name="twitter:image:alt" content="Article hero image">

  <!-- Attribution -->
  <meta name="twitter:site" content="@mywebsite">
  <meta name="twitter:creator" content="@janedoe">

  <!-- App card (for mobile apps) -->
  <meta name="twitter:app:id:iphone" content="123456789">
  <meta name="twitter:app:id:googleplay" content="com.example.app">
</head>
```

### Card Types

**summary**
- Title, description, thumbnail image
- Small square image (120x120)
- Good for: Articles, blog posts

**summary_large_image**
- Title, description, large image
- Large image (280x150)
- Good for: Featured articles, visual content

**app**
- Mobile app information
- Download links for iOS/Android
- Good for: App promotion

**player**
- Video/audio player embed
- Good for: Video content, podcasts

### Best For

- Twitter/X sharing
- Content with strong visual component
- Mobile apps
- Video/audio content

### Pros & Cons

**Pros:**
- Controls Twitter appearance
- Fallback to Open Graph if missing
- Multiple card types for different content

**Cons:**
- Twitter/X only
- Image size requirements
- Duplicates Open Graph data

### Twitter vs Open Graph

Twitter will fall back to Open Graph tags if Twitter tags are missing:

```
twitter:title → og:title → <title>
twitter:description → og:description → <meta name="description">
twitter:image → og:image
```

**Best practice**: Use both for maximum compatibility.

---

## Schema.org JSON-LD

**Adoption**: 41% of websites (growing rapidly)
**Primary Use**: Google Rich Results, AI/LLM training, SEO
**Location**: HTML head or body (script tag)

### What It Is

JSON-LD (JavaScript Object Notation for Linked Data) is Google's preferred format for structured data. It's a script tag containing JSON describing your page content.

### Why It Matters

- **Google Rich Results**: Recipe cards, event listings, product reviews
- **AI Training**: Used to train LLMs (ChatGPT, Claude, etc.)
- **Knowledge Graphs**: Powers Google's Knowledge Graph
- **Voice Search**: Enables voice assistants to understand content

### Example

```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Article",
  "headline": "The Best Article Ever",
  "description": "This article is amazing...",
  "image": "https://example.com/image.jpg",
  "datePublished": "2024-01-01T12:00:00Z",
  "dateModified": "2024-01-02T10:00:00Z",
  "author": {
    "@type": "Person",
    "name": "Jane Doe",
    "url": "https://example.com/authors/jane"
  },
  "publisher": {
    "@type": "Organization",
    "name": "My Website",
    "logo": {
      "@type": "ImageObject",
      "url": "https://example.com/logo.png"
    }
  }
}
</script>
```

### Common Types

#### Content Types

**Article / NewsArticle / BlogPosting**
```json
{
  "@type": "Article",
  "headline": "...",
  "datePublished": "...",
  "author": {...},
  "image": "...",
  "articleBody": "..."
}
```

**WebPage / WebSite**
```json
{
  "@type": "WebSite",
  "name": "My Website",
  "url": "https://example.com",
  "potentialAction": {
    "@type": "SearchAction",
    "target": "https://example.com/search?q={search_term}",
    "query-input": "required name=search_term"
  }
}
```

#### People & Organizations

**Person**
```json
{
  "@type": "Person",
  "name": "Jane Doe",
  "jobTitle": "Software Engineer",
  "image": "https://example.com/jane.jpg",
  "sameAs": [
    "https://twitter.com/janedoe",
    "https://github.com/janedoe"
  ]
}
```

**Organization / LocalBusiness**
```json
{
  "@type": "LocalBusiness",
  "name": "Joe's Coffee Shop",
  "address": {
    "@type": "PostalAddress",
    "streetAddress": "123 Main St",
    "addressLocality": "Seattle",
    "addressRegion": "WA",
    "postalCode": "98101"
  },
  "telephone": "+1-206-555-1234",
  "openingHours": "Mo-Fr 08:00-17:00"
}
```

#### Products & Commerce

**Product**
```json
{
  "@type": "Product",
  "name": "Amazing Widget",
  "image": "https://example.com/widget.jpg",
  "description": "The best widget ever made",
  "brand": {
    "@type": "Brand",
    "name": "WidgetCo"
  },
  "offers": {
    "@type": "Offer",
    "price": "29.99",
    "priceCurrency": "USD",
    "availability": "https://schema.org/InStock"
  },
  "aggregateRating": {
    "@type": "AggregateRating",
    "ratingValue": "4.5",
    "reviewCount": "123"
  }
}
```

#### Events

**Event**
```json
{
  "@type": "Event",
  "name": "Tech Conference 2024",
  "startDate": "2024-09-10T09:00:00-07:00",
  "endDate": "2024-09-12T17:00:00-07:00",
  "location": {
    "@type": "Place",
    "name": "Convention Center",
    "address": {
      "@type": "PostalAddress",
      "streetAddress": "123 Convention Blvd",
      "addressLocality": "Seattle",
      "addressRegion": "WA"
    }
  },
  "offers": {
    "@type": "Offer",
    "price": "299.00",
    "priceCurrency": "USD",
    "url": "https://example.com/tickets"
  }
}
```

#### Recipes

**Recipe**
```json
{
  "@type": "Recipe",
  "name": "Chocolate Chip Cookies",
  "image": "https://example.com/cookies.jpg",
  "author": {
    "@type": "Person",
    "name": "Jane Doe"
  },
  "datePublished": "2024-01-01",
  "description": "The best chocolate chip cookies",
  "prepTime": "PT15M",
  "cookTime": "PT10M",
  "totalTime": "PT25M",
  "recipeYield": "24 cookies",
  "recipeIngredient": [
    "2 cups flour",
    "1 cup butter",
    "1 cup chocolate chips"
  ],
  "recipeInstructions": [
    {
      "@type": "HowToStep",
      "text": "Mix flour and butter"
    },
    {
      "@type": "HowToStep",
      "text": "Add chocolate chips"
    }
  ],
  "nutrition": {
    "@type": "NutritionInformation",
    "calories": "150 calories"
  }
}
```

#### Navigation

**BreadcrumbList**
```json
{
  "@type": "BreadcrumbList",
  "itemListElement": [
    {
      "@type": "ListItem",
      "position": 1,
      "name": "Home",
      "item": "https://example.com"
    },
    {
      "@type": "ListItem",
      "position": 2,
      "name": "Category",
      "item": "https://example.com/category"
    },
    {
      "@type": "ListItem",
      "position": 3,
      "name": "Article",
      "item": "https://example.com/category/article"
    }
  ]
}
```

#### FAQ

**FAQPage**
```json
{
  "@type": "FAQPage",
  "mainEntity": [
    {
      "@type": "Question",
      "name": "What is MetaOxide?",
      "acceptedAnswer": {
        "@type": "Answer",
        "text": "MetaOxide is a Rust library for extracting structured data from web pages."
      }
    }
  ]
}
```

### Best For

- SEO and Google Rich Results
- E-commerce sites (products, reviews)
- Content sites (articles, recipes)
- Local businesses
- Events
- Any site wanting better search visibility

### Pros & Cons

**Pros:**
- Google's preferred format
- Rich Results eligibility
- Clean separation from HTML
- Multiple schemas on one page
- AI/LLM training data
- Growing adoption (41% and rising)

**Cons:**
- Can be verbose
- Requires validation
- No direct social media benefit
- Learning curve for complex types

---

## Microdata

**Adoption**: 26% of websites (declining)
**Primary Use**: SEO (older sites)
**Location**: HTML body (attributes on elements)

### What It Is

HTML5 specification for embedding structured data using HTML attributes. Uses same Schema.org vocabulary as JSON-LD but inline in HTML.

### Example

```html
<div itemscope itemtype="https://schema.org/Person">
  <span itemprop="name">Jane Doe</span>
  <span itemprop="jobTitle">Software Engineer</span>
  <div itemprop="address" itemscope itemtype="https://schema.org/PostalAddress">
    <span itemprop="streetAddress">123 Main St</span>
    <span itemprop="addressLocality">Seattle</span>
    <span itemprop="addressRegion">WA</span>
  </div>
  <a href="mailto:jane@example.com" itemprop="email">Email</a>
</div>
```

### Best For

- Legacy sites already using microdata
- When you want markup inline with content
- When JSON-LD is not an option

### Pros & Cons

**Pros:**
- Same Schema.org vocabulary as JSON-LD
- Inline with content
- Google supports it

**Cons:**
- Declining adoption (JSON-LD preferred)
- Clutters HTML
- Harder to maintain
- More verbose than microformats

**Migration**: Google recommends migrating from Microdata to JSON-LD.

---

## RDFa

**Adoption**: <10% of websites
**Primary Use**: Government sites, academic institutions
**Location**: HTML body (attributes on elements)

### What It Is

Resource Description Framework in Attributes. A W3C recommendation for embedding RDF data in HTML.

### Example

```html
<div vocab="https://schema.org/" typeof="Person">
  <span property="name">Jane Doe</span>
  <span property="jobTitle">Software Engineer</span>
  <a href="https://example.com" property="url">Website</a>
</div>
```

### Best For

- Government websites
- Academic/research sites
- Semantic web applications
- When RDF compatibility is required

### Pros & Cons

**Pros:**
- W3C standard
- RDF compatibility
- Flexible vocabularies

**Cons:**
- Low adoption
- Complex syntax
- JSON-LD/Microdata preferred by Google
- Steeper learning curve

---

## Standard HTML Meta Tags

**Adoption**: 100% of websites
**Primary Use**: Basic metadata, SEO
**Location**: HTML head

### What It Is

Standard HTML meta tags and link elements. The foundation of web metadata.

### Examples

```html
<head>
  <!-- Page info -->
  <title>Page Title</title>
  <meta name="description" content="Page description for search results">
  <meta name="keywords" content="keyword1, keyword2">
  <meta name="author" content="Jane Doe">

  <!-- SEO -->
  <link rel="canonical" href="https://example.com/page">
  <meta name="robots" content="index, follow">

  <!-- Viewport (mobile) -->
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <!-- Favicons -->
  <link rel="icon" type="image/png" href="/favicon.png">
  <link rel="apple-touch-icon" href="/apple-touch-icon.png">

  <!-- Feeds -->
  <link rel="alternate" type="application/rss+xml" href="/feed.xml" title="RSS Feed">

  <!-- Theme -->
  <meta name="theme-color" content="#ffffff">

  <!-- Pagination -->
  <link rel="prev" href="https://example.com/page/1">
  <link rel="next" href="https://example.com/page/3">
</head>
```

### Best For

- Every website (required!)
- Basic SEO
- Browser features
- Feed discovery

---

## Which Format Should I Use?

### Recommended Stack for Modern Websites (2024)

```html
<head>
  <!-- 1. REQUIRED: Standard Meta (everyone) -->
  <title>Your Page Title</title>
  <meta name="description" content="...">
  <link rel="canonical" href="https://example.com/page">

  <!-- 2. RECOMMENDED: Open Graph (social sharing) -->
  <meta property="og:title" content="...">
  <meta property="og:description" content="...">
  <meta property="og:image" content="...">
  <meta property="og:url" content="...">

  <!-- 3. RECOMMENDED: Twitter Cards (Twitter/X) -->
  <meta name="twitter:card" content="summary_large_image">
  <meta name="twitter:title" content="...">
  <meta name="twitter:image" content="...">

  <!-- 4. HIGHLY RECOMMENDED: JSON-LD (SEO, AI) -->
  <script type="application/ld+json">
  {
    "@context": "https://schema.org",
    "@type": "Article",
    "headline": "...",
    "author": {...}
  }
  </script>

  <!-- 5. OPTIONAL: Microformats (IndieWeb) -->
</head>
<body>
  <article class="h-entry">
    <h1 class="p-name">...</h1>
  </article>
</body>
```

### Decision Matrix

| Your Goal | Use These Formats |
|-----------|------------------|
| **Maximum social sharing** | Open Graph + Twitter Cards |
| **Best SEO** | JSON-LD + Standard Meta |
| **E-commerce** | JSON-LD (Product, Offer, Review) |
| **Blog/Content site** | JSON-LD + Open Graph + Twitter |
| **IndieWeb** | Microformats + Open Graph |
| **Local business** | JSON-LD (LocalBusiness) |
| **Events** | JSON-LD + Microformats (h-event) |
| **Recipes** | JSON-LD (Recipe) |
| **AI/LLM optimization** | JSON-LD + Microformats |

### Priority Order

1. **Standard Meta** (required)
2. **Open Graph** (social)
3. **Twitter Cards** (Twitter/X)
4. **JSON-LD** (SEO, AI)
5. **Microformats** (optional, semantic)

---

## Testing Your Structured Data

### Google

- [Rich Results Test](https://search.google.com/test/rich-results)
- [Schema Markup Validator](https://validator.schema.org/)

### Facebook

- [Sharing Debugger](https://developers.facebook.com/tools/debug/)

### Twitter

- [Card Validator](https://cards-dev.twitter.com/validator)

### Microformats

- [Microformats Parser](https://microformats.io/)

---

## References

- [Microformats.org](http://microformats.org/)
- [Open Graph Protocol](https://ogp.me/)
- [Twitter Cards](https://developer.twitter.com/en/docs/twitter-for-websites/cards/overview/abouts-cards)
- [Schema.org](https://schema.org/)
- [Google Search Central](https://developers.google.com/search/docs/appearance/structured-data)
- [W3C Microdata](https://www.w3.org/TR/microdata/)
- [W3C RDFa](https://www.w3.org/TR/rdfa-primer/)
