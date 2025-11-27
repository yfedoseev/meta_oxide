//! C-ABI Foreign Function Interface
//!
//! This module provides a complete C-compatible API for MetaOxide, enabling
//! language bindings for Go, Node.js, Java, C#, WebAssembly, and any other
//! language with C FFI support.
//!
//! # Memory Management
//!
//! All strings and structs returned by FFI functions are allocated on the heap
//! and must be freed by the caller using the appropriate `*_free()` functions.
//!
//! # Error Handling
//!
//! Functions return NULL on error and set the thread-local error state.
//! Use `meta_oxide_last_error()` and `meta_oxide_error_message()` to retrieve
//! error information.
//!
//! # Thread Safety
//!
//! All functions are stateless and thread-safe. Error state is thread-local.

use std::cell::Cell;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

use crate::extractors;
use crate::parser;

/// Error codes returned by FFI functions
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MetaOxideError {
    /// No error occurred
    #[default]
    Ok = 0,
    /// HTML parsing error
    ParseError = 1,
    /// Invalid URL format
    InvalidUrl = 2,
    /// Invalid UTF-8 string
    InvalidUtf8 = 3,
    /// Memory allocation error
    MemoryError = 4,
    /// JSON serialization error
    JsonError = 5,
    /// NULL pointer passed as argument
    NullPointer = 6,
}

// Thread-local storage for the last error that occurred
thread_local! {
    static LAST_ERROR: Cell<(MetaOxideError, Option<String>)> =
        Cell::new((MetaOxideError::Ok, None));
}

/// Set the last error with an error code and optional message
fn set_last_error(error: MetaOxideError, message: Option<String>) {
    LAST_ERROR.with(|cell| {
        cell.set((error, message));
    });
}

/// Clear the last error
fn clear_last_error() {
    set_last_error(MetaOxideError::Ok, None);
}

/// Result structure containing all extracted metadata
///
/// Each field is a JSON string or NULL if no data was found.
/// The caller must free this struct using `meta_oxide_result_free()`.
#[repr(C)]
pub struct MetaOxideResult {
    /// Standard HTML meta tags (JSON object)
    pub meta: *mut c_char,
    /// Open Graph metadata (JSON object)
    pub open_graph: *mut c_char,
    /// Twitter Card metadata (JSON object)
    pub twitter: *mut c_char,
    /// JSON-LD structured data (JSON array)
    pub json_ld: *mut c_char,
    /// Microdata items (JSON array)
    pub microdata: *mut c_char,
    /// Microformats data (JSON object with h-card, h-entry, etc.)
    pub microformats: *mut c_char,
    /// RDFa structured data (JSON array)
    pub rdfa: *mut c_char,
    /// Dublin Core metadata (JSON object)
    pub dublin_core: *mut c_char,
    /// Web App Manifest discovery (JSON object)
    pub manifest: *mut c_char,
    /// oEmbed endpoint discovery (JSON object)
    pub oembed: *mut c_char,
    /// rel-* link relationships (JSON object)
    pub rel_links: *mut c_char,
}

/// Manifest discovery result with URL and parsed content
#[repr(C)]
pub struct ManifestDiscovery {
    /// Manifest URL (may be NULL)
    pub href: *mut c_char,
    /// Full manifest JSON (may be NULL)
    pub manifest: *mut c_char,
}

// Helper function to convert Rust string to owned C string
fn to_c_string(s: String) -> *mut c_char {
    match CString::new(s) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => {
            set_last_error(
                MetaOxideError::InvalidUtf8,
                Some("String contains null byte".to_string()),
            );
            ptr::null_mut()
        }
    }
}

// Helper function to safely convert C string to Rust &str
unsafe fn from_c_string<'a>(s: *const c_char) -> Result<&'a str, MetaOxideError> {
    if s.is_null() {
        set_last_error(
            MetaOxideError::NullPointer,
            Some("NULL pointer passed as argument".to_string()),
        );
        return Err(MetaOxideError::NullPointer);
    }

    CStr::from_ptr(s).to_str().map_err(|_| {
        set_last_error(
            MetaOxideError::InvalidUtf8,
            Some("Invalid UTF-8 in input string".to_string()),
        );
        MetaOxideError::InvalidUtf8
    })
}

// Helper function to convert optional C string
unsafe fn from_c_string_opt<'a>(s: *const c_char) -> Option<&'a str> {
    if s.is_null() {
        None
    } else {
        CStr::from_ptr(s).to_str().ok()
    }
}

