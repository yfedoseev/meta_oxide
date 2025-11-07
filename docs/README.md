# MetaOxide Documentation

Welcome to the MetaOxide documentation! This directory contains comprehensive guides for using and developing MetaOxide.

## Documentation Index

### For Users

1. **[Getting Started](getting-started.md)**
   - Installation instructions
   - Basic usage examples
   - Prerequisites and setup
   - Quick start guide

2. **[API Reference](api-reference.md)**
   - Complete Python API documentation
   - Complete Rust API documentation
   - Function signatures and parameters
   - Return types and error handling
   - Microformat class reference

3. **[Examples](examples.md)**
   - Practical usage examples
   - Real-world use cases
   - Python and Rust code samples
   - Advanced patterns and techniques

### For Developers

4. **[Architecture](architecture.md)**
   - System design and components
   - Data flow and processing
   - Design decisions and rationale
   - Performance considerations

5. **[Development Guide](development.md)**
   - Setting up development environment
   - Building and testing
   - Adding new microformat types
   - Code style and conventions
   - Contributing guidelines
   - Release process

## Quick Navigation

### I want to...

- **Install and use MetaOxide** → Start with [Getting Started](getting-started.md)
- **See code examples** → Check out [Examples](examples.md)
- **Learn about a specific function** → Read the [API Reference](api-reference.md)
- **Understand how it works** → See the [Architecture](architecture.md)
- **Contribute to the project** → Follow the [Development Guide](development.md)

## What are Microformats?

Microformats are simple conventions for embedding structured data in HTML. They allow you to mark up people, events, blog posts, and more in a way that's both human-readable and machine-parsable.

Example:
```html
<div class="h-card">
    <span class="p-name">Jane Doe</span>
    <a class="u-email" href="mailto:jane@example.com">Email</a>
</div>
```

This simple HTML contains structured information about a person that can be automatically extracted by MetaOxide.

## Supported Microformats

MetaOxide currently supports:

- **h-card**: Contact information (people, organizations)
- **h-entry**: Blog posts, articles, notes
- **h-event**: Events with dates, times, and locations

More formats are planned for future releases.

## Documentation by Experience Level

### Beginner
1. Read [Getting Started](getting-started.md) - Installation and basic usage
2. Try the examples in [Examples](examples.md) - See it in action
3. Reference the [API Reference](api-reference.md) as needed

### Intermediate
1. Explore [Examples](examples.md) - Advanced patterns
2. Understand the [Architecture](architecture.md) - How it works
3. Use the [API Reference](api-reference.md) - Full capabilities

### Advanced / Contributors
1. Study the [Architecture](architecture.md) - Design philosophy
2. Follow the [Development Guide](development.md) - Set up for development
3. Reference [API Reference](api-reference.md) - Implementation details

## External Resources

- [Microformats.org](http://microformats.org/) - Official microformats specification
- [PyO3 Documentation](https://pyo3.rs/) - Python bindings for Rust
- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [IndieWeb](https://indieweb.org/) - Community using microformats

## Getting Help

If you can't find what you're looking for in the documentation:

1. Check the [examples](examples.md) - Many common questions are answered there
2. Search [existing issues](https://github.com/yourusername/meta_oxide/issues)
3. Open a [new issue](https://github.com/yourusername/meta_oxide/issues/new)
4. Join community discussions

## Contributing to Documentation

Found an error or want to improve the docs? Contributions are welcome!

1. Fork the repository
2. Edit the relevant markdown file in `docs/`
3. Submit a pull request

See the [Development Guide](development.md) for more details on contributing.

---

**Happy parsing!** 🦀🐍
