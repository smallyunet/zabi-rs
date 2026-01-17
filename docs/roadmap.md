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

### v0.0.9 - Maintenance & Cleanup (DONE)
- [x] **Project Structure**:
    - [x] Fix `zabi-derive` publishing order and dependencies.
- [x] **Documentation**:
    - [x] Updated versioning and instructions.

### v0.0.8 - Comprehensive Features (DONE)
- [x] **Iterators & Traits**:
    - [x] `ZArray` iterator support (`iter()`, `IntoIterator`).
    - [x] `Ord`, `PartialOrd`, `Eq`, `Hash` for all types.
- [x] **Serde Support**:
    - [x] Optional `serde` feature.
    - [x] `Serialize` implementation for all types.
- [x] **Unchecked APIs**:
    - [x] `read_u256_unchecked`, `read_address_unchecked` for max performance.
- [x] **Zero-Allocation Encoding**:
    - [x] `encoder` module for writing to `&mut [u8]`.
    - [x] `encode_u256`, `encode_address`, `encode_bool`.

### v0.0.7 - Function Calls & Reverts (DONE)
- [x] **Revert Reason Decoding**:
    - [x] `ZRevert` enum supporting `Error(string)` and `Panic(uint256)`.
    - [x] `decode_revert()` function.
    - [x] `revert_to_string!` macro.
- [x] **Function Call Support**:
    - [x] `ZCallResult<T>` for handling results or reverts.
    - [x] `decode_call_result()` function.
    - [x] `decode_call!` macro for argument dispatching.
- [x] **Integer Utilities**:
    - [x] `ZU256::is_max()` helper.
    - [x] `ZInt256::abs()`, `signum()`, `is_positive()` helpers.

### v0.0.6 - Core Enhancements (DONE)
- [x] **Derive Macros**:
    - `#[derive(ZDecode)]` for structs.
- [x] **Tuple Support**:
    - `ZDecode` for tuples up to 12 elements.
- [x] **Nested Types**:
    - Support for nested structs and tuples in decoding.
- [x] **HEAD_SIZE trait member**:
    - For proper offset management in nested structures.
- [x] **Extended Conversions**:
    - More `to_xxx()` helpers for integer types.

### v1.0.0 - Production Readiness (DONE)
Focus on stability, security, and proven performance.

- [x] **Security Audits**:
    - [x] Comprehensive audit of all `unsafe` blocks.
    - [x] Fuzz testing against `alloy-rs` and `ethers-rs` to ensure correctness (Property tests added).
- [x] **Performance Validation**:
    - [x] Benchmarks proving 0-allocation claims.
    - [x] Comparison benchmarks against standard libraries.
- [x] **Documentation**:
    - [x] Complete API documentation and usage guides.
    - [x] Migration guides from other libraries.

## 🔮 Future Ideas
- [ ] **Zero-Copy Encoding**: extending the library to support encoding without allocation.
- [ ] **WASM Bindings**: optimized for web usage.
