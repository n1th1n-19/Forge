#!/usr/bin/env sh
set -e

REPO="n1th1n-19/forge"
BIN_NAME="forge"
INSTALL_DIR="/usr/local/bin"

# Detect OS
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
  x86_64)  ARCH="x86_64" ;;
  aarch64|arm64) ARCH="aarch64" ;;
  *)
    echo "Unsupported architecture: $ARCH"
    exit 1
    ;;
esac

case "$OS" in
  linux)  TARGET="${ARCH}-unknown-linux-gnu" ;;
  darwin) TARGET="${ARCH}-apple-darwin" ;;
  *)
    echo "Unsupported OS: $OS"
    echo "On Windows, download forge.exe from: https://github.com/${REPO}/releases/latest"
    exit 1
    ;;
esac

# Fetch latest release tag
LATEST=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
  | grep '"tag_name"' | head -1 | cut -d'"' -f4)

if [ -z "$LATEST" ]; then
  echo "Could not fetch latest release from https://github.com/${REPO}/releases"
  exit 1
fi

DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST}/forge-${TARGET}"

echo "  forge ${LATEST} — ${TARGET}"
echo "  Downloading..."

TMP=$(mktemp)
curl -fsSL "$DOWNLOAD_URL" -o "$TMP"
chmod +x "$TMP"

if [ -w "$INSTALL_DIR" ]; then
  mv "$TMP" "${INSTALL_DIR}/${BIN_NAME}"
else
  sudo mv "$TMP" "${INSTALL_DIR}/${BIN_NAME}"
fi

echo "  Installed to ${INSTALL_DIR}/${BIN_NAME}"
echo ""
echo "  Run:  forge"
echo ""
