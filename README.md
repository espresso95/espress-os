# EspressOS

A bare-metal operating system written in Rust, targeting the x86_64 architecture.

## Prerequisites

Before you can build and run EspressOS, you need to install the following tools:

### 1. Rust Toolchain
```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add the nightly toolchain (required for kernel development)
rustup toolchain install nightly
rustup default nightly

# Add the x86_64 target
rustup target add x86_64-unknown-none
```

### 2. Build Tools
```bash
# Install bootimage tool for creating bootable disk images
cargo install bootimage

# Install QEMU for running the OS in a virtual machine
# On Ubuntu/Debian:
sudo apt update
sudo apt install qemu-system-x86

# On macOS:
brew install qemu

# On Windows:
# Download and install QEMU from https://qemu.org
```

### 3. Additional Components
```bash
# Add the rust-src component (required for building core library)
rustup component add rust-src

# Add llvm-tools for additional build tools
rustup component add llvm-tools-preview
```

## Building

To build the kernel:

```bash
# Build the kernel
cargo build

# Build a bootable disk image
cargo bootimage
```

## Running

To run EspressOS in QEMU:

```bash
# Run with bootimage (recommended)
cargo run

# Or run the generated image directly with QEMU
qemu-system-x86_64 -drive format=raw,file=target/x86_64-espress_os/debug/bootimage-espress-os.bin
```

## Development

### Project Structure

```
espress-os/
├── src/
│   └── main.rs          # Kernel entry point
├── .cargo/
│   └── config.toml      # Cargo configuration
├── x86_64-espress_os.json # Target specification
├── Cargo.toml           # Project configuration
└── README.md           # This file
```

### Key Features

- **No Standard Library**: The kernel runs in a `no_std` environment
- **VGA Text Mode**: Basic text output via VGA buffer
- **Panic Handler**: Custom panic handling for kernel panics
- **X86_64 Target**: Specifically designed for 64-bit x86 architecture

### Adding New Features

1. Create new modules in the `src/` directory
2. Add dependencies to `Cargo.toml` as needed
3. Ensure all code is `no_std` compatible
4. Test in QEMU before running on real hardware

## Architecture

EspressOS is designed as a microkernel with the following components:

- **Boot Process**: Uses the `bootloader` crate for initial setup
- **Memory Management**: Direct memory access and management
- **VGA Output**: Text-mode display driver
- **Interrupt Handling**: x86_64 interrupt descriptor table setup

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly in QEMU
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Resources

- [Writing an OS in Rust](https://os.phil-opp.com/) - Excellent tutorial series
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [x86_64 Assembly Language Reference](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)