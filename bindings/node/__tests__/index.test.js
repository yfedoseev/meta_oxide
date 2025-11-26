/**
 * Comprehensive test suite for meta-oxide-node bindings
 * Tests all extraction functions with various HTML inputs
 */

const {
  extractAll,
  extractMeta,
  extractOpengraph,
  extractTwitter,
} = require('../index.js')

describe('meta-oxide-node bindings', () => {
  describe('extractAll', () => {
    it('should extract all metadata from HTML with multiple formats', () => {
      const html = `
        <html>
          <head>
            <title>Test Page</title>
            <meta name="description" content="A test page">
            <meta property="og:title" content="Open Graph Title">
            <meta name="twitter:card" content="summary">
          </head>
        </html>
      `
      const result = extractAll(html)
      expect(result).toBeDefined()
      expect(typeof result).toBe('string')

      const parsed = JSON.parse(result)
      expect(parsed).toHaveProperty('meta')
      expect(parsed).toHaveProperty('opengraph')
      expect(parsed).toHaveProperty('twitter')
    })

    it('should handle HTML with base URL', () => {
      const html = '<html><head><meta property="og:url" content="/page"></head></html>'
      const result = extractAll(html, 'https://example.com/')

      expect(result).toBeDefined()
      const parsed = JSON.parse(result)
      expect(parsed).toHaveProperty('opengraph')
    })

    it('should return valid JSON from extractAll', () => {
      const html = '<html><head><meta name="description" content="Test"></head></html>'
      const result = extractAll(html)

      expect(() => JSON.parse(result)).not.toThrow()
      const parsed = JSON.parse(result)
      expect(typeof parsed).toBe('object')
    })

    it('should extract from minimal HTML', () => {
      const html = '<html></html>'
      const result = extractAll(html)

      expect(result).toBeDefined()
      const parsed = JSON.parse(result)
      expect(typeof parsed).toBe('object')
    })

    it('should handle HTML with multiple meta tags', () => {
      const html = `
        <html>
          <head>
            <meta name="description" content="Description 1">
            <meta name="keywords" content="tag1, tag2">
            <meta property="og:title" content="Title">
            <meta property="og:image" content="image.jpg">
          </head>
        </html>
      `
      const result = extractAll(html)
      const parsed = JSON.parse(result)

      expect(parsed.meta).toBeDefined()
      expect(parsed.opengraph).toBeDefined()
    })

    it('should extract JSON-LD data', () => {
      const html = `
        <html>
          <head>
            <script type="application/ld+json">
            {
              "@context": "https://schema.org",
              "@type": "Article",
              "headline": "Test Article"
            }
            </script>
          </head>
        </html>
      `
      const result = extractAll(html)
      const parsed = JSON.parse(result)

      expect(parsed).toHaveProperty('jsonld')
    })

    it('should extract microdata', () => {
      const html = `
        <html>
          <body>
            <div itemscope itemtype="https://schema.org/Person">
              <span itemprop="name">John Doe</span>
            </div>
          </body>
        </html>
      `
      const result = extractAll(html)
      const parsed = JSON.parse(result)

      expect(parsed).toHaveProperty('microdata')
    })

    it('should handle empty/null optional parameter', () => {
      const html = '<html></html>'
      const result1 = extractAll(html)
      const result2 = extractAll(html, null)

      expect(result1).toBeDefined()
      expect(result2).toBeDefined()
    })
  })

  describe('extractMeta', () => {
    it('should extract standard HTML meta tags', () => {
      const html = `
        <html>
          <head>
            <title>Test Title</title>
            <meta name="description" content="Test description">
            <meta name="keywords" content="test, meta">
            <meta name="author" content="John Doe">
          </head>
        </html>
      `
      const result = extractMeta(html)

      expect(result).toBeDefined()
      expect(typeof result).toBe('string')

      const parsed = JSON.parse(result)
      expect(parsed).toHaveProperty('title')
      expect(parsed).toHaveProperty('description')
    })

    it('should extract meta tags with various properties', () => {
      const html = `
        <html>
          <head>
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <meta name="theme-color" content="#000000">
            <meta name="charset" value="utf-8">
          </head>
        </html>
      `
      const result = extractMeta(html)

      expect(result).toBeDefined()
      const parsed = JSON.parse(result)
      expect(typeof parsed).toBe('object')
    })

    it('should handle duplicate meta tags', () => {
      const html = `
        <html>
          <head>
            <meta name="description" content="First">
            <meta name="description" content="Second">
          </head>
        </html>
      `
      const result = extractMeta(html)

      expect(() => JSON.parse(result)).not.toThrow()
    })

    it('should extract title from different sources', () => {
      const html = '<html><head><title>Page Title</title></head></html>'
      const result = extractMeta(html)
      const parsed = JSON.parse(result)

      expect(parsed.title).toBeDefined()
    })

    it('should handle meta tags with base URL', () => {
      const html = '<html><head><meta name="description" content="Test"></head></html>'
      const result = extractMeta(html, 'https://example.com')

      expect(result).toBeDefined()
      expect(() => JSON.parse(result)).not.toThrow()
    })

    it('should extract charset and language meta tags', () => {
      const html = `
        <html lang="en">
          <head>
            <meta charset="utf-8">
            <meta http-equiv="Content-Language" content="en-US">
          </head>
        </html>
      `
      const result = extractMeta(html)

      expect(result).toBeDefined()
    })

    it('should return valid JSON', () => {
      const html = '<html><head></head></html>'
      const result = extractMeta(html)

      expect(() => JSON.parse(result)).not.toThrow()
    })
  })

  describe('extractOpengraph', () => {
    it('should extract Open Graph tags', () => {
      const html = `
        <html>
          <head>
            <meta property="og:title" content="Test Title">
            <meta property="og:description" content="Test Description">
            <meta property="og:image" content="https://example.com/image.jpg">
            <meta property="og:url" content="https://example.com/page">
          </head>
        </html>
      `
      const result = extractOpengraph(html)

      expect(result).toBeDefined()
      expect(typeof result).toBe('string')

      const parsed = JSON.parse(result)
      expect(parsed).toHaveProperty('title')
      expect(parsed).toHaveProperty('description')
      expect(parsed).toHaveProperty('image')
    })

    it('should extract Open Graph type', () => {
      const html = `
        <html>
          <head>
            <meta property="og:type" content="article">
          </head>
        </html>
      `
      const result = extractOpengraph(html)
      const parsed = JSON.parse(result)

      expect(parsed.type).toBeDefined()
    })

    it('should handle multiple Open Graph images', () => {
      const html = `
        <html>
          <head>
            <meta property="og:image" content="image1.jpg">
            <meta property="og:image" content="image2.jpg">
          </head>
        </html>
      `
      const result = extractOpengraph(html)

      expect(() => JSON.parse(result)).not.toThrow()
    })

    it('should extract Open Graph for different content types', () => {
      const types = ['article', 'website', 'video', 'music', 'book']

      types.forEach(type => {
        const html = `
          <html>
            <head>
              <meta property="og:type" content="${type}">
              <meta property="og:title" content="Test">
            </head>
          </html>
        `
        const result = extractOpengraph(html)

        expect(() => JSON.parse(result)).not.toThrow()
      })
    })

    it('should handle missing Open Graph tags', () => {
      const html = '<html><head></head></html>'
      const result = extractOpengraph(html)

      expect(() => JSON.parse(result)).not.toThrow()
    })

    it('should resolve relative URLs with base_url', () => {
      const html = `
        <html>
          <head>
            <meta property="og:image" content="/images/pic.jpg">
          </head>
        </html>
      `
      const result = extractOpengraph(html, 'https://example.com')

      expect(result).toBeDefined()
    })

    it('should extract article-specific Open Graph tags', () => {
      const html = `
        <html>
          <head>
            <meta property="og:type" content="article">
            <meta property="article:published_time" content="2024-01-01">
            <meta property="article:author" content="John Doe">
          </head>
        </html>
      `
      const result = extractOpengraph(html)

      expect(() => JSON.parse(result)).not.toThrow()
    })

    it('should return valid JSON', () => {
      const html = '<html><head></head></html>'
      const result = extractOpengraph(html)

      expect(() => JSON.parse(result)).not.toThrow()
    })
  })

  describe('extractTwitter', () => {
    it('should extract Twitter Card tags', () => {
      const html = `
        <html>
          <head>
            <meta name="twitter:card" content="summary">
            <meta name="twitter:title" content="Test Title">
            <meta name="twitter:description" content="Test Description">
            <meta name="twitter:image" content="https://example.com/image.jpg">
          </head>
        </html>
      `
      const result = extractTwitter(html)

      expect(result).toBeDefined()
      expect(typeof result).toBe('string')

      const parsed = JSON.parse(result)
      expect(parsed).toHaveProperty('card')
      expect(parsed).toHaveProperty('title')
    })

    it('should extract different Twitter Card types', () => {
      const cardTypes = ['summary', 'summary_large_image', 'app', 'player']

      cardTypes.forEach(cardType => {
        const html = `
          <html>
            <head>
              <meta name="twitter:card" content="${cardType}">
            </head>
          </html>
        `
        const result = extractTwitter(html)

        expect(() => JSON.parse(result)).not.toThrow()
      })
    })

    it('should extract Twitter creator and site', () => {
      const html = `
        <html>
          <head>
            <meta name="twitter:creator" content="@author">
            <meta name="twitter:site" content="@company">
          </head>
        </html>
      `
      const result = extractTwitter(html)
      const parsed = JSON.parse(result)

      expect(parsed.creator || parsed.site).toBeDefined()
    })

    it('should handle Twitter Card with player', () => {
      const html = `
        <html>
          <head>
            <meta name="twitter:card" content="player">
            <meta name="twitter:player" content="https://example.com/player">
            <meta name="twitter:player:width" content="640">
            <meta name="twitter:player:height" content="480">
          </head>
        </html>
      `
      const result = extractTwitter(html)

      expect(() => JSON.parse(result)).not.toThrow()
    })

    it('should handle missing Twitter Card tags', () => {
      const html = '<html><head></head></html>'
      const result = extractTwitter(html)

      expect(() => JSON.parse(result)).not.toThrow()
    })

    it('should resolve relative URLs with base_url', () => {
      const html = `
        <html>
          <head>
            <meta name="twitter:image" content="/images/pic.jpg">
          </head>
        </html>
      `
      const result = extractTwitter(html, 'https://example.com')

      expect(result).toBeDefined()
    })

    it('should extract Twitter app card data', () => {
      const html = `
        <html>
          <head>
            <meta name="twitter:card" content="app">
            <meta name="twitter:app:name:iphone" content="My App">
            <meta name="twitter:app:id:iphone" content="12345">
          </head>
        </html>
      `
      const result = extractTwitter(html)

      expect(() => JSON.parse(result)).not.toThrow()
    })

    it('should return valid JSON', () => {
      const html = '<html><head></head></html>'
      const result = extractTwitter(html)

      expect(() => JSON.parse(result)).not.toThrow()
    })
  })

  describe('Error handling', () => {
    it('should handle malformed HTML gracefully', () => {
      const html = '<html><head><title>Unclosed'

      expect(() => {
        extractAll(html)
      }).not.toThrow()
    })

    it('should handle HTML with special characters', () => {
      const html = `
        <html>
          <head>
            <meta name="description" content="Test with special chars: &lt;&gt;&amp;&quot;">
          </head>
        </html>
      `

      expect(() => {
        const result = extractMeta(html)
        JSON.parse(result)
      }).not.toThrow()
    })

    it('should handle very large HTML documents', () => {
      let html = '<html><head>'
      for (let i = 0; i < 1000; i++) {
        html += `<meta name="test${i}" content="value${i}">`
      }
      html += '</head></html>'

      expect(() => {
        extractMeta(html)
      }).not.toThrow()
    })

    it('should handle HTML with unicode characters', () => {
      const html = `
        <html>
          <head>
            <meta name="description" content="Description with unicode: 你好世界 🌍">
          </head>
        </html>
      `

      expect(() => {
        const result = extractMeta(html)
        JSON.parse(result)
      }).not.toThrow()
    })

    it('should handle empty HTML string', () => {
      const html = ''

      expect(() => {
        extractAll(html)
      }).not.toThrow()
    })

    it('should handle HTML with nested scripts', () => {
      const html = `
        <html>
          <head>
            <script type="application/ld+json">
            {
              "@context": "https://schema.org",
              "description": "Test with <nested> tags"
            }
            </script>
          </head>
        </html>
      `

      expect(() => {
        extractAll(html)
      }).not.toThrow()
    })
  })

  describe('Integration tests', () => {
    it('should extract consistently across functions', () => {
      const html = `
        <html>
          <head>
            <meta name="description" content="Test">
            <meta property="og:title" content="OG Title">
            <meta name="twitter:card" content="summary">
          </head>
        </html>
      `

      const all = JSON.parse(extractAll(html))
      const meta = JSON.parse(extractMeta(html))
      const og = JSON.parse(extractOpengraph(html))
      const twitter = JSON.parse(extractTwitter(html))

      expect(all.meta).toBeDefined()
      expect(all.opengraph).toBeDefined()
      expect(all.twitter).toBeDefined()
      expect(meta.description).toBeDefined()
      expect(og.title).toBeDefined()
      expect(twitter.card).toBeDefined()
    })

    it('should handle real-world HTML from blogs', () => {
      const html = `
        <!DOCTYPE html>
        <html lang="en">
          <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>My Blog Post</title>
            <meta name="description" content="An interesting blog post about web development">
            <meta property="og:type" content="article">
            <meta property="og:title" content="My Blog Post">
            <meta property="og:description" content="An interesting blog post about web development">
            <meta property="og:image" content="https://example.com/images/post.jpg">
            <meta name="twitter:card" content="summary_large_image">
            <meta name="twitter:title" content="My Blog Post">
          </head>
          <body>
            <article>
              <h1>My Blog Post</h1>
            </article>
          </body>
        </html>
      `

      expect(() => {
        JSON.parse(extractAll(html))
        JSON.parse(extractMeta(html))
        JSON.parse(extractOpengraph(html))
        JSON.parse(extractTwitter(html))
      }).not.toThrow()
    })

    it('should handle real-world HTML from e-commerce sites', () => {
      const html = `
        <!DOCTYPE html>
        <html>
          <head>
            <title>Product Page</title>
            <meta name="description" content="Premium Product">
            <meta property="og:type" content="product">
            <meta property="og:title" content="Premium Product">
            <meta property="og:price:amount" content="99.99">
            <meta property="og:price:currency" content="USD">
            <meta property="og:image" content="product.jpg">
            <meta name="twitter:card" content="product">
          </head>
          <body>
            <div itemscope itemtype="https://schema.org/Product">
              <h1 itemprop="name">Premium Product</h1>
              <span itemprop="price" content="99.99">$99.99</span>
            </div>
          </body>
        </html>
      `

      const all = JSON.parse(extractAll(html))
      expect(all).toHaveProperty('opengraph')
      expect(all).toHaveProperty('microdata')
    })

    it('should handle real-world JSON-LD structured data', () => {
      const html = `
        <html>
          <head>
            <script type="application/ld+json">
            {
              "@context": "https://schema.org",
              "@type": "NewsArticle",
              "headline": "Article Headline",
              "image": ["image.jpg"],
              "datePublished": "2024-01-01T08:00:00+00:00",
              "author": {
                "@type": "Person",
                "name": "John Doe"
              }
            }
            </script>
          </head>
        </html>
      `

      const result = JSON.parse(extractAll(html))
      expect(result.jsonld).toBeDefined()
    })
  })
})
