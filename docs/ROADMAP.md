# MetaOxide Roadmap - Prioritized by Adoption

**MetaOxide** is a comprehensive structured data extraction library. This roadmap is **ordered by real-world adoption statistics** to maximize impact.

## Vision

Extract ALL structured data from web pages in a single, fast, type-safe library with Python bindings.

## Priority Strategy

**Phases are ordered by adoption rate** - we build what websites actually use first:

1. **100% adoption** → Phase 1 (Standard Meta)
2. **60% adoption** → Phase 2 (Open Graph)
3. **45% adoption** → Phase 2 (Twitter Cards)
4. **41% adoption ↗️** → Phase 3 (JSON-LD - HIGHEST IMPACT)
5. **26% adoption ↘️** → Phase 4 (Microdata)
6. **5-10% adoption** → Phase 7 (Microformats - LOW priority)
7. **<10% adoption** → Phase 8+ (RDFa, Dublin Core - VERY LOW)

---

## Structured Data Formats - By Popularity

### Phase 1: Standard HTML Meta Tags (100% of websites) 🎯

**Status**: Planned - FOUNDATION
**Priority**: CRITICAL (universal)
**Adoption**: 100% of websites

Every website has these. Essential foundation.

#### Basic Meta Tags

```html
<title>Page Title</title>
<meta name="description" content="Page description for search results">
<meta name="keywords" content="keyword1, keyword2">
<meta name="author" content="Author Name">
<meta name="robots" content="index, follow">
<meta name="googlebot" content="index, follow">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="theme-color" content="#ffffff">
<meta name="generator" content="WordPress 6.0">
```

**Properties to extract**:
- [ ] title (also from `<title>` tag)
- [ ] description
- [ ] keywords
- [ ] author
- [ ] robots (index, follow, noindex, nofollow)
- [ ] googlebot (Google-specific directives)
- [ ] viewport
- [ ] theme-color
- [ ] generator

#### Link Elements

```html
<link rel="canonical" href="https://example.com/page">
<link rel="alternate" type="application/rss+xml" href="/feed.xml" title="RSS">
<link rel="alternate" type="application/atom+xml" href="/atom.xml" title="Atom">
<link rel="alternate" hreflang="es" href="https://example.com/es/">
<link rel="prev" href="https://example.com/page/1">
<link rel="next" href="https://example.com/page/3">
<link rel="icon" type="image/png" href="/favicon.png">
<link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png">
<link rel="manifest" href="/manifest.json">
```

**Properties to extract**:
- [ ] canonical URL
- [ ] RSS feed links
- [ ] Atom feed links
- [ ] Alternate language versions (hreflang)
- [ ] Pagination (prev/next)
- [ ] Favicons (all sizes)
- [ ] Apple touch icons (all sizes)
- [ ] PWA manifest link

**Use Case**: Foundation for all websites, basic SEO, browser features
**Impact**: CRITICAL - needed by every site

---

### Phase 2: Social Media Meta (60%+ adoption) 🚀

**Status**: Planned - HIGH PRIORITY
**Priority**: VERY HIGH
**Adoption**: Open Graph 60%+, Twitter 45%

#### Open Graph Protocol (Facebook, LinkedIn, WhatsApp)

```html
<meta property="og:title" content="Amazing Article">
<meta property="og:type" content="article">
<meta property="og:url" content="https://example.com/article">
<meta property="og:image" content="https://example.com/image.jpg">
<meta property="og:description" content="Description">
<meta property="og:site_name" content="My Website">
<meta property="og:locale" content="en_US">
```

**Basic Properties**:
- [ ] og:title
- [ ] og:type (article, website, video, etc.)
- [ ] og:url
- [ ] og:image (+ og:image:width, og:image:height, og:image:alt)
- [ ] og:description
- [ ] og:site_name
- [ ] og:locale

**Article-specific**:
- [ ] article:published_time
- [ ] article:modified_time
- [ ] article:author
- [ ] article:section
- [ ] article:tag

