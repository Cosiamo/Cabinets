#!/usr/bin/env bash
set -euo pipefail

cargo fmt

# --- Detect host target triple ---
HOST_TARGET=$(rustc -vV | sed -n 's|host: ||p')
echo "==> Detected host target: $HOST_TARGET"

# --- Determine output directory and binary extension ---
case "$HOST_TARGET" in
    *linux*)   BUILD_DIR="build/linux";   EXT="" ;;
    *darwin*)  BUILD_DIR="build/macos";   EXT="" ;;
    *windows*) BUILD_DIR="build/windows"; EXT=".exe" ;;
    *)
        echo "ERROR: Unknown host target '$HOST_TARGET'"
        exit 1
        ;;
esac

PACKAGES=("app-core" "app-gui" "app-cli")
BINS=("app-gui" "app-cli")

# --- Build for host platform ---
echo "==> Building for $HOST_TARGET"
for PKG in "${PACKAGES[@]}"; do
    cargo build --release --package "$PKG"
done

# --- Prepare build output directory ---
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

# --- Copy binaries ---
for BIN in "${BINS[@]}"; do
    cp "target/release/${BIN}${EXT}" "${BUILD_DIR}/${BIN}${EXT}"
done

echo "==> Build complete. Output in ${BUILD_DIR}/"
