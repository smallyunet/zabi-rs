use core::fmt;
use core::marker::PhantomData;
use crate::ZError;

// We need to refer to ZDecode trait. 
// Since we are in a submodule, we can use crate::ZDecode
use crate::ZDecode;


/// Wrapper for EVM Arrays (fixed or dynamic).
/// Provides zero-copy access to elements.
#[derive(Clone, Copy)]
pub struct ZArray<'a, T> {
    pub data: &'a [u8],
    pub start_offset: usize,
    pub length: usize,
    pub _marker: PhantomData<T>,
}

impl<'a, T> ZArray<'a, T> {
    pub fn new(data: &'a [u8], start_offset: usize, length: usize) -> Self {
        Self {
            data,
            start_offset,
            length,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn get(&self, index: usize) -> Result<T, ZError> 
    where T: ZDecode<'a>
    {
        if index >= self.length {
            return Err(ZError::OutOfBounds(index, self.length));
        }
        let offset = self.start_offset + index * 32;
        T::decode(self.data, offset)
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for ZArray<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZArray(len={})", self.length)
    }
}

/// Wrapper around a 20-byte Ethereum address reference.
#[derive(Clone, Copy, PartialEq)]
pub struct ZAddress<'a>(pub &'a [u8; 20]);

impl<'a> fmt::Debug for ZAddress<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZAddress(0x")?;
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        write!(f, ")")
    }
}

impl<'a> fmt::Display for ZAddress<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl<'a> ZAddress<'a> {
    /// Copy the address bytes to a new [u8; 20] array.
    #[inline]
    pub fn to_bytes(&self) -> [u8; 20] {
        *self.0
    }

    /// Returns the inner byte array reference.
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 20] {
        self.0
    }
}

/// Wrapper around a 32-byte EVM word (uint256) reference.
#[derive(Clone, Copy, PartialEq)]
pub struct ZU256<'a>(pub &'a [u8; 32]);

impl<'a> fmt::Debug for ZU256<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZU256(0x")?;
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        write!(f, ")")
    }
}

impl<'a> fmt::Display for ZU256<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl<'a> ZU256<'a> {
    /// Convert to u128 if the value fits (upper 16 bytes are zero).
    /// Returns None if the value overflows u128.
    #[inline]
    pub fn to_u128(&self) -> Option<u128> {
        // Check if upper 16 bytes are zero
        for i in 0..16 {
            if self.0[i] != 0 {
                return None;
            }
        }
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&self.0[16..32]);
        Some(u128::from_be_bytes(bytes))
    }

    /// Convert to u64 if the value fits (upper 24 bytes are zero).
    /// Returns None if the value overflows u64.
    #[inline]
    pub fn to_u64(&self) -> Option<u64> {
        // Check if upper 24 bytes are zero
        for i in 0..24 {
            if self.0[i] != 0 {
                return None;
            }
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.0[24..32]);
        Some(u64::from_be_bytes(bytes))
    }

    /// Returns the inner byte array reference.
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 32] {
        self.0
    }

    /// Copy the bytes to a new [u8; 32] array.
    #[inline]
    pub fn to_bytes(&self) -> [u8; 32] {
        *self.0
    }

    /// Check if the value is zero.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }
}

/// Wrapper around a 32-byte EVM word (int256) reference.
/// Semantically represents a signed integer.
#[derive(Clone, Copy, PartialEq)]
pub struct ZInt256<'a>(pub &'a [u8; 32]);

impl<'a> fmt::Debug for ZInt256<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZInt256(0x")?;
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        write!(f, ")")
    }
}

impl<'a> fmt::Display for ZInt256<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We display hex for now, interpreting as signed decimal would require big logic
        write!(f, "0x")?;
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl<'a> ZInt256<'a> {
    /// Convert to i128 if the value fits.
    /// Returns None if the value overflows i128.
    #[inline]
    pub fn to_i128(&self) -> Option<i128> {
        // For signed, check sign extension
        let is_negative = self.0[0] & 0x80 != 0;
        let expected_padding = if is_negative { 0xff } else { 0x00 };
        
        // Check if upper 16 bytes are proper sign extension
        for i in 0..16 {
            if self.0[i] != expected_padding {
                return None;
            }
        }
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&self.0[16..32]);
        Some(i128::from_be_bytes(bytes))
    }

    /// Convert to i64 if the value fits.
    /// Returns None if the value overflows i64.
    #[inline]
    pub fn to_i64(&self) -> Option<i64> {
        let is_negative = self.0[0] & 0x80 != 0;
        let expected_padding = if is_negative { 0xff } else { 0x00 };
        
        // Check if upper 24 bytes are proper sign extension
        for i in 0..24 {
            if self.0[i] != expected_padding {
                return None;
            }
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.0[24..32]);
        Some(i64::from_be_bytes(bytes))
    }

    /// Returns the inner byte array reference.
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 32] {
        self.0
    }

    /// Check if the value is negative (MSB is set).
    #[inline]
    pub fn is_negative(&self) -> bool {
        self.0[0] & 0x80 != 0
    }
}

/// Wrapper around a variable-length byte array reference.
#[derive(Clone, Copy, PartialEq)]
pub struct ZBytes<'a>(pub &'a [u8]);

impl<'a> fmt::Debug for ZBytes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZBytes(len={}, data=0x", self.0.len())?;
        for (i, byte) in self.0.iter().enumerate() {
            if i >= 32 { // Truncate for debug
                write!(f, "...")?;
                break;
            }
            write!(f, "{:02x}", byte)?;
        }
        write!(f, ")")
    }
}

impl<'a> fmt::Display for ZBytes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

/// Wrapper around a boolean value.
/// Note: EVM booleans are uint256 (0 or 1).
#[derive(Clone, Copy, PartialEq)]
pub struct ZBool(pub bool);

impl fmt::Debug for ZBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZBool({})", self.0)
    }
}

impl fmt::Display for ZBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Wrapper around a UTF-8 string slice reference.
#[derive(Clone, Copy, PartialEq)]
pub struct ZString<'a>(pub &'a str);

impl<'a> fmt::Debug for ZString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZString({:?})", self.0)
    }
}

impl<'a> fmt::Display for ZString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
