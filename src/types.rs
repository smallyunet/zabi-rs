use core::fmt;

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
