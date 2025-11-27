//! Declarative macro for generating Python binding functions
//!
//! This macro eliminates ~200 lines of duplicated code by providing a single
//! declaration point for microformat extractor Python bindings.
//!
//! # Usage
//!
//! ```rust
//! py_extractor_binding!(extract_hcard, hcard, HCard);
//! py_extractor_binding!(extract_hentry, hentry, HEntry);
//! ```
//!
//! # What It Generates
//!
//! For each invocation, the macro generates a complete PyO3 function with:
//! - Proper `#[pyfunction]` annotation
//! - `#[pyo3(signature = (html, base_url=None))]` for optional parameters
//! - GIL acquisition via `Python::with_gil`
//! - Error conversion to PyValueError
//! - Automatic conversion to Python objects via `.to_py_dict()`
//!
//! # Example Expansion
//!
//! ```rust
//! py_extractor_binding!(extract_hcard, hcard, HCard);
//! ```
//!
//! Expands to:
//!
//! ```rust
//! /// Extract h-card microformat data
//! #[pyfunction]
//! #[pyo3(signature = (html, base_url=None))]
//! fn extract_hcard(html: &str, base_url: Option<&str>) -> PyResult<Vec<PyObject>> {
//!     Python::with_gil(|py| {
//!         let items = extractors::microformats::hcard::extract(html, base_url)
//!             .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
//!
//!         Ok(items.iter().map(|item| item.to_py_dict(py).into()).collect())
//!     })
//! }
//! ```

/// Generate a Python binding function for a microformat extractor
///
/// # Parameters
///
/// - `$func_name`: The name of the Python function (e.g., `extract_hcard`)
/// - `$module`: The extractor module name (e.g., `hcard`)
/// - `$type_name`: The Rust type name (e.g., `HCard`) - currently unused but reserved for future enhancements
///
/// # Examples
///
/// ```rust
/// // Generate binding for h-card extractor
/// py_extractor_binding!(extract_hcard, hcard, HCard);
///
/// // Generate binding for h-entry extractor
/// py_extractor_binding!(extract_hentry, hentry, HEntry);
///
/// // Generate binding for h-event extractor
/// py_extractor_binding!(extract_hevent, hevent, HEvent);
/// ```
#[macro_export]
macro_rules! py_extractor_binding {
    ($func_name:ident, $module:ident, $type_name:ident) => {
        /// Extract microformat data
        #[pyfunction]
        #[pyo3(signature = (html, base_url=None))]
        fn $func_name(html: &str, base_url: Option<&str>) -> PyResult<Vec<PyObject>> {
            Python::with_gil(|py| {
                let items = extractors::microformats::$module::extract(html, base_url)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

                Ok(items.iter().map(|item| item.to_py_dict(py).into()).collect())
            })
        }
    };
}

#[cfg(test)]
mod tests {
    /// Test that the macro compiles correctly
    ///
    /// Note: These are compile-time tests. If this module compiles,
    /// the macro syntax is valid.
    #[test]
    fn test_macro_compiles() {
        // This test simply ensures the macro is syntactically valid
        // The actual functionality is tested through integration tests
    }
}
