use crate::error::ZError;
use crate::types::{ZAddress, ZArray, ZBool, ZBytes, ZCallResult, ZRevert, ZString, ZU256};
use core::convert::TryInto;
use core::str;

#[inline(always)]
fn word_tail_u64(word: &[u8; 32]) -> u64 {
    u64::from_be_bytes(word[24..32].try_into().expect("slice is 8 bytes"))
}

#[inline(always)]
fn u64_to_usize(value: u64) -> Result<usize, ZError> {
    value
        .try_into()
        .map_err(|_| ZError::Custom("Value too large for usize"))
}

/// Read the 4-byte function selector from calldata.
/// Returns a reference to the first 4 bytes.
///
/// # Example
/// ```
/// use zabi_rs::decoder::read_selector;
///
/// let calldata = [0xde, 0xad, 0xbe, 0xef, 0x00, 0x00];
/// let selector = read_selector(&calldata).unwrap();
/// assert_eq!(selector, &[0xde, 0xad, 0xbe, 0xef]);
/// ```
#[inline]
pub fn read_selector(data: &[u8]) -> Result<&[u8; 4], ZError> {
    if data.len() < 4 {
        return Err(ZError::OutOfBounds(4, data.len()));
    }
    Ok(data[0..4].try_into().unwrap())
}

/// Returns the calldata without the 4-byte selector.
/// Useful for passing the remaining data to tuple decoders.
///
/// # Example
/// ```
/// use zabi_rs::decoder::skip_selector;
///
/// let calldata = [0xde, 0xad, 0xbe, 0xef, 0x01, 0x02, 0x03];
/// let params = skip_selector(&calldata).unwrap();
/// assert_eq!(params, &[0x01, 0x02, 0x03]);
/// ```
#[inline]
pub fn skip_selector(data: &[u8]) -> Result<&[u8], ZError> {
    if data.len() < 4 {
        return Err(ZError::OutOfBounds(4, data.len()));
    }
    Ok(&data[4..])
}

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
    let array_ref: &[u8; 32] = slice
        .try_into()
        .map_err(|_| ZError::Custom("Slice conversion failed"))?;
    Ok(array_ref)
}

/// Helper to read a 32-byte word without bounds checking.
///
/// # Safety
/// Caller must ensure `offset + 32 <= data.len()`.
#[inline(always)]
pub unsafe fn peek_word_unchecked(data: &[u8], offset: usize) -> &[u8; 32] {
    // SAFETY: Caller guarantees offset + 32 <= data.len().
    let slice = data.get_unchecked(offset..offset + 32);
    // Transmute slice to array reference
    // SAFETY: The slice has length 32, so it's valid to cast to &[u8; 32].
    // Alignment of u8 is 1, so alignment requirements are met.
    &*(slice.as_ptr() as *const [u8; 32])
}

/// Helper to read address (last 20 bytes of a 32-byte word).
#[inline(always)]
pub fn read_address_from_word(data: &[u8], offset: usize) -> Result<ZAddress<'_>, ZError> {
    let word = peek_word(data, offset)?;
    // Address is the last 20 bytes of the 32-byte word.
    let addr_slice = &word[12..32];
    let addr_ref: &[u8; 20] = addr_slice
        .try_into()
        .map_err(|_| ZError::Custom("Address slice conversion failed"))?;
    Ok(ZAddress(addr_ref))
}

/// Read address without bounds checking.
///
/// # Safety
/// Caller must ensure `offset + 32 <= data.len()`.
#[inline(always)]
pub unsafe fn read_address_unchecked(data: &[u8], offset: usize) -> ZAddress<'_> {
    // SAFETY: Caller guarantees offset + 32 <= data.len().
    let word = peek_word_unchecked(data, offset);
    // SAFETY: 12..32 is within 0..32 bounds of the word.
    let addr_slice = word.get_unchecked(12..32);
    // SAFETY: Slice is length 20, cast to [u8; 20] is valid.
    let addr_ref = &*(addr_slice.as_ptr() as *const [u8; 20]);
    ZAddress(addr_ref)
}

