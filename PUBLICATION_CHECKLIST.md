# MetaOxide v0.1.0 Publication Checklist

**Status**: Ready for Publication
**Last Updated**: November 25, 2025
**Version**: 0.1.0

---

## Pre-Publication Requirements

### ✅ Code & Repository
- [x] All code committed
- [x] Working directory clean
- [x] Git history clean
- [x] No sensitive information in commits
- [x] .gitignore configured properly
- [x] No build artifacts in repository

### ✅ Package Configuration
- [x] Cargo.toml configured correctly
  - [x] Name: `meta_oxide`
  - [x] Version: 0.1.0
  - [x] License: MIT OR Apache-2.0
  - [x] Repository URL: https://github.com/yfedoseev/meta_oxide
  - [x] Keywords and categories updated

- [x] pyproject.toml configured correctly
  - [x] Name: `meta-oxide`
  - [x] Version: 0.1.0
  - [x] License: MIT OR Apache-2.0
  - [x] Repository URLs updated
  - [x] Author information correct
  - [x] Keywords updated

### ✅ Documentation
- [x] README.md complete and accurate
- [x] CHANGELOG.md with v0.1.0 entry
- [x] PUBLISHING_GUIDE.md created
- [x] CONTRIBUTING.md present
- [x] CODE_OF_CONDUCT.md present
- [x] SECURITY.md present
- [x] LICENSE-MIT present
- [x] LICENSE-APACHE present
- [x] Getting Started guides (7 languages)
- [x] API References (7 languages)
- [x] RELEASE_NOTES_v0.1.0.md complete

### ✅ Tests
- [x] All tests passing
- [x] Test coverage 95%+
- [x] No compiler warnings
- [x] No clippy warnings

### ✅ Licenses
- [x] LICENSE-MIT file present and correct
- [x] LICENSE-APACHE file present and correct
- [x] License headers in source files (or clearly stated in README)
- [x] LICENSES/ directory (if needed)

---

## Publishing Checklist

### Step 1: GitHub Repository
**Estimated Time**: 5-10 minutes

**Before**:
- [ ] Have GitHub account with repository creation permissions
- [ ] Have git configured locally
- [ ] Have ssh keys configured (if using SSH) or GitHub token (if using HTTPS)

**Actions**:
- [ ] Create repository: https://github.com/new
  - [ ] Repository name: `meta-oxide`
  - [ ] Visibility: **Public**
  - [ ] Do NOT initialize repository
- [ ] Add GitHub remote:
  ```bash
  git remote add origin https://github.com/yfedoseev/meta-oxide.git
  ```
- [ ] Push main branch:
  ```bash
  git push -u origin main
  ```
- [ ] Create and push v0.1.0 tag:
  ```bash
  git tag -a v0.1.0 -m "Release v0.1.0"
  git push origin v0.1.0
  ```
- [ ] Create GitHub Release:
  - [ ] Go to https://github.com/yfedoseev/meta-oxide/releases/new
  - [ ] Select tag: v0.1.0
  - [ ] Title: "MetaOxide v0.1.0 - Universal Metadata Extraction Library"
  - [ ] Description: Paste RELEASE_NOTES_v0.1.0.md content
  - [ ] Publish release

**Verification**:
- [ ] Repository is public
- [ ] All files visible on GitHub
- [ ] README.md displays correctly
- [ ] Release page is complete

---

### Step 2: crates.io (Rust)
**Estimated Time**: 10-15 minutes

**Before**:
- [ ] Have crates.io account
- [ ] Have crates.io API token
- [ ] Rust installed and updated (`rustup update stable`)

**Actions**:
- [ ] Login to crates.io:
  ```bash
  cargo login
  ```
  Paste your API token when prompted

- [ ] Verify package:
  ```bash
  cargo package --list
  ```

- [ ] Test publish (dry-run):
  ```bash
  cargo publish --dry-run
  ```

- [ ] If dry-run succeeds, publish:
  ```bash
  cargo publish
  ```

**Verification**:
- [ ] Package appears on https://crates.io/crates/meta_oxide
- [ ] Documentation generated on https://docs.rs/meta_oxide
- [ ] Installation works:
  ```bash
  cargo install meta_oxide --version 0.1.0
  ```

---

### Step 3: PyPI (Python)
**Estimated Time**: 15-20 minutes

**Before**:
- [ ] Have PyPI account
- [ ] Have PyPI API token
- [ ] Python 3.8+ installed
- [ ] Build tools installed:
  ```bash
  pip install --upgrade pip build twine
  ```

**Actions**:
- [ ] Create ~/.pypirc file:
  ```bash
  cat > ~/.pypirc << 'EOF'
  [distutils]
  index-servers =
      pypi

  [pypi]
  repository = https://upload.pypi.org/legacy/
  username = __token__
  password = pypi-AgE...  # Your PyPI token
  EOF
  chmod 600 ~/.pypirc
  ```

