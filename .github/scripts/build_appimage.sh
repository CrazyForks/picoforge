#!/bin/bash
set -eo pipefail

VARIANT=${1:-"glibc-x86_64"}

echo "=== Building PicoForge AppImage for variant: ${VARIANT} ==="

# Locate PicoForge source directory
if [ -f "/workspace/Cargo.toml" ]; then
    SRC_DIR="/workspace"
elif [ -f "/workspace/picoforge/Cargo.toml" ]; then
    SRC_DIR="/workspace/picoforge"
else
    echo "Error: Cargo.toml not found in /workspace or /workspace/picoforge"
    exit 1
fi

cd "$SRC_DIR"

# Extract version from Cargo.toml
VERSION=$(grep -m 1 '^version' Cargo.toml | sed -E 's/.*"([0-9]+\.[0-9]+\.[0-9]+)".*/\1/')
if [ -z "$VERSION" ]; then
    VERSION="0.0.0"
fi
echo "PicoForge Version: ${VERSION}"

# Determine architecture and target naming
ARCH=$(uname -m)
if [ "$ARCH" == "x86_64" ]; then
    ARCH_NAME="x86-64"
elif [ "$ARCH" == "aarch64" ]; then
    ARCH_NAME="aarch64"
else
    ARCH_NAME="$ARCH"
fi

# Define output filename based on variant
case "$VARIANT" in
    glibc-2.28-x86_64|glibc-2.28-aarch64)
        OUTPUT_NAME="picoforge_${VERSION}_glibc-2.28_${ARCH_NAME}.AppImage"
        ;;
    musl-x86_64|musl-aarch64)
        OUTPUT_NAME="picoforge_${VERSION}_musl_${ARCH_NAME}.AppImage"
        ;;
    *)
        OUTPUT_NAME="picoforge_${VERSION}_${VARIANT}_${ARCH_NAME}.AppImage"
        ;;
esac

echo "Building release binary with Cargo..."
cargo build --release

BINARY_PATH="target/release/picoforge"
DESKTOP_FILE="data/in.suyogtandel.picoforge.desktop"
ICON_FILE="static/appIcons/in.suyogtandel.picoforge.svg"

if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Binary not found at $BINARY_PATH"
    exit 1
fi

if [ ! -f "$DESKTOP_FILE" ]; then
    echo "Error: Desktop file not found at $DESKTOP_FILE"
    exit 1
fi

if [ ! -f "$ICON_FILE" ]; then
    echo "Error: Icon file not found at $ICON_FILE"
    exit 1
fi

echo "Packaging AppImage using linuxdeploy..."
export APPIMAGE_EXTRACT_AND_RUN=1

# Run linuxdeploy to bundle executable, desktop entry, icon, excluding host daemon libpcsclite
rm -rf AppDir
linuxdeploy \
    --appdir AppDir \
    --executable "$BINARY_PATH" \
    --desktop-file "$DESKTOP_FILE" \
    --icon-file "$ICON_FILE" \
    --exclude-library libpcsclite.so.1 \
    --output appimage

# Find generated AppImage file and rename to standard naming
GENERATED_APPIMAGE=$(ls -t *.AppImage 2>/dev/null | head -n 1)

if [ -z "$GENERATED_APPIMAGE" ]; then
    echo "Error: AppImage generation failed"
    exit 1
fi

mkdir -p /workspace/target/release
mv "$GENERATED_APPIMAGE" "/workspace/target/release/${OUTPUT_NAME}"
rm -rf AppDir

echo "=== Build Complete! Output: /workspace/target/release/${OUTPUT_NAME} ==="
ls -lh "/workspace/target/release/${OUTPUT_NAME}"
