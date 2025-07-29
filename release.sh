#!/bin/bash

set -e

APP_NAME="eanray"
VERSION="v0.1.0"
TARGET_DIR="target/release"
RELEASE_DIR="$APP_NAME-$VERSION"
DIST_DIR="$RELEASE_DIR/dist"
EXAMPLES_DIR="examples"

# Build the binary
cargo build --release

# Setup for the release folder
rm -rf "$RELEASE_DIR"
mkdir "$RELEASE_DIR"

cp "$TARGET_DIR/$APP_NAME" "$DIST_DIR"
cp config.toml "$DIST_DIR"
cp README.md "$DIST_DIR"
cp LICENSE "$DIST_DIR"
cp -r "$EXAMPLES_DIR" "$RELEASE_DIR"
rm "$RELEASE_DIR/$EXAMPLES_DIR/scratch.lua"

ZIP_FILE="$RELEASE_DIR.zip"
zip -r "$ZIP_FILE" "$RELEASE_DIR"

echo "Created $ZIP_FILE"