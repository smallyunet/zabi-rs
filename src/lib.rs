#![no_std]

extern crate alloc;
#[cfg(test)]
extern crate std;

pub mod decoder;
pub mod encoder;
pub mod error;
pub mod event;
pub mod types;
pub mod zbytes_fixed;

pub use decoder::{
    decode_call_result, decode_revert, read_address_from_word, read_array_dyn, read_array_fixed,
    read_bool, read_bytes, read_i128, read_i16, read_i32, read_i64, read_i8, read_int256,
    read_selector, read_string, read_u128, read_u16, read_u256, read_u32, read_u64, read_u8,
    skip_selector,
};
pub use encoder::{encode_address, encode_bool, encode_bytes_data, encode_u256};
pub use error::ZError;
pub use event::{
    read_topic_address, read_topic_bool, read_topic_int256, read_topic_u256, ZEventLog,
};
pub use types::{ZAddress, ZArray, ZBool, ZBytes, ZCallResult, ZInt256, ZRevert, ZString, ZU256};
pub use zbytes_fixed::{
    read_bytes1, read_bytes16, read_bytes2, read_bytes20, read_bytes3, read_bytes32, read_bytes4,
    read_bytes8, read_bytes_n, ZBytesN,
};

#[cfg(feature = "derive")]
pub use zabi_derive::ZDecode;

/// Decode a tuple of types from ABI-encoded data.
///
/// This macro decodes multiple values sequentially from ABI-encoded data,
/// assuming each value occupies a 32-byte slot (head portion).
///
/// # Example
/// ```
/// use zabi_rs::{decode_tuple, ZU256, ZAddress, ZBool, ZDecode};
///
/// let mut data = [0u8; 96];
/// data[31] = 1;  // uint256 = 1
/// data[63] = 0xAA; // address (last byte)
/// data[95] = 1;  // bool = true
///
/// let (a, b, c) = decode_tuple!(&data, ZU256, ZAddress, ZBool).unwrap();
/// ```
#[macro_export]
macro_rules! decode_tuple {
    ($data:expr, $($T:ty),+ $(,)?) => {{
        let data: &[u8] = $data;
        let mut offset: usize = 0;
        (|| -> Result<($($T,)+), $crate::ZError> {
            Ok((
                $(
                    {
                        let val = <$T as $crate::ZDecode>::decode(data, offset)?;
                        offset += <$T as $crate::ZDecode>::HEAD_SIZE;
                        val
                    }
                ,)+
            ))
        })()
    }};
}

/// Decodes a function call from calldata.
///
/// This macro extracts the selector and decodes the arguments based on the selector.
/// It assumes the arguments start immediately after the 4-byte selector.
///
/// # Example
/// ```
/// use zabi_rs::{decode_call, ZU256, ZAddress};
///
/// fn test() {
///     let calldata = [0u8; 68];
///     // Example selector for `transfer(address,uint256)`
///     let transfer_selector = [0xa9, 0x05, 0x9c, 0xbb];
///
///     if calldata.starts_with(&transfer_selector) {
///         let (to, amount) = decode_call!(&calldata, ZAddress, ZU256).unwrap();
///     }
/// }
/// ```
#[macro_export]
macro_rules! decode_call {
    ($data:expr, $($T:ty),+ $(,)?) => {{
        let data: &[u8] = $data;
        (|| -> Result<($($T,)+), $crate::ZError> {
            let params = $crate::skip_selector(data)?;
            let mut offset: usize = 0;
            Ok((
                $({
                    let val = <$T as $crate::ZDecode>::decode(params, offset)?;
                    offset += <$T as $crate::ZDecode>::HEAD_SIZE;
                    val
                },)+
            ))
        })()
    }};
}

/// Helper to get a human-readable string from a ZRevert.
#[macro_export]
macro_rules! revert_to_string {
    ($revert:expr) => {
        match $revert {
            $crate::ZRevert::Error(s) => s.0,
            $crate::ZRevert::Panic(p) => {
                let code = p.to_u32().unwrap_or(0);
                match code {
                    0x01 => "Assertion violation",
                    0x11 => "Arithmetic overflow/underflow",
                    0x12 => "Division by zero",
                    0x21 => "Invalid enum value",
                    0x22 => "Invalid storage byte array",
                    0x31 => "Pop on empty array",
                    0x32 => "Index out of bounds",
                    0x41 => "Out of memory",
                    0x51 => "Invalid internal function",
                    _ => "Unknown Panic",
                }
            }
            $crate::ZRevert::Custom(sel, _) => "Custom Error",
            $crate::ZRevert::Unknown => "Unknown Error",
        }
    };
}

