#!/bin/bash
set -euo pipefail

APP_NAME="DeepSeekBalance"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR/DeepSeekBalance"
BUILD_DIR="$SCRIPT_DIR/.build"
APP_DIR="$BUILD_DIR/$APP_NAME.app"

echo "==> Cleaning..."
rm -rf "$APP_DIR"

echo "==> Creating app bundle..."
mkdir -p "$APP_DIR/Contents/MacOS"
mkdir -p "$APP_DIR/Contents/Resources"

echo "==> Compiling Swift sources (arm64, optimized)..."
swiftc -O -target arm64-apple-macosx13.0 \
    -framework AppKit \
    -framework Security \
    -framework Foundation \
    -o "$APP_DIR/Contents/MacOS/$APP_NAME" \
    "$PROJECT_DIR"/Sources/*.swift

echo "==> Stripping debug symbols..."
strip "$APP_DIR/Contents/MacOS/$APP_NAME"

echo "==> Embedding Info.plist..."
cp "$PROJECT_DIR/Resources/Info.plist" "$APP_DIR/Contents/Info.plist"

BINARY_SIZE=$(du -h "$APP_DIR/Contents/MacOS/$APP_NAME" | cut -f1)
BUNDLE_SIZE=$(du -sh "$APP_DIR" | cut -f1)

echo ""
echo "✅ Build complete: $APP_DIR"
echo "   Binary: $BINARY_SIZE  |  Bundle: $BUNDLE_SIZE"
echo ""
echo "   To install:  ./install.sh"
echo "   To run:      open $APP_DIR"
