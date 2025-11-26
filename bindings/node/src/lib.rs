#![allow(non_snake_case)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::ffi::{CStr, CString};

/// Extract all metadata from HTML and return as JSON string
///
/// Extracts metadata in 13 formats and returns the result as a JSON string
/// that can be parsed in JavaScript.
#[napi]
pub fn extractAll(html: String, base_url: Option<String>) -> Result<String> {
    unsafe {
        let c_html = CString::new(html)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        let c_base_url = base_url.as_ref().map(|url| {
            CString::new(url.clone())
                .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
        }).transpose()?;

        let result_ptr = meta_oxide::ffi::meta_oxide_extract_all(
            c_html.as_ptr(),
            c_base_url.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null()),
        );

        if result_ptr.is_null() {
            return Err(Error::new(Status::GenericFailure, "Failed to extract metadata"));
        }

        let result = std::ptr::read(result_ptr);
        let mut output = serde_json::json!({});

        // Collect all extracted formats into a JSON object
        if !result.meta.is_null() {
            if let Ok(meta_json) = serde_json::from_str::<serde_json::Value>(
                &CStr::from_ptr(result.meta).to_string_lossy()
            ) {
                output["meta"] = meta_json;
            }
        }

        if !result.open_graph.is_null() {
            if let Ok(og_json) = serde_json::from_str::<serde_json::Value>(
                &CStr::from_ptr(result.open_graph).to_string_lossy()
            ) {
                output["opengraph"] = og_json;
            }
        }

        if !result.twitter.is_null() {
            if let Ok(twitter_json) = serde_json::from_str::<serde_json::Value>(
                &CStr::from_ptr(result.twitter).to_string_lossy()
            ) {
                output["twitter"] = twitter_json;
            }
        }

        if !result.json_ld.is_null() {
            if let Ok(jsonld_json) = serde_json::from_str::<serde_json::Value>(
                &CStr::from_ptr(result.json_ld).to_string_lossy()
            ) {
                output["jsonld"] = jsonld_json;
            }
        }

        if !result.microdata.is_null() {
            if let Ok(microdata_json) = serde_json::from_str::<serde_json::Value>(
                &CStr::from_ptr(result.microdata).to_string_lossy()
            ) {
                output["microdata"] = microdata_json;
            }
        }

        if !result.microformats.is_null() {
            if let Ok(mf_json) = serde_json::from_str::<serde_json::Value>(
                &CStr::from_ptr(result.microformats).to_string_lossy()
            ) {
                output["microformats"] = mf_json;
            }
        }

        if !result.rdfa.is_null() {
            if let Ok(rdfa_json) = serde_json::from_str::<serde_json::Value>(
                &CStr::from_ptr(result.rdfa).to_string_lossy()
            ) {
                output["rdfa"] = rdfa_json;
            }
        }

        if !result.dublin_core.is_null() {
            if let Ok(dc_json) = serde_json::from_str::<serde_json::Value>(
                &CStr::from_ptr(result.dublin_core).to_string_lossy()
            ) {
                output["dublin_core"] = dc_json;
            }
        }

        if !result.manifest.is_null() {
            if let Ok(manifest_json) = serde_json::from_str::<serde_json::Value>(
                &CStr::from_ptr(result.manifest).to_string_lossy()
            ) {
                output["manifest"] = manifest_json;
            }
        }

        if !result.oembed.is_null() {
            if let Ok(oembed_json) = serde_json::from_str::<serde_json::Value>(
                &CStr::from_ptr(result.oembed).to_string_lossy()
            ) {
                output["oembed"] = oembed_json;
            }
        }

        if !result.rel_links.is_null() {
            if let Ok(rel_json) = serde_json::from_str::<serde_json::Value>(
                &CStr::from_ptr(result.rel_links).to_string_lossy()
            ) {
                output["rel_links"] = rel_json;
            }
        }

        // Free memory
        meta_oxide::ffi::meta_oxide_result_free(result_ptr as _);

        Ok(serde_json::to_string(&output).unwrap_or_else(|_| "{}".to_string()))
    }
}

/// Extract standard HTML meta tags
#[napi]
pub fn extractMeta(html: String, base_url: Option<String>) -> Result<String> {
    unsafe {
        let c_html = CString::new(html)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        let c_base_url = base_url.as_ref().map(|url| {
            CString::new(url.clone())
                .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
        }).transpose()?;

        let result_ptr = meta_oxide::ffi::meta_oxide_extract_meta(
            c_html.as_ptr(),
            c_base_url.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null()),
        );

        if result_ptr.is_null() {
            return Err(Error::new(Status::GenericFailure, "Failed to extract meta tags"));
        }

        let result = CStr::from_ptr(result_ptr).to_string_lossy().to_string();
        meta_oxide::ffi::meta_oxide_string_free(result_ptr);

        Ok(result)
    }
}

/// Extract Open Graph metadata
#[napi]
pub fn extractOpengraph(html: String, base_url: Option<String>) -> Result<String> {
    unsafe {
        let c_html = CString::new(html)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        let c_base_url = base_url.as_ref().map(|url| {
            CString::new(url.clone())
                .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
        }).transpose()?;

        let result_ptr = meta_oxide::ffi::meta_oxide_extract_open_graph(
            c_html.as_ptr(),
            c_base_url.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null()),
        );

        if result_ptr.is_null() {
            return Err(Error::new(Status::GenericFailure, "Failed to extract Open Graph"));
        }

        let result = CStr::from_ptr(result_ptr).to_string_lossy().to_string();
        meta_oxide::ffi::meta_oxide_string_free(result_ptr);

        Ok(result)
    }
}

/// Extract Twitter Card metadata
#[napi]
pub fn extractTwitter(html: String, base_url: Option<String>) -> Result<String> {
    unsafe {
        let c_html = CString::new(html)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        let c_base_url = base_url.as_ref().map(|url| {
            CString::new(url.clone())
                .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
        }).transpose()?;

        let result_ptr = meta_oxide::ffi::meta_oxide_extract_twitter(
            c_html.as_ptr(),
            c_base_url.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null()),
        );

        if result_ptr.is_null() {
            return Err(Error::new(Status::GenericFailure, "Failed to extract Twitter Card"));
        }

        let result = CStr::from_ptr(result_ptr).to_string_lossy().to_string();
        meta_oxide::ffi::meta_oxide_string_free(result_ptr);

        Ok(result)
    }
}
