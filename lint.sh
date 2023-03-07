#!/bin/bash
#
# Usage example:
#     bash lint.sh
#

echo "==> Formatting project files..."
cargo fmt --all
rustfmt src/*.rs