/// The main trait for zero-copy decoding.
/// The main trait for zero-copy decoding.
pub trait ZDecode<'a>: Sized {
    const HEAD_SIZE: usize = 32; // Default for words and offsets
    fn decode(data: &'a [u8], offset: usize) -> Result<Self, ZError>;
}

impl<'a> ZDecode<'a> for ZU256<'a> {
    const HEAD_SIZE: usize = 32;
    fn decode(data: &'a [u8], offset: usize) -> Result<Self, ZError> {
        decoder::read_u256(data, offset)
    }
}

impl<'a> ZDecode<'a> for ZAddress<'a> {
    const HEAD_SIZE: usize = 32;
    fn decode(data: &'a [u8], offset: usize) -> Result<Self, ZError> {
        decoder::read_address_from_word(data, offset)
    }
}

impl<'a> ZDecode<'a> for ZBool {
    const HEAD_SIZE: usize = 32;
    fn decode(data: &'a [u8], offset: usize) -> Result<Self, ZError> {
        decoder::read_bool(data, offset)
    }
}

impl<'a> ZDecode<'a> for ZInt256<'a> {
    const HEAD_SIZE: usize = 32;
    fn decode(data: &'a [u8], offset: usize) -> Result<Self, ZError> {
        decoder::read_int256(data, offset)
    }
}

macro_rules! impl_zdecode_primitive {
    ($t:ty, $func:path) => {
        impl<'a> ZDecode<'a> for $t {
            const HEAD_SIZE: usize = 32;
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

impl<'a, T: ZDecode<'a>> ZDecode<'a> for ZArray<'a, T> {
    const HEAD_SIZE: usize = 32;
    fn decode(data: &'a [u8], offset: usize) -> Result<Self, ZError> {
        decoder::read_array_dyn(data, offset)
    }
}

impl<'a, const N: usize> ZDecode<'a> for ZBytesN<'a, N> {
    const HEAD_SIZE: usize = 32;
    fn decode(data: &'a [u8], offset: usize) -> Result<Self, ZError> {
        zbytes_fixed::read_bytes_n(data, offset)
    }
}

macro_rules! impl_zdecode_tuple {
    ($($T:ident),+) => {
        impl<'a, $($T: ZDecode<'a>),+> ZDecode<'a> for ($($T,)+) {
            const HEAD_SIZE: usize = 0 $(+ <$T as ZDecode>::HEAD_SIZE)*;
            fn decode(data: &'a [u8], offset: usize) -> Result<Self, ZError> {
                if offset > data.len() {
                    return Err(ZError::OutOfBounds(offset, data.len()));
                }
                let data = &data[offset..];

                let mut offset: usize = 0;
                #[allow(unused_assignments)]
                Ok((
                    $({
                        let val = <$T as ZDecode>::decode(data, offset)?;
                        offset += <$T as ZDecode>::HEAD_SIZE;
                        val
                    },)+
                ))
            }
        }
    };
}

impl_zdecode_tuple!(T1);
impl_zdecode_tuple!(T1, T2);
impl_zdecode_tuple!(T1, T2, T3);
impl_zdecode_tuple!(T1, T2, T3, T4);
impl_zdecode_tuple!(T1, T2, T3, T4, T5);
impl_zdecode_tuple!(T1, T2, T3, T4, T5, T6);
impl_zdecode_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_zdecode_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_zdecode_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_zdecode_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_zdecode_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_zdecode_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);

impl<'a> ZDecode<'a> for ZString<'a> {
    const HEAD_SIZE: usize = 32;
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
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00, 0x11, 0x22, 0x33, 0x44,
            0x55, 0x66, 0x77, 0x88, 0x99, 0x00,
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
        use crate::decoder::{read_array_dyn, read_array_fixed};
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
        let arr_fixed: crate::types::ZArray<ZU256> =
            read_array_fixed(&data, 0, 2).expect("fixed array");
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
        let w3 = [0xff; 32];
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