// Helper to serialize to JSON and return C string
fn to_json_c_string<T: serde::Serialize>(value: &T) -> *mut c_char {
    match serde_json::to_string(value) {
        Ok(json) => to_c_string(json),
        Err(_) => {
            set_last_error(
                MetaOxideError::JsonError,
                Some("Failed to serialize to JSON".to_string()),
            );
            ptr::null_mut()
        }
    }
}

/// Extract ALL metadata from HTML
///
/// Returns a MetaOxideResult containing all extracted data as JSON strings.
/// Returns NULL on error.
///
/// # Arguments
/// * `html` - HTML content (must not be NULL)
/// * `base_url` - Base URL for resolving relative URLs (may be NULL)
///
/// # Memory
/// The caller must free the returned struct using `meta_oxide_result_free()`.
///
/// # Safety
/// - `html` must be a valid null-terminated C string
/// - `base_url` may be NULL or a valid null-terminated C string
#[no_mangle]
pub unsafe extern "C" fn meta_oxide_extract_all(
    html: *const c_char,
    base_url: *const c_char,
) -> *mut MetaOxideResult {
    clear_last_error();

    let html_str = match from_c_string(html) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let base_url_str = from_c_string_opt(base_url);

    // Allocate result structure
    let result = Box::new(MetaOxideResult {
        meta: ptr::null_mut(),
        open_graph: ptr::null_mut(),
        twitter: ptr::null_mut(),
        json_ld: ptr::null_mut(),
        microdata: ptr::null_mut(),
        microformats: ptr::null_mut(),
        rdfa: ptr::null_mut(),
        dublin_core: ptr::null_mut(),
        manifest: ptr::null_mut(),
        oembed: ptr::null_mut(),
        rel_links: ptr::null_mut(),
    });

    let result = Box::into_raw(result);

    // Extract meta tags
    if let Ok(meta) = extractors::meta::extract(html_str, base_url_str) {
        (*result).meta = to_json_c_string(&meta);
    }

    // Extract Open Graph
    if let Ok(og) = extractors::social::extract_opengraph(html_str, base_url_str) {
        (*result).open_graph = to_json_c_string(&og);
    }

    // Extract Twitter Cards
    if let Ok(twitter) = extractors::social::extract_twitter_with_fallback(html_str, base_url_str) {
        (*result).twitter = to_json_c_string(&twitter);
    }

    // Extract JSON-LD
    if let Ok(json_ld) = extractors::jsonld::extract(html_str, base_url_str) {
        if !json_ld.is_empty() {
            (*result).json_ld = to_json_c_string(&json_ld);
        }
    }

    // Extract Microdata
    if let Ok(microdata) = extractors::microdata::extract(html_str, base_url_str) {
        if !microdata.is_empty() {
            (*result).microdata = to_json_c_string(&microdata);
        }
    }

    // Extract Microformats (all 9 types)
    if let Ok(microformats) = parser::parse_html(html_str, base_url_str) {
        if !microformats.is_empty() {
            (*result).microformats = to_json_c_string(&microformats);
        }
    }

    // Extract RDFa
    if let Ok(rdfa) = extractors::rdfa::extract(html_str, base_url_str) {
        if !rdfa.is_empty() {
            (*result).rdfa = to_json_c_string(&rdfa);
        }
    }

    // Extract Dublin Core
    if let Ok(dc) = extractors::dublin_core::extract(html_str) {
        (*result).dublin_core = to_json_c_string(&dc);
    }

    // Extract Web App Manifest
    if let Ok(manifest) = extractors::manifest::extract(html_str, base_url_str) {
        if manifest.href.is_some() {
            (*result).manifest = to_json_c_string(&manifest);
        }
    }

    // Extract oEmbed
    if let Ok(oembed) = extractors::oembed::extract(html_str, base_url_str) {
        if oembed.has_endpoints() {
            (*result).oembed = to_json_c_string(&oembed);
        }
    }

    // Extract rel-* links
    if let Ok(rel_links) = extractors::rel_links::extract(html_str, base_url_str) {
        if !rel_links.is_empty() {
            (*result).rel_links = to_json_c_string(&rel_links);
        }
    }

    result
}

