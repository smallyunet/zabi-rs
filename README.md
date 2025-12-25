# zabi-rs

**Zero-Allocation ABI Decoder** for Rust.

![Crates.io Version](https://img.shields.io/crates/v/zabi-rs)
![Crates.io License](https://img.shields.io/crates/l/zabi-rs)


`zabi-rs` is a high-performance, `#![no_std]` compatible library designed for decoding Ethereum Virtual Machine (EVM) ABI encoded data **without any heap allocation**.

Unlike standard libraries like `ethers-rs` or `alloy-rs` which decode data into owned types (`Vec`, `String`, `BigInt`), `zabi-rs` maps Rust structs directly to the underlying raw byte slice (`&'a [u8]`) using explicit lifetimes.

## Features

- 🚀 **Zero Allocation**: No `malloc`, no `Box`, no `Vec`. All outcomes are references.
- ⚡ **High Performance**: Designed for hot-path decoding, MEV bots, and embedded environments.
- 🛡️ **Safe & Unsafe Encapsulation**: Uses pointer arithmetic for speed but provides safe wrappers with bounds checking.
- 🔧 **no_std Compatible**: Ready for strict embedded or WASM environments.
- 📦 **Primitive Support**: `address`, `uint256`, `bytes` (Basic types supported currently).

## Performance

<!-- BENCHMARK_TABLE_START -->

| Scenario | zabi-rs | alloy | ethers |
|----------|---|---|---|
| HeavyArray | 2.1766 ns | 825.76 ns | 5.1155 µs | 
| SimpleTuple | 7.7762 ns | 63.456 ns | 121.36 ns | 
| Uint256 | 934.09 ps | 20.065 ns | 78.212 ns | 
| Uint64 | 4.5093 ns | 22.066 ns | 66.942 ns | 

<!-- BENCHMARK_TABLE_END -->

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
zabi-rs = "0.0.6"
```

## Usage

```rust
use zabi_rs::{read_u256, read_address_from_word};

fn main() -> Result<(), zabi_rs::ZError> {
    // Example: A raw ABI encoded byte array (mocked)
    // 32 bytes for uint256(1) + 32 bytes for address
    let mut data = [0u8; 64];
    data[31] = 1; // uint256 = 1
    data[44] = 0xaa; // address starts at offset 44 (12 padding + 20 bytes)
    
    // Decode without copying
    // Returns ZU256<'a> which wraps the slice
    let value = read_u256(&data, 0)?;
    
    // Returns ZAddress<'a>
    let sender = read_address_from_word(&data, 32)?;
    
    println!("decoded value: {:?}", value);
    println!("decoded sender: {:?}", sender);
    
    Ok(())
}
```

## Testing

Run the test suite:

```bash
cargo test
```

## License

MIT

## Roadmap
See [docs/roadmap.md](docs/roadmap.md) for future plans.

