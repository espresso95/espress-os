#!/bin/bash

# Development script for EspressOS
# Usage: ./dev.sh [command]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

case "${1:-build}" in
    "setup")
        echo "Setting up development environment..."
        rustup toolchain install nightly
        rustup default nightly
        rustup component add rust-src
        rustup component add llvm-tools-preview
        rustup target add x86_64-unknown-none
        cargo install bootimage
        echo "Setup complete!"
        ;;
    "build")
        echo "Building kernel..."
        cargo build
        ;;
    "run")
        echo "Building and running EspressOS..."
        cargo run
        ;;
    "qemu")
        echo "Building bootimage and running in QEMU..."
        cargo bootimage
        qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-espress-os.bin
        ;;
    "clean")
        echo "Cleaning build artifacts..."
        cargo clean
        ;;
    "fmt")
        echo "Formatting code..."
        cargo fmt
        ;;
    "check")
        echo "Checking code..."
        cargo check
        ;;
    "clippy")
        echo "Running clippy..."
        cargo clippy
        ;;
    "size")
        echo "Binary sizes:"
        ls -lh target/x86_64-unknown-none/debug/espress-os 2>/dev/null || echo "No kernel binary found"
        ls -lh target/x86_64-unknown-none/debug/bootimage-espress-os.bin 2>/dev/null || echo "No bootimage found"
        ;;
    "help")
        echo "Available commands:"
        echo "  setup  - Set up development environment"
        echo "  build  - Build the kernel"
        echo "  run    - Build and run using cargo runner"
        echo "  qemu   - Build and run in QEMU directly"
        echo "  clean  - Clean build artifacts"
        echo "  fmt    - Format code"
        echo "  check  - Check code"
        echo "  clippy - Run clippy lints"
        echo "  size   - Show binary sizes"
        echo "  help   - Show this help"
        ;;
    *)
        echo "Unknown command: $1"
        echo "Use './dev.sh help' for available commands"
        exit 1
        ;;
esac