/// Extract standard HTML meta tags
///
/// # Returns
/// JSON string or NULL on error
#[no_mangle]
pub unsafe extern "C" fn meta_oxide_extract_meta(
    html: *const c_char,
    base_url: *const c_char,
) -> *mut c_char {
    clear_last_error();

    let html_str = match from_c_string(html) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    let base_url_str = from_c_string_opt(base_url);

    match extractors::meta::extract(html_str, base_url_str) {
        Ok(meta) => to_json_c_string(&meta),
        Err(e) => {
            set_last_error(MetaOxideError::ParseError, Some(e.to_string()));
            ptr::null_mut()
        }
    }
}

/// Extract Open Graph metadata
///
/// # Returns
/// JSON string or NULL on error
#[no_mangle]
pub unsafe extern "C" fn meta_oxide_extract_open_graph(
    html: *const c_char,
    base_url: *const c_char,
) -> *mut c_char {
    clear_last_error();

    let html_str = match from_c_string(html) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    let base_url_str = from_c_string_opt(base_url);

    match extractors::social::extract_opengraph(html_str, base_url_str) {
        Ok(og) => to_json_c_string(&og),
        Err(e) => {
            set_last_error(MetaOxideError::ParseError, Some(e.to_string()));
            ptr::null_mut()
        }
    }
}

/// Extract Twitter Card metadata
///
/// # Returns
/// JSON string or NULL on error
#[no_mangle]
pub unsafe extern "C" fn meta_oxide_extract_twitter(
    html: *const c_char,
    base_url: *const c_char,
) -> *mut c_char {
    clear_last_error();

    let html_str = match from_c_string(html) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    let base_url_str = from_c_string_opt(base_url);

    match extractors::social::extract_twitter_with_fallback(html_str, base_url_str) {
        Ok(twitter) => to_json_c_string(&twitter),
        Err(e) => {
            set_last_error(MetaOxideError::ParseError, Some(e.to_string()));
            ptr::null_mut()
        }
    }
}

/// Extract JSON-LD structured data
///
/// # Returns
/// JSON array string or NULL on error
#[no_mangle]
pub unsafe extern "C" fn meta_oxide_extract_json_ld(
    html: *const c_char,
    base_url: *const c_char,
) -> *mut c_char {
    clear_last_error();

    let html_str = match from_c_string(html) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    let base_url_str = from_c_string_opt(base_url);

    match extractors::jsonld::extract(html_str, base_url_str) {
        Ok(items) => to_json_c_string(&items),
        Err(e) => {
            set_last_error(MetaOxideError::ParseError, Some(e.to_string()));
            ptr::null_mut()
        }
    }
}

/// Extract Microdata
///
/// # Returns
/// JSON array string or NULL on error
#[no_mangle]
pub unsafe extern "C" fn meta_oxide_extract_microdata(
    html: *const c_char,
    base_url: *const c_char,
) -> *mut c_char {
    clear_last_error();

    let html_str = match from_c_string(html) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    let base_url_str = from_c_string_opt(base_url);

    match extractors::microdata::extract(html_str, base_url_str) {
        Ok(items) => to_json_c_string(&items),
        Err(e) => {
            set_last_error(MetaOxideError::ParseError, Some(e.to_string()));
            ptr::null_mut()
        }
    }
}

/// Extract Microformats (all 9 types: h-card, h-entry, h-event, etc.)
///
/// # Returns
/// JSON object string with format types as keys, or NULL on error
#[no_mangle]
pub unsafe extern "C" fn meta_oxide_extract_microformats(
    html: *const c_char,
    base_url: *const c_char,
) -> *mut c_char {
    clear_last_error();

    let html_str = match from_c_string(html) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    let base_url_str = from_c_string_opt(base_url);

    match parser::parse_html(html_str, base_url_str) {
        Ok(microformats) => to_json_c_string(&microformats),
        Err(e) => {
            set_last_error(MetaOxideError::ParseError, Some(e.to_string()));
            ptr::null_mut()
        }
    }
}

/// Extract RDFa structured data
///
/// # Returns
/// JSON array string or NULL on error
#[no_mangle]
pub unsafe extern "C" fn meta_oxide_extract_rdfa(
    html: *const c_char,
    base_url: *const c_char,
) -> *mut c_char {
    clear_last_error();

    let html_str = match from_c_string(html) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    let base_url_str = from_c_string_opt(base_url);

    match extractors::rdfa::extract(html_str, base_url_str) {
        Ok(items) => to_json_c_string(&items),
        Err(e) => {
            set_last_error(MetaOxideError::ParseError, Some(e.to_string()));
            ptr::null_mut()
        }
    }
}

