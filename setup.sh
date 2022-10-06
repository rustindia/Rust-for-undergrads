#!/bin/bash
#
# Usage example:
#     bash setup.sh -b
#

RUST_TOOLCHAIN="$(awk -F'[ ="]+' '$1 == "channel" { print $2 }' rust-toolchain.toml)"
INSTALL_BUILD_TOOLS="false"

function start_message() {
  cat <<EOF
  This script will download and install the necessary dependencies needed to
  build, formating the scripts.

  Based on your selection, these tools will be included:
EOF

  if [[ "$INSTALL_BUILD_TOOLS" == "true" ]]; then
    cat <<EOF
  Build tools (since -b or no option was provided):
    * Rust (and the necessary components, e.g. rust-fmt, clippy)

EOF
  fi

  cat <<EOF
  If you'd prefer to install these dependencies yourself, please use the command
  Ctrl+C do exit the script.
  
EOF
}

function usage() {
  echo "usage: $0 [-b] exchange method [...args]"
  echo "	-b      Install build tools"
  exit 1
}

function install_rustup {
  RUST_TOOLCHAIN=$1

  echo "==> Installing Rust......"
  if rustup --version &>/dev/null; then
    echo "Rust is already installed"
    rustc --version
  else
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain "${RUST_TOOLCHAIN}" --profile minimal
    PATH="${HOME}/.cargo/bin:${PATH}"
    source $HOME/.cargo/env
  fi
}

function install_default_toolchain {
  RUST_TOOLCHAIN=$1

  echo "==> Setting nightly toolchain..."
  rustup install "${RUST_TOOLCHAIN}"
  rustup override set "${RUST_TOOLCHAIN}"

  rustup component add rustfmt --toolchain "${RUST_TOOLCHAIN}"
  rustup component add clippy --toolchain "${RUST_TOOLCHAIN}"
}

start_message

printf "Proceed with installing necessary dependencies? (y/N) > "
read -e -r input
if [[ "$input" != "y"* ]]; then
  echo "Exiting..."
  exit 0
fi

# Loop through command line arguments
while getopts "bc" arg; do
  case "$arg" in
  b)
    INSTALL_BUILD_TOOLS="true"
    ;;
  *)
    usage exit 0
    ;;
  esac
done

if [[ "$INSTALL_BUILD_TOOLS" == "true" ]]; then
  install_rustup "$RUST_TOOLCHAIN"
  install_default_toolchain "$RUST_TOOLCHAIN"

  # Any call to cargo will make rustup install the correct toolchain
  cargo version
fi

echo "==> Rust configured!"