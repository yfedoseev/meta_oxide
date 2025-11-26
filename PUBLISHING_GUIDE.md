# MetaOxide v0.1.0 Publishing Guide

**Version**: 0.1.0
**Release Date**: November 25, 2025
**Status**: Ready for Public Release

This guide covers the complete process of publishing MetaOxide to:
1. GitHub (public repository)
2. crates.io (Rust package registry)
3. PyPI (Python package registry)

---

## 📋 Pre-Publishing Checklist

### Local Setup Verification
- [x] Repository is clean and ready
- [x] All code committed
- [x] Package metadata updated (Cargo.toml, pyproject.toml)
- [x] Licenses included (MIT, Apache-2.0)
- [x] README.md present and complete
- [x] CHANGELOG.md up to date
- [x] Documentation complete

### Credentials Required
- [ ] GitHub account with repository creation permissions
- [ ] Crates.io account with API token
- [ ] PyPI account with API token
- [ ] Git configured locally

---

## 🚀 Step 1: Create GitHub Repository

### Option A: Using GitHub Web UI

1. Go to https://github.com/new
2. **Repository name**: `meta-oxide`
3. **Description**: "Universal metadata extraction library supporting 13 formats with 7 language bindings"
4. **Visibility**: **Public** ✅
5. **Initialize repository**: **Do NOT** initialize (we have code ready)
6. Click "Create repository"

### Option B: Using GitHub CLI

```bash
gh repo create meta-oxide \
  --public \
  --description "Universal metadata extraction library supporting 13 formats with 7 language bindings" \
  --source=. \
  --remote=origin \
  --push
```

---

## 📤 Step 2: Push to GitHub

### Add Remote and Push

```bash
cd /home/yfedoseev/projects/meta_oxide

# Add GitHub remote
git remote add origin https://github.com/yfedoseev/meta-oxide.git

# Verify remote
git remote -v

# Push main branch
git push -u origin main

# Push tags (if any)
git push origin --tags
```

### Verify on GitHub

1. Visit https://github.com/yfedoseev/meta-oxide
2. Verify all files are present
3. Check README.md displays correctly
4. Confirm repository is public

---

## 🏷️ Step 3: Create Release Tag

### Create v0.1.0 Tag

```bash
# If tag doesn't exist, create it
git tag -a v0.1.0 -m "Release v0.1.0 - Universal metadata extraction library"

# Push tag to GitHub
git push origin v0.1.0

# Verify tag
git tag -l -n5 v0.1.0
```

### Create GitHub Release

**Using GitHub Web UI**:
1. Go to https://github.com/yfedoseev/meta-oxide/releases
2. Click "Create a new release"
3. Select tag: `v0.1.0`
4. Title: "MetaOxide v0.1.0 - Universal Metadata Extraction Library"
5. Description: Paste content from [RELEASE_NOTES_v0.1.0.md](RELEASE_NOTES_v0.1.0.md)
6. Click "Publish release"

**Using GitHub CLI**:
```bash
gh release create v0.1.0 \
  --title "MetaOxide v0.1.0 - Universal Metadata Extraction Library" \
  --notes-file RELEASE_NOTES_v0.1.0.md
```

---

## 📦 Step 4: Publish to crates.io (Rust)

### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update Rust
rustup update stable

# Verify installation
cargo --version
```

### Create crates.io Account

1. Go to https://crates.io
2. Click "Login" → "GitHub"
3. Authorize the application
4. Visit https://crates.io/me and click "Generate new token"
5. Copy the API token

### Configure Cargo

```bash
# Login to crates.io with your API token
cargo login

# When prompted, paste your crates.io API token
# This saves token to ~/.cargo/credentials.toml
```

### Publish to crates.io

```bash
cd /home/yfedoseev/projects/meta_oxide

# Verify package contents
cargo package --list

# Do a dry-run first (recommended)
cargo publish --dry-run

# Publish to crates.io
cargo publish

# Watch the progress in terminal
# Publishing typically takes 1-2 minutes
```

### Verify Publication

```bash
# Check crates.io
# https://crates.io/crates/meta_oxide

