# Complete Structured Data Format Checklist

Comprehensive list of ALL structured data formats that MetaOxide should support.

## Core Formats (The Big 6) ✅

You asked about these specifically - all covered!

- [x] **Dublin Core** - Academic/library metadata
  - Status: 📋 Documented in roadmap (Phase 7)
  - Priority: LOW (niche usage)
  - Location: `<meta name="DC.*">`

- [x] **RDFa** - Generic semantic web markup
  - Status: 📋 Documented in roadmap (Phase 6)
  - Priority: LOW-MEDIUM (government, academic)
  - Location: HTML attributes (`vocab`, `typeof`, `property`)

- [x] **JSON-LD** - Schema.org in JSON
  - Status: 📋 Documented in roadmap (Phase 3)
  - Priority: ⚡ HIGHEST (41% adoption, growing)
  - Location: `<script type="application/ld+json">`

- [x] **Microdata** - Schema.org in HTML
  - Status: 📋 Documented in roadmap (Phase 4)
  - Priority: MEDIUM (26% adoption, declining)
  - Location: HTML attributes (`itemscope`, `itemprop`)

- [x] **Open Graph** - Facebook/social sharing
  - Status: 📋 Documented in roadmap (Phase 2)
  - Priority: ⚡ HIGH (60%+ adoption)
  - Location: `<meta property="og:*">`

- [x] **Twitter/X Cards** - Twitter sharing
  - Status: 📋 Documented in roadmap (Phase 2)
  - Priority: ⚡ HIGH (45% adoption)
  - Location: `<meta name="twitter:*">`

---

## Additional Formats to Consider

### Social Media & Sharing Platforms

- [x] **Pinterest Rich Pins**
  - Status: ✅ Covered (uses Open Graph + oEmbed)
  - Implementation: Extract Open Graph data
  - Types: Article, Product, Recipe
  - Note: Pinterest reads Open Graph tags

- [x] **LinkedIn Sharing**
  - Status: ✅ Covered (uses Open Graph + oEmbed)
  - Implementation: Extract Open Graph data
  - Note: LinkedIn uses Open Graph protocol

- [x] **WhatsApp Preview**
  - Status: ✅ Covered (uses Open Graph)
  - Implementation: Extract Open Graph data
  - Note: WhatsApp uses `og:*` tags

- [x] **Slack Unfurling**
  - Status: ✅ Covered (uses Open Graph + oEmbed)
  - Implementation: Extract Open Graph data
  - Note: Slack uses Open Graph + oEmbed

- [x] **Discord Embeds**
  - Status: ✅ Covered (uses Open Graph)
  - Implementation: Extract Open Graph data
  - Note: Discord uses `og:*` tags

- [x] **Telegram Instant View**
  - Status: ✅ Covered (uses Open Graph)
  - Implementation: Extract Open Graph data

- [ ] **oEmbed Protocol** ⚠️ NEEDS EMPHASIS
  - Status: 📋 Mentioned in roadmap (Phase 7)
  - Priority: MEDIUM-HIGH (embedded content)
  - Location: `<link rel="alternate" type="application/json+oembed">`
  - Use case: YouTube, Twitter, Instagram embeds
  - **ACTION**: Should move to Phase 3 or 4

---

### Microformats Family

- [x] **Microformats2** (h-*)
  - Status: ✅ Partially implemented
  - Formats covered in roadmap:
    - ✅ h-card, h-entry, h-event (implemented)
    - 📋 h-feed, h-review, h-product (planned)
    - 📋 h-recipe, h-adr, h-geo (planned)
    - 📋 h-resume, h-review-aggregate, h-item, h-listing (planned)

- [ ] **rel-* microformats** ⚠️ MISSING
  - Status: ❌ Not documented yet
  - Priority: LOW-MEDIUM
  - Examples:
    - `rel="author"` - Link to author page
    - `rel="license"` - Content license
    - `rel="me"` - Identity consolidation
    - `rel="tag"` - Tag links
    - `rel="nofollow"`, `rel="noopener"`
  - Location: `<a>` and `<link>` elements
  - **ACTION**: Add to Phase 4

