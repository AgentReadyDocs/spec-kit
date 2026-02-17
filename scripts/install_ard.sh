#!/bin/sh
set -eu

usage() {
  cat <<'EOF'
install_ard.sh --version vX.Y.Z [--to DIR]

Downloads a prebuilt `ard` binary from GitHub Releases, verifies SHA-256, and installs it.

Defaults:
  --to ~/.local/bin
EOF
}

VERSION=""
INSTALL_TO=""

while [ "${1-}" != "" ]; do
  case "$1" in
    --version)
      VERSION="${2-}"
      shift 2
      ;;
    --to)
      INSTALL_TO="${2-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[FAIL] unknown arg: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [ -z "$VERSION" ]; then
  echo "[FAIL] missing --version vX.Y.Z" >&2
  exit 2
fi

if [ -z "$INSTALL_TO" ]; then
  INSTALL_TO="${HOME:-.}/.local/bin"
fi

OS="$(uname -s)"
ARCH="$(uname -m)"

TARGET=""
ARCH_NORM="$ARCH"
if [ "$ARCH" = "arm64" ]; then
  ARCH_NORM="aarch64"
fi

case "$OS" in
  Darwin)
    case "$ARCH_NORM" in
      x86_64) TARGET="x86_64-apple-darwin" ;;
      aarch64) TARGET="aarch64-apple-darwin" ;;
      *) echo "[FAIL] unsupported arch: $ARCH" >&2; exit 2 ;;
    esac
    ;;
  Linux)
    case "$ARCH_NORM" in
      x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
      *) echo "[FAIL] unsupported arch: $ARCH" >&2; exit 2 ;;
    esac
    ;;
  *)
    echo "[FAIL] unsupported OS: $OS" >&2
    exit 2
    ;;
esac

BASE_URL="https://github.com/AgentReadyDocs/spec-kit/releases/download/${VERSION}"
ASSET="ard-${TARGET}.tar.gz"
SUMS="ard-${TARGET}.sha256"

TMP_DIR="$(mktemp -d)"
cleanup() { rm -rf "$TMP_DIR"; }
trap cleanup EXIT

echo "[INFO] downloading ${ASSET}"
curl -fsSL -o "${TMP_DIR}/${ASSET}" "${BASE_URL}/${ASSET}"
curl -fsSL -o "${TMP_DIR}/${SUMS}" "${BASE_URL}/${SUMS}"

cd "$TMP_DIR"

if command -v sha256sum >/dev/null 2>&1; then
  sha256sum -c "${SUMS}"
elif command -v shasum >/dev/null 2>&1; then
  EXPECTED="$(cut -d' ' -f1 "${SUMS}")"
  ACTUAL="$(shasum -a 256 "${ASSET}" | cut -d' ' -f1)"
  if [ "$EXPECTED" != "$ACTUAL" ]; then
    echo "[FAIL] sha256 mismatch" >&2
    exit 1
  fi
else
  echo "[FAIL] missing sha256 verifier (sha256sum or shasum)" >&2
  exit 1
fi

tar -xzf "${ASSET}"

mkdir -p "${INSTALL_TO}"
cp -f "./ard" "${INSTALL_TO}/ard"
chmod +x "${INSTALL_TO}/ard"

echo "[OK] installed ${INSTALL_TO}/ard"
echo "[INFO] ensure ${INSTALL_TO} is on your PATH"