#[inline(always)]
pub fn read_u256(data: &[u8], offset: usize) -> Result<ZU256<'_>, ZError> {
    let word = peek_word(data, offset)?;
    Ok(ZU256(word))
}

/// Read uint256 without bounds checking.
///
/// # Safety
/// Caller must ensure `offset + 32 <= data.len()`.
#[inline(always)]
pub unsafe fn read_u256_unchecked(data: &[u8], offset: usize) -> ZU256<'_> {
    // SAFETY: Caller guarantees offset check.
    let word = peek_word_unchecked(data, offset);
    ZU256(word)
}

#[inline(always)]
pub fn read_int256(data: &[u8], offset: usize) -> Result<crate::types::ZInt256<'_>, ZError> {
    let word = peek_word(data, offset)?;
    Ok(crate::types::ZInt256(word))
}

#[inline(always)]
pub fn read_u8(data: &[u8], offset: usize) -> Result<u8, ZError> {
    let word = peek_word(data, offset)?;
    // Check padding (bytes 0..31 must be 0)
    if word.iter().take(31).any(|&b| b != 0) {
        return Err(ZError::Custom("u8 value invalid (high bits set)"));
    }
    Ok(word[31])
}

#[inline(always)]
pub fn read_i8(data: &[u8], offset: usize) -> Result<i8, ZError> {
    let word = peek_word(data, offset)?;
    let val = word[31] as i8;
    let padding_byte = if val < 0 { 0xff } else { 0x00 };
    if word.iter().take(31).any(|&b| b != padding_byte) {
        return Err(ZError::Custom("i8 value invalid (bad padding)"));
    }
    Ok(val)
}

#[inline(always)]
pub fn read_u16(data: &[u8], offset: usize) -> Result<u16, ZError> {
    let word = peek_word(data, offset)?;
    if !word[0..30].iter().all(|&b| b == 0) {
        return Err(ZError::Custom("u16 value invalid (high bits set)"));
    }
    Ok(u16::from_be_bytes([word[30], word[31]]))
}

#[inline(always)]
pub fn read_i16(data: &[u8], offset: usize) -> Result<i16, ZError> {
    let word = peek_word(data, offset)?;
    let val = i16::from_be_bytes([word[30], word[31]]);
    let padding_byte = if val < 0 { 0xff } else { 0x00 };
    if !word[0..30].iter().all(|&b| b == padding_byte) {
        return Err(ZError::Custom("i16 value invalid (bad padding)"));
    }
    Ok(val)
}

#[inline(always)]
pub fn read_u32(data: &[u8], offset: usize) -> Result<u32, ZError> {
    let word = peek_word(data, offset)?;
    if !word[0..28].iter().all(|&b| b == 0) {
        return Err(ZError::Custom("u32 value invalid (high bits set)"));
    }
    // Safe slice access
    Ok(u32::from_be_bytes(word[28..32].try_into().unwrap()))
}

#[inline(always)]
pub fn read_i32(data: &[u8], offset: usize) -> Result<i32, ZError> {
    let word = peek_word(data, offset)?;
    let val = i32::from_be_bytes(word[28..32].try_into().unwrap());
    let padding_byte = if val < 0 { 0xff } else { 0x00 };
    if !word[0..28].iter().all(|&b| b == padding_byte) {
        return Err(ZError::Custom("i32 value invalid (bad padding)"));
    }
    Ok(val)
}

#[inline(always)]
pub fn read_u64(data: &[u8], offset: usize) -> Result<u64, ZError> {
    let word = peek_word(data, offset)?;
    if !word[0..24].iter().all(|&b| b == 0) {
        return Err(ZError::Custom("u64 value invalid (high bits set)"));
    }
    Ok(u64::from_be_bytes(word[24..32].try_into().unwrap()))
}