- [ ] **XFN (XHTML Friends Network)** ⚠️ MISSING
  - Status: ❌ Not documented yet
  - Priority: LOW (legacy, IndieWeb)
  - Examples: `rel="friend"`, `rel="colleague"`, `rel="met"`
  - Use case: Social relationships
  - **ACTION**: Add to Phase 4 or 5

- [ ] **hAtom** (legacy microformat)
  - Status: ❌ Not documented
  - Priority: VERY LOW (superseded by h-entry)
  - Note: Replaced by Microformats2
  - **ACTION**: Probably skip, legacy

---

### Standard HTML Elements & Meta

- [x] **Standard Meta Tags**
  - Status: 📋 Documented in roadmap (Phase 2)
  - Covered: description, keywords, author, robots, etc.

- [x] **Link Elements**
  - Status: 📋 Documented (partially)
  - Covered: canonical, alternate, prev, next

- [ ] **RSS/Atom Feed Autodiscovery** ⚠️ NEEDS CLARITY
  - Status: 📋 Mentioned but not detailed
  - Priority: MEDIUM
  - Location: `<link rel="alternate" type="application/rss+xml">`
  - Also: `type="application/atom+xml"`
  - **ACTION**: Clarify in Phase 2 or 3

- [ ] **AMP (Accelerated Mobile Pages)** ⚠️ MISSING
  - Status: ❌ Not documented
  - Priority: LOW-MEDIUM (declining popularity)
  - Location: `<html ⚡>` or `<html amp>`
  - Also: `<link rel="amphtml" href="...">`
  - Use case: Mobile-optimized pages
  - **ACTION**: Add to Phase 5 or skip

---

### Mobile & App Meta

- [x] **Apple Mobile Web App Meta**
  - Status: 📋 Mentioned in roadmap (Phase 7)
  - Examples:
    - `apple-mobile-web-app-capable`
    - `apple-mobile-web-app-status-bar-style`
    - `apple-touch-icon` (various sizes)

- [x] **Microsoft Meta Tags**
  - Status: 📋 Mentioned in roadmap (Phase 7)
  - Examples:
    - `msapplication-TileColor`
    - `msapplication-TileImage`
    - `msapplication-config`

- [ ] **PWA Manifest** ⚠️ MISSING
  - Status: ❌ Not documented
  - Priority: MEDIUM (Progressive Web Apps)
  - Location: `<link rel="manifest" href="manifest.json">`
  - Use case: Install as app, theme colors, icons
  - **ACTION**: Add to Phase 4 or 5
  - Note: Requires fetching and parsing JSON file

- [x] **Viewport Meta**
  - Status: 📋 Documented in roadmap (Phase 2)
  - Example: `<meta name="viewport" content="width=device-width">`

- [x] **Theme Color**
  - Status: 📋 Documented in roadmap (Phase 2)
  - Example: `<meta name="theme-color" content="#ffffff">`

---

### Image & Media Metadata

- [ ] **EXIF/IPTC in Images** ⚠️ OUT OF SCOPE?
  - Status: ❌ Not documented
  - Priority: LOW (requires image parsing)
  - Use case: Photo metadata
  - **ACTION**: Probably out of scope (not HTML metadata)
  - Note: Would need separate image parser

- [ ] **Video/Audio Metadata** ⚠️ PARTIAL
  - Status: ⚠️ Covered via Open Graph video properties
  - Also covered: Schema.org VideoObject, AudioObject
  - HTML5 `<video>` and `<audio>` attributes: out of scope?

---

### Specialized Formats

- [ ] **Google-specific formats** ⚠️ MISSING
  - Status: ❌ Not fully documented
  - Priority: MEDIUM-HIGH

  **Google Rich Results** (via JSON-LD):
  - [x] Covered in Schema.org section

  **Google-specific meta**:
  - [ ] `google-site-verification` - Site ownership
  - [ ] `googlebot` - Google crawler directives
  - [ ] Google News meta tags
  - **ACTION**: Add to Phase 3

