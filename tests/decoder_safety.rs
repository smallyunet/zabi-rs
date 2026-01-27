use zabi_rs::decoder::{read_array_dyn, read_bytes};
use zabi_rs::ZU256;

#[test]
fn test_read_bytes_large_length_does_not_panic() {
    // Head (offset=32) + Tail (length=u64::MAX)
    let mut data = [0u8; 64];

    // offset word (points to 32)
    data[31] = 32;

    // length word at offset 32: u64::MAX in the last 8 bytes
    for b in &mut data[56..64] {
        *b = 0xff;
    }

    let result = read_bytes(&data, 0);
    assert!(result.is_err());
}

#[test]
fn test_read_array_dyn_large_length_does_not_panic() {
    // Head (offset=32) + Tail (length=u64::MAX)
    let mut data = [0u8; 64];

    // offset word (points to 32)
    data[31] = 32;

    // length word at offset 32: u64::MAX in the last 8 bytes
    for b in &mut data[56..64] {
        *b = 0xff;
    }

    let result = read_array_dyn::<ZU256>(&data, 0);
    assert!(result.is_err());
}
