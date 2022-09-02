#!/bin/bash


# IMPORTANT! YOU NEED TO PERFORM THE FOLLOING STEPS BEFORE THIS WORKS:
# Source: https://github.com/chinedufn/cross-compile-rust-from-mac-to-linux
# Source: https://timryan.org/2018/07/27/cross-compiling-linux-binaries-from-macos.html

# 1. Add the target toolchain containing the standard libraries for the target platform:
# rustup target add x86_64-unknown-linux-gnu

# 1. Install a pre-built cross compiler
# brew tap SergioBenitez/osxct
# brew install x86_64-unknown-linux-gnu


set -e

export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="x86_64-unknown-linux-gnu-gcc"
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER="x86_64-linux-musl-gcc"

# export OPENSSL_DIR="$(pwd)/target/usr/"
# export OPENSSL_LIB_DIR="$(pwd)/target/usr/lib/x86_64-linux-gnu/"

# Build with GNU:
# cargo build --target=x86_64-unknown-linux-gnu
# cargo build --release --target=x86_64-unknown-linux-gnu

# Build with MUSL (static linking)
# cargo build --target=x86_64-unknown-linux-musl
cargo build --release --target=x86_64-unknown-linux-musl