- [ ] **Facebook-specific** ⚠️ PARTIAL
  - Status: ⚠️ Covered via Open Graph
  - Additional Facebook meta:
    - `fb:app_id` - Facebook App ID
    - `fb:admins` - Facebook Admin IDs
  - **ACTION**: Include in Open Graph implementation

- [ ] **Bing/Microsoft-specific** ⚠️ MISSING
  - Status: ❌ Not documented
  - Priority: LOW-MEDIUM
  - Examples:
    - `msvalidate.01` - Bing Webmaster verification
  - **ACTION**: Add to Phase 4 or 5

- [ ] **Yandex Meta** ⚠️ MISSING
  - Status: ❌ Not documented
  - Priority: LOW (Russia-specific)
  - Examples:
    - `yandex-verification` - Yandex Webmaster
  - **ACTION**: Low priority or skip

---

### Semantic Web & Linked Data

- [x] **RDFa** - Already covered ✅

- [x] **JSON-LD** - Already covered ✅

- [x] **Microdata** - Already covered ✅

- [ ] **Turtle/N-Triples** ⚠️ OUT OF SCOPE?
  - Status: ❌ Not in HTML typically
  - Priority: VERY LOW
  - Note: Usually separate files, not in HTML
  - **ACTION**: Skip (not HTML metadata)

---

### E-commerce & Shopping

- [x] **Google Merchant Center** (via JSON-LD Product)
  - Status: ✅ Covered in Schema.org roadmap
  - Types: Product, Offer, AggregateRating

- [x] **Amazon Product Markup**
  - Status: ✅ Uses Schema.org (covered)

- [ ] **Shopify-specific meta** ⚠️ PARTIAL
  - Status: ⚠️ Uses standard formats (OG, JSON-LD)
  - Note: No Shopify-specific format needed

---

### Analytics & Tracking

- [ ] **Google Analytics Tags** ⚠️ OUT OF SCOPE
  - Status: ❌ Not metadata (tracking code)
  - Priority: N/A
  - **ACTION**: Out of scope (not structured data)

- [ ] **Pixel Tracking (Facebook, etc.)** ⚠️ OUT OF SCOPE
  - Status: ❌ Not metadata
  - **ACTION**: Out of scope

---

### Security & Verification

- [ ] **Site Verification Meta Tags** ⚠️ MISSING
  - Status: ❌ Not documented
  - Priority: MEDIUM
  - Examples:
    - `google-site-verification`
    - `msvalidate.01` (Bing)
    - `yandex-verification`
    - `p:domain_verify` (Pinterest)
  - **ACTION**: Add to Phase 4

---

### Content Syndication

- [x] **RSS/Atom Feeds**
  - Status: 📋 Mentioned in roadmap
  - **ACTION**: Clarify in documentation

- [ ] **WebSub (formerly PubSubHubbub)** ⚠️ MISSING
  - Status: ❌ Not documented
  - Priority: LOW
  - Use case: Real-time feed updates
  - Location: `<link rel="hub" href="...">`
  - **ACTION**: Add to Phase 5 or skip

- [ ] **ActivityStreams** ⚠️ OUT OF SCOPE?
  - Status: ❌ Not documented
  - Priority: LOW (Mastodon, ActivityPub)
  - Format: JSON (separate from HTML)
  - **ACTION**: Future consideration

---

## Summary: What's Missing?

### HIGH Priority Missing Items ⚠️

1. **oEmbed** - Should emphasize more (Phase 3 or 4)
   - Critical for embedded content
   - Used by YouTube, Twitter, Instagram, Vimeo, etc.

2. **rel-* microformats** - Add to Phase 4
   - `rel="author"`, `rel="license"`, `rel="me"`, `rel="tag"`
   - Important for IndieWeb

