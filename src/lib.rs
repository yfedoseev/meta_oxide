// PyO3 macro expansions can trigger false positive clippy warnings
#![allow(clippy::useless_conversion)]

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::{PyDict, PyList};
#[cfg(feature = "python")]
use std::collections::HashMap;

mod errors;
mod extractors;
pub mod ffi;
#[macro_use]
mod macros;
mod parser;
mod types;

pub use errors::{MicroformatError, Result};
pub use types::*;

// Re-export utilities needed by macros (required for macro expansion, not Python-specific)
#[doc(hidden)]
pub use extractors::common::{html_utils, url_utils};

#[cfg(feature = "python")]
/// Extract microformats data from HTML content
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (html, base_url=None))]
fn extract_microformats(
    html: &str,
    base_url: Option<&str>,
) -> PyResult<HashMap<String, Vec<PyObject>>> {
    Python::with_gil(|py| {
        let result = parser::parse_html(html, base_url)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let mut py_result = HashMap::new();

        // Convert Rust data structures to Python objects
        for (format_type, items) in result.iter() {
            let py_items: Vec<PyObject> =
                items.iter().map(|item| item.to_py_dict(py).into()).collect();
            py_result.insert(format_type.clone(), py_items);
        }

        Ok(py_result)
    })
}

#[cfg(feature = "python")]
py_extractor_binding!(extract_hcard, hcard, HCard);

#[cfg(feature = "python")]
py_extractor_binding!(extract_hentry, hentry, HEntry);

#[cfg(feature = "python")]
py_extractor_binding!(extract_hevent, hevent, HEvent);
#[cfg(feature = "python")]
py_extractor_binding!(extract_hreview, hreview, HReview);
#[cfg(feature = "python")]
py_extractor_binding!(extract_hrecipe, hrecipe, HRecipe);

#[cfg(feature = "python")]
py_extractor_binding!(extract_hproduct, hproduct, HProduct);

#[cfg(feature = "python")]
py_extractor_binding!(extract_hfeed, hfeed, HFeed);

#[cfg(feature = "python")]
py_extractor_binding!(extract_hadr, hadr, HAdr);

#[cfg(feature = "python")]
py_extractor_binding!(extract_hgeo, hgeo, HGeo);

#[cfg(feature = "python")]
/// Extract standard HTML meta tags
///
/// Args:
///     html (str): HTML content to extract from
///     base_url (str, optional): Base URL for resolving relative URLs
///
/// Returns:
///     dict: Dictionary containing standard meta tags:
///         - title: Page title
///         - description: Meta description
///         - keywords: List of keywords
///         - canonical: Canonical URL
///         - viewport: Viewport meta tag
///         - charset: Character encoding
///         - language: Page language
///         - robots: Robots directives
///         - alternate: List of alternate links
///         - feeds: List of feed links
///         - and more...
///
/// Example:
///     >>> import meta_oxide
///     >>> meta = meta_oxide.extract_meta(html, "https://example.com")
///     >>> print(meta['title'])
///     >>> print(meta['description'])
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (html, base_url=None))]
fn extract_meta(py: Python, html: &str, base_url: Option<&str>) -> PyResult<Py<PyDict>> {
    let meta = extractors::meta::extract(html, base_url)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(meta.to_py_dict(py))
}

/// Extract Open Graph metadata
///
/// Args:
///     html (str): HTML content to extract from
///     base_url (str, optional): Base URL for resolving relative URLs
///
/// Returns:
///     dict: Dictionary containing Open Graph data used by Facebook, LinkedIn, etc.
///
/// Example:
///     >>> import meta_oxide
///     >>> og = meta_oxide.extract_opengraph(html)
///     >>> print(og['title'])
///     >>> print(og['image'])
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (html, base_url=None))]
fn extract_opengraph(py: Python, html: &str, base_url: Option<&str>) -> PyResult<Py<PyDict>> {
    let og = extractors::social::extract_opengraph(html, base_url)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(og.to_py_dict(py))
}

