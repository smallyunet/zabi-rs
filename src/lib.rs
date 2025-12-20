#![no_std]

#[cfg(test)]
extern crate alloc;
#[cfg(test)]
extern crate std;

pub mod decoder;
pub mod error;
pub mod types;

pub use decoder::{read_address_from_word, read_u256, read_bytes, read_bool, read_string, read_array_fixed, read_array_dyn};
pub use error::ZError;
pub use types::{ZAddress, ZU256, ZBytes, ZBool, ZString, ZArray};

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
}