**Video-specific**:
- [ ] video:url, video:secure_url
- [ ] video:type, video:width, video:height
- [ ] video:duration

**Other Types**:
- [ ] Profile (first_name, last_name, username)
- [ ] Book (author, isbn, release_date)
- [ ] Music (duration, album, musician)

**Adoption**: 60%+ of websites
**Use Case**: Facebook, LinkedIn, Slack, Discord, WhatsApp previews
**Impact**: VERY HIGH - controls social sharing appearance

#### Twitter Cards

```html
<meta name="twitter:card" content="summary_large_image">
<meta name="twitter:site" content="@website">
<meta name="twitter:creator" content="@author">
<meta name="twitter:title" content="Article Title">
<meta name="twitter:description" content="Description">
<meta name="twitter:image" content="https://example.com/image.jpg">
<meta name="twitter:image:alt" content="Image description">
```

**Card Types**:
- [ ] summary
- [ ] summary_large_image
- [ ] app
- [ ] player

**Properties**:
- [ ] twitter:card
- [ ] twitter:site
- [ ] twitter:creator
- [ ] twitter:title
- [ ] twitter:description
- [ ] twitter:image
- [ ] twitter:image:alt
- [ ] App cards: app:id:iphone, app:id:googleplay, app:url
- [ ] Player cards: player:url, player:width, player:height

**Adoption**: 45% of websites
**Use Case**: Twitter/X link previews
**Impact**: VERY HIGH - Twitter is still major platform

**Note**: Twitter falls back to Open Graph if Twitter tags are missing, but explicit Twitter tags provide better control.

---

### Phase 3: Schema.org JSON-LD (41% adoption ↗️) ⚡

**Status**: Planned - HIGHEST IMPACT
**Priority**: CRITICAL (growing fast!)
**Adoption**: 41% and rising rapidly

**Why this is #1 priority after basics:**
- Google Rich Results eligibility
- AI/LLM training data (ChatGPT, Claude, etc.)
- Fastest growing format
- E-commerce essential
- Future-proof

```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Article",
  "headline": "Amazing Article",
  "datePublished": "2024-01-15T10:00:00Z"
}
</script>
```

#### Phase 3.1: Core Content Types (Most Common)

**Article Types** (blogs, news, content sites):
- [ ] Article
- [ ] NewsArticle
- [ ] BlogPosting
- [ ] TechArticle
- [ ] ScholarlyArticle

**Web Structure**:
- [ ] WebPage
- [ ] WebSite
- [ ] SearchAction (for site search)
- [ ] BreadcrumbList

**Properties**:
- headline, description, image, datePublished, dateModified
- author (Person/Organization), publisher
- articleBody, articleSection
- wordCount, keywords

#### Phase 3.2: E-commerce (Critical for online stores)

**Product & Offers**:
- [ ] Product
- [ ] Offer / AggregateOffer
- [ ] Brand
- [ ] Review
- [ ] AggregateRating

**Properties**:
- name, description, image, sku, gtin, mpn
- brand, manufacturer
- offers (price, priceCurrency, availability)
- aggregateRating (ratingValue, reviewCount, bestRating)
- review (author, reviewRating, reviewBody)

#### Phase 3.3: People & Organizations

- [ ] Person
- [ ] Organization
- [ ] LocalBusiness (+ all subtypes)
- [ ] ContactPoint
- [ ] PostalAddress

**Properties**:
- name, url, image, logo
- address (streetAddress, addressLocality, postalCode, etc.)
- telephone, email
- openingHours, geo (latitude, longitude)
- sameAs (social media profiles)

#### Phase 3.4: Events

- [ ] Event
- [ ] VirtualLocation
- [ ] Place

**Properties**:
- name, description, startDate, endDate
- location (Place or VirtualLocation)
- organizer, performer
- offers (ticket info)
- eventStatus, eventAttendanceMode

#### Phase 3.5: Recipes & HowTo

