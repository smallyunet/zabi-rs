use zabi_rs::{ZDecode, ZU256, ZAddress, ZBool, ZArray, ZString};

#[derive(Debug, ZDecode, PartialEq)]
struct InnerStruct<'a> {
    val: ZU256<'a>,
    flag: ZBool,
}

#[derive(Debug, ZDecode, PartialEq)]
struct OuterStruct<'a> {
    addr: ZAddress<'a>,
    inner: InnerStruct<'a>,
    message: ZString<'a>,
}

#[test]
fn test_derive_and_nested() {
    let mut data = [0u8; 32 * 6];
    
    // OuterStruct:
    // 0..32: addr
    // 32..64: inner.val
    // 64..96: inner.flag
    // 96..128: offset to message
    // 128..160: message length
    // 160..192: message data
    
    // addr
    data[31] = 0xAA;
    
    // inner.val
    data[63] = 42;
    
    // inner.flag
    data[95] = 1;
    
    // message offset (relative to start of OuterStruct)
    data[127] = 128; 
    
    // message length
    data[159] = 5;
    
    // message data "Hello"
    data[160..165].copy_from_slice(b"Hello");
    
    let decoded: OuterStruct = OuterStruct::decode(&data, 0).expect("failed to decode OuterStruct");
    
    assert_eq!(decoded.addr.as_bytes()[19], 0xAA);
    assert_eq!(decoded.inner.val.as_bytes()[31], 42);
    assert_eq!(decoded.inner.flag.as_bool(), true);
    assert_eq!(decoded.message.as_str(), "Hello");
}

#[test]
fn test_tuple_decode() {
    let mut data = [0u8; 96];
    data[31] = 1;
    data[63] = 2;
    data[95] = 3;
    
    let (a, b, c) = <(ZU256, ZU256, ZU256)>::decode(&data, 0).expect("failed to decode tuple");
    assert_eq!(a.as_bytes()[31], 1);
    assert_eq!(b.as_bytes()[31], 2);
    assert_eq!(c.as_bytes()[31], 3);
}