/// Extract Twitter Card metadata
///
/// Args:
///     html (str): HTML content to extract from
///     base_url (str, optional): Base URL for resolving relative URLs
///
/// Returns:
///     dict: Dictionary containing Twitter Card data
///
/// Example:
///     >>> import meta_oxide
///     >>> card = meta_oxide.extract_twitter(html)
///     >>> print(card['card'])
///     >>> print(card['title'])
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (html, base_url=None))]
fn extract_twitter(py: Python, html: &str, base_url: Option<&str>) -> PyResult<Py<PyDict>> {
    let card = extractors::social::extract_twitter(html, base_url)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(card.to_py_dict(py))
}

/// Extract Twitter Card metadata with Open Graph fallback
///
/// Args:
///     html (str): HTML content to extract from
///     base_url (str, optional): Base URL for resolving relative URLs
///
/// Returns:
///     dict: Dictionary containing Twitter Card data with OG fallback
///
/// Example:
///     >>> import meta_oxide
///     >>> card = meta_oxide.extract_twitter_with_fallback(html)
///     >>> print(card['title'])  # Falls back to og:title if twitter:title missing
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (html, base_url=None))]
fn extract_twitter_with_fallback(
    py: Python,
    html: &str,
    base_url: Option<&str>,
) -> PyResult<Py<PyDict>> {
    let card = extractors::social::extract_twitter_with_fallback(html, base_url)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(card.to_py_dict(py))
}

/// Extract JSON-LD structured data
///
/// Args:
///     html (str): HTML content to extract from
///     base_url (str, optional): Base URL (not used for JSON-LD but included for consistency)
///
/// Returns:
///     list: List of JSON-LD objects (dicts) found in the HTML
///
/// Example:
///     >>> import meta_oxide
///     >>> jsonld = meta_oxide.extract_jsonld(html)
///     >>> for obj in jsonld:
///     ...     print(obj.get('@type'))
///     ...     print(obj.get('headline'))
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (html, base_url=None))]
fn extract_jsonld(py: Python, html: &str, base_url: Option<&str>) -> PyResult<Py<PyList>> {
    let objects = extractors::jsonld::extract(html, base_url)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    let list = PyList::empty_bound(py);
    for obj in objects {
        list.append(obj.to_py_dict(py)).unwrap();
    }
    Ok(list.unbind())
}

/// Extract HTML5 Microdata (Phase 4)
///
/// Extracts microdata using itemscope, itemtype, and itemprop attributes.
///
/// Args:
///     html (str): HTML content to extract from
///     base_url (str, optional): Base URL for resolving relative URLs
///
/// Returns:
///     list: List of microdata items (dicts) found in the HTML
///
/// Example:
///     >>> import meta_oxide
///     >>> microdata = meta_oxide.extract_microdata(html, base_url="https://example.com")
///     >>> for item in microdata:
///     ...     print(item.get('type'))
///     ...     print(item.get('name'))
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (html, base_url=None))]
fn extract_microdata(py: Python, html: &str, base_url: Option<&str>) -> PyResult<Py<PyList>> {
    let items = extractors::microdata::extract(html, base_url)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    let list = PyList::empty_bound(py);
    for item in items {
        list.append(item.to_py_dict(py)).unwrap();
    }
    Ok(list.unbind())
}

/// Extract Dublin Core metadata (Phase 9)
///
/// Extracts Dublin Core metadata elements commonly used in digital libraries and archives.
///
/// Args:
///     html (str): HTML content to extract from
///
/// Returns:
///     dict: Dictionary containing Dublin Core elements (title, creator, subject, etc.)
///
/// Example:
///     >>> import meta_oxide
///     >>> dc = meta_oxide.extract_dublin_core(html)
///     >>> print(dc.get('title'))
///     >>> print(dc.get('creator'))
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (html))]
fn extract_dublin_core(py: Python, html: &str) -> PyResult<Py<PyDict>> {
    let dc = extractors::dublin_core::extract(html)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(dc.to_py_dict(py))
}