- [ ] Recipe
- [ ] HowTo
- [ ] HowToStep
- [ ] NutritionInformation

**Recipe Properties**:
- name, image, description, author
- recipeIngredient, recipeInstructions
- prepTime, cookTime, totalTime
- recipeYield, recipeCategory, recipeCuisine
- nutrition (calories, fatContent, etc.)
- aggregateRating, review

#### Phase 3.6: Job Postings

- [ ] JobPosting

**Properties**:
- title, description, datePosted
- hiringOrganization, jobLocation
- baseSalary, employmentType
- validThrough

#### Phase 3.7: FAQ & Q&A

- [ ] FAQPage
- [ ] Question
- [ ] Answer

#### Phase 3.8: Media Objects

- [ ] VideoObject
- [ ] AudioObject
- [ ] ImageObject
- [ ] MediaObject

**Adoption**: 41% of websites (2024), growing rapidly
**E-commerce adoption**: 41% (critical for product pages)
**Use Case**: Google Rich Results, AI training, voice search, knowledge graphs
**Impact**: ⚡ HIGHEST - enables Rich Results, future-proof for AI

---

### Phase 4: Microdata (26% adoption ↘️) 📉

**Status**: Planned
**Priority**: MEDIUM (declining but still significant)
**Adoption**: 26% of websites, declining

HTML5 specification using Schema.org vocabulary inline in HTML.

```html
<div itemscope itemtype="https://schema.org/Person">
  <span itemprop="name">Jane Doe</span>
  <span itemprop="jobTitle">Software Engineer</span>
  <div itemprop="address" itemscope itemtype="https://schema.org/PostalAddress">
    <span itemprop="streetAddress">123 Main St</span>
  </div>
</div>
```

**Implementation**:
- [ ] Parse itemscope, itemtype, itemprop attributes
- [ ] Support same Schema.org types as JSON-LD
- [ ] Convert to unified format
- [ ] Handle nested items

**Adoption**: 26% of websites
**Trend**: Declining (Google recommends migrating to JSON-LD)
**Use Case**: Older sites, inline markup preference
**Impact**: MEDIUM - still used but being phased out

---

### Phase 5: Content Discovery & Embedding 🔗

**Status**: Planned
**Priority**: MEDIUM-HIGH (high utility)
**Adoption**: Moderate but essential features

#### oEmbed Protocol

```html
<link rel="alternate" type="application/json+oembed"
      href="https://example.com/oembed?url=...">
```

- [ ] Detect oEmbed links
- [ ] Fetch and parse oEmbed JSON
- [ ] Support: video, photo, rich, link types
- [ ] Popular providers: YouTube, Twitter, Instagram, Vimeo, Spotify

**Use Case**: Embedded content, CMS integrations, rich previews
**Impact**: HIGH - critical for content platforms

#### rel-* Microformats (Link Relationships)

```html
<a rel="author" href="...">Author Page</a>
<a rel="license" href="...">License</a>
<a rel="me" href="https://twitter.com/me">Twitter</a>
<a rel="tag" href="...">Tag</a>
```

- [ ] rel="author" - Author attribution
- [ ] rel="license" - Content licensing
- [ ] rel="me" - Identity consolidation (IndieWeb)
- [ ] rel="tag" - Tag/category links
- [ ] rel="nofollow", rel="noopener", rel="noreferrer" - SEO/security
- [ ] rel="canonical" - Already in Phase 1
- [ ] rel="alternate" - Already in Phase 1

**Use Case**: Author attribution, IndieWeb, SEO
**Impact**: MEDIUM

---

### Phase 6: Verification & Platform Integration 🔐

**Status**: Planned
**Priority**: MEDIUM
**Adoption**: Common on business sites

#### Site Verification

```html
<meta name="google-site-verification" content="...">
<meta name="msvalidate.01" content="...">
<meta name="yandex-verification" content="...">
<meta name="p:domain_verify" content="...">
```

