use crate::error::ZError;

/// Write a 32-byte word to the buffer at the specific offset.
#[inline(always)]
fn write_word(buf: &mut [u8], offset: usize, word: &[u8; 32]) -> Result<(), ZError> {
    if offset + 32 > buf.len() {
        return Err(ZError::OutOfBounds(offset + 32, buf.len()));
    }
    buf[offset..offset + 32].copy_from_slice(word);
    Ok(())
}

/// Encode a U256 (32 bytes) into the buffer.
#[inline]
pub fn encode_u256(buf: &mut [u8], offset: usize, value: &[u8; 32]) -> Result<(), ZError> {
    write_word(buf, offset, value)
}

/// Encode an Address (20 bytes) into the buffer (padded to 32 bytes).
#[inline]
pub fn encode_address(buf: &mut [u8], offset: usize, value: &[u8; 20]) -> Result<(), ZError> {
    if offset + 32 > buf.len() {
        return Err(ZError::OutOfBounds(offset + 32, buf.len()));
    }
    // Zero out high bytes
    buf[offset..offset + 12].fill(0);
    // Copy address
    buf[offset + 12..offset + 32].copy_from_slice(value);
    Ok(())
}

/// Encode a boolean.
#[inline]
pub fn encode_bool(buf: &mut [u8], offset: usize, value: bool) -> Result<(), ZError> {
    if offset + 32 > buf.len() {
        return Err(ZError::OutOfBounds(offset + 32, buf.len()));
    }
    buf[offset..offset + 31].fill(0);
    buf[offset + 31] = if value { 1 } else { 0 };
    Ok(())
}

/// Encode raw bytes (dynamic).
///
/// Use this when constructing the data part of bytes/string.
/// Does NOT write length prefix or offset (that's for higher level logic).
/// Just pads the data to 32-byte boundary if needed?
/// Actually, ABI encoding of bytes is:
/// [offset] [length] [data + padding]
/// This helper writes `[data + padding]` given a destination.
/// Returns number of bytes written (including padding).
pub fn encode_bytes_data(buf: &mut [u8], offset: usize, data: &[u8]) -> Result<usize, ZError> {
    let len = data.len();
    let padded_len = (len + 31) & !31; // Round up to nearest 32

    if offset + padded_len > buf.len() {
        return Err(ZError::OutOfBounds(offset + padded_len, buf.len()));
    }

    // Copy data
    buf[offset..offset + len].copy_from_slice(data);
    // Zero padding
    if padded_len > len {
        buf[offset + len..offset + padded_len].fill(0);
    }

    Ok(padded_len)
}
