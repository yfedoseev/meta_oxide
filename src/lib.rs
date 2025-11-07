use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

mod parser;
mod extractors;
mod types;
mod errors;

pub use errors::MicroformatError;
pub use types::*;

/// Extract microformats data from HTML content
#[pyfunction]
fn extract_microformats(html: &str, base_url: Option<&str>) -> PyResult<HashMap<String, Vec<PyObject>>> {
    Python::with_gil(|py| {
        let result = parser::parse_html(html, base_url)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let mut py_result = HashMap::new();

        // Convert Rust data structures to Python objects
        for (format_type, items) in result.iter() {
            let py_items: Vec<PyObject> = items
                .iter()
                .map(|item| item.to_py_dict(py).into())
                .collect();
            py_result.insert(format_type.clone(), py_items);
        }

        Ok(py_result)
    })
}

/// Extract h-card microformat data
#[pyfunction]
fn extract_hcard(html: &str, base_url: Option<&str>) -> PyResult<Vec<PyObject>> {
    Python::with_gil(|py| {
        let cards = extractors::hcard::extract(html, base_url)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        Ok(cards.iter().map(|card| card.to_py_dict(py).into()).collect())
    })
}

/// Extract h-entry microformat data
#[pyfunction]
fn extract_hentry(html: &str, base_url: Option<&str>) -> PyResult<Vec<PyObject>> {
    Python::with_gil(|py| {
        let entries = extractors::hentry::extract(html, base_url)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        Ok(entries.iter().map(|entry| entry.to_py_dict(py).into()).collect())
    })
}

/// Extract h-event microformat data
#[pyfunction]
fn extract_hevent(html: &str, base_url: Option<&str>) -> PyResult<Vec<PyObject>> {
    Python::with_gil(|py| {
        let events = extractors::hevent::extract(html, base_url)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        Ok(events.iter().map(|event| event.to_py_dict(py).into()).collect())
    })
}

/// MetaOxide: A fast Rust library for extracting microformats data
#[pymodule]
fn meta_oxide(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(extract_microformats, m)?)?;
    m.add_function(wrap_pyfunction!(extract_hcard, m)?)?;
    m.add_function(wrap_pyfunction!(extract_hentry, m)?)?;
    m.add_function(wrap_pyfunction!(extract_hevent, m)?)?;

    // Add version
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