- [ ] Google Search Console
- [ ] Bing Webmaster Tools
- [ ] Yandex Webmaster
- [ ] Pinterest domain verification

#### Platform IDs

```html
<meta property="fb:app_id" content="...">
<meta property="fb:admins" content="...">
```

- [ ] Facebook App ID
- [ ] Facebook Admins

**Use Case**: Prove site ownership, platform integration
**Impact**: MEDIUM - common on business sites

---

### Phase 7: Microformats2 (5-10% adoption) 📝

**Status**: Partially Implemented
**Priority**: LOW-MEDIUM (niche but valuable)
**Adoption**: 5-10% of websites

**Note**: Despite low adoption, microformats are important for:
- IndieWeb community
- Semantic HTML
- Decentralized social web
- Some forward-thinking sites

Currently Implemented:
- [x] h-card - Contact information
- [x] h-entry - Blog posts, articles
- [x] h-event - Events

Planned:
- [ ] h-feed - Content feeds
- [ ] h-review - Reviews
- [ ] h-product - Products
- [ ] h-recipe - Recipes
- [ ] h-adr - Physical addresses
- [ ] h-geo - Geographic coordinates
- [ ] h-resume - Resumes/CVs
- [ ] h-review-aggregate - Aggregate reviews
- [ ] h-item - Generic items
- [ ] h-listing - Classified listings

#### XFN (XHTML Friends Network)

```html
<a rel="friend met" href="...">Friend</a>
```

- [ ] Social relationship markup
- [ ] Multiple values: friend, colleague, met, etc.

**Adoption**: 5-10% of websites
**Use Case**: IndieWeb, semantic HTML, decentralized social
**Impact**: LOW - niche but growing community

**Why still include?**
- Important for IndieWeb movement
- Clean, semantic HTML
- No external dependencies
- Good for personal sites/blogs
- Future-proof for decentralized web

---

### Phase 8: Progressive Web Apps & Mobile 📱

**Status**: Planned
**Priority**: MEDIUM (growing)
**Adoption**: Growing with PWA adoption

#### PWA Manifest

```html
<link rel="manifest" href="/manifest.json">
```

- [ ] Detect manifest link
- [ ] Optionally fetch and parse manifest.json
- [ ] Extract: name, short_name, icons, theme_color, display, start_url

**Use Case**: Installable web apps, app-like experience

#### Mobile App Meta

```html
<meta name="apple-mobile-web-app-capable" content="yes">
<meta name="apple-mobile-web-app-status-bar-style" content="black">
<meta name="msapplication-TileColor" content="#ffffff">
```

**Apple**:
- [ ] apple-mobile-web-app-capable
- [ ] apple-mobile-web-app-status-bar-style
- [ ] apple-mobile-web-app-title
- [ ] apple-touch-icon (all sizes)
- [ ] apple-touch-startup-image

**Microsoft**:
- [ ] msapplication-TileColor
- [ ] msapplication-TileImage
- [ ] msapplication-config
- [ ] All tile sizes (70x70, 150x150, 310x150, 310x310)

**Adoption**: Moderate and growing
**Use Case**: PWAs, mobile web apps, home screen icons
**Impact**: MEDIUM - growing importance

---

### Phase 9: RDFa (<10% adoption) 📚

**Status**: Planned
**Priority**: LOW (legacy, niche)
**Adoption**: <10% of websites

W3C standard for semantic web markup.

```html
<div vocab="https://schema.org/" typeof="Person">
  <span property="name">Jane Doe</span>
  <span property="jobTitle">Software Engineer</span>
</div>
```

- [ ] vocab, typeof, property attributes
- [ ] Schema.org vocabulary
- [ ] Dublin Core vocabulary
- [ ] Other RDF vocabularies

**Adoption**: <10% of websites
**Use Case**: Government sites, academic institutions, semantic web
**Impact**: LOW - very niche

---

### Phase 10: Academic & Legacy Formats (<5% adoption) 🏛️

**Status**: Planned
**Priority**: VERY LOW
**Adoption**: <5% of websites

