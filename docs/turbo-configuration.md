# Turbo Configuration Documentation

This document provides detailed documentation for the `turbo.json` configuration file used in the EspressOS monorepo.

## Overview

The `turbo.json` file configures Turborepo's build pipeline to coordinate tasks across three distinct packages in our monorepo:

- **`packages/espress-os/`** - The original bare-metal OS kernel (Rust)
- **`packages/espress-wasm/`** - WebAssembly bindings for browser integration (Rust → WASM)
- **`packages/frontend/`** - Interactive web application (Node.js/Vite)

## Configuration Structure

### Schema Reference

```json
"$schema": "https://turbo.build/schema.json"
```

This enables IDE autocompletion and validation for the Turbo configuration.

## Task Definitions

### `build`

**Purpose**: Compiles all packages in dependency order, handling both Rust compilation (for OS and WASM targets) and frontend bundling.

```json
"build": {
  "dependsOn": ["^build"],
  "outputs": ["target/**", "dist/**", "build/**", "pkg/**"]
}
```

**Configuration Details**:
- **`dependsOn: ["^build"]`**: Ensures that dependencies are built before dependents. This is crucial because:
  - `espress-frontend` depends on `espress-wasm` being built first
  - WASM packages must be generated before the frontend can import them
  - The `^` prefix means "upstream dependencies" in Turbo terminology

- **`outputs`**: Defines which directories contain build artifacts that should be cached:
  - `target/**` - Rust/Cargo build artifacts (used by both OS and WASM packages)
  - `dist/**` - Frontend distribution files (Vite output)
  - `build/**` - General build outputs from any package
  - `pkg/**` - wasm-pack generated packages (WebAssembly bindings)

**Build Process Flow**:
1. `espress-os` builds using `cargo build` (bare-metal target)
2. `espress-wasm` builds using `wasm-pack build --target web`
3. `espress-frontend` builds using `vite build` (imports WASM from step 2)

### `dev`

**Purpose**: Starts development servers with live reload capabilities for rapid iteration.

```json
"dev": {
  "cache": false,
  "persistent": true
}
```

**Configuration Details**:
- **`cache: false`**: Development servers need to reflect real-time changes, so caching is disabled to ensure fresh builds on every file change
- **`persistent: true`**: Keeps development processes running indefinitely. Essential for:
  - Frontend dev server (`vite dev`) that serves the application
  - File watchers that rebuild on changes
  - Hot module replacement (HMR) functionality

**Development Workflow**:
- Frontend runs on development server with HMR
- WASM changes trigger rebuilds that frontend picks up automatically
- OS development uses traditional Rust toolchain (unaffected by web dev)

### `test`

**Purpose**: Runs comprehensive test suites across all packages in the monorepo.

```json
"test": {
  "dependsOn": ["build"],
  "outputs": []
}
```

**Configuration Details**:
- **`dependsOn: ["build"]`**: Tests require built artifacts because:
  - Integration tests may need compiled WASM modules
  - Frontend tests might import and test WASM functionality
  - Ensures all packages are in a buildable state before testing
- **`outputs: []`**: Test results are typically not cached as build outputs since they're validation rather than artifacts

**Test Coverage**:
- Rust unit tests for OS kernel components
- Rust unit tests for WASM bindings
- Frontend integration tests with WASM modules
- End-to-end tests of the complete system

### `lint`

**Purpose**: Enforces code quality and formatting standards across all languages and packages.

```json
"lint": {
  "outputs": []
}
```

**Configuration Details**:
- **`outputs: []`**: Linting produces reports and fixes but no cacheable build artifacts
- **No dependencies**: Linting can run independently of builds for faster feedback

**Linting Tools**:
- **Rust packages**: `clippy` for lints, `rustfmt` for formatting
- **Frontend package**: ESLint for JavaScript/TypeScript, Prettier for formatting
- **Configuration files**: JSON/YAML validation where applicable

### `clean`

**Purpose**: Removes all build artifacts, caches, and temporary files for a fresh start.

```json
"clean": {
  "cache": false
}
```

**Configuration Details**:
- **`cache: false`**: Clean operations should always run fresh and never be cached
- **No outputs**: Clean operations remove files rather than create them
- **No dependencies**: Can run independently to resolve build issues

**Clean Targets**:
- Rust `target/` directories
- Node.js `node_modules/` and build outputs
- Turbo cache files
- Generated WASM packages

## Caching Strategy

Turbo's intelligent caching system:

1. **Hashes inputs** (source files, dependencies, environment) for each task
2. **Caches outputs** in local and optionally remote cache
3. **Skips execution** if inputs haven't changed and outputs exist in cache
4. **Shares caches** across team members and CI/CD pipelines

This is particularly beneficial for our monorepo because:
- WASM compilation can be slow and benefits greatly from caching
- Frontend builds only re-run when WASM or frontend code changes
- Cross-package dependencies are tracked automatically

## Package Dependencies

```
espress-os (independent)
    ↓
espress-wasm (depends on nothing, but conceptually uses OS components)
    ↓
espress-frontend (depends on espress-wasm)
```

The dependency chain ensures proper build order while allowing parallel builds where possible.

## Usage Examples

```bash
# Build all packages in dependency order
npm run build

# Start all development servers
npm run dev

# Run tests across all packages
npm run test

# Lint all code
npm run lint

# Clean all build artifacts
npm run clean

# Build only specific packages
npx turbo build --filter=espress-wasm
npx turbo build --filter=espress-frontend
```

## Performance Optimizations

1. **Parallel execution** where dependencies allow
2. **Incremental builds** through input/output hashing
3. **Remote caching** capability for team collaboration
4. **Scoped tasks** to run only affected packages

This configuration ensures efficient development workflow while maintaining the integrity of cross-package dependencies in our multi-language, multi-target monorepo.