/// Extract rel-* link relationships
///
/// Extracts HTML link relationships using the `rel` attribute from both `<link>` and `<a>` tags.
/// Supports common rel types like author, me, webmention, license, payment, etc.
///
/// Args:
///     html (str): HTML content to extract from
///     base_url (str, optional): Base URL for resolving relative URLs
///
/// Returns:
///     dict: Dictionary mapping rel type to list of URLs
///
/// Example:
///     >>> import meta_oxide
///     >>> links = meta_oxide.extract_rel_links(html, "https://example.com")
///     >>> print(links.get('author'))  # ['/about']
///     >>> print(links.get('me'))  # ['https://twitter.com/user', 'https://github.com/user']
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (html, base_url=None))]
fn extract_rel_links(html: &str, base_url: Option<&str>) -> PyResult<HashMap<String, Vec<String>>> {
    let links = extractors::rel_links::extract(html, base_url)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    Ok(links)
}

/// Discover oEmbed endpoints (Phase 5)
///
/// Extracts oEmbed endpoint URLs from HTML link tags. oEmbed is used by
/// platforms like YouTube, Vimeo, Twitter for easy content embedding.
///
/// Args:
///     html (str): HTML content to extract from
///     base_url (str, optional): Base URL for resolving relative URLs
///
/// Returns:
///     dict: Dictionary containing discovered oEmbed endpoints:
///         - json_endpoints: List of JSON oEmbed endpoints
///         - xml_endpoints: List of XML oEmbed endpoints
///
/// Example:
///     >>> import meta_oxide
///     >>> oembed = meta_oxide.extract_oembed(html, "https://example.com")
///     >>> for endpoint in oembed.get('json_endpoints', []):
///     ...     print(endpoint['href'])
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (html, base_url=None))]
fn extract_oembed(py: Python, html: &str, base_url: Option<&str>) -> PyResult<Py<PyDict>> {
    let oembed = extractors::oembed::extract(html, base_url)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(oembed.to_py_dict(py))
}

/// Extract RDFa structured data from HTML
///
/// RDFa (Resource Description Framework in Attributes) is a W3C standard
/// for embedding structured data in HTML using attributes like typeof, property, vocab.
///
/// Args:
///     html (str): HTML content to extract from
///     base_url (str, optional): Base URL for resolving relative URLs
///
/// Returns:
///     list: List of RDFa items extracted from the page
///
/// Example:
///     >>> import meta_oxide
///     >>> items = meta_oxide.extract_rdfa(html)
///     >>> for item in items:
///     ...     print(item.get('type'))
///     ...     print(item.get('properties'))
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (html, base_url=None))]
fn extract_rdfa(py: Python, html: &str, base_url: Option<&str>) -> PyResult<PyObject> {
    let items = extractors::rdfa::extract(html, base_url)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    let list = PyList::empty_bound(py);
    for item in items {
        list.append(item.to_py_dict(py)).unwrap();
    }
    Ok(list.to_object(py))
}

/// Extract Web App Manifest link from HTML
///
/// Finds and resolves the manifest link from <link rel="manifest"> tags.
/// Use parse_manifest() separately to parse the manifest JSON content.
///
/// Args:
///     html (str): HTML content to extract from
///     base_url (str, optional): Base URL for resolving relative URLs
///
/// Returns:
///     dict: Dictionary with 'href' key containing the manifest URL
///
/// Example:
///     >>> import meta_oxide
///     >>> discovery = meta_oxide.extract_manifest(html, "https://example.com")
///     >>> print(discovery['href'])
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (html, base_url=None))]
fn extract_manifest(py: Python, html: &str, base_url: Option<&str>) -> PyResult<Py<PyDict>> {
    let discovery = extractors::manifest::extract(html, base_url)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(discovery.to_py_dict(py))
}