3. **Site Verification Meta** - Add to Phase 4
   - Google, Bing, Pinterest verification tags

4. **RSS/Atom Autodiscovery** - Clarify in Phase 2/3
   - Already mentioned but needs better documentation

### MEDIUM Priority Missing Items

5. **PWA Manifest** - Add to Phase 4 or 5
   - Progressive Web App support

6. **Google-specific meta** - Add to Phase 3
   - `google-site-verification`, `googlebot`, Google News

7. **XFN (XHTML Friends Network)** - Add to Phase 4
   - Social relationships (IndieWeb)

### LOW Priority / Out of Scope

8. **AMP (Accelerated Mobile Pages)** - Declining, maybe skip
9. **EXIF/IPTC** - Image metadata, requires separate parser
10. **ActivityStreams** - Separate JSON format, not HTML
11. **Tracking pixels/analytics** - Not structured data
12. **hAtom** - Legacy, superseded by h-entry

---

## Updated Checklist

### ✅ Fully Covered (Documented in Roadmap)

1. Dublin Core ✅
2. RDFa ✅
3. JSON-LD ✅
4. Microdata ✅
5. Open Graph ✅
6. Twitter Cards ✅
7. Microformats2 (all types) ✅
8. Standard Meta Tags ✅
9. Standard Link Elements ✅
10. Apple Mobile Meta ✅
11. Microsoft Meta ✅

### ⚠️ Mentioned but Needs More Detail

12. oEmbed ⚠️ (mentioned, needs emphasis)
13. RSS/Atom Autodiscovery ⚠️ (mentioned, needs clarity)

### ❌ Missing from Documentation (Should Add)

14. rel-* microformats ❌ (rel="author", rel="license", etc.)
15. XFN (rel="friend", etc.) ❌
16. Site Verification Meta ❌ (Google, Bing, Pinterest)
17. PWA Manifest Link ❌
18. Google-specific meta ❌ (googlebot, verification)
19. Facebook-specific (fb:app_id) ❌

### 🚫 Out of Scope (Probably Skip)

20. AMP 🚫 (declining popularity)
21. EXIF/IPTC 🚫 (not HTML)
22. ActivityStreams 🚫 (separate JSON)
23. Tracking/Analytics 🚫 (not structured data)
24. hAtom 🚫 (legacy)

---

## Recommended Actions

### Immediate (Documentation Update)

1. **Emphasize oEmbed** - Move to Phase 3
2. **Add rel-* microformats** section to roadmap
3. **Clarify RSS/Atom autodiscovery** in Phase 2
4. **Add Site Verification meta** to Phase 4

### Future Phases

5. Add **PWA Manifest** to Phase 5
6. Add **XFN** to Phase 4
7. Add **Google-specific meta** to Phase 3
8. Add **Facebook-specific** (fb:app_id) to Open Graph section

### Skip/Defer

9. AMP - declining, low priority
10. EXIF/IPTC - requires image parser
11. ActivityStreams - different scope
12. Tracking code - not structured data

---

## Final Answer to Your Question

**Did we cover all of them?**

### ✅ YES - Core Formats
- Dublin Core ✅
- RDFa ✅
- JSON-LD ✅
- Microdata ✅
- Open Graph ✅
- Twitter/X Cards ✅

### ⚠️ MOSTLY - Platform-Specific
- Pinterest Rich Pins ✅ (uses Open Graph)
- LinkedIn ✅ (uses Open Graph)
- WhatsApp ✅ (uses Open Graph)
- Slack ✅ (uses Open Graph + oEmbed)
- Discord ✅ (uses Open Graph)

### ❌ GAPS - Need to Add
- **oEmbed** - Mentioned but needs more emphasis
- **rel-* microformats** - Missing entirely
- **Site Verification tags** - Missing
- **PWA Manifest** - Missing
- **XFN** - Missing

**Recommendation**: Update ROADMAP.md to include the missing items above.