- [ ] Clean previous builds:
  ```bash
  rm -rf build/ dist/ *.egg-info
  ```

- [ ] Build packages:
  ```bash
  python -m build
  ```

- [ ] Verify distributions:
  ```bash
  twine check dist/*
  ```

- [ ] Upload to PyPI:
  ```bash
  twine upload dist/*
  ```

**Verification**:
- [ ] Package appears on https://pypi.org/project/meta-oxide/
- [ ] Package information displays correctly
- [ ] Installation works:
  ```bash
  pip install meta-oxide==0.1.0
  python -c "import meta_oxide; print('Success!')"
  ```

---

## Post-Publication Tasks

### Immediate (Day 1)
- [ ] Verify all three platforms (GitHub, crates.io, PyPI)
- [ ] Test installations from all platforms
- [ ] Announce on Rust forum (https://users.rust-lang.org/)
- [ ] Announce on Python community (if applicable)
- [ ] Share on Twitter/social media

### Week 1
- [ ] Monitor downloads and feedback
- [ ] Respond to any issues or questions
- [ ] Create blog post (optional)
- [ ] Share in relevant communities

### Ongoing
- [ ] Monitor GitHub issues
- [ ] Plan v0.2.0 based on feedback
- [ ] Keep documentation updated
- [ ] Maintain security.md

---

## Troubleshooting

### cargo publish issues
If `cargo publish` fails:
1. Check that you're logged in: `cargo login`
2. Verify Cargo.toml is correct
3. Check for docs test errors: `cargo test --doc`
4. Try again with: `cargo publish`

### twine upload issues
If `twine upload` fails:
1. Check .pypirc credentials
2. Verify package was built: `ls dist/`
3. Try checking: `twine check dist/*`
4. Regenerate PyPI token if needed

### Git push issues
If `git push` fails:
1. Check remote: `git remote -v`
2. Verify credentials (SSH key or GitHub token)
3. Ensure you have push permissions
4. Try: `git push -u origin main`

---

## Safety Checklist

**Security**:
- [ ] API tokens NOT committed to repository
- [ ] .pypirc file is NOT in repository
- [ ] ~/.cargo/credentials.toml is NOT shared
- [ ] SSH keys remain private

**Immutability**:
- [ ] Understand that published versions cannot be changed
- [ ] Plan for v0.1.1 if critical bug is found
- [ ] Keep old versions in CHANGELOG

**Backup**:
- [ ] Repository backed up (GitHub is your backup)
- [ ] Commit history preserved
- [ ] Original source code safe

---

## Success Criteria

### GitHub ✅
- Repository is public
- All files visible
- README displays correctly
- Release page has detailed notes
- Community can clone/fork

### crates.io ✅
- Package visible: https://crates.io/crates/meta_oxide
- Documentation: https://docs.rs/meta_oxide
- Installation works: `cargo install meta_oxide`
- Download statistics visible

### PyPI ✅
- Package visible: https://pypi.org/project/meta-oxide/
- Information displays correctly
- Installation works: `pip install meta-oxide`
- Download statistics visible

---

## Timeline Estimate

| Step | Time | Status |
|------|------|--------|
| GitHub setup | 5-10 min | ⏳ Pending |
| crates.io | 10-15 min | ⏳ Pending |
| PyPI | 15-20 min | ⏳ Pending |
| Verification | 10 min | ⏳ Pending |
| **Total** | **40-55 min** | ⏳ Pending |

---

## Quick Commands

```bash
# All-in-one publishing
./scripts/publish.sh all

# Or step-by-step:
./scripts/publish.sh github  # GitHub only
./scripts/publish.sh crates  # crates.io only
./scripts/publish.sh pypi    # PyPI only
```

---

## Final Verification

Run this after all publications:

```bash
# GitHub
git remote -v
gh repo view yfedoseev/meta-oxide

# crates.io
cargo search meta_oxide
cargo install meta_oxide --version 0.1.0

# PyPI
pip install meta-oxide==0.1.0
python -c "import meta_oxide; print('✅ Installation successful!')"
```

---

## Ready? Start Here:

1. **Read**: [PUBLISHING_GUIDE.md](PUBLISHING_GUIDE.md)
2. **Prepare**: Complete the "Before" sections above
3. **Execute**: Run `./scripts/publish.sh all` or step-by-step
4. **Verify**: Test installations from each platform
5. **Announce**: Share with communities
6. **Monitor**: Watch for feedback

---

**Last Updated**: November 25, 2025
**MetaOxide v0.1.0 - Ready for Publication** 🚀
