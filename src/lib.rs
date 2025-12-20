#![no_std]

#[cfg(test)]
extern crate alloc;
#[cfg(test)]
extern crate std;

pub mod decoder;
pub mod error;
pub mod types;

pub use decoder::{
    read_address_from_word, read_u256, read_int256, read_bytes, read_bool, read_string, read_array_fixed, read_array_dyn,
    read_u8, read_u16, read_u32, read_u64, read_u128,
    read_i8, read_i16, read_i32, read_i64, read_i128
};
pub use error::ZError;
pub use types::{ZAddress, ZU256, ZInt256, ZBytes, ZBool, ZString, ZArray};

/// The main trait for zero-copy decoding.
/// The main trait for zero-copy decoding.
pub trait ZDecode<'a>: Sized {
    fn decode(data: &'a [u8], offset: usize) -> Result<Self, ZError>;
}

impl<'a> ZDecode<'a> for ZU256<'a> {
    fn decode(data: &'a [u8], offset: usize) -> Result<Self, ZError> {
        decoder::read_u256(data, offset)
    }
}

impl<'a> ZDecode<'a> for ZAddress<'a> {
    fn decode(data: &'a [u8], offset: usize) -> Result<Self, ZError> {
        decoder::read_address_from_word(data, offset)
    }
}

impl<'a> ZDecode<'a> for ZBool {
    fn decode(data: &'a [u8], offset: usize) -> Result<Self, ZError> {
        decoder::read_bool(data, offset)
    }
}

impl<'a> ZDecode<'a> for ZInt256<'a> {
    fn decode(data: &'a [u8], offset: usize) -> Result<Self, ZError> {
        decoder::read_int256(data, offset)
    }
}

macro_rules! impl_zdecode_primitive {
    ($t:ty, $func:path) => {
        impl<'a> ZDecode<'a> for $t {
            fn decode(data: &'a [u8], offset: usize) -> Result<Self, ZError> {
                $func(data, offset)
            }
        }
    };
}

impl_zdecode_primitive!(u8, decoder::read_u8);
impl_zdecode_primitive!(u16, decoder::read_u16);
impl_zdecode_primitive!(u32, decoder::read_u32);
impl_zdecode_primitive!(u64, decoder::read_u64);
impl_zdecode_primitive!(u128, decoder::read_u128);

impl_zdecode_primitive!(i8, decoder::read_i8);
impl_zdecode_primitive!(i16, decoder::read_i16);
impl_zdecode_primitive!(i32, decoder::read_i32);
impl_zdecode_primitive!(i64, decoder::read_i64);
impl_zdecode_primitive!(i128, decoder::read_i128);

