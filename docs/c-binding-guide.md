# MetaOxide C Binding Guide for Language Implementers

This guide is for developers creating MetaOxide bindings for other programming languages (Go, Node.js, Java, C#, Python, etc.).

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [ABI Stability](#abi-stability)
4. [Memory Management Contract](#memory-management-contract)
5. [Type Mappings](#type-mappings)
6. [Error Handling](#error-handling)
7. [Language-Specific Examples](#language-specific-examples)
8. [Testing Your Binding](#testing-your-binding)
9. [Best Practices](#best-practices)

## Overview

MetaOxide provides a stable C ABI that serves as the foundation for all language bindings. The C library handles all metadata extraction and returns JSON strings, making bindings straightforward to implement.

**Benefits of the C-ABI Approach:**
- Single source of truth (one Rust implementation)
- Consistent behavior across all languages
- Easy to maintain (update C library, all bindings benefit)
- No need to rewrite extraction logic per language
- Cross-platform support (Linux, macOS, Windows)

## Architecture

```
┌─────────────────────────────────────────┐
│   Your Language Binding (Go/Node/Java)  │
│   - High-level API                      │
│   - Native types                        │
│   - Idiomatic error handling            │
└─────────────────┬───────────────────────┘
                  │ FFI calls
┌─────────────────▼───────────────────────┐
│   MetaOxide C Library (libmeta_oxide)   │
│   - Stable C ABI                        │
│   - JSON string outputs                 │
│   - Memory management                   │
└─────────────────┬───────────────────────┘
                  │
┌─────────────────▼───────────────────────┐
│   Rust Core Implementation              │
│   - HTML parsing (scraper)              │
│   - Metadata extraction                 │
│   - URL resolution                      │
└─────────────────────────────────────────┘
```

## ABI Stability

### Version Compatibility

- **C ABI is stable within major versions** (e.g., 0.x.x, 1.x.x)
- **Breaking changes only on major version bumps** (0.x → 1.0, 1.x → 2.0)
- **Patch/minor versions are drop-in compatible**

### What's Stable

✅ Function signatures (names, parameter types, return types)
✅ Struct layouts (field order, types)
✅ Error codes and meanings
✅ JSON output schemas
✅ Memory ownership rules

### What May Change in Minor Versions

⚠️ JSON output may include new fields (always additive)
⚠️ New functions may be added
⚠️ Performance characteristics may improve

## Memory Management Contract

### Ownership Rules

**Rule 1: Library Allocates, Caller Frees**

All pointers returned by MetaOxide are allocated by the library and must be freed by the caller using the appropriate free function.

```c
// C library allocates
char* meta = meta_oxide_extract_meta(html, NULL);

// Caller MUST free
meta_oxide_string_free(meta);
```

**Rule 2: Use Correct Free Functions**

| Type | Free Function |
|------|---------------|
| `MetaOxideResult*` | `meta_oxide_result_free()` |
| `char*` (strings) | `meta_oxide_string_free()` |
| `ManifestDiscovery*` | `meta_oxide_manifest_discovery_free()` |

**Never use your language's native `free()` - always use MetaOxide's functions.**

**Rule 3: NULL Pointers Are Safe**

All free functions check for NULL and handle it gracefully:

```c
meta_oxide_string_free(NULL);  // Safe, does nothing
```

### Finalizers/Destructors

In garbage-collected languages, attach finalizers to wrapper objects:

**Python Example:**
```python
class MetaOxideResult:
    def __init__(self, ptr):
        self._ptr = ptr

    def __del__(self):
        if self._ptr:
            lib.meta_oxide_result_free(self._ptr)
            self._ptr = None
```

**Go Example:**
```go
type MetaOxideResult struct {
    ptr *C.MetaOxideResult
}

func NewMetaOxideResult(ptr *C.MetaOxideResult) *MetaOxideResult {
    result := &MetaOxideResult{ptr: ptr}
    runtime.SetFinalizer(result, (*MetaOxideResult).free)
    return result
}

func (r *MetaOxideResult) free() {
    if r.ptr != nil {
        C.meta_oxide_result_free(r.ptr)
        r.ptr = nil
    }
}
```

## Type Mappings

### C to Language Type Mappings

| C Type | Go | Node.js (N-API) | Java (JNI) | C# (P/Invoke) | Python (ctypes) |
|--------|----|-----------------| -----------|---------------|-----------------|
| `const char*` | `*C.char` | `napi_value` (string) | `String` | `string` | `c_char_p` |
| `char*` (owned) | `*C.char` | `napi_value` (string) | `String` | `string` | `c_char_p` |
| `int` | `C.int` | `int32_t` | `int` | `int` | `c_int` |
| `MetaOxideResult*` | `*C.MetaOxideResult` | `napi_ref` | `long` | `IntPtr` | `POINTER(MetaOxideResult)` |
| `void` | - | `void` | `void` | `void` | `None` |

### Struct Field Access

**MetaOxideResult Layout:**
```c
typedef struct MetaOxideResult {
    char* meta;           // offset 0
    char* open_graph;     // offset 8 (64-bit)
    char* twitter;        // offset 16
    char* json_ld;        // offset 24
    char* microdata;      // offset 32
    char* microformats;   // offset 40
    char* rdfa;           // offset 48
    char* dublin_core;    // offset 56
    char* manifest;       // offset 64
    char* oembed;         // offset 72
    char* rel_links;      // offset 80
} MetaOxideResult;
```

**Field Access Pattern:**
```c
// In your binding, create a struct with same layout
// Then access fields directly:
if (result->meta != NULL) {
    char* meta_json = result->meta;
    // Convert to native string
}
```

## Error Handling

### Error Detection

Functions return `NULL` on error:

```c
MetaOxideResult* result = meta_oxide_extract_all(html, base_url);
if (result == NULL) {
    // Error occurred
    int error_code = meta_oxide_last_error();
    const char* error_msg = meta_oxide_error_message();
}
```

### Error Codes

```c
enum MetaOxideError {
    Ok = 0,              // No error
    ParseError = 1,      // HTML parsing failed
    InvalidUrl = 2,      // Invalid URL format
    InvalidUtf8 = 3,     // Invalid UTF-8 in input
    MemoryError = 4,     // Memory allocation failed
    JsonError = 5,       // JSON serialization failed
    NullPointer = 6      // NULL pointer passed
};
```

### Thread-Local Error State

Error state is stored per-thread, so it's safe in multithreaded environments:

```c
// Thread 1
meta_oxide_extract_meta(NULL, NULL);  // Sets error in thread 1
int error1 = meta_oxide_last_error();  // Gets thread 1's error

// Thread 2 (simultaneously)
meta_oxide_extract_meta(html, NULL);   // Success, no error in thread 2
int error2 = meta_oxide_last_error();  // Returns 0 (no error)
```

### Idiomatic Error Handling in Bindings

**Go:**
```go
func ExtractMeta(html, baseURL string) (string, error) {
    cHTML := C.CString(html)
    defer C.free(unsafe.Pointer(cHTML))

    var cBaseURL *C.char
    if baseURL != "" {
        cBaseURL = C.CString(baseURL)
        defer C.free(unsafe.Pointer(cBaseURL))
    }

    result := C.meta_oxide_extract_meta(cHTML, cBaseURL)
    if result == nil {
        errCode := C.meta_oxide_last_error()
        errMsg := C.GoString(C.meta_oxide_error_message())
        return "", fmt.Errorf("meta_oxide error %d: %s", errCode, errMsg)
    }
    defer C.meta_oxide_string_free(result)

    return C.GoString(result), nil
}
```

**Node.js:**
```javascript
function extractMeta(html, baseUrl) {
  const result = binding.meta_oxide_extract_meta(html, baseUrl);
  if (result === null) {
    const errorCode = binding.meta_oxide_last_error();
    const errorMsg = binding.meta_oxide_error_message();
    throw new Error(`MetaOxide error ${errorCode}: ${errorMsg}`);
  }
  return JSON.parse(result);
}
```

**Python:**
```python
def extract_meta(html: str, base_url: Optional[str] = None) -> dict:
    result = lib.meta_oxide_extract_meta(
        html.encode('utf-8'),
        base_url.encode('utf-8') if base_url else None
    )
    if not result:
        error_code = lib.meta_oxide_last_error()
        error_msg = lib.meta_oxide_error_message().decode('utf-8')
        raise RuntimeError(f"MetaOxide error {error_code}: {error_msg}")

    json_str = ctypes.string_at(result).decode('utf-8')
    lib.meta_oxide_string_free(result)
    return json.loads(json_str)
```

## Language-Specific Examples

### Go (cgo)

**File: meta_oxide.go**
```go
package metaoxide

// #cgo CFLAGS: -I../include
// #cgo LDFLAGS: -L../target/release -lmeta_oxide
// #include "meta_oxide.h"
// #include <stdlib.h>
import "C"
import (
    "encoding/json"
    "fmt"
    "runtime"
    "unsafe"
)

type MetaOxideResult struct {
    Meta         map[string]interface{} `json:"meta,omitempty"`
    OpenGraph    map[string]interface{} `json:"open_graph,omitempty"`
    Twitter      map[string]interface{} `json:"twitter,omitempty"`
    JsonLd       []interface{}          `json:"json_ld,omitempty"`
    Microdata    []interface{}          `json:"microdata,omitempty"`
    Microformats map[string]interface{} `json:"microformats,omitempty"`
    RDFa         []interface{}          `json:"rdfa,omitempty"`
    DublinCore   map[string]interface{} `json:"dublin_core,omitempty"`
    Manifest     map[string]interface{} `json:"manifest,omitempty"`
    OEmbed       map[string]interface{} `json:"oembed,omitempty"`
    RelLinks     map[string]interface{} `json:"rel_links,omitempty"`
}

func ExtractAll(html, baseURL string) (*MetaOxideResult, error) {
    cHTML := C.CString(html)
    defer C.free(unsafe.Pointer(cHTML))

    var cBaseURL *C.char
    if baseURL != "" {
        cBaseURL = C.CString(baseURL)
        defer C.free(unsafe.Pointer(cBaseURL))
    }

    cResult := C.meta_oxide_extract_all(cHTML, cBaseURL)
    if cResult == nil {
        errCode := C.meta_oxide_last_error()
        errMsg := C.GoString(C.meta_oxide_error_message())
        return nil, fmt.Errorf("error %d: %s", errCode, errMsg)
    }
    defer C.meta_oxide_result_free(cResult)

    result := &MetaOxideResult{}

    // Convert each field
    if cResult.meta != nil {
        json.Unmarshal([]byte(C.GoString(cResult.meta)), &result.Meta)
    }
    if cResult.open_graph != nil {
        json.Unmarshal([]byte(C.GoString(cResult.open_graph)), &result.OpenGraph)
    }
    // ... convert other fields

    return result, nil
}
```

### Node.js (N-API)

**File: binding.cpp**
```cpp
#include <node_api.h>
#include "meta_oxide.h"

napi_value ExtractAll(napi_env env, napi_callback_info info) {
    size_t argc = 2;
    napi_value args[2];
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    // Get HTML string
    size_t html_len;
    napi_get_value_string_utf8(env, args[0], nullptr, 0, &html_len);
    char* html = new char[html_len + 1];
    napi_get_value_string_utf8(env, args[0], html, html_len + 1, &html_len);

    // Get base URL (optional)
    char* base_url = nullptr;
    if (argc > 1) {
        size_t url_len;
        napi_get_value_string_utf8(env, args[1], nullptr, 0, &url_len);
        base_url = new char[url_len + 1];
        napi_get_value_string_utf8(env, args[1], base_url, url_len + 1, &url_len);
    }

    // Call C library
    MetaOxideResult* result = meta_oxide_extract_all(html, base_url);

    delete[] html;
    if (base_url) delete[] base_url;

    if (result == nullptr) {
        napi_throw_error(env, nullptr, meta_oxide_error_message());
        return nullptr;
    }

    // Create JS object
    napi_value js_result;
    napi_create_object(env, &js_result);

    // Set fields
    if (result->meta) {
        napi_value meta;
        napi_create_string_utf8(env, result->meta, NAPI_AUTO_LENGTH, &meta);
        napi_set_named_property(env, js_result, "meta", meta);
    }
    // ... set other fields

    meta_oxide_result_free(result);
    return js_result;
}

napi_value Init(napi_env env, napi_value exports) {
    napi_value fn;
    napi_create_function(env, nullptr, 0, ExtractAll, nullptr, &fn);
    napi_set_named_property(env, exports, "extractAll", fn);
    return exports;
}

NAPI_MODULE(NODE_GYP_MODULE_NAME, Init)
```

### Python (ctypes)

**File: meta_oxide.py**
```python
import ctypes
import json
from typing import Optional, Dict, Any, List
from dataclasses import dataclass

# Load library
lib = ctypes.CDLL("libmeta_oxide.so")

# Define types
class CMetaOxideResult(ctypes.Structure):
    _fields_ = [
        ("meta", ctypes.c_char_p),
        ("open_graph", ctypes.c_char_p),
        ("twitter", ctypes.c_char_p),
        ("json_ld", ctypes.c_char_p),
        ("microdata", ctypes.c_char_p),
        ("microformats", ctypes.c_char_p),
        ("rdfa", ctypes.c_char_p),
        ("dublin_core", ctypes.c_char_p),
        ("manifest", ctypes.c_char_p),
        ("oembed", ctypes.c_char_p),
        ("rel_links", ctypes.c_char_p),
    ]

# Declare functions
lib.meta_oxide_extract_all.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
lib.meta_oxide_extract_all.restype = ctypes.POINTER(CMetaOxideResult)
lib.meta_oxide_result_free.argtypes = [ctypes.POINTER(CMetaOxideResult)]
lib.meta_oxide_last_error.restype = ctypes.c_int
lib.meta_oxide_error_message.restype = ctypes.c_char_p

@dataclass
class MetaOxideResult:
    meta: Optional[Dict[str, Any]] = None
    open_graph: Optional[Dict[str, Any]] = None
    twitter: Optional[Dict[str, Any]] = None
    json_ld: Optional[List[Dict[str, Any]]] = None
    microdata: Optional[List[Dict[str, Any]]] = None
    microformats: Optional[Dict[str, Any]] = None
    rdfa: Optional[List[Dict[str, Any]]] = None
    dublin_core: Optional[Dict[str, Any]] = None
    manifest: Optional[Dict[str, Any]] = None
    oembed: Optional[Dict[str, Any]] = None
    rel_links: Optional[Dict[str, Any]] = None

def extract_all(html: str, base_url: Optional[str] = None) -> MetaOxideResult:
    html_bytes = html.encode('utf-8')
    base_url_bytes = base_url.encode('utf-8') if base_url else None

    c_result = lib.meta_oxide_extract_all(html_bytes, base_url_bytes)
    if not c_result:
        error_code = lib.meta_oxide_last_error()
        error_msg = lib.meta_oxide_error_message().decode('utf-8')
        raise RuntimeError(f"Error {error_code}: {error_msg}")

    try:
        result = MetaOxideResult()
        if c_result.contents.meta:
            result.meta = json.loads(c_result.contents.meta.decode('utf-8'))
        if c_result.contents.open_graph:
            result.open_graph = json.loads(c_result.contents.open_graph.decode('utf-8'))
        # ... parse other fields
        return result
    finally:
        lib.meta_oxide_result_free(c_result)
```

## Testing Your Binding

### Minimum Test Suite

1. **Basic extraction test**
   ```
   Test extract_all with simple HTML
   Verify all fields are accessible
   ```

2. **NULL handling**
   ```
   Pass NULL/nil for optional parameters
   Verify it doesn't crash
   ```

3. **Error handling**
   ```
   Pass invalid input
   Verify error is caught and reported
   ```

4. **Memory leak test**
   ```
   Call extract_all 10,000 times
   Verify memory doesn't grow
   ```

5. **Unicode test**
   ```
   Extract from HTML with unicode characters
   Verify correct encoding
   ```

6. **Concurrent test**
   ```
   Call from multiple threads
   Verify thread safety
   ```

### Reference Test HTML

Use the same test HTML across all bindings for consistency:

```html
<html>
<head>
  <title>Test Page</title>
  <meta name="description" content="Test description">
  <meta property="og:title" content="OG Title">
  <meta name="twitter:card" content="summary">
  <script type="application/ld+json">
  {"@type": "Article", "headline": "Test"}
  </script>
</head>
<body>
  <div class="h-card">
    <span class="p-name">John Doe</span>
  </div>
</body>
</html>
```

**Expected Results:**
- `meta.title` = "Test Page"
- `meta.description` = "Test description"
- `open_graph["og:title"]` = "OG Title"
- `twitter["twitter:card"]` = "summary"
- `json_ld[0]["@type"]` = "Article"
- `microformats["h-card"][0].name` = "John Doe"

## Best Practices

### 1. Hide Low-Level Details

Don't expose C pointers in your high-level API:

**Bad:**
```go
// Exposes C implementation details
func ExtractMeta(html string) *C.char { /* ... */ }
```

**Good:**
```go
// Clean, idiomatic interface
func ExtractMeta(html string) (map[string]interface{}, error) { /* ... */ }
```

### 2. Parse JSON Once

Don't return raw JSON strings - parse them into native types:

**Bad:**
```javascript
// Returns raw JSON string
extractAll(html)  // => '{"meta": {...}}'
```

**Good:**
```javascript
// Returns parsed object
extractAll(html)  // => { meta: {...} }
```

### 3. Use Finalizers/Destructors

Automatically free C memory when binding objects are garbage collected:

```python
def __del__(self):
    if self._ptr:
        lib.meta_oxide_result_free(self._ptr)
```

### 4. Document Memory Safety

Make it clear to users that they don't need to worry about memory:

```go
// ExtractAll extracts all metadata from HTML.
// Memory is automatically managed - no manual cleanup needed.
func ExtractAll(html, baseURL string) (*Result, error)
```

### 5. Match Language Conventions

- **Go:** Return `(value, error)`, use `CamelCase`
- **Node.js:** Use callbacks or promises, `camelCase`
- **Python:** Raise exceptions, use `snake_case`
- **Java:** Use exceptions, `camelCase`
- **C#:** Use exceptions, `PascalCase`

### 6. Provide Type Definitions

Give users IDE autocompletion and type checking:

**TypeScript:**
```typescript
interface MetaOxideResult {
  meta?: MetaTags;
  openGraph?: OpenGraphData;
  twitter?: TwitterCard;
  jsonLd?: JsonLdObject[];
  // ...
}

export function extractAll(html: string, baseUrl?: string): MetaOxideResult;
```

### 7. Version Check

Verify compatible C library version at runtime:

```python
MIN_VERSION = "0.1.3"
lib_version = lib.meta_oxide_version().decode('utf-8')
if lib_version < MIN_VERSION:
    raise RuntimeError(f"Incompatible library version: {lib_version}")
```

## Distribution

### Bundling the C Library

**Option 1: System Library**
- User installs `libmeta_oxide` separately
- Binding finds it via standard paths

**Option 2: Bundle Library**
- Include platform-specific libraries in package
- Load from package directory at runtime

**Option 3: Download on Install**
- Fetch library during package installation
- Cache locally

### Platform Support

Provide binaries for:
- Linux x86_64
- Linux aarch64
- macOS x86_64 (Intel)
- macOS arm64 (Apple Silicon)
- Windows x86_64

## Debugging

### Enable C Library Logs

Set environment variable:
```bash
RUST_LOG=debug ./your_program
```

### Check ABI Compatibility

Verify struct sizes match:
```c
printf("sizeof(MetaOxideResult) = %zu\n", sizeof(MetaOxideResult));
// Should be 88 bytes on 64-bit systems (11 pointers × 8 bytes)
```

### Use Valgrind

Check for memory leaks:
```bash
valgrind --leak-check=full ./your_test_program
```

## Versioning Your Binding

- **Match major version** with C library (e.g., binding 1.x.x for library 1.x.x)
- **Independent minor/patch versions** for binding-specific fixes
- **Document minimum C library version** required

Example:
```
Binding Version: 1.2.3
Requires: libmeta_oxide >= 1.0.0, < 2.0.0
```

## Contributing

When contributing a new binding:

1. Create a `bindings/<language>/` directory
2. Include comprehensive tests
3. Add CI/CD for your language
4. Update main README with link to your binding
5. Follow this guide's best practices

## Support

- C API Issues: https://github.com/yfedoseev/meta_oxide/issues
- Binding-Specific Issues: Use your binding's repository

## License

Your binding can use any license, but we recommend matching the library's MIT OR Apache-2.0 license.
