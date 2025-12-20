//! Event/Log decoding module for Ethereum logs.
//!
//! Ethereum events encode data in two parts:
//! - **Topics**: Up to 4 indexed values (topic[0] = event signature hash, topic[1-3] = indexed params)
//! - **Data**: Non-indexed parameters, ABI-encoded
//!
//! This module provides zero-copy decoding for both.

use core::convert::TryInto;
use crate::error::ZError;
use crate::types::{ZAddress, ZU256, ZInt256};

/// Wrapper for Ethereum event log data.
/// Provides access to topics and non-indexed data.
#[derive(Clone, Copy)]
pub struct ZEventLog<'a> {
    /// The topics array (up to 4 topics, first is event signature)
    topics: &'a [&'a [u8; 32]],
    /// The non-indexed data
    data: &'a [u8],
}

impl<'a> ZEventLog<'a> {
    /// Create a new event log wrapper.
    #[inline]
    pub fn new(topics: &'a [&'a [u8; 32]], data: &'a [u8]) -> Self {
        Self { topics, data }
    }

    /// Returns the number of topics.
    #[inline]
    pub fn topic_count(&self) -> usize {
        self.topics.len()
    }

    /// Returns the raw data slice.
    #[inline]
    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    /// Get raw topic bytes at index.
    #[inline]
    pub fn raw_topic(&self, index: usize) -> Result<&'a [u8; 32], ZError> {
        if index >= self.topics.len() {
            return Err(ZError::OutOfBounds(index, self.topics.len()));
        }
        Ok(self.topics[index])
    }

    /// Get the event signature (topic[0]) as bytes32.
    #[inline]
    pub fn event_signature(&self) -> Result<&'a [u8; 32], ZError> {
        self.raw_topic(0)
    }

    /// Decode an indexed topic as uint256.
    #[inline]
    pub fn topic_as_u256(&self, index: usize) -> Result<ZU256<'a>, ZError> {
        let topic = self.raw_topic(index)?;
        Ok(ZU256(topic))
    }

    /// Decode an indexed topic as int256.
    #[inline]
    pub fn topic_as_int256(&self, index: usize) -> Result<ZInt256<'a>, ZError> {
        let topic = self.raw_topic(index)?;
        Ok(ZInt256(topic))
    }

    /// Decode an indexed topic as address.
    /// Address is right-aligned in the 32-byte topic (last 20 bytes).
    #[inline]
    pub fn topic_as_address(&self, index: usize) -> Result<ZAddress<'a>, ZError> {
        let topic = self.raw_topic(index)?;
        let addr_slice = &topic[12..32];
        let addr_ref: &[u8; 20] = addr_slice.try_into()
            .map_err(|_| ZError::Custom("Address slice conversion failed"))?;
        Ok(ZAddress(addr_ref))
    }

    /// Decode data field using standard ABI decoding at offset.
    /// This allows reusing all existing decoder functions.
    #[inline]
    pub fn decode_data<T, F>(&self, offset: usize, decoder: F) -> Result<T, ZError>
    where
        F: FnOnce(&'a [u8], usize) -> Result<T, ZError>,
    {
        decoder(self.data, offset)
    }
}

/// Read a topic from raw topic bytes as ZU256.
#[inline]
pub fn read_topic_u256<'a>(topic: &'a [u8; 32]) -> ZU256<'a> {
    ZU256(topic)
}

/// Read a topic from raw topic bytes as ZInt256.
#[inline]
pub fn read_topic_int256<'a>(topic: &'a [u8; 32]) -> ZInt256<'a> {
    ZInt256(topic)
}

/// Read a topic from raw topic bytes as ZAddress.
/// Address occupies the last 20 bytes of the 32-byte topic.
#[inline]
pub fn read_topic_address(topic: &[u8; 32]) -> Result<ZAddress<'_>, ZError> {
    let addr_slice = &topic[12..32];
    let addr_ref: &[u8; 20] = addr_slice.try_into()
        .map_err(|_| ZError::Custom("Address slice conversion failed"))?;
    Ok(ZAddress(addr_ref))
}

/// Read a topic from raw topic bytes as bool.
/// Bool is stored as uint256, only last byte matters (0 or 1).
#[inline]
pub fn read_topic_bool(topic: &[u8; 32]) -> Result<bool, ZError> {
    // Check that all bytes except the last are zero
    if topic[0..31].iter().any(|&b| b != 0) {
        return Err(ZError::Custom("Boolean topic has dirty high bits"));
    }
    match topic[31] {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(ZError::Custom("Boolean topic value invalid (not 0 or 1)")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate alloc;
    use alloc::vec::Vec;

    #[test]
    fn test_event_log_basic() {
        // Create sample topics
        let mut topic0 = [0u8; 32];
        topic0[0] = 0xde; // Event signature
        
        let mut topic1 = [0u8; 32];
        topic1[31] = 0x01; // uint256(1)
        
        let topics: Vec<&[u8; 32]> = alloc::vec![&topic0, &topic1];
        let data = [0u8; 64];
        
        let event = ZEventLog::new(&topics, &data);
        
        assert_eq!(event.topic_count(), 2);
        assert_eq!(event.data().len(), 64);
        
        let sig = event.event_signature().unwrap();
        assert_eq!(sig[0], 0xde);
        
        let val = event.topic_as_u256(1).unwrap();
        assert_eq!(val.0[31], 0x01);
    }

    #[test]
    fn test_topic_as_address() {
        let mut topic = [0u8; 32];
        // Address in last 20 bytes
        for i in 12..32 {
            topic[i] = (i - 11) as u8;
        }
        
        let addr = read_topic_address(&topic).unwrap();
        assert_eq!(addr.0[0], 1);
        assert_eq!(addr.0[19], 20);
    }

    #[test]
    fn test_topic_bool() {
        let mut topic_true = [0u8; 32];
        topic_true[31] = 1;
        assert_eq!(read_topic_bool(&topic_true).unwrap(), true);
        
        let topic_false = [0u8; 32];
        assert_eq!(read_topic_bool(&topic_false).unwrap(), false);
        
        let mut topic_invalid = [0u8; 32];
        topic_invalid[31] = 2;
        assert!(read_topic_bool(&topic_invalid).is_err());
    }

    #[test]
    fn test_decode_event_data() {
        // Simulate event with data: (uint256(42), address(...))
        let topic0 = [0u8; 32];
        let topics: Vec<&[u8; 32]> = alloc::vec![&topic0];
        
        let mut data = [0u8; 64];
        data[31] = 42; // uint256(42)
        data[63] = 0xAA; // address last byte
        
        let event = ZEventLog::new(&topics, &data);
        
        // Decode uint256 at offset 0
        let val = event.decode_data(0, crate::decoder::read_u256).unwrap();
        assert_eq!(val.0[31], 42);
        
        // Decode address at offset 32
        let addr = event.decode_data(32, crate::decoder::read_address_from_word).unwrap();
        assert_eq!(addr.0[19], 0xAA);
    }
}
