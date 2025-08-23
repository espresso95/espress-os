.PHONY: all build clean run

# Default target
all: build

# Build the kernel
build:
	cargo +nightly build

# Build release version
release:
	cargo +nightly build --release

# Clean build artifacts
clean:
	cargo clean

# Run with QEMU (requires QEMU to be installed)
run: build
	qemu-system-i386 -kernel target/i686-espress-os/debug/espress-os

# Run release version with QEMU
run-release: release
	qemu-system-i386 -kernel target/i686-espress-os/release/espress-os

# Check the kernel with objdump
objdump: build
	objdump -h target/i686-espress-os/debug/espress-os