#### Dublin Core

```html
<meta name="DC.title" content="Document Title">
<meta name="DC.creator" content="Author Name">
<meta name="DC.date" content="2024-01-15">
```

**All 15 core elements**:
- [ ] DC.title, DC.creator, DC.date
- [ ] DC.description, DC.subject, DC.publisher
- [ ] DC.contributor, DC.type, DC.format
- [ ] DC.identifier (ISBN, DOI), DC.source
- [ ] DC.language, DC.relation, DC.coverage
- [ ] DC.rights

**Adoption**: <5% (academic, library, government)
**Use Case**: Digital libraries, archives, institutional repositories
**Impact**: VERY LOW - extremely niche

---

## Implementation Timeline - Revised

### Q1 2024: Foundation ✅
- [x] Project structure
- [x] Basic microformats (h-card, h-entry, h-event) - LOWER PRIORITY NOW
- [x] Python bindings
- [x] Documentation

### Q2 2024: Essential Meta (100% + 60%) 🎯
- [ ] **Phase 1**: Standard HTML meta tags (100% adoption)
- [ ] **Phase 2**: Open Graph (60%) + Twitter Cards (45%)
- [ ] Basic testing and validation

### Q3 2024: JSON-LD - HIGHEST IMPACT ⚡
- [ ] **Phase 3.1-3.3**: JSON-LD core types
  - Article, Product, Person, Organization
- [ ] **Phase 3.4-3.5**: Events, Recipes
- [ ] Schema.org validation
- [ ] Rich Results testing

### Q4 2024: Completeness
- [ ] **Phase 4**: Microdata (26%)
- [ ] **Phase 5**: oEmbed, rel-* links
- [ ] **Phase 6**: Verification tags
- [ ] More microformats (complete Phase 7)

### 2025: Advanced & Niche
- [ ] **Phase 3.6-3.8**: Remaining JSON-LD types (Jobs, FAQ, Media)
- [ ] **Phase 8**: PWA, Mobile meta
- [ ] **Phase 9**: RDFa
- [ ] **Phase 10**: Dublin Core
- [ ] Performance optimization
- [ ] CLI tool, browser extension

---

## Adoption Statistics Summary

| Phase | Format | Adoption | Trend | Impact |
|-------|--------|----------|-------|--------|
| **1** | **Standard Meta** | **100%** | → | **CRITICAL** |
| **2** | **Open Graph** | **60%+** | → | **VERY HIGH** |
| **2** | **Twitter Cards** | **45%** | → | **VERY HIGH** |
| **3** | **JSON-LD** | **41%** | **↗️** | **⚡ HIGHEST** |
| **4** | **Microdata** | **26%** | **↘️** | **MEDIUM** |
| **5** | **oEmbed** | Moderate | → | **MEDIUM-HIGH** |
| **7** | **Microformats** | **5-10%** | → | **LOW-MEDIUM** |
| **9** | **RDFa** | **<10%** | **↘️** | **LOW** |
| **10** | **Dublin Core** | **<5%** | → | **VERY LOW** |

---

## Success Metrics - Revised

### Coverage Goals
- **Phase 1-2 Complete**: Extract from 100% of modern websites (standard meta + social)
- **Phase 3 Complete**: Full Rich Results support (JSON-LD)
- **Phase 4-6 Complete**: Comprehensive extraction from 95%+ of websites

### Adoption Coverage
- Phase 1: 100% of sites
- Phase 1-2: 100% of sites (with rich social previews)
- Phase 1-3: ~100% of sites (with SEO/AI optimization)
- Phase 1-4: ~100% of sites (complete)
- Phase 5-10: Edge cases and niche formats

---

## Why This Order?

### Old Approach (Wrong)
Started with microformats (5-10% adoption) because it seemed simple.

