use crate::ZError;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};

// We need to refer to ZDecode trait.
// Since we are in a submodule, we can use crate::ZDecode
use crate::ZDecode;

/// Wrapper for EVM Arrays (fixed or dynamic).
/// Provides zero-copy access to elements.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(bound = "T: Serialize + ZDecode<'a>"))]
pub struct ZArray<'a, T> {
    pub data: &'a [u8],
    pub start_offset: usize,
    pub length: usize,
    pub _marker: PhantomData<T>,
}

impl<'a, T> Clone for ZArray<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> Copy for ZArray<'a, T> {}

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
    where
        T: ZDecode<'a>,
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

/// Iterator for ZArray.
pub struct ZArrayIterator<'a, T> {
    array: ZArray<'a, T>,
    index: usize,
}

impl<'a, T: ZDecode<'a>> Iterator for ZArrayIterator<'a, T> {
    type Item = Result<T, ZError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.array.len() {
            None
        } else {
            let res = self.array.get(self.index);
            self.index += 1;
            Some(res)
        }
    }
}

impl<'a, T: ZDecode<'a>> IntoIterator for ZArray<'a, T> {
    type Item = Result<T, ZError>;
    type IntoIter = ZArrayIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ZArrayIterator {
            array: self,
            index: 0,
        }
    }
}

impl<'a, T> ZArray<'a, T> {
    /// Create an iterator over the array elements.
    pub fn iter(&self) -> ZArrayIterator<'a, T> {
        ZArrayIterator {
            array: *self,
            index: 0,
        }
    }
}

/// Wrapper around a 20-byte Ethereum address reference.
#[derive(Clone, Copy, PartialEq, Eq)]
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

impl<'a> PartialOrd for ZAddress<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for ZAddress<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(other.0)
    }
}

impl<'a> Hash for ZAddress<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[cfg(feature = "serde")]
impl<'a> Serialize for ZAddress<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as "0x..." string
        let mut hex_str = alloc::string::String::with_capacity(42);
        hex_str.push_str("0x");
        for byte in self.0 {
            use core::fmt::Write;
            write!(hex_str, "{:02x}", byte).unwrap();
        }
        serializer.serialize_str(&hex_str)
    }
}

/// Wrapper around a 32-byte EVM word (uint256) reference.
#[derive(Clone, Copy, PartialEq, Eq)]
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

    /// Convert to u32 if the value fits (upper 28 bytes are zero).
    /// Returns None if the value overflows u32.
    #[inline]
    pub fn to_u32(&self) -> Option<u32> {
        // Check if upper 28 bytes are zero
        for i in 0..28 {
            if self.0[i] != 0 {
                return None;
            }
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.0[28..32]);
        Some(u32::from_be_bytes(bytes))
    }

    /// Convert to u16 if the value fits.
    #[inline]
    pub fn to_u16(&self) -> Option<u16> {
        self.to_u32().and_then(|v| v.try_into().ok())
    }

    /// Convert to u8 if the value fits.
    #[inline]
    pub fn to_u8(&self) -> Option<u8> {
        self.to_u32().and_then(|v| v.try_into().ok())
    }

    /// Check if the value is all ones (max uint256).
    #[inline]
    pub fn is_max(&self) -> bool {
        self.0.iter().all(|&b| b == 0xff)
    }
}

impl<'a> PartialOrd for ZU256<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for ZU256<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(other.0)
    }
}

impl<'a> Hash for ZU256<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[cfg(feature = "serde")]
impl<'a> Serialize for ZU256<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut hex_str = alloc::string::String::with_capacity(66);
        hex_str.push_str("0x");
        for byte in self.0 {
            use core::fmt::Write;
            write!(hex_str, "{:02x}", byte).unwrap();
        }
        serializer.serialize_str(&hex_str)
    }
}

/// Wrapper around a 32-byte EVM word (int256) reference.
/// Semantically represents a signed integer.
#[derive(Clone, Copy, PartialEq, Eq)]
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

    /// Convert to i32 if the value fits.
    #[inline]
    pub fn to_i32(&self) -> Option<i32> {
        let is_negative = self.is_negative();
        let expected_padding = if is_negative { 0xff } else { 0x00 };
        for i in 0..28 {
            if self.0[i] != expected_padding {
                return None;
            }
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.0[28..32]);
        Some(i32::from_be_bytes(bytes))
    }

    /// Convert to i16 if the value fits.
    #[inline]
    pub fn to_i16(&self) -> Option<i16> {
        self.to_i32().and_then(|v| v.try_into().ok())
    }

    /// Convert to i8 if the value fits.
    #[inline]
    pub fn to_i8(&self) -> Option<i8> {
        self.to_i32().and_then(|v| v.try_into().ok())
    }

    /// Check if the value is positive (not zero and MSB is 0).
    #[inline]
    pub fn is_positive(&self) -> bool {
        !self.is_negative() && !self.0.iter().all(|&b| b == 0)
    }

    /// Get the absolute value (returns a new array since we can't easily return a slice of a computed value).
    /// Note: This violates the zero-copy principle slightly by returning a value, but ZU256 normally wraps a slice.
    /// For absolute value, we might want to return `[u8; 32]`.
    #[inline]
    pub fn abs_bytes(&self) -> [u8; 32] {
        if !self.is_negative() {
            *self.0
        } else {
            let mut res = [0u8; 32];
            let mut carry = 1u16;
            for i in (0..32).rev() {
                let val = (!self.0[i]) as u16 + carry;
                res[i] = val as u8;
                carry = val >> 8;
            }
            res
        }
    }

    /// Returns the signum of the number (-1, 0, or 1).
    #[inline]
    pub fn signum(&self) -> i8 {
        if self.0.iter().all(|&b| b == 0) {
            0
        } else if self.is_negative() {
            -1
        } else {
            1
        }
    }
}

