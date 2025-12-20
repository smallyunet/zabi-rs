//! Fixed-size bytes types (bytes1 to bytes32).
//! 
//! Unlike dynamic `bytes`, fixed-size bytes are left-aligned in the 32-byte word.
//! Common uses include function selectors (bytes4) and storage keys (bytes32).

use core::fmt;
use core::convert::TryInto;
use crate::error::ZError;

/// Wrapper for fixed-size bytes (bytes1 to bytes32).
/// The bytes are left-aligned in the 32-byte EVM word.
#[derive(Clone, Copy, PartialEq)]
pub struct ZBytesN<'a, const N: usize>(pub &'a [u8; N]);

impl<'a, const N: usize> ZBytesN<'a, N> {
    /// Returns the inner byte array reference.
    #[inline]
    pub fn as_bytes(&self) -> &[u8; N] {
        self.0
    }

    /// Returns the length of the fixed bytes.
    #[inline]
    pub const fn len(&self) -> usize {
        N
    }

    /// Returns whether the bytes are empty (always false for N > 0).
    #[inline]
    pub const fn is_empty(&self) -> bool {
        N == 0
    }
}

impl<'a, const N: usize> fmt::Debug for ZBytesN<'a, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZBytes{}(0x", N)?;
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        write!(f, ")")
    }
}

impl<'a, const N: usize> fmt::Display for ZBytesN<'a, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

/// Helper to read a 32-byte word from a slice at a given offset.
#[inline(always)]
fn peek_word(data: &[u8], offset: usize) -> Result<&[u8; 32], ZError> {
    if offset + 32 > data.len() {
        return Err(ZError::OutOfBounds(offset + 32, data.len()));
    }
    let slice = &data[offset..offset + 32];
    let array_ref: &[u8; 32] = slice.try_into().map_err(|_| ZError::Custom("Slice conversion failed"))?;
    Ok(array_ref)
}

/// Generic function to read fixed-size bytes (bytesN) from ABI-encoded data.
/// Fixed-size bytes are left-aligned in the 32-byte word.
/// The remaining bytes must be zero-padded.
#[inline]
pub fn read_bytes_n<'a, const N: usize>(data: &'a [u8], offset: usize) -> Result<ZBytesN<'a, N>, ZError> {
    if N == 0 || N > 32 {
        return Err(ZError::Custom("bytesN size must be between 1 and 32"));
    }
    
    let word = peek_word(data, offset)?;
    
    // Check that trailing bytes are zero (right-padded)
    if word.iter().skip(N).any(|&b| b != 0) {
        return Err(ZError::Custom("bytesN has non-zero padding bytes"));
    }
    
    // Get reference to the first N bytes
    let bytes_slice = &data[offset..offset + N];
    let bytes_ref: &[u8; N] = bytes_slice.try_into().map_err(|_| ZError::Custom("bytesN slice conversion failed"))?;
    
    Ok(ZBytesN(bytes_ref))
}

/// Read bytes1 from ABI-encoded data.
#[inline]
pub fn read_bytes1(data: &[u8], offset: usize) -> Result<ZBytesN<'_, 1>, ZError> {
    read_bytes_n::<1>(data, offset)
}

/// Read bytes4 from ABI-encoded data.
/// Commonly used for function selectors.
#[inline]
pub fn read_bytes4(data: &[u8], offset: usize) -> Result<ZBytesN<'_, 4>, ZError> {
    read_bytes_n::<4>(data, offset)
}

/// Read bytes8 from ABI-encoded data.
#[inline]
pub fn read_bytes8(data: &[u8], offset: usize) -> Result<ZBytesN<'_, 8>, ZError> {
    read_bytes_n::<8>(data, offset)
}

/// Read bytes20 from ABI-encoded data.
/// Same size as an address but left-aligned.
#[inline]
pub fn read_bytes20(data: &[u8], offset: usize) -> Result<ZBytesN<'_, 20>, ZError> {
    read_bytes_n::<20>(data, offset)
}

/// Read bytes32 from ABI-encoded data.
#[inline]
pub fn read_bytes32(data: &[u8], offset: usize) -> Result<ZBytesN<'_, 32>, ZError> {
    read_bytes_n::<32>(data, offset)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate alloc;

    #[test]
    fn test_bytes4_selector() {
        // bytes4 = 0xdeadbeef (left-aligned, padded with zeros)
        let mut data = [0u8; 32];
        data[0] = 0xde;
        data[1] = 0xad;
        data[2] = 0xbe;
        data[3] = 0xef;
        // Rest is zeros

        let result = read_bytes4(&data, 0).expect("should decode bytes4");
        assert_eq!(result.0, &[0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_bytes32() {
        let mut data = [0u8; 32];
        for i in 0..32 {
            data[i] = i as u8;
        }

        let result = read_bytes32(&data, 0).expect("should decode bytes32");
        for i in 0..32 {
            assert_eq!(result.0[i], i as u8);
        }
    }

    #[test]
    fn test_bytes1() {
        let mut data = [0u8; 32];
        data[0] = 0xff;

        let result = read_bytes1(&data, 0).expect("should decode bytes1");
        assert_eq!(result.0[0], 0xff);
    }

    #[test]
    fn test_invalid_padding() {
        // bytes4 with non-zero padding should fail
        let mut data = [0u8; 32];
        data[0] = 0xde;
        data[1] = 0xad;
        data[2] = 0xbe;
        data[3] = 0xef;
        data[4] = 0x01; // Invalid: should be zero

        let result = read_bytes4(&data, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_bytes20() {
        let mut data = [0u8; 32];
        for i in 0..20 {
            data[i] = (i + 1) as u8;
        }

        let result = read_bytes20(&data, 0).expect("should decode bytes20");
        for i in 0..20 {
            assert_eq!(result.0[i], (i + 1) as u8);
        }
    }

    #[test]
    fn test_out_of_bounds() {
        let data = [0u8; 16]; // Too small for a 32-byte word
        let result = read_bytes4(&data, 0);
        assert!(result.is_err());
    }
}
