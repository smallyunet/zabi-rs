use zabi_rs::{decode_tuple, ZBool, ZDecode, ZString, ZU256};

#[test]
fn test_decode_tuple_reports_type_and_offset_context() {
    let mut data = [0u8; 64];
    data[63] = 2;

    let err = decode_tuple!(&data, ZU256, ZBool).unwrap_err();
    let message = err.to_string();

    assert!(message.contains("ZBool"), "unexpected error: {message}");
    assert!(
        message.contains("byte offset 32"),
        "unexpected error: {message}"
    );
    assert!(
        message.contains("not 0 or 1"),
        "unexpected error: {message}"
    );
}

#[allow(dead_code)]
#[derive(Debug, ZDecode)]
struct Inner<'a> {
    msg: ZString<'a>,
}

#[allow(dead_code)]
#[derive(Debug, ZDecode)]
struct Outer<'a> {
    inner: Inner<'a>,
}

#[test]
fn test_derive_reports_nested_field_context() {
    let mut data = [0u8; 96];

    data[31] = 32;
    data[63] = 1;
    data[64] = 0xff;

    let err = Outer::decode(&data, 0).unwrap_err();
    let message = err.to_string();

    assert!(
        message.contains("Outer.inner"),
        "unexpected error: {message}"
    );
    assert!(message.contains("Inner.msg"), "unexpected error: {message}");
    assert!(message.contains("ZString"), "unexpected error: {message}");
    assert!(
        message.contains("Invalid UTF-8"),
        "unexpected error: {message}"
    );
}
