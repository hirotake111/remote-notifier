#!/bin/bash

set -e

if [ -z "$1" ]; then
    echo "Usage: ./release <version>"
    echo "Example: ./release v0.1.1"
    exit 1
fi

VERSION="$1"

# Update version in Cargo.toml
sed -i "s/^version = \".*\"/version = \"${VERSION#v}\"/" Cargo.toml

# Commit changes
git add Cargo.toml
git commit -m "Release $VERSION"

# Create and push tag
git tag "$VERSION"
git push origin "$VERSION"

echo "Released $VERSION"
