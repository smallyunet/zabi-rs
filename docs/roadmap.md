# Roadmap

This document outlines the development plan for `zabi-rs`, focusing on zero-allocation efficiency, ergonomics, and production readiness.

## 🚀 Released

### v0.0.5 - Event Decoding & Helpers (Current)
- [x] **Event/Log Decoding**:
    - [x] `ZEventLog` struct for zero-copy event log handling.
    - [x] `read_topic_u256()`, `read_topic_int256()`, `read_topic_address()`, `read_topic_bool()`.
    - [x] Topic and data decoding with existing decoders.
- [x] **Type Helpers**:
    - [x] `ZU256::to_u32()` conversion method.
    - [x] `ZBytes::len()`, `is_empty()`, `as_slice()`.
    - [x] `ZString::len()`, `is_empty()`, `as_str()`.
    - [x] `ZBool::as_bool()` accessor.
- [x] **Fixed-Size Bytes Extensions**:
    - [x] `read_bytes2`, `read_bytes3`, `read_bytes16` convenience functions.
    - [x] `ZBytesN::to_bytes()` copy method.

### v0.0.4 - Utilities & Convenience
- [x] **Fixed-Size Bytes**:
    - [x] `bytes1` to `bytes32` types (`ZBytesN<N>`).
    - [x] Left-aligned decoding with padding validation.
- [x] **Function Selector Parsing**:
    - [x] `read_selector()` to extract 4-byte selector.
    - [x] `skip_selector()` to get calldata without selector.
- [x] **Tuple Decoding**:
    - [x] `decode_tuple!` macro for decoding multiple types.
- [x] **Type Conversion Helpers**:
    - [x] `ZU256::to_u64()`, `ZU256::to_u128()`, `ZU256::is_zero()`.
    - [x] `ZInt256::to_i64()`, `ZInt256::to_i128()`, `ZInt256::is_negative()`.
    - [x] `ZAddress::to_bytes()`, `ZAddress::as_bytes()`.

### v0.0.3 - Primitives & Perf
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
- [x] **Function Dispatch**:
    - ~~Helper to parse 4-byte selector and dispatch to decoders.~~ (Moved to v0.0.4)
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
