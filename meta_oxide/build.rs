use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_file = PathBuf::from(&crate_dir).join("include").join("meta_oxide.h");

    // Create include directory if it doesn't exist
    std::fs::create_dir_all(PathBuf::from(&crate_dir).join("include"))
        .expect("Failed to create include directory");

    // Generate C bindings
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_namespace("meta_oxide")
        .with_documentation(true)
        .with_pragma_once(true)
        .with_include_guard("META_OXIDE_H")
        .with_header(
            r#"/**
 * MetaOxide C API
 *
 * A fast, comprehensive metadata extraction library for HTML content.
 * Extracts 13 different metadata formats including:
 * - Standard HTML meta tags
 * - Open Graph (Facebook, LinkedIn)
 * - Twitter Cards
 * - JSON-LD (Schema.org)
 * - Microdata
 * - Microformats (h-card, h-entry, h-event, etc.)
 * - RDFa
 * - Dublin Core
 * - Web App Manifest
 * - oEmbed
 * - rel-* link relationships
 *
 * ## Memory Management
 *
 * All functions that return pointers allocate memory that must be freed by the caller.
 * Use the appropriate `*_free()` functions to deallocate memory:
 * - `meta_oxide_result_free()` for MetaOxideResult structs
 * - `meta_oxide_string_free()` for individual strings
 * - `meta_oxide_manifest_discovery_free()` for ManifestDiscovery structs
 *
 * ## Error Handling
 *
 * Functions return NULL on error and set the thread-local error state.
 * Check errors using:
 * - `meta_oxide_last_error()` - returns error code
 * - `meta_oxide_error_message()` - returns error description
 *
 * ## Thread Safety
 *
 * All functions are thread-safe. Error state is thread-local.
 *
 * ## Example Usage
 *
 * ```c
 * #include "meta_oxide.h"
 * #include <stdio.h>
 * #include <stdlib.h>
 *
 * int main() {
 *     const char* html = "<html><head><title>Test</title></head></html>";
 *
 *     MetaOxideResult* result = meta_oxide_extract_all(html, NULL);
 *     if (result == NULL) {
 *         fprintf(stderr, "Error: %s\n", meta_oxide_error_message());
 *         return 1;
 *     }
 *
 *     if (result->meta != NULL) {
 *         printf("Meta tags: %s\n", result->meta);
 *     }
 *
 *     meta_oxide_result_free(result);
 *     return 0;
 * }
 * ```
 *
 * @version 0.1.0
 * @license MIT OR Apache-2.0
 */"#,
        )
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file(&output_file);

    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=build.rs");
}
