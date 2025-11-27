//! Example usage of the py_extractor_binding! macro
//!
//! This file demonstrates how to use the macro to generate Python binding functions.
//! It is NOT compiled into the library but serves as documentation.
//!
//! # Before: Manual Implementation (Duplicated Code)
//!
//! ```rust
//! /// Extract h-card microformat data
//! #[pyfunction]
//! #[pyo3(signature = (html, base_url=None))]
//! fn extract_hcard(html: &str, base_url: Option<&str>) -> PyResult<Vec<PyObject>> {
//!     Python::with_gil(|py| {
//!         let cards = extractors::microformats::hcard::extract(html, base_url)
//!             .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
//!
//!         Ok(cards.iter().map(|card| card.to_py_dict(py).into()).collect())
//!     })
//! }
//!
//! /// Extract h-entry microformat data
//! #[pyfunction]
//! #[pyo3(signature = (html, base_url=None))]
//! fn extract_hentry(html: &str, base_url: Option<&str>) -> PyResult<Vec<PyObject>> {
//!     Python::with_gil(|py| {
//!         let entries = extractors::microformats::hentry::extract(html, base_url)
//!             .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
//!
//!         Ok(entries.iter().map(|entry| entry.to_py_dict(py).into()).collect())
//!     })
//! }
//!
//! /// Extract h-event microformat data
//! #[pyfunction]
//! #[pyo3(signature = (html, base_url=None))]
//! fn extract_hevent(html: &str, base_url: Option<&str>) -> PyResult<Vec<PyObject>> {
//!     Python::with_gil(|py| {
//!         let events = extractors::microformats::hevent::extract(html, base_url)
//!             .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
//!
//!         Ok(events.iter().map(|event| event.to_py_dict(py).into()).collect())
//!     })
//! }
//!
//! // ... and 6 more identical functions with only the name and module changed
//! ```
//!
//! # After: Using the Macro (DRY)
//!
//! ```rust
//! // All microformat extractors - just 9 lines instead of ~200!
//! py_extractor_binding!(extract_hcard, hcard, HCard);
//! py_extractor_binding!(extract_hentry, hentry, HEntry);
//! py_extractor_binding!(extract_hevent, hevent, HEvent);
//! py_extractor_binding!(extract_hreview, hreview, HReview);
//! py_extractor_binding!(extract_hrecipe, hrecipe, HRecipe);
//! py_extractor_binding!(extract_hproduct, hproduct, HProduct);
//! py_extractor_binding!(extract_hfeed, hfeed, HFeed);
//! py_extractor_binding!(extract_hadr, hadr, HAdr);
//! py_extractor_binding!(extract_hgeo, hgeo, HGeo);
//! ```
//!
//! # How It Works
//!
//! The macro takes three parameters:
//!
//! 1. **Function name** - The Python function name (e.g., `extract_hcard`)
//! 2. **Module name** - The extractor module path (e.g., `hcard`)
//! 3. **Type name** - The Rust type (e.g., `HCard`)
//!
//! And generates a complete function with:
//! - Proper PyO3 annotations
//! - GIL handling
//! - Error conversion
//! - Result mapping to Python objects
//!
//! # Benefits
//!
//! - **Less code**: 9 lines instead of ~200 lines
//! - **Type safety**: Compile-time validation of extractor paths
//! - **Consistency**: All bindings follow the same pattern
//! - **Maintainability**: Changes to the pattern only need to be made once
//! - **Performance**: Zero runtime overhead (macro expansion at compile time)
//!
//! # Adding New Extractors
//!
//! To add a new microformat extractor:
//!
//! 1. Create the extractor in `src/extractors/microformats/your_format.rs`
//! 2. Define the type in `src/types/microformats.rs`
//! 3. Add ONE line using the macro:
//!    ```rust
//!    py_extractor_binding!(extract_your_format, your_format, YourFormat);
//!    ```
//! 4. Register the function in the PyModule
//!
//! That's it! No more copy-pasting boilerplate.
