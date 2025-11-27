#!/bin/bash

# MetaOxide v0.1.2 Publishing Script
# Quick reference for publishing to GitHub, crates.io, and PyPI
#
# Usage: ./scripts/publish.sh [step]
# Where step is: github, crates, pypi, all, or nothing (shows help)

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="0.1.2"
REPO_URL="https://github.com/yfedoseev/meta-oxide"

echo "🚀 MetaOxide v${VERSION} Publishing Script"
echo "================================================"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

show_help() {
    echo ""
    echo "Usage: ./scripts/publish.sh [command]"
    echo ""
    echo "Commands:"
    echo "  github   - Push to GitHub and create release"
    echo "  crates   - Publish to crates.io (Rust)"
    echo "  pypi     - Publish to PyPI (Python)"
    echo "  all      - Run all publishing steps"
    echo "  help     - Show this help message"
    echo ""
    echo "Example:"
    echo "  ./scripts/publish.sh github    # Push to GitHub"
    echo "  ./scripts/publish.sh crates    # Publish to crates.io"
    echo "  ./scripts/publish.sh pypi      # Publish to PyPI"
    echo "  ./scripts/publish.sh all       # Do all of the above"
    echo ""
}

publish_github() {
    echo -e "${YELLOW}📤 Step 1: Publishing to GitHub${NC}"
    cd "${PROJECT_DIR}"

    # Check if remote exists
    if ! git remote | grep -q origin; then
        echo -e "${YELLOW}Adding GitHub remote...${NC}"
        git remote add origin "${REPO_URL}.git"
    fi

    echo "Pushing to GitHub..."
    git push -u origin main

    echo "Creating release tag..."
    if ! git rev-parse "${VERSION}" >/dev/null 2>&1; then
        git tag -a "v${VERSION}" -m "Release v${VERSION} - Universal metadata extraction library"
        git push origin "v${VERSION}"
    else
        echo -e "${YELLOW}Tag v${VERSION} already exists, skipping tag creation${NC}"
    fi

    echo -e "${GREEN}✅ GitHub publishing complete!${NC}"
    echo "   Repository: ${REPO_URL}"
    echo ""
}

publish_crates() {
    echo -e "${YELLOW}📤 Step 2: Publishing to crates.io${NC}"
    cd "${PROJECT_DIR}"

    # Check if cargo is installed
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}❌ Cargo not found. Please install Rust:${NC}"
        echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        return 1
    fi

    # Check if logged in
    if [ ! -f ~/.cargo/credentials.toml ]; then
        echo -e "${YELLOW}Not logged in to crates.io. Running cargo login...${NC}"
        cargo login
    fi

    echo "Verifying package..."
    cargo package --list

    echo "Running publish dry-run..."
    cargo publish --dry-run

    echo -n "Ready to publish to crates.io? (yes/no): "
    read -r response
    if [ "$response" = "yes" ]; then
        cargo publish
        echo -e "${GREEN}✅ Crates.io publishing complete!${NC}"
        echo "   Package: https://crates.io/crates/meta_oxide"
        echo "   Docs: https://docs.rs/meta_oxide"
    else
        echo "Aborted."
    fi
    echo ""
}

publish_pypi() {
    echo -e "${YELLOW}📤 Step 3: Publishing to PyPI${NC}"
    cd "${PROJECT_DIR}"

    # Check if Python is installed
    if ! command -v python &> /dev/null; then
        echo -e "${RED}❌ Python not found. Please install Python 3.8+${NC}"
        return 1
    fi

    # Check if build tools are installed
    if ! python -m pip show build &> /dev/null; then
        echo -e "${YELLOW}Installing build tools...${NC}"
        python -m pip install --upgrade build twine
    fi

    echo "Cleaning previous builds..."
    rm -rf build/ dist/ *.egg-info

    echo "Building distribution packages..."
    python -m build

    echo "Verifying distributions..."
    twine check dist/*

    echo -n "Ready to publish to PyPI? (yes/no): "
    read -r response
    if [ "$response" = "yes" ]; then
        twine upload dist/*
        echo -e "${GREEN}✅ PyPI publishing complete!${NC}"
        echo "   Package: https://pypi.org/project/meta-oxide/"
    else
        echo "Aborted."
    fi
    echo ""
}

publish_all() {
    publish_github
    publish_crates
    publish_pypi

    echo -e "${GREEN}═════════════════════════════════════════${NC}"
    echo -e "${GREEN}✅ All publishing steps complete!${NC}"
    echo -e "${GREEN}═════════════════════════════════════════${NC}"
    echo ""
    echo "Next steps:"
    echo "1. Verify publications on each platform"
    echo "2. Share the release with the community"
    echo "3. Monitor downloads and feedback"
    echo ""
}

# Main
case "${1:-help}" in
    github)
        publish_github
        ;;
    crates)
        publish_crates
        ;;
    pypi)
        publish_pypi
        ;;
    all)
        publish_all
        ;;
    help|"")
        show_help
        ;;
    *)
        echo -e "${RED}Unknown command: ${1}${NC}"
        show_help
        exit 1
        ;;
esac
