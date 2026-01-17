use alloy_sol_types::sol_data::{Address, Bool, Uint};
use alloy_sol_types::{SolType, SolValue};
use proptest::prelude::*;
use zabi_rs::{ZAddress, ZBool, ZDecode, ZU256};

proptest! {
    #[test]
    fn test_u256_decoding(val in any::<[u8; 32]>()) {
        let uint = alloy_sol_types::private::U256::from_le_bytes(val);
        let encoded = Uint::<256>::abi_encode(&uint);

        let decoded = ZU256::decode(&encoded, 0).expect("Failed to decode ZU256");
        // alloy encodes as 32 bytes big endian in the abi
        // ZU256 as_bytes should match the encoded data (since it's just a view)
        // actually ZU256 is just a slice wrapper, so it should be exactly the bytes
        prop_assert_eq!(decoded.as_bytes(), &encoded[..32]);
    }

    #[test]
    fn test_address_decoding(val in any::<[u8; 20]>()) {
        let addr = alloy_sol_types::private::Address::from(val);
        let encoded = Address::abi_encode(&addr);

        // encoded address is 32 bytes (padded)
        let decoded = ZAddress::decode(&encoded, 0).expect("Failed to decode ZAddress");

        // ZAddress.as_bytes() returns [u8; 20] right aligned in the 32 byte word
        prop_assert_eq!(decoded.as_bytes(), val.as_slice());
    }

    #[test]
    fn test_bool_decoding(val in any::<bool>()) {
        let encoded = Bool::abi_encode(&val);
        let decoded = ZBool::decode(&encoded, 0).expect("Failed to decode ZBool");
        prop_assert_eq!(decoded.as_bool(), val);
    }
}
