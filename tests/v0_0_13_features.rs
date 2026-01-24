use zabi_rs::{ZDecode, ZString, ZU256};

#[test]
fn test_tuple_dynamic_string_non_zero_offset() {
    // Place an ABI-encoded (string,uint256) tuple at a non-zero offset.
    // Offsets for dynamic values are relative to the start of the tuple encoding.
    let base = 32;
    let mut data = vec![0u8; base + 32 * 4];

    // Head[0] = offset to string tail (0x40)
    data[base + 31] = 64;

    // Head[1] = uint256 value = 5
    data[base + 32 + 31] = 5;

    // Tail at base + 64:
    // length = 2
    data[base + 64 + 31] = 2;
    // data = "Hi" (padded)
    data[base + 96..base + 98].copy_from_slice(b"Hi");

    let (s, v) = <(ZString, ZU256)>::decode(&data, base).expect("failed to decode tuple");
    assert_eq!(s.as_str(), "Hi");
    assert_eq!(v.to_u32().unwrap(), 5);
}

#[derive(Debug, ZDecode, PartialEq)]
struct WithString<'a> {
    msg: ZString<'a>,
    val: ZU256<'a>,
}

#[test]
fn test_derive_dynamic_string_non_zero_offset() {
    // Place an ABI-encoded WithString at a non-zero offset.
    let base = 64;
    let mut data = vec![0u8; base + 32 * 4];

    // Head[0] = offset to msg tail (0x40)
    data[base + 31] = 64;

    // Head[1] = uint256 value = 9
    data[base + 32 + 31] = 9;

    // Tail at base + 64:
    // length = 5
    data[base + 64 + 31] = 5;
    // data = "Hello" (padded)
    data[base + 96..base + 101].copy_from_slice(b"Hello");

    let decoded = WithString::decode(&data, base).expect("failed to decode struct");
    assert_eq!(decoded.msg.as_str(), "Hello");
    assert_eq!(decoded.val.to_u32().unwrap(), 9);
}
