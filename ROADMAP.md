# Roadmap

## v0.1.0 (Current)
- [x] `#![no_std]` project structure.
- [x] Zero-copy primitives:
    - `ZAddress` (wraps `&[u8; 20]`).
    - `ZU256` (wraps `&[u8; 32]`).
    - `ZBytes` (wraps `&[u8]`).
- [x] Basic decoder helpers (`read_u256`, `read_address`, `read_bytes`).
- [x] Bounds checking and error handling.

## v0.2.0 - Extended Types
- [x] `bool` support.
- [x] `string` support (UTF-8 validation on zero-copy slices).
- [x] Fixed-size arrays `T[N]`.
- [x] Dynamic arrays `T[]` (returning iterators over slices).
- [ ] Nested tuples.

## v0.3.0 - Ergonomics
- [ ] `ZDecode` derive macro.
    - Generate struct definitions that wrap raw slices.
    - Auto-implement `decode` to parse offsets.
- [ ] `Solidity` type mapping (e.g., `sol!("struct MyData { ... }")`).

## v1.0.0 - Production Ready
- [ ] Extensive fuzzing against `alloy-rs` and `ethers-rs`.
- [ ] Benchmarks proving 0-allocation claims.
- [ ] Audited `unsafe` blocks.
