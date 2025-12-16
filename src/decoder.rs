use crate::error::ZError;
use crate::types::{ZAddress, ZU256, ZBytes};
use core::convert::TryInto;

/// Helper to read a 32-byte word from a slice at a given offset.
/// Returns reference to the array to avoid copying.
#[inline(always)]
pub fn peek_word(data: &[u8], offset: usize) -> Result<&[u8; 32], ZError> {
    if offset + 32 > data.len() {
        return Err(ZError::OutOfBounds(offset + 32, data.len()));
    }
    // SAFETY: We checked bounds above. The slice matches the size of [u8; 32].
    // We cast the pointer to &[u8; 32].
    // Note: slice.as_ptr() returns *const u8.
    // We strictly use normal safe Rust usually, specifically `try_into`.
    // But to ensure zero-copy and 'reference' semantics, we rely on slice conversion.
    
    let slice = &data[offset..offset + 32];
    let array_ref: &[u8; 32] = slice.try_into().map_err(|_| ZError::Custom("Slice conversion failed"))?;
    Ok(array_ref)
}

/// Helper to read address (last 20 bytes of a 32-byte word).
#[inline(always)]
pub fn read_address_from_word(data: &[u8], offset: usize) -> Result<ZAddress<'_>, ZError> {
    let word = peek_word(data, offset)?;
    // Address is the last 20 bytes of the 32-byte word.
    let addr_slice = &word[12..32];
    let addr_ref: &[u8; 20] = addr_slice.try_into().map_err(|_| ZError::Custom("Address slice conversion failed"))?;
    Ok(ZAddress(addr_ref))
}

#[inline(always)]
pub fn read_u256(data: &[u8], offset: usize) -> Result<ZU256<'_>, ZError> {
    let word = peek_word(data, offset)?;
    Ok(ZU256(word))
}

/// Decodes dynamic bytes (length prefixed).
/// The offset points to the 'Head' which contains the relative offset to the data.
/// We follow the pointer to find the length word, then the data.
pub fn read_bytes(data: &[u8], initial_offset: usize) -> Result<ZBytes<'_>, ZError> {
    // 1. Read the relative offset from the head.
    let offset_word = peek_word(data, initial_offset)?;
    let data_offset_usize = usize::from_be_bytes(offset_word[24..32].try_into().unwrap()); // Last 8 bytes for usize is safe assumption for now < 2^64
    
    // ABI encoding offsets are usually absolute from the start of the encoded tuple? 
    // Wait, in dynamic types, the value in the "static" part is the offset from the START of the current encoding.
    // If we assume `data` is the full encoding block.
    
    if data_offset_usize >= data.len() {
        return Err(ZError::OutOfBounds(data_offset_usize, data.len()));
    }

    // 2. Read length of bytes at the data location.
    let len_word = peek_word(data, data_offset_usize)?;
    let length = usize::from_be_bytes(len_word[24..32].try_into().unwrap());

    // 3. Read the actual data bytes.
    let start = data_offset_usize + 32;
    let end = start + length;
    
    if end > data.len() {
        return Err(ZError::OutOfBounds(end, data.len()));
    }
    
    Ok(ZBytes(&data[start..end]))
}