/// Parse Web App Manifest JSON content
///
/// Parses a manifest.json file and resolves all relative URLs.
///
/// Args:
///     json (str): Manifest JSON content
///     base_url (str, optional): Base URL for resolving relative URLs
///
/// Returns:
///     dict: Parsed manifest with all fields
///
/// Example:
///     >>> import meta_oxide
///     >>> manifest = meta_oxide.parse_manifest(json_content, "https://example.com")
///     >>> print(manifest['name'])
///     >>> print(manifest['icons'])
#[cfg(feature = "python")]
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (json, base_url=None))]
fn parse_manifest(py: Python, json: &str, base_url: Option<&str>) -> PyResult<Py<PyDict>> {
    let manifest = extractors::manifest::parse_manifest(json, base_url)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(manifest.to_py_dict(py))
}

/// Extract ALL supported structured data from HTML (Phases 1-4)
///
/// This is the main convenience function that extracts:
/// - Standard HTML meta tags (Phase 1, 100% adoption)
/// - Open Graph metadata (Phase 2, 60%+ adoption)
/// - Twitter Card metadata (Phase 2, 45% adoption)
/// - JSON-LD structured data (Phase 3, 41% adoption) - NEW!
/// - Microformats (Phase 7, already implemented)
///
/// Args:
///     html (str): HTML content to extract from
///     base_url (str, optional): Base URL for resolving relative URLs
///
/// Returns:
///     dict: Dictionary containing all extracted data with keys:
///         - meta: Standard HTML meta tags (title, description, etc.)
///         - opengraph: Open Graph Protocol data
///         - twitter: Twitter Card data
///         - jsonld: JSON-LD / Schema.org structured data (list of objects)
///         - microformats: Microformats data (h-card, h-entry, h-event)
///         - rel_links: HTML link relationships (rel-author, rel-me, etc.)
///
/// Example:
///     >>> import meta_oxide
///     >>> data = meta_oxide.extract_all(html, "https://example.com")
///     >>> print(data['meta']['title'])
///     >>> print(data['opengraph']['image'])
///     >>> print(data['twitter']['card'])
///     >>> for obj in data.get('jsonld', []):
///     ...     print(obj.get('@type'))
#[cfg(feature = "python")]
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (html, base_url=None))]
fn extract_all(py: Python, html: &str, base_url: Option<&str>) -> PyResult<Py<PyDict>> {
    let dict = PyDict::new_bound(py);

    // Extract Phase 1: Standard Meta Tags
    match extractors::meta::extract(html, base_url) {
        Ok(meta_tags) => {
            dict.set_item("meta", meta_tags.to_py_dict(py))?;
        }
        Err(e) => {
            // Log error but continue with other extractors
            eprintln!("Meta extraction warning: {}", e);
        }
    }

    // Extract Phase 2: Open Graph
    match extractors::social::extract_opengraph(html, base_url) {
        Ok(og) => {
            dict.set_item("opengraph", og.to_py_dict(py))?;
        }
        Err(e) => {
            eprintln!("OpenGraph extraction warning: {}", e);
        }
    }

    // Extract Phase 2: Twitter Cards (with fallback to OG)
    match extractors::social::extract_twitter_with_fallback(html, base_url) {
        Ok(twitter) => {
            dict.set_item("twitter", twitter.to_py_dict(py))?;
        }
        Err(e) => {
            eprintln!("Twitter extraction warning: {}", e);
        }
    }

    // Extract Phase 3: JSON-LD (41% adoption, HIGHEST IMPACT)
    match extractors::jsonld::extract(html, base_url) {
        Ok(objects) => {
            if !objects.is_empty() {
                let list = PyList::empty_bound(py);
                for obj in objects {
                    list.append(obj.to_py_dict(py)).unwrap();
                }
                dict.set_item("jsonld", list)?;
            }
        }
        Err(e) => {
            eprintln!("JSON-LD extraction warning: {}", e);
        }
    }

    // Extract Phase 4: Microdata (26% adoption)
    match extractors::microdata::extract(html, base_url) {
        Ok(items) => {
            if !items.is_empty() {
                let list = PyList::empty_bound(py);
                for item in items {
                    list.append(item.to_py_dict(py)).unwrap();
                }
                dict.set_item("microdata", list)?;
            }
        }
        Err(e) => {
            eprintln!("Microdata extraction warning: {}", e);
        }
    }

    // Extract Phase 7: Microformats (already implemented)
    let mf_dict = PyDict::new_bound(py);
    let mut has_microformats = false;

    // Extract h-card
    if let Ok(hcards) = extractors::microformats::hcard::extract(html, base_url) {
        if !hcards.is_empty() {
            let cards: Vec<_> = hcards.iter().map(|card| card.to_py_dict(py).into_py(py)).collect();
            mf_dict.set_item("h-card", cards)?;
            has_microformats = true;
        }
    }

    // Extract h-entry
    if let Ok(entries) = extractors::microformats::hentry::extract(html, base_url) {
        if !entries.is_empty() {
            let entries_py: Vec<_> = entries.iter().map(|e| e.to_py_dict(py).into_py(py)).collect();
            mf_dict.set_item("h-entry", entries_py)?;
            has_microformats = true;
        }
    }

    // Extract h-event
    if let Ok(events) = extractors::microformats::hevent::extract(html, base_url) {
        if !events.is_empty() {
            let events_py: Vec<_> = events.iter().map(|e| e.to_py_dict(py).into_py(py)).collect();
            mf_dict.set_item("h-event", events_py)?;
            has_microformats = true;
        }
    }

    // Extract h-review
    if let Ok(reviews) = extractors::microformats::hreview::extract(html, base_url) {
        if !reviews.is_empty() {
            let reviews_py: Vec<_> = reviews.iter().map(|r| r.to_py_dict(py).into_py(py)).collect();
            mf_dict.set_item("h-review", reviews_py)?;
            has_microformats = true;
        }
    }

    // Extract h-recipe
    if let Ok(recipes) = extractors::microformats::hrecipe::extract(html, base_url) {
        if !recipes.is_empty() {
            let recipes_py: Vec<_> = recipes.iter().map(|r| r.to_py_dict(py).into_py(py)).collect();
            mf_dict.set_item("h-recipe", recipes_py)?;
            has_microformats = true;
        }
    }

    // Extract h-product
    if let Ok(products) = extractors::microformats::hproduct::extract(html, base_url) {
        if !products.is_empty() {
            let products_py: Vec<_> =
                products.iter().map(|p| p.to_py_dict(py).into_py(py)).collect();
            mf_dict.set_item("h-product", products_py)?;
            has_microformats = true;
        }
    }

    // Extract h-feed
    if let Ok(feeds) = extractors::microformats::hfeed::extract(html, base_url) {
        if !feeds.is_empty() {
            let feeds_py: Vec<_> = feeds.iter().map(|f| f.to_py_dict(py).into_py(py)).collect();
            mf_dict.set_item("h-feed", feeds_py)?;
            has_microformats = true;
        }
    }

    // Extract h-adr
    if let Ok(addresses) = extractors::microformats::hadr::extract(html, base_url) {
        if !addresses.is_empty() {
            let addresses_py: Vec<_> =
                addresses.iter().map(|a| a.to_py_dict(py).into_py(py)).collect();
            mf_dict.set_item("h-adr", addresses_py)?;
            has_microformats = true;
        }
    }

    // Extract h-geo
    if let Ok(geos) = extractors::microformats::hgeo::extract(html, base_url) {
        if !geos.is_empty() {
            let geos_py: Vec<_> = geos.iter().map(|g| g.to_py_dict(py).into_py(py)).collect();
            mf_dict.set_item("h-geo", geos_py)?;
            has_microformats = true;
        }
    }

    if has_microformats {
        dict.set_item("microformats", mf_dict)?;
    }

    // Extract Phase 5: oEmbed endpoint discovery
    match extractors::oembed::extract(html, base_url) {
        Ok(oembed) => {
            if oembed.has_endpoints() {
                dict.set_item("oembed", oembed.to_py_dict(py))?;
            }
        }
        Err(e) => {
            eprintln!("oEmbed extraction warning: {}", e);
        }
    }

    // Extract Phase 9: Dublin Core metadata
    match extractors::dublin_core::extract(html) {
        Ok(dc) => {
            dict.set_item("dublin_core", dc.to_py_dict(py))?;
        }
        Err(e) => {
            eprintln!("Dublin Core extraction warning: {}", e);
        }
    }

    // Extract rel-* link relationships
    match extractors::rel_links::extract(html, base_url) {
        Ok(rel_links) => {
            if !rel_links.is_empty() {
                dict.set_item("rel_links", rel_links)?;
            }
        }
        Err(e) => {
            eprintln!("rel_links extraction warning: {}", e);
        }
    }

    // Extract RDFa (W3C standard with 62% adoption)
    match extractors::rdfa::extract(html, base_url) {
        Ok(rdfa_items) => {
            if !rdfa_items.is_empty() {
                let list = PyList::empty_bound(py);
                for item in rdfa_items {
                    list.append(item.to_py_dict(py)).unwrap();
                }
                dict.set_item("rdfa", list)?;
            }
        }
        Err(e) => {
            eprintln!("RDFa extraction warning: {}", e);
        }
    }

    // Extract Web App Manifest link
    match extractors::manifest::extract(html, base_url) {
        Ok(discovery) => {
            if discovery.href.is_some() {
                dict.set_item("manifest", discovery.to_py_dict(py))?;
            }
        }
        Err(e) => {
            eprintln!("Manifest extraction warning: {}", e);
        }
    }

    Ok(dict.unbind())
}

