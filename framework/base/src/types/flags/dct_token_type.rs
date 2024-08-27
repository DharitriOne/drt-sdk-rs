use drt_sc_derive::{type_abi, ManagedVecItem};

use crate::codec::{
    self,
    derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
};

const DCT_TYPE_FUNGIBLE: &[u8] = b"FungibleDCT";
const DCT_TYPE_NON_FUNGIBLE: &[u8] = b"NonFungibleDCT";
const DCT_TYPE_SEMI_FUNGIBLE: &[u8] = b"SemiFungibleDCT";
const DCT_TYPE_META: &[u8] = b"MetaDCT";
const DCT_TYPE_INVALID: &[u8] = &[];

use crate as drt_sc; // needed by the TypeAbi generated code

// Note: In the current implementation, SemiFungible is never returned

#[type_abi]
#[derive(
    TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, PartialEq, Eq, Debug, ManagedVecItem,
)]
pub enum DctTokenType {
    Fungible,
    NonFungible,
    SemiFungible,
    Meta,
    Invalid,
}

impl DctTokenType {
    pub fn based_on_token_nonce(token_nonce: u64) -> Self {
        if token_nonce == 0 {
            DctTokenType::Fungible
        } else {
            DctTokenType::NonFungible
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Fungible => 0,
            Self::NonFungible => 1,
            Self::SemiFungible => 2,
            Self::Meta => 3,
            Self::Invalid => 4,
        }
    }

    pub fn as_type_name(&self) -> &'static [u8] {
        match self {
            Self::Fungible => DCT_TYPE_FUNGIBLE,
            Self::NonFungible => DCT_TYPE_NON_FUNGIBLE,
            Self::SemiFungible => DCT_TYPE_SEMI_FUNGIBLE,
            Self::Meta => DCT_TYPE_META,
            Self::Invalid => DCT_TYPE_INVALID,
        }
    }
}

impl From<u8> for DctTokenType {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Fungible,
            1 => Self::NonFungible,
            2 => Self::SemiFungible,
            3 => Self::Meta,
            _ => Self::Invalid,
        }
    }
}

impl<'a> From<&'a [u8]> for DctTokenType {
    #[inline]
    fn from(byte_slice: &'a [u8]) -> Self {
        if byte_slice == DCT_TYPE_FUNGIBLE {
            Self::Fungible
        } else if byte_slice == DCT_TYPE_NON_FUNGIBLE {
            Self::NonFungible
        } else if byte_slice == DCT_TYPE_SEMI_FUNGIBLE {
            Self::SemiFungible
        } else if byte_slice == DCT_TYPE_META {
            Self::Meta
        } else {
            Self::Invalid
        }
    }
}
