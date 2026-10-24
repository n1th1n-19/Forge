#!/usr/bin/env sh
set -eu

REPO="n1th1n-19/forge"
BIN_NAME="forge"
INSTALL_DIR="/usr/local/bin"

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
    echo "Supported: Linux and macOS (x86_64, aarch64). On Windows, use WSL."
    exit 1
    ;;
esac

# releases/latest/download redirects to the newest release — no API call or rate limit
BASE="https://github.com/${REPO}/releases/latest/download"
ASSET="forge-${TARGET}"

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

echo "  forge — ${TARGET}"
echo "  Downloading..."
curl -fsSL --proto '=https' --tlsv1.2 "${BASE}/${ASSET}" -o "${TMP}/${ASSET}"
curl -fsSL --proto '=https' --tlsv1.2 "${BASE}/SHA256SUMS" -o "${TMP}/SHA256SUMS"

echo "  Verifying checksum..."
grep " ${ASSET}\$" "${TMP}/SHA256SUMS" > "${TMP}/want" || {
  echo "  No checksum for ${ASSET} in SHA256SUMS — refusing to install"
  exit 1
}
if command -v sha256sum >/dev/null 2>&1; then
  (cd "$TMP" && sha256sum -c want >/dev/null)
else
  (cd "$TMP" && shasum -a 256 -c want >/dev/null)
fi

# install sets mode 755; with sudo the binary is root-owned, so user-level
# malware can't swap it for a trojan that later runs `sudo`.
if [ -w "$INSTALL_DIR" ]; then
  install -m 755 "${TMP}/${ASSET}" "${INSTALL_DIR}/${BIN_NAME}"
else
  sudo install -m 755 "${TMP}/${ASSET}" "${INSTALL_DIR}/${BIN_NAME}"
fi

echo "  Installed to ${INSTALL_DIR}/${BIN_NAME}"
echo ""
echo "  Run:  forge"
echo ""
