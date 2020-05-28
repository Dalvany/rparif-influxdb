#!/bin/bash
set -euo pipefail

export TARGET=arm-unknown-linux-gnueabihf
export CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc

cargo install cargo-deb

# Install the Rust stdlib for the current target
rustup target add $TARGET

# Download the Raspberry Pi cross-compilation toolchain if needed
if [ -d "/tmp/tools" ]; then
  rm -Rf /tmp/tools
fi
if [ "$TARGET" = "arm-unknown-linux-gnueabihf" ]; then
  git clone --depth=1 https://github.com/raspberrypi/tools.git /tmp/tools
  export PATH=/tmp/tools/arm-bcm2708/arm-linux-gnueabihf/bin:$PATH
fi

# Compile the binary and make a debian package
cargo-deb --target=$TARGET --no-strip
