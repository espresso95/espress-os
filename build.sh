#!/bin/bash

# EspressOS Build Script
# This script sets up the environment and builds the project correctly

set -e

echo "🔧 Setting up EspressOS build environment..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -d "packages/espress-os" ]; then
    echo "❌ Error: Please run this script from the root of the espress-os project"
    exit 1
fi

# Install required Rust targets
echo "📦 Installing required Rust targets..."

if ! rustup target list --installed | grep -q "x86_64-unknown-none"; then
    echo "Installing x86_64-unknown-none target..."
    rustup target add x86_64-unknown-none
fi

if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Install bootimage tool if not present
if ! cargo install --list | grep -q "bootimage"; then
    echo "Installing bootimage tool..."
    cargo install bootimage
fi

# Build specific packages with correct targets
echo "🔨 Building espress-os kernel..."
cargo build -p espress-os --target x86_64-unknown-none

echo "🌐 Building espress-wasm..."
cargo build -p espress-wasm --target wasm32-unknown-unknown

echo "✅ Build completed successfully!"
echo ""
echo "📋 Available commands:"
echo "  cargo build -p espress-os --target x86_64-unknown-none    # Build kernel"
echo "  cargo build -p espress-wasm --target wasm32-unknown-unknown # Build WASM"
echo "  bootimage run -p espress-os --target x86_64-unknown-none  # Run in QEMU"
echo ""
echo "⚠️  Note: Always specify the target when building individual packages"
echo "   The kernel requires x86_64-unknown-none target"
echo "   The WASM package requires wasm32-unknown-unknown target"