/// Extract Dublin Core metadata
///
/// # Returns
/// JSON object string or NULL on error
#[no_mangle]
pub unsafe extern "C" fn meta_oxide_extract_dublin_core(html: *const c_char) -> *mut c_char {
    clear_last_error();

    let html_str = match from_c_string(html) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    match extractors::dublin_core::extract(html_str) {
        Ok(dc) => to_json_c_string(&dc),
        Err(e) => {
            set_last_error(MetaOxideError::ParseError, Some(e.to_string()));
            ptr::null_mut()
        }
    }
}

/// Extract Web App Manifest link
///
/// # Returns
/// JSON object string or NULL on error
#[no_mangle]
pub unsafe extern "C" fn meta_oxide_extract_manifest(
    html: *const c_char,
    base_url: *const c_char,
) -> *mut c_char {
    clear_last_error();

    let html_str = match from_c_string(html) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    let base_url_str = from_c_string_opt(base_url);

    match extractors::manifest::extract(html_str, base_url_str) {
        Ok(manifest) => to_json_c_string(&manifest),
        Err(e) => {
            set_last_error(MetaOxideError::ParseError, Some(e.to_string()));
            ptr::null_mut()
        }
    }
}

/// Parse Web App Manifest JSON content
///
/// # Returns
/// JSON object string or NULL on error
#[no_mangle]
pub unsafe extern "C" fn meta_oxide_parse_manifest(
    json: *const c_char,
    base_url: *const c_char,
) -> *mut c_char {
    clear_last_error();

    let json_str = match from_c_string(json) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    let base_url_str = from_c_string_opt(base_url);

    match extractors::manifest::parse_manifest(json_str, base_url_str) {
        Ok(manifest) => to_json_c_string(&manifest),
        Err(e) => {
            set_last_error(MetaOxideError::ParseError, Some(e.to_string()));
            ptr::null_mut()
        }
    }
}

/// Extract oEmbed endpoint discovery
///
/// # Returns
/// JSON object string or NULL on error
#[no_mangle]
pub unsafe extern "C" fn meta_oxide_extract_oembed(
    html: *const c_char,
    base_url: *const c_char,
) -> *mut c_char {
    clear_last_error();

    let html_str = match from_c_string(html) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    let base_url_str = from_c_string_opt(base_url);

    match extractors::oembed::extract(html_str, base_url_str) {
        Ok(oembed) => to_json_c_string(&oembed),
        Err(e) => {
            set_last_error(MetaOxideError::ParseError, Some(e.to_string()));
            ptr::null_mut()
        }
    }
}

/// Extract rel-* link relationships
///
/// # Returns
/// JSON object string or NULL on error
#[no_mangle]
pub unsafe extern "C" fn meta_oxide_extract_rel_links(
    html: *const c_char,
    base_url: *const c_char,
) -> *mut c_char {
    clear_last_error();

    let html_str = match from_c_string(html) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    let base_url_str = from_c_string_opt(base_url);

    match extractors::rel_links::extract(html_str, base_url_str) {
        Ok(links) => to_json_c_string(&links),
        Err(e) => {
            set_last_error(MetaOxideError::ParseError, Some(e.to_string()));
            ptr::null_mut()
        }
    }
}

/// Get the last error code
///
/// Returns MetaOxideError::Ok (0) if no error occurred
#[no_mangle]
pub extern "C" fn meta_oxide_last_error() -> c_int {
    LAST_ERROR.with(|cell| {
        let (error, _) = cell.take();
        cell.set((error, None));
        error as c_int
    })
}

/// Get the last error message
///
/// Returns a static string describing the error, or NULL if no error occurred.
/// The returned string is valid until the next FFI call on this thread.
///
/// # Note
/// This function returns a pointer to thread-local storage. The string does
/// not need to be freed by the caller.
#[no_mangle]
pub extern "C" fn meta_oxide_error_message() -> *const c_char {
    LAST_ERROR.with(|cell| {
        let (error, message) = cell.take();
        let msg = match error {
            MetaOxideError::Ok => "No error\0",
            MetaOxideError::ParseError => "HTML parsing error\0",
            MetaOxideError::InvalidUrl => "Invalid URL format\0",
            MetaOxideError::InvalidUtf8 => "Invalid UTF-8 string\0",
            MetaOxideError::MemoryError => "Memory allocation error\0",
            MetaOxideError::JsonError => "JSON serialization error\0",
            MetaOxideError::NullPointer => "NULL pointer passed as argument\0",
        };

        // If there's a detailed message, we'd need to store it in thread-local storage
        // For now, return the static error string
        cell.set((error, message));
        msg.as_ptr() as *const c_char
    })
}

