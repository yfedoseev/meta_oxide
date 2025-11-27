/**
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
 */

#ifndef META_OXIDE_H
#define META_OXIDE_H

#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Result structure containing all extracted metadata
 *
 * Each field is a JSON string or NULL if no data was found.
 * The caller must free this struct using `meta_oxide_result_free()`.
 */
typedef struct MetaOxideResult {
  /**
   * Standard HTML meta tags (JSON object)
   */
  char *meta;
  /**
   * Open Graph metadata (JSON object)
   */
  char *open_graph;
  /**
   * Twitter Card metadata (JSON object)
   */
  char *twitter;
  /**
   * JSON-LD structured data (JSON array)
   */
  char *json_ld;
  /**
   * Microdata items (JSON array)
   */
  char *microdata;
  /**
   * Microformats data (JSON object with h-card, h-entry, etc.)
   */
  char *microformats;
  /**
   * RDFa structured data (JSON array)
   */
  char *rdfa;
  /**
   * Dublin Core metadata (JSON object)
   */
  char *dublin_core;
  /**
   * Web App Manifest discovery (JSON object)
   */
  char *manifest;
  /**
   * oEmbed endpoint discovery (JSON object)
   */
  char *oembed;
  /**
   * rel-* link relationships (JSON object)
   */
  char *rel_links;
} MetaOxideResult;

/**
 * Manifest discovery result with URL and parsed content
 */
typedef struct ManifestDiscovery {
  /**
   * Manifest URL (may be NULL)
   */
  char *href;
  /**
   * Full manifest JSON (may be NULL)
   */
  char *manifest;
} ManifestDiscovery;

/**
 * Extract ALL metadata from HTML
 *
 * Returns a MetaOxideResult containing all extracted data as JSON strings.
 * Returns NULL on error.
 *
 * # Arguments
 * * `html` - HTML content (must not be NULL)
 * * `base_url` - Base URL for resolving relative URLs (may be NULL)
 *
 * # Memory
 * The caller must free the returned struct using `meta_oxide_result_free()`.
 *
 * # Safety
 * - `html` must be a valid null-terminated C string
 * - `base_url` may be NULL or a valid null-terminated C string
 */
struct MetaOxideResult *meta_oxide_extract_all(const char *html, const char *base_url);

/**
 * Extract standard HTML meta tags
 *
 * # Returns
 * JSON string or NULL on error
 */
char *meta_oxide_extract_meta(const char *html, const char *base_url);

/**
 * Extract Open Graph metadata
 *
 * # Returns
 * JSON string or NULL on error
 */
char *meta_oxide_extract_open_graph(const char *html, const char *base_url);

/**
 * Extract Twitter Card metadata
 *
 * # Returns
 * JSON string or NULL on error
 */
char *meta_oxide_extract_twitter(const char *html, const char *base_url);

/**
 * Extract JSON-LD structured data
 *
 * # Returns
 * JSON array string or NULL on error
 */
char *meta_oxide_extract_json_ld(const char *html, const char *base_url);

/**
 * Extract Microdata
 *
 * # Returns
 * JSON array string or NULL on error
 */
char *meta_oxide_extract_microdata(const char *html, const char *base_url);

/**
 * Extract Microformats (all 9 types: h-card, h-entry, h-event, etc.)
 *
 * # Returns
 * JSON object string with format types as keys, or NULL on error
 */
char *meta_oxide_extract_microformats(const char *html, const char *base_url);

/**
 * Extract RDFa structured data
 *
 * # Returns
 * JSON array string or NULL on error
 */
char *meta_oxide_extract_rdfa(const char *html, const char *base_url);

/**
 * Extract Dublin Core metadata
 *
 * # Returns
 * JSON object string or NULL on error
 */
char *meta_oxide_extract_dublin_core(const char *html);

/**
 * Extract Web App Manifest link
 *
 * # Returns
 * JSON object string or NULL on error
 */
char *meta_oxide_extract_manifest(const char *html, const char *base_url);

/**
 * Parse Web App Manifest JSON content
 *
 * # Returns
 * JSON object string or NULL on error
 */
char *meta_oxide_parse_manifest(const char *json, const char *base_url);

/**
 * Extract oEmbed endpoint discovery
 *
 * # Returns
 * JSON object string or NULL on error
 */
char *meta_oxide_extract_oembed(const char *html, const char *base_url);

/**
 * Extract rel-* link relationships
 *
 * # Returns
 * JSON object string or NULL on error
 */
char *meta_oxide_extract_rel_links(const char *html, const char *base_url);

/**
 * Get the last error code
 *
 * Returns MetaOxideError::Ok (0) if no error occurred
 */
int meta_oxide_last_error(void);

/**
 * Get the last error message
 *
 * Returns a static string describing the error, or NULL if no error occurred.
 * The returned string is valid until the next FFI call on this thread.
 *
 * # Note
 * This function returns a pointer to thread-local storage. The string does
 * not need to be freed by the caller.
 */
const char *meta_oxide_error_message(void);

/**
 * Free a MetaOxideResult structure
 *
 * # Safety
 * - `result` must be a pointer returned by `meta_oxide_extract_all()`
 * - `result` must not be NULL
 * - `result` must not have been freed previously
 */
void meta_oxide_result_free(struct MetaOxideResult *result);

/**
 * Free a string returned by any MetaOxide function
 *
 * # Safety
 * - `s` must be a pointer returned by a MetaOxide function
 * - `s` must not be NULL
 * - `s` must not have been freed previously
 */
void meta_oxide_string_free(char *s);

/**
 * Free a ManifestDiscovery structure
 *
 * # Safety
 * - `discovery` must be a pointer returned by `meta_oxide_extract_manifest_discovery()`
 * - `discovery` must not be NULL
 * - `discovery` must not have been freed previously
 */
void meta_oxide_manifest_discovery_free(struct ManifestDiscovery *discovery);

/**
 * Get the library version string
 *
 * Returns a static string that does not need to be freed
 */
const char *meta_oxide_version(void);

#endif /* META_OXIDE_H */
