# Migrating from ethers-rs to zabi-rs

`zabi-rs` is designed to be a zero-allocation, high-performance alternative to `ethers-rs`'s ABI decoding. This guide helps you migrate your code.

## Key Differences

| Feature | ethers-rs | zabi-rs |
|---------|-----------|---------|
| **Allocation** | Allocates generic `Token` objects on the heap. | Zero allocation. Returns references (`&[u8]`) or thin wrappers. |
| **Input** | `&[u8]` | `&[u8]` |
| **Output** | `Vec<Token>` or Tuples | Tuples of `ZType` wrappers |
| **Decoding** | `abi::decode(&[ParamType], data)` | `decode_tuple!(data, T1, T2...)` |
| **Values** | Owned (e.g., `U256`, `String`) | Borrowed (e.g., `ZU256<'a>`, `ZString<'a>`) |

## Examples

### Decoding a Tuple

**ethers-rs:**
```rust
use ethers::abi::{decode, ParamType, Token};

let data = hex::decode("...").unwrap();
let params = vec![ParamType::Uint(256), ParamType::Address];
let tokens = decode(&params, &data).unwrap();

let value = tokens[0].clone().into_uint().unwrap();
let addr = tokens[1].clone().into_address().unwrap();
```

**zabi-rs:**
```rust
use zabi_rs::{decode_tuple, ZU256, ZAddress};

let data = hex::decode("...").unwrap(); // or just use &[u8]
let (value, addr) = decode_tuple!(&data, ZU256, ZAddress).unwrap();

// value is ZU256 (wrapper around slice)
// addr is ZAddress (wrapper around slice)
```

### Decoding a Function Call

**ethers-rs:**
```rust
// Usually done via Abigen / Contract bindings
```

**zabi-rs:**
```rust
use zabi_rs::{decode_call, ZU256, ZAddress};

// Hand-rolled dispatch
if calldata.starts_with(&[0xa9, 0x05, 0x9c, 0xbb]) { // transfer(address,uint256)
    let (to, amount) = decode_call!(&calldata, ZAddress, ZU256).unwrap();
}
```

## Handling Types

### Integers (U256)
`zabi-rs` uses `ZU256<'a>`. It does not include arithmetic operations. To perform math, convert to a standard type or `alloy_primitives::U256`.

```rust
let val: ZU256 = ...;
// Convert to primitive if small enough
let u: u64 = val.to_u64().unwrap();
// Or access bytes directly
let bytes: &[u8; 32] = val.as_bytes();
```

### Strings
`ZString<'a>` is a wrapper around `&'a str`.

```rust
let s: ZString = ...;
println!("String: {}", s.as_str());
```

### Arrays
`ZArray<'a, T>` is a lazy view. It does not allocate a vector.

```rust
let arr: ZArray<ZU256> = ...;
for item in arr {
    let val = item.unwrap();
}
```
