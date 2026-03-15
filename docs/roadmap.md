# Roadmap

## 🌟 Vision
To be the standard bare-metal ABI toolkit for the Rust Ethereum ecosystem.

## ✅ Completed (v0.0.x)
- [x] **Core Decoding**: Primitives (u256, address, bool), Fixed Bytes, Arrays, Tuples.
- [x] **Zero-Allocation Architecture**: Lifetime-bound wrappers (`ZU256<'a>`, `ZBytes<'a>`).
- [x] **Macros**: `decode_tuple!`, `decode_call!`, `ZDecode` derive.
- [x] **Production Readiness**: Security audits, fuzzing, and benchmarks.

## 🚀 Just Shipped (v0.0.14)

### Developer Experience
- [x] **Better Error Messages**: Decode failures now include type, field, and byte-offset context.
- [x] **Alloy Interop**: `ZU256` and `ZAddress` can convert into `alloy-primitives` types via the optional feature.

## 🚧 Next Steps (v0.0.15+)

### Ecosystem Integration
- [ ] **WASM Support**: First-class support for `wasm32-unknown-unknown` (CI verification).

### Developer Experience
- [ ] **CLI Tool**: `zabi decode <calldata>` for quick debugging in terminal.

## 🔮 Future (v2.0)

### Zero-Copy Encoding
Currently, `zabi-rs` focuses on decoding. v2.0 will introduce an encoder that writes directly to a mutable buffer, maintaining the zero-allocation philosophy.

- [ ] `ZEncode` trait.
- [ ] `encode_tuple_to_slice(&mut [u8], ...)`
