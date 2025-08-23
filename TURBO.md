# Turbo Configuration Overview

This file documents the `turbo.json` configuration in the same directory.

## Quick Reference

| Task | Purpose | Key Settings |
|------|---------|-------------|
| `build` | Compile all packages in dependency order | `dependsOn: ["^build"]` ensures proper build sequence |
| `dev` | Start development servers | `persistent: true` keeps servers running, `cache: false` for live updates |
| `test` | Run test suites | `dependsOn: ["build"]` requires built artifacts for testing |
| `lint` | Code quality checks | No dependencies, can run independently |
| `clean` | Remove build artifacts | `cache: false` always runs fresh |

## Package Build Order

1. `espress-os` → Builds bare-metal OS kernel
2. `espress-wasm` → Builds WebAssembly bindings  
3. `espress-frontend` → Builds web app (depends on WASM)

## Output Directories

- `target/**` - Rust/Cargo build artifacts
- `dist/**` - Frontend distribution files (Vite)
- `build/**` - General build outputs
- `pkg/**` - wasm-pack generated packages

## Commands

```bash
npm run build    # Build all packages
npm run dev      # Start development servers
npm run test     # Run all tests
npm run lint     # Lint all code
npm run clean    # Clean build artifacts
```

For complete documentation, see [docs/turbo-configuration.md](docs/turbo-configuration.md).