#[cfg(feature = "python")]
/// MetaOxide: A fast Rust library for extracting structured data
#[pymodule]
fn meta_oxide(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Phase 1: Standard Meta
    m.add_function(wrap_pyfunction!(extract_meta, m)?)?;

    // Phase 2: Social Media
    m.add_function(wrap_pyfunction!(extract_opengraph, m)?)?;
    m.add_function(wrap_pyfunction!(extract_twitter, m)?)?;
    m.add_function(wrap_pyfunction!(extract_twitter_with_fallback, m)?)?;

    // Phase 3: JSON-LD
    m.add_function(wrap_pyfunction!(extract_jsonld, m)?)?;

    // Phase 4: Microdata
    m.add_function(wrap_pyfunction!(extract_microdata, m)?)?;

    // Phase 5: oEmbed
    m.add_function(wrap_pyfunction!(extract_oembed, m)?)?;

    // Phase 9: Dublin Core
    m.add_function(wrap_pyfunction!(extract_dublin_core, m)?)?;

    // RDFa
    m.add_function(wrap_pyfunction!(extract_rdfa, m)?)?;

    // Web App Manifest
    m.add_function(wrap_pyfunction!(extract_manifest, m)?)?;
    m.add_function(wrap_pyfunction!(parse_manifest, m)?)?;

    // rel-* link relationships
    m.add_function(wrap_pyfunction!(extract_rel_links, m)?)?;

    // Phase 7: Microformats
    m.add_function(wrap_pyfunction!(extract_microformats, m)?)?;
    m.add_function(wrap_pyfunction!(extract_hcard, m)?)?;
    m.add_function(wrap_pyfunction!(extract_hentry, m)?)?;
    m.add_function(wrap_pyfunction!(extract_hevent, m)?)?;
    m.add_function(wrap_pyfunction!(extract_hreview, m)?)?;
    m.add_function(wrap_pyfunction!(extract_hrecipe, m)?)?;
    m.add_function(wrap_pyfunction!(extract_hproduct, m)?)?;
    m.add_function(wrap_pyfunction!(extract_hfeed, m)?)?;
    m.add_function(wrap_pyfunction!(extract_hadr, m)?)?;
    m.add_function(wrap_pyfunction!(extract_hgeo, m)?)?;

    // Main convenience function
    m.add_function(wrap_pyfunction!(extract_all, m)?)?;

    // Add version
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    #[cfg(feature = "python")]
    fn test_extract_all_basic() {
        Python::with_gil(|py| {
            let html = r#"
            <html>
                <head>
                    <title>Test Page</title>
                    <meta name="description" content="Test description">
                </head>
            </html>
            "#;
            let result = extract_all(py, html, None);
            assert!(result.is_ok());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_extract_all_with_opengraph() {
        Python::with_gil(|py| {
            let html = r#"
            <html>
                <head>
                    <title>Test</title>
                    <meta property="og:title" content="OG Title">
                </head>
            </html>
            "#;
            let result = extract_all(py, html, None);
            assert!(result.is_ok());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_extract_all_with_jsonld() {
        Python::with_gil(|py| {
            let html = r#"
            <html>
                <head>
                    <script type="application/ld+json">
                    {"@type": "Article", "headline": "Test"}
                    </script>
                </head>
            </html>
            "#;
            let result = extract_all(py, html, None);
            assert!(result.is_ok());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_extract_all_with_microdata() {
        Python::with_gil(|py| {
            let html = r#"
            <html>
                <body>
                    <div itemscope itemtype="https://schema.org/Person">
                        <span itemprop="name">John</span>
                    </div>
                </body>
            </html>
            "#;
            let result = extract_all(py, html, None);
            assert!(result.is_ok());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_extract_all_with_microformats() {
        Python::with_gil(|py| {
            let html = r#"
            <div class="h-card">
                <span class="p-name">Jane</span>
            </div>
            "#;
            let result = extract_all(py, html, None);
            assert!(result.is_ok());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_extract_all_with_base_url() {
        Python::with_gil(|py| {
            let html = r#"
            <html>
                <head>
                    <link rel="canonical" href="/page">
                </head>
            </html>
            "#;
            let result = extract_all(py, html, Some("https://example.com"));
            assert!(result.is_ok());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_extract_all_comprehensive() {
        Python::with_gil(|py| {
            let html = r#"
            <html>
                <head>
                    <title>Comprehensive Test</title>
                    <meta name="description" content="Test description">
                    <meta property="og:title" content="OG Title">
                    <meta name="twitter:card" content="summary">
                    <script type="application/ld+json">
                    {"@type": "Article", "headline": "Test Article"}
                    </script>
                    <link rel="canonical" href="https://example.com/page">
                </head>
                <body>
                    <div class="h-card">
                        <span class="p-name">John Doe</span>
                    </div>
                    <div itemscope itemtype="https://schema.org/Person">
                        <span itemprop="name">Jane Doe</span>
                    </div>
                </body>
            </html>
            "#;
            let result = extract_all(py, html, Some("https://example.com"));
            assert!(result.is_ok());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_extract_all_empty_html() {
        Python::with_gil(|py| {
            let html = "<html><head></head></html>";
            let result = extract_all(py, html, None);
            assert!(result.is_ok());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_extract_all_with_malformed_jsonld() {
        Python::with_gil(|py| {
            let html = r#"
            <html>
                <head>
                    <title>Test</title>
                    <script type="application/ld+json">
                    {BROKEN JSON}
                    </script>
                </head>
            </html>
            "#;
            let result = extract_all(py, html, None);
            // Should still succeed, just with no JSON-LD
            assert!(result.is_ok());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_extract_all_with_dublin_core() {
        Python::with_gil(|py| {
            let html = r#"
            <html>
                <head>
                    <meta name="DC.title" content="Document Title">
                </head>
            </html>
            "#;
            let result = extract_all(py, html, None);
            assert!(result.is_ok());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_extract_all_with_oembed() {
        Python::with_gil(|py| {
            let html = r#"
            <html>
                <head>
                    <link rel="alternate" type="application/json+oembed"
                          href="https://example.com/oembed">
                </head>
            </html>
            "#;
            let result = extract_all(py, html, None);
            assert!(result.is_ok());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_extract_all_with_rel_links() {
        Python::with_gil(|py| {
            let html = r#"
            <html>
                <head>
                    <link rel="canonical" href="https://example.com/page">
                    <link rel="alternate" href="/page-de" hreflang="de">
                </head>
            </html>
            "#;
            let result = extract_all(py, html, Some("https://example.com"));
            assert!(result.is_ok());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_extract_all_multiple_formats_overlap() {
        Python::with_gil(|py| {
            // News article with both OG and JSON-LD
            let html = r#"
            <html>
                <head>
                    <title>Breaking News</title>
                    <meta name="description" content="News article">
                    <meta property="og:title" content="Breaking News">
                    <meta property="og:description" content="News article">
                    <meta property="og:type" content="article">
                    <script type="application/ld+json">
                    {
                        "@type": "NewsArticle",
                        "headline": "Breaking News",
                        "description": "News article"
                    }
                    </script>
                </head>
            </html>
            "#;
            let result = extract_all(py, html, None);
            assert!(result.is_ok());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_extract_all_unicode_content() {
        Python::with_gil(|py| {
            let html = r#"
            <html>
                <head>
                    <title>测试页面 - テスト</title>
                    <meta name="description" content="日本語と中文の説明">
                </head>
            </html>
            "#;
            let result = extract_all(py, html, None);
            assert!(result.is_ok());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_extract_all_html_entities() {
        Python::with_gil(|py| {
            let html = r#"
            <html>
                <head>
                    <title>Test &amp; Demo &lt;Page&gt;</title>
                    <meta name="description" content="&quot;Quoted&quot; content">
                </head>
            </html>
            "#;
            let result = extract_all(py, html, None);
            assert!(result.is_ok());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_extract_all_with_comments() {
        Python::with_gil(|py| {
            let html = r#"
            <html>
                <head>
                    <!-- <meta name="fake" content="content"> -->
                    <title>Real Title</title>
                    <!-- <meta property="og:title" content="fake"> -->
                </head>
            </html>
            "#;
            let result = extract_all(py, html, None);
            assert!(result.is_ok());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_extract_all_deeply_nested() {
        Python::with_gil(|py| {
            let mut html = String::from("<html><body>");
            for _ in 0..50 {
                html.push_str("<div>");
            }
            html.push_str("<span class=\"h-card\"><span class=\"p-name\">Test</span></span>");
            for _ in 0..50 {
                html.push_str("</div>");
            }
            html.push_str("</body></html>");

            let result = extract_all(py, &html, None);
            assert!(result.is_ok());
        });
    }

    #[test]
    #[cfg(feature = "python")]
    fn test_extract_all_many_items() {
        Python::with_gil(|py| {
            let mut html = String::from("<html><body>");
            for i in 0..100 {
                html.push_str(&format!(
                    r#"<div class="h-card"><span class="p-name">Person {}</span></div>"#,
                    i
                ));
            }
            html.push_str("</body></html>");

            let result = extract_all(py, &html, None);
            assert!(result.is_ok());
        });
    }

    #[test]
    fn test_extract_each_format_separately() {
        let html = r#"
        <html>
            <head>
                <title>Test</title>
                <meta property="og:title" content="OG">
                <script type="application/ld+json">
                {"@type": "Article"}
                </script>
            </head>
        </html>
        "#;

        // Test each extractor individually
        let _ = extractors::meta::extract(html, None);
        let _ = extractors::social::extract_opengraph(html, None);
        let _ = extractors::social::extract_twitter(html, None);
        let _ = extractors::jsonld::extract(html, None);
        let _ = extractors::oembed::extract(html, None);
        let _ = extractors::dublin_core::extract(html);
        let _ = extractors::rel_links::extract(html, None);
    }
}