# Or use cargo
cargo search meta_oxide

# Test installation
cargo install meta_oxide --version 0.1.0
```

---

## 🐍 Step 5: Publish to PyPI (Python)

### Prerequisites

```bash
# Ensure you have Python 3.8+
python --version

# Install build tools
pip install --upgrade pip build twine maturin

# Verify installations
python -m build --version
twine --version
```

### Create PyPI Account

1. Go to https://pypi.org/account/register/
2. Create a new account (or login if you have one)
3. Set up Two-Factor Authentication (recommended)
4. Visit https://pypi.org/manage/account/token/
5. Create a new API token (scope: "Entire account")
6. Copy the token (you'll only see it once!)

### Store PyPI Credentials

**Option A: Using .pypirc** (Recommended)

```bash
# Create or edit ~/.pypirc
cat > ~/.pypirc << 'EOF'
[distutils]
index-servers =
    pypi

[pypi]
repository = https://upload.pypi.org/legacy/
username = __token__
password = pypi-AgEIcHlwaS5vcmc...  # Your token from PyPI
EOF

# Set restrictive permissions
chmod 600 ~/.pypirc
```

**Option B: Using keyring**

```bash
# Install keyring
pip install keyring

# Store credentials
keyring set https://upload.pypi.org/legacy/ __token__
# When prompted, paste your PyPI token
```

### Build Python Package

```bash
cd /home/yfedoseev/projects/meta_oxide

# Clean previous builds
rm -rf build/ dist/ *.egg-info

# Build wheel and source distribution
python -m build

# Verify built files
ls -lah dist/

