#!/usr/bin/env bash
set -euo pipefail

# Ensure the Cargo bin directory is on PATH for the current shell.
export PATH="$HOME/.cargo/bin:$PATH"

RUST_VERSION="${RUST_VERSION:-1.90.0}"

if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain "${RUST_VERSION}"
  export PATH="$HOME/.cargo/bin:$PATH"
else
  rustup default "${RUST_VERSION}"
fi

rustup target add wasm32-unknown-unknown

if ! command -v trunk >/dev/null 2>&1; then
  cargo install trunk --locked --version 0.17.5
fi

trunk build --release --public-url . --dist prod/dist
