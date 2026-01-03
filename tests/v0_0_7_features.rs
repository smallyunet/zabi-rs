use zabi_rs::*;

#[test]
fn test_revert_error_string() {
    // 0x08c379a0 (selector)
    // 0x0000...20 (offset to string)
    // 0x0000...0d (length 13)
    // "Hello, World!" (padded)
    let mut data = Vec::new();
    data.extend_from_slice(&[0x08, 0xc3, 0x79, 0xa0]);

    // offset
    let mut offset = [0u8; 32];
    offset[31] = 32;
    data.extend_from_slice(&offset);

    // length
    let mut len = [0u8; 32];
    len[31] = 13;
    data.extend_from_slice(&len);

    // content
    let mut content = [0u8; 32];
    content[0..13].copy_from_slice(b"Hello, World!");
    data.extend_from_slice(&content);

    let result = decode_revert(&data).unwrap();
    if let ZRevert::Error(s) = result {
        assert_eq!(s.0, "Hello, World!");
        assert_eq!(revert_to_string!(result), "Hello, World!");
    } else {
        panic!("Expected ZRevert::Error");
    }
}

#[test]
fn test_revert_panic() {
    // 0x4e487b71 (selector)
    // 0x0000...11 (uint256 code)
    let mut data = Vec::new();
    data.extend_from_slice(&[0x4e, 0x48, 0x7b, 0x71]);

    let mut code = [0u8; 32];
    code[31] = 0x11;
    data.extend_from_slice(&code);

    let result = decode_revert(&data).unwrap();
    if let ZRevert::Panic(p) = result {
        assert_eq!(p.to_u32().unwrap(), 0x11);
        assert_eq!(revert_to_string!(result), "Arithmetic overflow/underflow");
    } else {
        panic!("Expected ZRevert::Panic");
    }
}

#[test]
fn test_int_helpers() {
    // ZU256 is_max
    let max_u256_bytes = [0xff; 32];
    let max_u256 = ZU256(&max_u256_bytes);
    assert!(max_u256.is_max());

    let zero_u256_bytes = [0u8; 32];
    let zero_u256 = ZU256(&zero_u256_bytes);
    assert!(!zero_u256.is_max());

    // ZInt256 signum and abs
    let mut neg_one_bytes = [0xff; 32];
    let neg_one = ZInt256(&neg_one_bytes);
    assert_eq!(neg_one.signum(), -1);
    assert!(!neg_one.is_positive());
    assert_eq!(
        neg_one.abs_bytes(),
        [
            0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1
        ]
    );

    let mut pos_one_bytes = [0u8; 32];
    pos_one_bytes[31] = 1;
    let pos_one = ZInt256(&pos_one_bytes);
    assert_eq!(pos_one.signum(), 1);
    assert!(pos_one.is_positive());
    assert_eq!(pos_one.abs_bytes(), pos_one_bytes);
}

#[test]
fn test_decode_call_macro() {
    // transfer(address,uint256)
    let transfer_selector = [0xa9, 0x05, 0x9c, 0xbb];
    let mut data = Vec::new();
    data.extend_from_slice(&transfer_selector);

    let mut addr = [0u8; 32];
    addr[31] = 0xaa;
    data.extend_from_slice(&addr);

    let mut amount = [0u8; 32];
    amount[31] = 0xff;
    data.extend_from_slice(&amount);

    let (to, val) = decode_call!(&data, ZAddress, ZU256).unwrap();
    assert_eq!(to.0[19], 0xaa);
    assert_eq!(val.0[31], 0xff);
}
