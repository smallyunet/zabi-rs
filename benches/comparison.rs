use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zabi_rs::{read_u256, read_address_from_word, read_bool, ZU256};
use alloy_sol_types::{SolType, sol};
use ethers::abi::AbiDecode;
use ethers::types::U256 as EthersU256;

// Define Sol types for Alloy
sol! {
    struct SimpleTuple {
        uint256 a;
        address b;
        bool c;
    }
}

// Scenarios
fn bench_uint256(c: &mut Criterion) {
    let mut group = c.benchmark_group("Decoding/Uint256");

    // Encoded: uint256(1)
    let mut data = [0u8; 32];
    data[31] = 1;

    // zabi-rs
    group.bench_function("zabi-rs", |b| {
        b.iter(|| {
            let res = read_u256(black_box(&data), 0).unwrap();
            black_box(res);
        })
    });

    // alloy
    // uint256 is just a primitive, alloy usually decodes tuples or specific types via SolType
    group.bench_function("alloy", |b| {
        b.iter(|| {
             let res = <alloy_sol_types::sol_data::Uint<256>>::abi_decode(black_box(&data), true).unwrap();
             black_box(res);
        })
    });

    // ethers
    group.bench_function("ethers", |b| {
        b.iter(|| {
             let res = EthersU256::decode(black_box(&data[..])).unwrap();
             black_box(res);
        })
    });

    group.finish();
}

fn bench_simple_tuple(c: &mut Criterion) {
    let mut group = c.benchmark_group("Decoding/SimpleTuple");

    // (uint256(1), address(0x...1), bool(true))
    // 32 * 3 = 96 bytes
    let mut data = Vec::new();
    // 1. uint256(1)
    let mut p1 = [0u8; 32]; p1[31] = 1; data.extend_from_slice(&p1);
    // 2. address
    let mut p2 = [0u8; 32]; p2[31] = 0xAA; data.extend_from_slice(&p2);
    // 3. bool(true)
    let mut p3 = [0u8; 32]; p3[31] = 1; data.extend_from_slice(&p3);
    
    let data_slice = data.as_slice();

    // zabi-rs
    group.bench_function("zabi-rs", |b| {
        b.iter(|| {
            let u = read_u256(black_box(data_slice), 0).unwrap();
            let a = read_address_from_word(black_box(data_slice), 32).unwrap();
            let boolean = read_bool(black_box(data_slice), 64).unwrap();
            black_box((u, a, boolean));
        })
    });

    // alloy
    group.bench_function("alloy", |b| {
        b.iter(|| {
             let res = SimpleTuple::abi_decode(black_box(data_slice), true).unwrap();
             black_box(res);
        })
    });

    // ethers
    // Ethers decode usually takes param types
    // We simulate decoding a tuple of inputs
    let params = vec![
        ethers::abi::ParamType::Uint(256),
        ethers::abi::ParamType::Address,
        ethers::abi::ParamType::Bool,
    ];
    group.bench_function("ethers", |b| {
        b.iter(|| {
             let res = ethers::abi::decode(&params, black_box(data_slice)).unwrap();
             black_box(res);
        })
    });

    group.finish();
}

fn bench_array(c: &mut Criterion) {
    let mut group = c.benchmark_group("Decoding/HeavyArray");
    // uint256[100]
    let mut data = Vec::new();
    for i in 0..100 {
        let mut word = [0u8; 32];
        word[31] = i as u8;
        data.extend_from_slice(&word);
    }
    let data_slice = data.as_slice();

    // zabi-rs
    group.bench_function("zabi-rs", |b| {
        b.iter(|| {
            // zero-allocation: just wrapping the slice
            let arr = zabi_rs::decoder::read_array_fixed::<ZU256>(black_box(data_slice), 0, 100).unwrap();
            // Access last element to ensure lazy evaluation doesn't skip everything (though construction itself is O(1))
            let last = arr.get(99).unwrap();
            black_box(last);
        })
    });

    // alloy
    // generic array decoding
    group.bench_function("alloy", |b| {
        b.iter(|| {
             let res = <alloy_sol_types::sol_data::FixedArray<alloy_sol_types::sol_data::Uint<256>, 100>>::abi_decode(black_box(data_slice), true).unwrap();
             black_box(res);
        })
    });

    // ethers
    let params = vec![
        ethers::abi::ParamType::FixedArray(Box::new(ethers::abi::ParamType::Uint(256)), 100),
    ];
    group.bench_function("ethers", |b| { // Decoding as a single param (tuple of 1)
        // Ethers decode returns vec<Token>
        b.iter(|| {
             // We need to wrap it in a tuple for ethers usually, or use decode_params?
             // abi::decode takes &[ParamType].
             // Note: data for FixedArray in top-level might be different if it's not a tuple?
             // ABI encoding is always a tuple of parameters. So if we have just uint256[100], it is treated as (uint256[100]).
             let res = ethers::abi::decode(&params, black_box(data_slice)).unwrap();
             black_box(res);
        })
    });

    group.finish();
}

criterion_group!(benches, bench_uint256, bench_simple_tuple, bench_array);
criterion_main!(benches);
