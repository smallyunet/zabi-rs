#![no_std]

#[cfg(test)]
extern crate alloc;
#[cfg(test)]
extern crate std;

pub mod decoder;
pub mod error;
pub mod types;

pub use decoder::{read_address_from_word, read_u256, read_bytes};
pub use error::ZError;
pub use types::{ZAddress, ZU256, ZBytes};

/// The main trait for zero-copy decoding.
pub trait ZDecode<'a>: Sized {
    fn decode(bytes: &'a [u8]) -> Result<Self, ZError>;
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
}
