#!/bin/bash

# EspressOS Build and Test Script

set -e

echo "Building EspressOS..."
cargo +nightly build

echo "Kernel built successfully!"
echo "Binary location: target/i686-espress-os/debug/espress-os"

# Check if QEMU is available
if command -v qemu-system-i386 &> /dev/null; then
    echo ""
    echo "QEMU found. You can run the kernel with:"
    echo "  qemu-system-i386 -kernel target/i686-espress-os/debug/espress-os"
    echo ""
    echo "Or use: make run"
    echo ""
    echo "Note: The kernel will display 'Hello from EspressOS!' in VGA text mode"
else
    echo ""
    echo "QEMU not found. Install QEMU to test the kernel:"
    echo "  sudo apt-get install qemu-system-x86  # On Ubuntu/Debian"
    echo "  brew install qemu                     # On macOS"
fi

# Show kernel info
echo ""
echo "Kernel information:"
file target/i686-espress-os/debug/espress-os
ls -lh target/i686-espress-os/debug/espress-os