// Note: ZInt256 comparison is tricky because it's stored as bytes (raw two's complement).
// Normal byte comparison works for positive numbers, but negative numbers (high bit set)
// will look "larger" than positive numbers if compared as unsigned bytes.
// So we must implement Ord carefully for signed comparison.
impl<'a> PartialOrd for ZInt256<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for ZInt256<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_neg = self.is_negative();
        let other_neg = other.is_negative();

        match (self_neg, other_neg) {
            (true, true) => {
                // Both negative: larger unsigned value is "closer to zero" (larger) in two's complement?
                // e.g. -1 is 0xFF, -2 is 0xFE. 0xFF > 0xFE. So -1 > -2.
                // Yes, byte comparison is correct for two negatives.
                self.0.cmp(other.0)
            }
            (false, false) => {
                // Both positive: byte comparison is correct.
                self.0.cmp(other.0)
            }
            (true, false) => {
                // Self is negative, Other is positive. Self < Other.
                Ordering::Less
            }
            (false, true) => {
                // Self is positive, Other is negative. Self > Other.
                Ordering::Greater
            }
        }
    }
}

impl<'a> Hash for ZInt256<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[cfg(feature = "serde")]
impl<'a> Serialize for ZInt256<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // For simple serialization, we can just use the hex representation
        let mut hex_str = alloc::string::String::with_capacity(66);
        hex_str.push_str("0x");
        for byte in self.0 {
            use core::fmt::Write;
            write!(hex_str, "{:02x}", byte).unwrap();
        }
        serializer.serialize_str(&hex_str)
    }
}

/// Wrapper around a variable-length byte array reference.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ZBytes<'a>(pub &'a [u8]);

impl<'a> fmt::Debug for ZBytes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZBytes(len={}, data=0x", self.0.len())?;
        for (i, byte) in self.0.iter().enumerate() {
            if i >= 32 {
                // Truncate for debug
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

impl<'a> ZBytes<'a> {
    /// Returns the length of the bytes.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the bytes are empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the inner byte slice.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.0
    }
}

impl<'a> PartialOrd for ZBytes<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for ZBytes<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(other.0)
    }
}

impl<'a> Hash for ZBytes<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[cfg(feature = "serde")]
impl<'a> Serialize for ZBytes<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut hex_str = alloc::string::String::with_capacity(2 + self.0.len() * 2);
        hex_str.push_str("0x");
        for byte in self.0 {
            use core::fmt::Write;
            write!(hex_str, "{:02x}", byte).unwrap();
        }
        serializer.serialize_str(&hex_str)
    }
}

/// Wrapper around a boolean value.
/// Note: EVM booleans are uint256 (0 or 1).
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
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

impl ZBool {
    /// Returns the inner boolean value.
    #[inline]
    pub fn as_bool(&self) -> bool {
        self.0
    }
}

/// Wrapper around a UTF-8 string slice reference.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ZString<'a>(pub &'a str);

impl<'a> PartialOrd for ZString<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for ZString<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(other.0)
    }
}

impl<'a> Hash for ZString<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

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

impl<'a> ZString<'a> {
    /// Returns the length of the string in bytes.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the string is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the inner string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0
    }
}

/// Represents an Ethereum revert reason.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZRevert<'a> {
    /// Standard error string: Error(string)
    Error(ZString<'a>),
    /// Solidity panic: Panic(uint256)
    Panic(ZU256<'a>),
    /// Custom error (selector and raw encoded data)
    Custom(&'a [u8; 4], &'a [u8]),
    /// Unknown or empty revert
    Unknown,
}

impl<'a> fmt::Display for ZRevert<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZRevert::Error(s) => write!(f, "Revert: {}", s.0),
            ZRevert::Panic(p) => write!(f, "Panic: {}", p),
            ZRevert::Custom(sel, _) => write!(
                f,
                "CustomError(0x{:02x}{:02x}{:02x}{:02x})",
                sel[0], sel[1], sel[2], sel[3]
            ),
            ZRevert::Unknown => write!(f, "Unknown Revert"),
        }
    }
}

/// Result of a function call, either success or revert.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZCallResult<'a, T> {
    Success(T),
    Revert(ZRevert<'a>),
}

impl<'a, T> ZCallResult<'a, T> {
    pub fn is_success(&self) -> bool {
        matches!(self, ZCallResult::Success(_))
    }

    pub fn is_revert(&self) -> bool {
        matches!(self, ZCallResult::Revert(_))
    }

    pub fn unwrap(self) -> T {
        match self {
            ZCallResult::Success(val) => val,
            ZCallResult::Revert(e) => panic!("Called unwrap on a Revert: {:?}", e),
        }
    }
}