/// Free a MetaOxideResult structure
///
/// # Safety
/// - `result` must be a pointer returned by `meta_oxide_extract_all()`
/// - `result` must not be NULL
/// - `result` must not have been freed previously
#[no_mangle]
pub unsafe extern "C" fn meta_oxide_result_free(result: *mut MetaOxideResult) {
    if result.is_null() {
        return;
    }

    let result = Box::from_raw(result);

    // Free all individual strings
    if !result.meta.is_null() {
        let _ = CString::from_raw(result.meta);
    }
    if !result.open_graph.is_null() {
        let _ = CString::from_raw(result.open_graph);
    }
    if !result.twitter.is_null() {
        let _ = CString::from_raw(result.twitter);
    }
    if !result.json_ld.is_null() {
        let _ = CString::from_raw(result.json_ld);
    }
    if !result.microdata.is_null() {
        let _ = CString::from_raw(result.microdata);
    }
    if !result.microformats.is_null() {
        let _ = CString::from_raw(result.microformats);
    }
    if !result.rdfa.is_null() {
        let _ = CString::from_raw(result.rdfa);
    }
    if !result.dublin_core.is_null() {
        let _ = CString::from_raw(result.dublin_core);
    }
    if !result.manifest.is_null() {
        let _ = CString::from_raw(result.manifest);
    }
    if !result.oembed.is_null() {
        let _ = CString::from_raw(result.oembed);
    }
    if !result.rel_links.is_null() {
        let _ = CString::from_raw(result.rel_links);
    }

    // Box is dropped automatically here
}

/// Free a string returned by any MetaOxide function
///
/// # Safety
/// - `s` must be a pointer returned by a MetaOxide function
/// - `s` must not be NULL
/// - `s` must not have been freed previously
#[no_mangle]
pub unsafe extern "C" fn meta_oxide_string_free(s: *mut c_char) {
    if !s.is_null() {
        let _ = CString::from_raw(s);
    }
}

/// Free a ManifestDiscovery structure
///
/// # Safety
/// - `discovery` must be a pointer returned by `meta_oxide_extract_manifest_discovery()`
/// - `discovery` must not be NULL
/// - `discovery` must not have been freed previously
#[no_mangle]
pub unsafe extern "C" fn meta_oxide_manifest_discovery_free(discovery: *mut ManifestDiscovery) {
    if discovery.is_null() {
        return;
    }

    let discovery = Box::from_raw(discovery);

    if !discovery.href.is_null() {
        let _ = CString::from_raw(discovery.href);
    }
    if !discovery.manifest.is_null() {
        let _ = CString::from_raw(discovery.manifest);
    }
}

/// Get the library version string
///
/// Returns a static string that does not need to be freed
#[no_mangle]
pub extern "C" fn meta_oxide_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const c_char
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_extract_all_basic() {
        let html = CString::new(
            r#"
            <html>
                <head>
                    <title>Test Page</title>
                    <meta name="description" content="Test description">
                </head>
            </html>
        "#,
        )
        .unwrap();

        unsafe {
            let result = meta_oxide_extract_all(html.as_ptr(), ptr::null());
            assert!(!result.is_null());
            assert!(!(*result).meta.is_null());
            meta_oxide_result_free(result);
        }
    }

    #[test]
    fn test_extract_meta() {
        let html = CString::new(
            r#"
            <html>
                <head>
                    <title>Test</title>
                    <meta name="description" content="Description">
                </head>
            </html>
        "#,
        )
        .unwrap();

        unsafe {
            let result = meta_oxide_extract_meta(html.as_ptr(), ptr::null());
            assert!(!result.is_null());
            meta_oxide_string_free(result);
        }
    }

    #[test]
    fn test_null_pointer_handling() {
        unsafe {
            let result = meta_oxide_extract_meta(ptr::null(), ptr::null());
            assert!(result.is_null());
            let error = meta_oxide_last_error();
            assert_eq!(error, MetaOxideError::NullPointer as c_int);
        }
    }

    #[test]
    fn test_version() {
        unsafe {
            let version = meta_oxide_version();
            assert!(!version.is_null());
            let version_str = CStr::from_ptr(version).to_str().unwrap();
            assert!(!version_str.is_empty());
        }
    }
}
