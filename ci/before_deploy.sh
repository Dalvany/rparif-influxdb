#!/bin/bash
set -euo pipefail

cargo install cargo-deb

# Install the Rust stdlib for the current target
rustup target add $TARGET

# Download the Raspberry Pi cross-compilation toolchain if needed
if [ "$TARGET" = "armv7-unknown-linux-gnueabihf" ]; then
  git clone --depth=1 https://github.com/raspberrypi/tools.git /tmp/tools
  export PATH=/tmp/tools/arm-bcm2708/armv7-unknown-linux-gnueabihf/bin:$PATH
fi

# Compile the binary and make a debian package
cargo-deb --target=$TARGET