### New Approach (Correct)
1. **Standard Meta (100%)** - Everyone needs this
2. **Open Graph + Twitter (60%+45%)** - Social sharing is critical
3. **JSON-LD (41% ↗️)** - SEO, AI, future-proof, GROWING
4. **Microdata (26% ↘️)** - Still used, declining
5. **Utilities (oEmbed, etc.)** - High value despite moderate adoption
6. **Microformats (5-10%)** - Niche but valuable for IndieWeb
7. **Legacy (RDFa, Dublin Core)** - Very niche

### Impact vs Effort

| Format | Adoption | Impact | Effort | Priority |
|--------|----------|--------|--------|----------|
| Standard Meta | 100% | Critical | Low | Do First |
| Open Graph | 60% | Very High | Low | Do First |
| Twitter | 45% | Very High | Low | Do First |
| JSON-LD | 41% ↗️ | Highest | Medium | Do Second |
| Microdata | 26% ↘️ | Medium | Medium | Do Third |
| oEmbed | Moderate | Medium-High | Medium | Do Third |
| Microformats | 5-10% | Low-Medium | Low | Later |
| RDFa | <10% | Low | Medium | Much Later |
| Dublin Core | <5% | Very Low | Low | Last |

---

## Key Changes from Previous Roadmap

### What Changed?
1. **Microformats moved from Phase 1 → Phase 7**
   - Reason: Only 5-10% adoption
   - Still valuable for IndieWeb, but not priority

2. **Standard Meta moved to Phase 1**
   - Reason: 100% adoption, foundation

3. **JSON-LD emphasized as HIGHEST IMPACT**
   - Reason: 41% and growing, enables Rich Results, AI/LLM training
   - E-commerce: 41% adoption (critical)

4. **Social (OG+Twitter) is Phase 2**
   - Reason: 60%+45% adoption, immediate value

5. **New phases added for utilities**
   - oEmbed, verification, PWA
   - Moderate adoption but high utility

### What Stayed?
- Python bindings approach
- Rust performance focus
- Type-safe design
- Comprehensive documentation

---

## Unified API Design

Same API as before, but returned data reflects priority:

```python
import meta_oxide

data = meta_oxide.extract_all(html, base_url=url)

# Returns (in priority order):
{
    # Phase 1 (100%)
    "meta": {
        "title": "...",
        "description": "...",
        "canonical": "..."
    },

    # Phase 2 (60%+45%)
    "opengraph": {
        "title": "...",
        "image": "...",
        "type": "article"
    },
    "twitter": {
        "card": "summary_large_image",
        "title": "..."
    },

    # Phase 3 (41% ↗️)
    "jsonld": [
        {
            "@type": "Article",
            "headline": "..."
        }
    ],

    # Phase 4 (26% ↘️)
    "microdata": [...],

    # Phase 7 (5-10%)
    "microformats": {
        "h-entry": [...],
        "h-card": [...]
    },

    # Phase 9 (<10%)
    "rdfa": [...],

    # Phase 10 (<5%)
    "dublin_core": {...}
}
```

---

## For Contributors

### Want to help? Start with highest impact:

**Immediate Impact (Do First)**:
1. Standard meta tags (Phase 1) - Easy, 100% coverage
2. Open Graph (Phase 2) - Easy, 60% coverage
3. Twitter Cards (Phase 2) - Easy, 45% coverage

**High Impact (Do Second)**:
4. JSON-LD parser (Phase 3) - Medium difficulty, 41% coverage, GROWING
5. Article/Product/Person types - Most common Schema.org types

**Good Impact (Do Third)**:
6. Microdata (Phase 4) - Medium difficulty, 26% coverage
7. oEmbed (Phase 5) - Moderate utility

**Nice to Have (Later)**:
8. Complete microformats (Phase 7)
9. RDFa (Phase 9)
10. Dublin Core (Phase 10)

---

## Last Updated

**Date**: 2024-11-06
**Status**: Phase 1 (microformats) started but REPRIORITIZED
**Next**: Phase 1 (Standard Meta) + Phase 2 (Social)
**Reason for update**: Aligned with real-world adoption data
