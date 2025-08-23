# EspressOS

A simple operating system written in Rust targeting x86 architecture.

## Features

- Bare metal kernel written in Rust
- Multiboot-compliant bootloader support
- VGA text mode output
- No standard library (`no_std`) environment

## Prerequisites

- Rust nightly toolchain
- `rust-src` component for building core library

## Building

1. Install the required Rust toolchain:
```bash
rustup toolchain install nightly
rustup +nightly component add rust-src
```

2. Build the kernel:
```bash
cargo +nightly build
```

The kernel binary will be generated at `target/i686-espress-os/debug/espress-os`.

## Running

The kernel can be run in an x86 emulator like QEMU with a multiboot-compliant bootloader:

```bash
# Example with QEMU (requires GRUB or similar bootloader)
qemu-system-i386 -kernel target/i686-espress-os/debug/espress-os
```

## Architecture

- **Target**: Custom i686 bare metal target (`i686-espress-os.json`)
- **Bootloader**: Multiboot-compliant
- **Memory Layout**: Defined in `linker.ld`
- **Output**: VGA text mode at address 0xb8000

## Project Structure

- `src/main.rs` - Kernel entry point and main logic
- `linker.ld` - Linker script for memory layout
- `i686-espress-os.json` - Custom target specification
- `.cargo/config.toml` - Cargo build configuration