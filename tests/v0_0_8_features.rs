use zabi_rs::*;

#[test]
fn test_zarray_iterator() {
    // uint256[2] = [1, 2]
    let mut data = Vec::new();
    let mut p0 = [0u8; 32];
    p0[31] = 1;
    let mut p1 = [0u8; 32];
    p1[31] = 2;
    data.extend_from_slice(&p0);
    data.extend_from_slice(&p1);

    let arr: ZArray<ZU256> = read_array_fixed(&data, 0, 2).unwrap();

    let mut iter = arr.iter();
    assert_eq!(iter.next().unwrap().unwrap().0[31], 1);
    assert_eq!(iter.next().unwrap().unwrap().0[31], 2);
    assert!(iter.next().is_none());

    // Loop
    let mut sum = 0;
    for item in arr {
        let val = item.unwrap().to_u32().unwrap();
        sum += val;
    }
    assert_eq!(sum, 3);
}

#[test]
fn test_traits() {
    let mut b1 = [0u8; 32];
    b1[31] = 1;
    let mut b2 = [0u8; 32];
    b2[31] = 2;

    let u1 = ZU256(&b1);
    let u2 = ZU256(&b2);

    assert!(u1 < u2);
    assert_ne!(u1, u2);

    // Hash
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(u1);
    assert!(set.contains(&u1));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde() {
    let mut b1 = [0u8; 32];
    b1[31] = 0xff;
    let u1 = ZU256(&b1); // 0x...ff

    let json = serde_json::to_string(&u1).unwrap();
    // Depends on implementation, currently "0x...ff"
    assert!(json.starts_with("\"0x"));
    assert!(json.ends_with("ff\""));
}

#[test]
fn test_encoder() {
    let mut buf = [0u8; 64];

    // Encode u256
    let val = [1u8; 32];
    encode_u256(&mut buf, 0, &val).unwrap();
    assert_eq!(&buf[0..32], &val);

    // Encode bool at offset 32
    encode_bool(&mut buf, 32, true).unwrap();
    assert_eq!(buf[63], 1); // Last byte of 2nd word is 1
}

#[test]
fn test_unchecked() {
    use zabi_rs::decoder::read_u256_unchecked;
    let mut data = [0u8; 32];
    data[31] = 100;

    let val = unsafe { read_u256_unchecked(&data, 0) };
    assert_eq!(val.0[31], 100);
}