#[inline(always)]
pub fn read_i64(data: &[u8], offset: usize) -> Result<i64, ZError> {
    let word = peek_word(data, offset)?;
    let val = i64::from_be_bytes(word[24..32].try_into().unwrap());
    let padding_byte = if val < 0 { 0xff } else { 0x00 };
    if !word[0..24].iter().all(|&b| b == padding_byte) {
        return Err(ZError::Custom("i64 value invalid (bad padding)"));
    }
    Ok(val)
}

#[inline(always)]
pub fn read_u128(data: &[u8], offset: usize) -> Result<u128, ZError> {
    let word = peek_word(data, offset)?;
    if !word[0..16].iter().all(|&b| b == 0) {
        return Err(ZError::Custom("u128 value invalid (high bits set)"));
    }
    Ok(u128::from_be_bytes(word[16..32].try_into().unwrap()))
}

#[inline(always)]
pub fn read_i128(data: &[u8], offset: usize) -> Result<i128, ZError> {
    let word = peek_word(data, offset)?;
    let val = i128::from_be_bytes(word[16..32].try_into().unwrap());
    let padding_byte = if val < 0 { 0xff } else { 0x00 };
    if !word[0..16].iter().all(|&b| b == padding_byte) {
        return Err(ZError::Custom("i128 value invalid (bad padding)"));
    }
    Ok(val)
}

#[inline(always)]
pub fn read_bool(data: &[u8], offset: usize) -> Result<ZBool, ZError> {
    let word = peek_word(data, offset)?;
    // Bool is uint256, last byte is 0 or 1.
    // We should check that all other bytes are 0?
    // Solidity requires clean high bits.

    let is_zero = word[0..31].iter().all(|&b| b == 0);
    if !is_zero {
        return Err(ZError::Custom("Boolean value has dirty high bits"));
    }

    match word[31] {
        0 => Ok(ZBool(false)),
        1 => Ok(ZBool(true)),
        _ => Err(ZError::Custom("Boolean value invalid (not 0 or 1)")),
    }
}

/// Decodes dynamic bytes (length prefixed).
/// The offset points to the 'Head' which contains the relative offset to the data.
/// We follow the pointer to find the length word, then the data.
pub fn read_bytes(data: &[u8], initial_offset: usize) -> Result<ZBytes<'_>, ZError> {
    // 1. Read the relative offset from the head.
    let offset_word = peek_word(data, initial_offset)?;
    let data_offset_usize = u64_to_usize(word_tail_u64(offset_word))?;

    // ABI encoding offsets are usually absolute from the start of the encoded tuple?
    // Wait, in dynamic types, the value in the "static" part is the offset from the START of the current encoding.
    // If we assume `data` is the full encoding block.

    if data_offset_usize >= data.len() {
        return Err(ZError::OutOfBounds(data_offset_usize, data.len()));
    }

    if data_offset_usize % 32 != 0 {
        return Err(ZError::Custom("ABI offset is not word-aligned"));
    }

    // 2. Read length of bytes at the data location.
    let len_word = peek_word(data, data_offset_usize)?;
    let length = u64_to_usize(word_tail_u64(len_word))?;

    // 3. Read the actual data bytes.
    let start = data_offset_usize
        .checked_add(32)
        .ok_or(ZError::Custom("Overflow computing bytes start"))?;
    let end = start
        .checked_add(length)
        .ok_or(ZError::Custom("Overflow computing bytes end"))?;

    if end > data.len() {
        return Err(ZError::OutOfBounds(end, data.len()));
    }

    Ok(ZBytes(&data[start..end]))
}

pub fn read_string(data: &[u8], initial_offset: usize) -> Result<ZString<'_>, ZError> {
    let zbytes = read_bytes(data, initial_offset)?;
    let s = str::from_utf8(zbytes.0).map_err(|_| ZError::Custom("Invalid UTF-8 string"))?;
    Ok(ZString(s))
}

