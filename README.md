# zabi-rs

[![Crates.io](https://img.shields.io/crates/v/zabi-rs.svg)](https://crates.io/crates/zabi-rs)
[![Docs.rs](https://docs.rs/zabi-rs/badge.svg)](https://docs.rs/zabi-rs)
[![CI](https://github.com/smallyunet/zabi-rs/workflows/CI/badge.svg)](https://github.com/smallyunet/zabi-rs/actions)

**The fastest, zero-allocation EVM ABI decoder for Rust.**

`zabi-rs` is designed for high-performance applications (MEV bots, indexers, embedded devices) where every microsecond and byte of memory counts. It decodes EVM ABI data by validating and wrapping the raw specific byte slices, avoiding `Vec` and `String` allocations entirely.

---

## ⚡ Features

- **Zero Allocation**: No heap allocations (`malloc`) during decoding.
- **no_std Compatible**: Ready for embedded, WASM, and kernel-mode usage.
- **Safe & Fast**: heavily audited `unsafe` pointer arithmetic wrapped in safe APIs.
- **Ecosystem Ready**: Drop-in compatible with `alloy-primitives` types (optional).

## 🚀 Quick Start

```toml
[dependencies]
zabi-rs = "0.0.14"
```

```rust
use zabi_rs::{decode_tuple, ZU256, ZAddress};

fn main() -> Result<(), zabi_rs::ZError> {
    let data = [0u8; 64]; // Your ABI encoded data
    
    // Decode (uint256, address) tuple
    let (balance, owner) = decode_tuple!(&data, ZU256, ZAddress)?;
    
    println!("Balance: {:?}", balance);
    println!("Owner: {:?}", owner);
    
    Ok(())
}
```

## 🏎️ Performance

`zabi-rs` is orders of magnitude faster than traditional decoders because it skips the parsing/allocation phase.

| Benchmark | zabi-rs | alloy-rs | ethers-rs |
|-----------|---------|----------|-----------|
| **Uint256** | **~0.9 ns** | ~27 ns | ~78 ns |
| **Simple Tuple** | **~7.8 ns** | ~63 ns | ~121 ns |
| **Large Array** | **~2.5 ns** | ~831 ns | ~5,170 ns |

*> Benchmarks run on M2 Air. See `benches/` for details.*

## 📚 Documentation
- [Migration Guide from ethers-rs](docs/migration.md)
- [Full API Documentation](https://docs.rs/zabi-rs)

## 🧭 v0.0.14
- More descriptive decode failures with type, field, and byte-offset context.

## 🗺️ Roadmap
Check out [docs/roadmap.md](docs/roadmap.md) for future plans (Zero-Copy Encoding, WASM).

## License
MIT