# Should create:
# - dist/meta_oxide-0.1.0-cp*.whl (wheels for different Python versions)
# - dist/meta-oxide-0.1.0.tar.gz (source distribution)
```

### Upload to PyPI

```bash
# Check distribution first (do a dry-run)
twine check dist/*

# Upload to PyPI
twine upload dist/*

# When prompted for credentials:
# Username: __token__
# Password: [paste your PyPI token]
```

### Verify Publication

```bash
# Check PyPI
# https://pypi.org/project/meta-oxide/

# Test installation
pip install meta-oxide==0.1.0

# Test in Python
python -c "import meta_oxide; print(meta_oxide.__version__)"
```

---

## ✅ Post-Publication Verification

### Verify All Publications

```bash
# Check crates.io
curl https://crates.io/api/v1/crates/meta_oxide | jq '.crate.max_version'

# Check PyPI
pip index versions meta-oxide

# Verify GitHub
gh repo view yfedoseev/meta-oxide

# Test installations
cargo install meta-oxide --version 0.1.0
pip install meta-oxide==0.1.0
```

### Create Documentation

1. **GitHub**:
   - Add GitHub Pages (optional)
   - Enable discussions
   - Set up security advisories

2. **crates.io**:
   - Documentation auto-generated from doc comments
   - Visit: https://docs.rs/meta_oxide

3. **PyPI**:
   - Documentation auto-generated from README
   - Visit: https://meta-oxide.readthedocs.io (optional, requires ReadTheDocs)

---

## 📊 Publication Summary

### crates.io

| Item | Status |
|------|--------|
| **Package Name** | `meta_oxide` |
| **Version** | 0.1.0 |
| **License** | MIT OR Apache-2.0 |
| **Documentation** | https://docs.rs/meta_oxide |
| **Repository** | https://github.com/yfedoseev/meta-oxide |
| **Keywords** | metadata, microformats, html, json-ld, parser |

### PyPI

| Item | Status |
|------|--------|
| **Package Name** | `meta-oxide` |
| **Version** | 0.1.0 |
| **License** | MIT OR Apache-2.0 |
| **Documentation** | https://github.com/yfedoseev/meta-oxide/tree/main/docs |
| **Repository** | https://github.com/yfedoseev/meta-oxide |
| **Python Support** | 3.8, 3.9, 3.10, 3.11, 3.12, 3.13 |

### GitHub

| Item | Details |
|------|---------|
| **Repository** | https://github.com/yfedoseev/meta-oxide |
| **Visibility** | Public |
| **Issues** | Enabled |
| **Discussions** | Enabled |
| **Releases** | v0.1.0 |

---

## 🔄 Multi-Language Bindings Publishing

After Rust and Python are published, consider publishing to other registries:

### Go
```bash
# Automatic via GitHub tags
# Users install with: go get github.com/yfedoseev/meta-oxide-go@v0.1.0
```

### Node.js/WASM
```bash
# npm
npm publish

# For scoped packages:
npm publish --access public
```

### Java
```bash
# Maven Central (requires Sonatype account)
cd bindings/java/meta-oxide-java
mvn clean deploy
```

### C#/.NET
```bash
# NuGet
dotnet nuget push bin/Release/MetaOxide.0.1.0.nupkg --api-key [key] --source https://api.nuget.org/v3/index.json
```

---

## ⚠️ Important Notes

### API Tokens Security
- **Never commit** API tokens to version control
- Use environment variables or `.pypirc` file
- Keep tokens private and secure
- Regenerate tokens if exposed

### Version Numbers
- Once published, versions are **immutable**
- Cannot re-publish the same version
- For changes, use v0.1.1, v0.2.0, etc.
- Follow semantic versioning

### Publishing Order
1. **GitHub** (source code must be public first)
2. **crates.io** (Rust users often check source on GitHub)
3. **PyPI** (Python packages)
4. Other registries (npm, Maven, NuGet, etc.)

### Rollback Strategy
- If critical bug found after publishing:
  1. Publish v0.1.1 with fix immediately
  2. Document issue on GitHub
  3. Update SECURITY.md if needed
  4. Mark v0.1.0 as deprecated in release notes

---

## 📝 Troubleshooting

### Cargo Publish Issues

**Problem**: "authentication required"
```bash
# Solution: Re-login
cargo login
```

**Problem**: "forbidden: crate `meta_oxide` is already reserved"
```bash
# Solution: Use exact crate name from Cargo.toml
# Verify Cargo.toml has correct [package] name
```

**Problem**: "documentation tests failed"
```bash
# Solution: Run tests before publishing
cargo test --all-features
cargo test --doc
```

### PyPI Upload Issues

**Problem**: "403 Forbidden: Invalid or expired authentication token"
```bash
# Solution: Generate new token from https://pypi.org/manage/account/token/
rm ~/.pypirc
python -m keyring set https://upload.pypi.org/legacy/ __token__
```

**Problem**: "The file meta_oxide-0.1.0.tar.gz already exists"
```bash
# Solution: PyPI doesn't allow re-uploads
# Use a new version (0.1.1) if changes are needed
```

**Problem**: "twine: command not found"
```bash
# Solution: Install twine
pip install twine
```

---

## ✨ Next Steps After Publication

1. **Monitor Downloads**: Watch crates.io and PyPI stats
2. **Gather Feedback**: Read issues and discussions
3. **Plan v0.2.0**: Based on feedback and roadmap
4. **Community Engagement**: Share on Rust/Python forums
5. **Blog Post**: Announce the release (optional)

---

## 🎯 Success Criteria

✅ **GitHub**:
- Repository is public
- All files visible
- README displays correctly
- Release page has detailed notes

✅ **crates.io**:
- Package visible on https://crates.io/crates/meta_oxide
- Documentation generated on https://docs.rs/meta_oxide
- Installation works: `cargo install meta_oxide`

✅ **PyPI**:
- Package visible on https://pypi.org/project/meta-oxide/
- Installation works: `pip install meta-oxide`
- Package information displays correctly

---

## 📞 Support

If you encounter issues during publication:

1. **Rust/crates.io**: https://internals.rust-lang.org/
2. **Python/PyPI**: https://mail.python.org/pipermail/distutils-sig/
3. **GitHub**: https://github.com/yfedoseev/meta-oxide/issues
4. **MetaOxide Documentation**: See [README.md](README.md)

---

**Ready to publish? Start with Step 1! 🚀**

Last Updated: November 25, 2025
