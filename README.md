# EspressOS Monorepo

A bare-metal operating system written in Rust, targeting the x86_64 architecture, now organized as a monorepo with WebAssembly support.

## 🚀 Quick Start

### Prerequisites

1. **Node.js** (version 18+)
2. **Rust toolchain** with nightly support
3. **Build tools** for OS development

### Setup

```bash
# Clone the repository
git clone https://github.com/espresso95/espress-os.git
cd espress-os

# Install Node.js dependencies
npm install

# Set up Rust development environment
rustup toolchain install nightly
rustup default nightly
rustup component add rust-src llvm-tools-preview
rustup target add x86_64-unknown-none
rustup target add wasm32-unknown-unknown
cargo install bootimage

# Build all packages
npm run build
```

### Development

```bash
# Build all packages
npm run build

# Run frontend development server
cd packages/frontend && npm run dev

# Build the OS kernel
cd packages/espress-os && cargo build

# Build WebAssembly components
cd packages/espress-wasm && npm run build
```

## 📦 Package Structure

This monorepo contains three main packages:

### `packages/espress-os/` - The Core Operating System
The original bare-metal OS kernel written in Rust:
- **VGA Text Mode**: Basic text output via VGA buffer
- **Memory Management**: Direct memory access and management  
- **Interrupt Handling**: x86_64 interrupt descriptor table setup
- **Boot Process**: Uses the `bootloader` crate for initial setup

```bash
cd packages/espress-os
cargo build          # Build the kernel
cargo bootimage       # Create bootable disk image
cargo run            # Run in QEMU (requires QEMU installation)
```

### `packages/espress-wasm/` - WebAssembly Components
Rust code compiled to WebAssembly for web integration:
- **VGA Emulator**: Web-based VGA text mode emulation
- **OS Simulation**: Browser-compatible OS component demonstrations
- **Color Management**: Full VGA color palette support

```bash
cd packages/espress-wasm
npm run build        # Build WASM package
npm run dev          # Development build
```

### `packages/frontend/` - Web Demo Application
Interactive web application showcasing EspressOS components:
- **Terminal Emulator**: Retro-style terminal interface
- **OS Demo**: Interactive kernel boot simulation
- **WebAssembly Integration**: Real-time OS component usage

```bash
cd packages/frontend
npm run dev          # Start development server
npm run build        # Build for production
```

## 🛠 Development Tools

### Turbo Monorepo Management
```bash
npm run build        # Build all packages
npm run dev          # Start all development servers
npm run test         # Run all tests
npm run lint         # Lint all packages
npm run clean        # Clean all build artifacts
```

For detailed information about the Turbo build system configuration, see [docs/turbo-configuration.md](docs/turbo-configuration.md).

### Individual Package Management
```bash
# OS Development
cd packages/espress-os
./dev.sh setup       # Setup OS development environment
./dev.sh build       # Build kernel
./dev.sh run         # Run in QEMU
./dev.sh clean       # Clean build artifacts

# WebAssembly Development  
cd packages/espress-wasm
npm run build        # Build WASM package
npm run test         # Run WASM tests

# Frontend Development
cd packages/frontend
npm run dev          # Development server
npm run preview      # Preview production build
```

## 🏗 Architecture

### Monorepo Structure
```
espress-os/
├── packages/
│   ├── espress-os/          # Core OS kernel (Rust)
│   │   ├── src/main.rs      # Kernel entry point
│   │   ├── Cargo.toml       # OS dependencies
│   │   └── .cargo/          # OS-specific build config
│   ├── espress-wasm/        # WebAssembly components (Rust→WASM)
│   │   ├── src/lib.rs       # WASM library
│   │   ├── Cargo.toml       # WASM dependencies
│   │   └── pkg/             # Generated WASM package
│   └── frontend/            # Web application (JavaScript)
│       ├── src/main.js      # Frontend application
│       ├── index.html       # Web interface
│       └── dist/            # Built frontend assets
├── turbo.json               # Turbo build configuration
├── package.json             # Workspace configuration
└── Cargo.toml              # Rust workspace configuration
```

### Build Pipeline
1. **espress-os**: Compiles to bare-metal x86_64 binary
2. **espress-wasm**: Compiles Rust to WebAssembly using wasm-pack
3. **frontend**: Bundles web application with Vite, integrating WASM components

## 🎯 Key Features

- **Dual-Target Architecture**: Same codebase supports both bare-metal and web platforms
- **Modern Development**: Uses Turbo for fast, cached builds across packages
- **Interactive Demo**: Web-based terminal emulator showcasing OS capabilities
- **Development Tools**: Comprehensive tooling for both OS and web development
- **Cross-Platform**: OS development on any platform supporting Rust and QEMU

## 🚧 Running the OS

### In QEMU (Bare Metal Simulation)
```bash
cd packages/espress-os
cargo run
# or
./dev.sh run
```

### In the Browser (WebAssembly Demo)
```bash
# Start the web demo
cd packages/frontend
npm run dev

# Visit http://localhost:5173
# Click "Simulate Boot" or "Run Demo" to see OS components in action
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes in the appropriate package
4. Test thoroughly: `npm run build && npm run test`
5. Commit your changes: `git commit -m 'Add amazing feature'`
6. Push to the branch: `git push origin feature/amazing-feature`  
7. Submit a pull request

### Development Guidelines
- OS code should remain `no_std` compatible
- WebAssembly components should work in modern browsers
- Follow existing code style and patterns
- Test both bare-metal and web targets when making core changes

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Resources

- [Writing an OS in Rust](https://os.phil-opp.com/) - Excellent tutorial series
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [WebAssembly Documentation](https://webassembly.org/)
- [Turbo Documentation](https://turbo.build/repo/docs)
- [x86_64 Assembly Reference](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)