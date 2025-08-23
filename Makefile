# Makefile for EspressOS

.PHONY: build run clean test check

# Default target
all: build

# Build the kernel
build:
	cargo build

# Build release version
release:
	cargo build --release

# Create bootable image
bootimage:
	cargo bootimage

# Run the OS in QEMU
run: bootimage
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-espress-os.bin

# Run with cargo bootimage runner
run-cargo:
	cargo run

# Clean build artifacts
clean:
	cargo clean

# Check code without building
check:
	cargo check

# Format code
fmt:
	cargo fmt

# Run clippy lints
clippy:
	cargo clippy

# Run tests (when we have them)
test:
	cargo test

# Install development dependencies
setup:
	rustup toolchain install nightly
	rustup default nightly
	rustup component add rust-src
	rustup component add llvm-tools-preview
	rustup target add x86_64-unknown-none
	cargo install bootimage

# Debug with GDB (requires gdb multiarch)
debug:
	qemu-system-x86_64 -s -S -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-espress-os.bin &
	gdb -ex "target remote :1234" -ex "symbol-file target/x86_64-unknown-none/debug/espress-os"

# Show disk usage of build artifacts
size:
	@echo "Kernel binary size:"
	@ls -lh target/x86_64-unknown-none/debug/espress-os || echo "No kernel binary found, run 'make build' first"
	@echo "\nBootimage size:"
	@ls -lh target/x86_64-unknown-none/debug/bootimage-espress-os.bin || echo "No bootimage found, run 'make bootimage' first"

help:
	@echo "Available targets:"
	@echo "  build      - Build the kernel"
	@echo "  release    - Build release version"
	@echo "  bootimage  - Create bootable disk image"
	@echo "  run        - Run OS in QEMU"
	@echo "  run-cargo  - Run OS using cargo runner"
	@echo "  clean      - Clean build artifacts"
	@echo "  check      - Check code without building"
	@echo "  fmt        - Format code"
	@echo "  clippy     - Run clippy lints"
	@echo "  test       - Run tests"
	@echo "  setup      - Install development dependencies"
	@echo "  debug      - Start OS with GDB debugging"
	@echo "  size       - Show binary sizes"
	@echo "  help       - Show this help"