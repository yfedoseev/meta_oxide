#!/bin/bash
# Extracts release notes for a given version from CHANGELOG.md
# Usage: extract-release-notes.sh <version>
# Outputs:
#   release-title.txt  — "v0.1.3 | Subtitle…"
#   release-notes.md   — Full body (changelog section + installation footer)

set -euo pipefail

VERSION="$1"
CHANGELOG="CHANGELOG.md"

if [ ! -f "$CHANGELOG" ]; then
  echo "Error: $CHANGELOG not found" >&2
  exit 1
fi

# Extract subtitle from "> ..." line after the version header (if present)
SUBTITLE=$(awk "/^## \[${VERSION}\]/{found=1; next} found && /^>/{gsub(/^> */, \"\"); print; exit}" "$CHANGELOG")

if [ -n "$SUBTITLE" ]; then
  echo "v${VERSION} | ${SUBTITLE}" > release-title.txt
else
  echo "v${VERSION}" > release-title.txt
fi

# Extract body: everything between this version's ## header and the next ## header
awk "/^## \[${VERSION}\]/{flag=1; next} /^## \[/{flag=0} flag" "$CHANGELOG" \
  | sed '/^> /d' \
  | sed '1{/^$/d}' > changelog-section.md

if [ ! -s changelog-section.md ]; then
  echo "Warning: No changelog content found for version ${VERSION}" >&2
fi

# Assemble release body = changelog section + language-specific install footer
cat changelog-section.md > release-notes.md
cat >> release-notes.md << 'FOOTER'

---

### Installation

**Rust (crates.io)**
```bash
cargo add meta_oxide
```

**Python (PyPI)**
```bash
pip install meta-oxide
```

**Node.js (npm)**
```bash
npm install meta-oxide
```

**WebAssembly (npm)**
```bash
npm install meta-oxide-wasm
```

**Go**
```bash
go get github.com/yfedoseev/meta-oxide-go
```

**Java (Maven)**
```xml
<dependency>
  <groupId>com.metaoxide</groupId>
  <artifactId>meta-oxide</artifactId>
  <version>VERSION_PLACEHOLDER</version>
</dependency>
```

**C# (NuGet)**
```bash
dotnet add package MetaOxide --version VERSION_PLACEHOLDER
```

### Platform Support (Python wheels)
| Platform | Architecture |
|----------|-------------|
| Linux    | x86_64 (glibc + musl) |
| Linux    | aarch64 |
| macOS    | x86_64 (Intel) |
| macOS    | aarch64 (Apple Silicon) |
| Windows  | x86_64 |

### Changelog
See [CHANGELOG.md](https://github.com/yfedoseev/meta_oxide/blob/main/CHANGELOG.md) for full details.
FOOTER

# Fill in the version where the footer templated it
sed -i.bak "s/VERSION_PLACEHOLDER/${VERSION}/g" release-notes.md && rm release-notes.md.bak

rm -f changelog-section.md

echo "Generated release-title.txt and release-notes.md for v${VERSION}"
