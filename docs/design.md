# Design Philosophy

`zabi-rs` is built on a strict "Zero-Allocation" principle. This document explains the architectural decisions and memory model.

## The Memory Model

In traditional ABI decoders, decoding a `bytes` field often involves allocating a new `Vec<u8>` and copying data from the input buffer.

```rust
// Traditional (Allocates)
let my_bytes: Vec<u8> = decode_bytes(input);
```

In `zabi-rs`, we treat the input buffer as the "Arena". All decoded types are simply **views** into this arena.

```rust
// zabi-rs (Zero-Copy)
let my_bytes: ZBytes<'a> = decode_bytes(input);
// ZBytes internal: struct ZBytes<'a>(&'a [u8]);
```

### Lifetimes

Everything is tied to the lifetime `'a` of the input slice `&'a [u8]`.
- You cannot outlive the input data.
- You reduce GC pressure (if embedded in managed langs) or allocator pressure.

## Safety & Unsafe

ABI decoding is inherently largely about pointer arithmetic, as EVM ABI uses 32-byte words and relative offsets.

To ensure performance, `zabi-rs` may use `unsafe` internally to cast slices to references of fixed arrays (e.g. `&[u8]` -> `&[u8; 32]`).

However, safety is paramount:
1.  **Bounds Checking**: Every read operation checks `offset + size <= len` before creating a reference.
2.  **Encapsulation**: Users of the library never interact with raw pointers. They interact with `Z` types that guarantee valid memory access within their lifetime.

## Why no_std?

High-frequency trading (HFT) bots, embedded wallets, and WASM smart contract containers often require minimal footprints. By avoiding `std`:
- We remove standard library overhead.
- We ensure the code can run on bare metal or specialized runtimes.