pub fn read_array_fixed<'a, T>(
    data: &'a [u8],
    offset: usize,
    length: usize,
) -> Result<ZArray<'a, T>, ZError> {
    // Basic bounds check for the whole block
    let end = offset
        .checked_add(
            length
                .checked_mul(32)
                .ok_or(ZError::Custom("Overflow computing fixed array size"))?,
        )
        .ok_or(ZError::Custom("Overflow computing fixed array end"))?;
    if end > data.len() {
        return Err(ZError::OutOfBounds(end, data.len()));
    }
    Ok(ZArray::new(data, offset, length))
}

pub fn read_array_dyn<'a, T>(
    data: &'a [u8],
    initial_offset: usize,
) -> Result<ZArray<'a, T>, ZError> {
    // 1. Read offset to array (relative to current position in tuple, usually passed as offset 0?)
    // No, initial_offset points to the 'Head' word containing the offset.
    let offset_word = peek_word(data, initial_offset)?;
    let data_offset_usize = u64_to_usize(word_tail_u64(offset_word))?;

    if data_offset_usize >= data.len() {
        return Err(ZError::OutOfBounds(data_offset_usize, data.len()));
    }

    if data_offset_usize % 32 != 0 {
        return Err(ZError::Custom("ABI offset is not word-aligned"));
    }

    // 2. Read length
    let len_word = peek_word(data, data_offset_usize)?;
    let length = u64_to_usize(word_tail_u64(len_word))?;

    // 3. Start of data is 32 bytes after the length word
    let start_offset = data_offset_usize
        .checked_add(32)
        .ok_or(ZError::Custom("Overflow computing dynamic array start"))?;

    // Bounds check?
    // start_offset + length * 32
    let byte_len = length
        .checked_mul(32)
        .ok_or(ZError::Custom("Overflow computing dynamic array size"))?;
    let end = start_offset
        .checked_add(byte_len)
        .ok_or(ZError::Custom("Overflow computing dynamic array end"))?;
    if end > data.len() {
        return Err(ZError::OutOfBounds(end, data.len()));
    }

    Ok(ZArray::new(data, start_offset, length))
}

/// Decodes an Ethereum revert reason from raw data.
pub fn decode_revert(data: &[u8]) -> Result<ZRevert<'_>, ZError> {
    if data.is_empty() {
        return Ok(ZRevert::Unknown);
    }

    if data.len() < 4 {
        return Ok(ZRevert::Unknown);
    }

    let selector = read_selector(data)?;
    let params = skip_selector(data)?;

    match selector {
        // Error(string) -> 0x08c379a0
        [0x08, 0xc3, 0x79, 0xa0] => {
            let s = read_string(params, 0)?;
            Ok(ZRevert::Error(s))
        }
        // Panic(uint256) -> 0x4e487b71
        [0x4e, 0x48, 0x7b, 0x71] => {
            let p = read_u256(params, 0)?;
            Ok(ZRevert::Panic(p))
        }
        _ => {
            // Treat as custom error
            Ok(ZRevert::Custom(selector, params))
        }
    }
}

/// Decodes the result of a function call.
/// If the call was successful (not a revert), it decodes T from the data.
/// If the data is identified as a revert, it returns ZCallResult::Revert.
pub fn decode_call_result<'a, T>(data: &'a [u8]) -> Result<ZCallResult<'a, T>, ZError>
where
    T: crate::ZDecode<'a>,
{
    // A heuristic for identifying reverts:
    // Reverts are usually short or have specific selectors.
    // However, a successful return could also have those selectors if T is bytes or similar.
    // In practice, we usually know if a call reverted from the RPC response.
    // But for raw data parsing (e.g. from traces), we can try to guess.

    if data.len() >= 4 {
        let sel = &data[0..4];
        if sel == [0x08, 0xc3, 0x79, 0xa0] || sel == [0x4e, 0x48, 0x7b, 0x71] {
            return Ok(ZCallResult::Revert(decode_revert(data)?));
        }
    }

    Ok(ZCallResult::Success(T::decode(data, 0)?))
}
