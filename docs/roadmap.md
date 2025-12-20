# Roadmap

This document outlines the development plan for `zabi-rs`, focusing on zero-allocation efficiency, ergonomics, and production readiness.

## 🚀 Released

### v0.0.3 - Primitives & Perf (Current)
- [x] **Integers**:
    - [x] Signed integers (`int8` to `int256`).
    - [x] Smaller unsigned integers (`uint8` to `uint128`).
    - [x] Safe casting helpers.

### v0.0.2 - Extended Types
- [x] **New Types Support**:
    - [x] `bool` (uint256 encoded).
    - [x] `string` (UTF-8 validation on zero-copy slices).
    - [x] Fixed-size arrays `T[N]`.
    - [x] Dynamic arrays `T[]` (returning iterators/wrappers).
- [x] **Decoder Improvements**:
    - [x] Offset-based decoding logic updates.

### v0.3.0 - Ergonomics & Developer Experience
Focus on reducing boilerplate and making the library easier to use.

- [ ] **Derive Macros**:
    - Implement `#[derive(ZDecode)]` to auto-generate struct wrappers and decoding logic.
    - Support for nested structs and complex schemas.
- [ ] **Solidity Compatibility**:
    - `sol!` macro integration (e.g., `sol!("struct MyData { ... }")`) to generate Rust types from Solidity definitions.
- [ ] **Nested Tuples**:
    - Support for decoding arbitrarily nested tuple types.
- [ ] **Function Dispatch**:
    - Helper to parse 4-byte selector and dispatch to decoders.
- [ ] **Event Decoding**:
    - Support for decoding Log topics and un-indexed data.

### v1.0.0 - Production Readiness
Focus on stability, security, and proven performance.

- [ ] **Security Audits**:
    - Comprehensive audit of all `unsafe` blocks.
    - Fuzz testing against `alloy-rs` and `ethers-rs` to ensure correctness.
- [ ] **Performance Validation**:
    - Benchmarks proving 0-allocation claims.
    - Comparison benchmarks against standard libraries.
- [ ] **Documentation**:
    - Complete API documentation and usage guides.
    - Migration guides from other libraries.

## 🔮 Future Ideas
- [ ] **Zero-Copy Encoding**: extending the library to support encoding without allocation.
- [ ] **WASM Bindings**: optimized for web usage.
