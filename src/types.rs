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