impl<'a> ZDecode<'a> for ZString<'a> {
    fn decode(data: &'a [u8], offset: usize) -> Result<Self, ZError> {
        decoder::read_string(data, offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn test_zero_copy_decode_manual() {
        // Construct a raw ABI encoded byte array.
        // Signature: (uint256, address)
        // 1. uint256: 0x...01 (32 bytes)
        // 2. address: 0x... (padded to 32 bytes)
        
        let mut data = Vec::new();
        
        // Param 1: uint256 = 1
        let mut p1 = [0u8; 32];
        p1[31] = 1;
        data.extend_from_slice(&p1);
        
        // Param 2: address = 0x1122334455667788990011223344556677889900
        let mut p2 = [0u8; 32];
        let addr_bytes = [
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00,
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00
        ];
        p2[12..32].copy_from_slice(&addr_bytes);
        data.extend_from_slice(&p2);

        // Verify we can read it back without copying
        let decoded_u256 = read_u256(&data, 0).expect("failed to decode u256");
        let decoded_addr = read_address_from_word(&data, 32).expect("failed to decode address");

        // Check values
        assert_eq!(decoded_u256.0[31], 1);
        assert_eq!(decoded_u256.0[0], 0);
        
        assert_eq!(decoded_addr.0, &addr_bytes);
        
        // Ensure they are truly references into `data`
        // We can check pointer distance if we were unsafe, but logically they must be
        // because the types define lifetimes tied to input.
    }

    #[test]
    fn test_extended_types() {
        use crate::decoder::{read_bool, read_string};

        let mut data = Vec::new();

        // 1. bool = true
        // encoded as uint256(1)
        let mut p1 = [0u8; 32];
        p1[31] = 1;
        data.extend_from_slice(&p1);

        // 2. string = "Hello"
        // Encoded as:
        // - Offset to data (from start)
        // - Length of string
        // - String data (padded to 32 bytes)
        
        // Offset is 64 (32 bytes for bool + 32 bytes for the offset itself? No.)
        // Tuple: (bool, string)
        // Head:
        // [0..32]: bool value
        // [32..64]: offset to string data (relative to start of tuple)
        
        // Offset should be 64 (32 bytes bool + 32 bytes offset word)
        let mut p2_offset = [0u8; 32];
        p2_offset[31] = 64;
        data.extend_from_slice(&p2_offset);

        // Data:
        // Length: 5
        let mut string_len = [0u8; 32];
        string_len[31] = 5;
        data.extend_from_slice(&string_len);

        // Content: "Hello"
        let mut string_content = [0u8; 32];
        let s_bytes = b"Hello";
        string_content[0..5].copy_from_slice(s_bytes);
        data.extend_from_slice(&string_content);

        // Decode
        let val_bool = read_bool(&data, 0).expect("failed bool");
        let val_str = read_string(&data, 32).expect("failed string");


        assert_eq!(val_bool.0, true);
        assert_eq!(val_str.0, "Hello");
    }

    #[test]
    fn test_array_decoding() {
        use crate::decoder::{read_array_fixed, read_array_dyn};
        use crate::types::ZU256;

        // 1. Fixed Array: uint256[2] = [1, 2]
        let mut data = Vec::new();
        // Element 0: 1
        let mut p0 = [0u8; 32];
        p0[31] = 1;
        data.extend_from_slice(&p0);
        // Element 1: 2
        let mut p1 = [0u8; 32];
        p1[31] = 2;
        data.extend_from_slice(&p1);

        // Decode
        let arr_fixed: crate::types::ZArray<ZU256> = read_array_fixed(&data, 0, 2).expect("fixed array");
        assert_eq!(arr_fixed.len(), 2);
        assert_eq!(arr_fixed.get(0).unwrap().0[31], 1);
        assert_eq!(arr_fixed.get(1).unwrap().0[31], 2);

        // 2. Dynamic Array: uint256[] = [3, 4]
        // Encoded as: Offset (head) -> Length -> Elements
        let mut dyn_data = Vec::new();
        
        // Offset to data (32 bytes)
        let mut offset_word = [0u8; 32];
        offset_word[31] = 32; // Data starts at offset 32
        dyn_data.extend_from_slice(&offset_word);

        // Length: 2
        let mut len_word = [0u8; 32];
        len_word[31] = 2;
        dyn_data.extend_from_slice(&len_word);

        // Element 0: 3
        let mut p2 = [0u8; 32];
        p2[31] = 3;
        dyn_data.extend_from_slice(&p2);

        // Element 1: 4
        let mut p3 = [0u8; 32];
        p3[31] = 4;
        dyn_data.extend_from_slice(&p3);

        // Decode
        let arr_dyn: crate::types::ZArray<ZU256> = read_array_dyn(&dyn_data, 0).expect("dyn array");
        assert_eq!(arr_dyn.len(), 2);
        assert_eq!(arr_dyn.get(0).unwrap().0[31], 3);
        assert_eq!(arr_dyn.get(1).unwrap().0[31], 4);
    }

    #[test]
    fn test_integers() {
        use crate::decoder::*;
        use alloc::vec::Vec;

        let mut data = Vec::new();

        // 1. u8 = 0xFF
        let mut w1 = [0u8; 32];
        w1[31] = 0xFF;
        data.extend_from_slice(&w1);

        // 2. u64 = 0xDEADBEEF
        let mut w2 = [0u8; 32];
        // 0xDEADBEEF = 3735928559
        let val_u64: u64 = 0xDEADBEEF;
        let bytes_u64 = val_u64.to_be_bytes();
        w2[24..32].copy_from_slice(&bytes_u64);
        data.extend_from_slice(&w2);

        // 3. i8 = -1 (0xFF...FF)
        let mut w3 = [0xff; 32];
        data.extend_from_slice(&w3);

        // 4. i8 = 1 (0x00...01)
        let mut w4 = [0u8; 32];
        w4[31] = 1;
        data.extend_from_slice(&w4);

        // 5. Invalid u8 (dirty high bits)
        let mut w5 = [0u8; 32];
        w5[30] = 1; // dirty
        w5[31] = 1;
        data.extend_from_slice(&w5);

        // Test Decode
        assert_eq!(read_u8(&data, 0).unwrap(), 0xFF);
        assert_eq!(read_u64(&data, 32).unwrap(), 0xDEADBEEF);
        
        assert_eq!(read_i8(&data, 64).unwrap(), -1);
        assert_eq!(read_i8(&data, 96).unwrap(), 1);

        // Test Invalid
        assert!(read_u8(&data, 128).is_err());
    }
}
