#!/bin/bash
set -e

TARGET=""
case "$GOOS" in
    linux)
        case "$GOARCH" in
            amd64) TARGET="x86_64-unknown-linux-gnu" ;;
            arm64) TARGET="aarch64-unknown-linux-gnu" ;;
        esac
        ;;
    darwin)
        case "$GOARCH" in
            amd64) TARGET="x86_64-apple-darwin" ;;
            arm64) TARGET="aarch64-apple-darwin" ;;
        esac
        ;;
esac

if [ -z "$TARGET" ]; then
    echo "Unknown GOOS/GOARCH: $GOOS/$GOARCH"
    exit 1
fi

echo "Building for target: $TARGET"
cross build --release --target "$TARGET"
