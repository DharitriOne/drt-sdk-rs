use hex_literal::hex;
use drt_sc_codec::{EncodeErrorHandler, TopEncode, TopEncodeOutput};

use crate::{
    abi::TypeAbiFrom,
    api::{CallTypeApi, ManagedTypeApi},
    types::{AnnotatedValue, ManagedAddress, ManagedBuffer, TxScEnv, TxTo, TxToSpecified},
};

/// Address of the system smart contract that manages DCT.
const SYSTEM_SC_ADDRESS_BYTES: [u8; 32] =
    hex!("000000000000000000010000000000000000000000000000000000000002ffff");
const SYSTEM_SC_ADDRESS_BECH32: &str =
    "moa1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls29jpxv";
const SYSTEM_SC_ADDRESS_ANNOTATION: &str =
    "bech32:moa1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls29jpxv";

/// Indicates the system SC address, which is the same on any DharitriOne blockchain.
pub struct DCTSystemSCAddress;

impl DCTSystemSCAddress {
    pub fn to_managed_address<Api>(self) -> ManagedAddress<Api>
    where
        Api: ManagedTypeApi,
    {
        ManagedAddress::from(SYSTEM_SC_ADDRESS_BYTES)
    }

    pub fn to_bech32_str(&self) -> &str {
        SYSTEM_SC_ADDRESS_BECH32
    }

    pub fn to_bech32_string(&self) -> alloc::string::String {
        SYSTEM_SC_ADDRESS_BECH32.into()
    }
}

impl<Api> AnnotatedValue<TxScEnv<Api>, ManagedAddress<Api>> for DCTSystemSCAddress
where
    Api: CallTypeApi,
{
    fn annotation(&self, _env: &TxScEnv<Api>) -> ManagedBuffer<Api> {
        ManagedBuffer::from(SYSTEM_SC_ADDRESS_ANNOTATION)
    }

    fn to_value(&self, _env: &TxScEnv<Api>) -> ManagedAddress<Api> {
        DCTSystemSCAddress.to_managed_address()
    }
}

impl<Api> TxTo<TxScEnv<Api>> for DCTSystemSCAddress where Api: CallTypeApi {}
impl<Api> TxToSpecified<TxScEnv<Api>> for DCTSystemSCAddress where Api: CallTypeApi {}

impl TopEncode for DCTSystemSCAddress {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        SYSTEM_SC_ADDRESS_BYTES.top_encode_or_handle_err(output, h)
    }
}

impl<M> TypeAbiFrom<DCTSystemSCAddress> for ManagedAddress<M> where M: ManagedTypeApi {}

impl core::fmt::Display for DCTSystemSCAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(SYSTEM_SC_ADDRESS_BECH32)
    }
}
