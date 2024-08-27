use num_bigint::BigUint;
use num_traits::Zero;

use crate::{
    display_util::*,
    types::{VMAddress, H256},
};
use std::fmt;

use super::{CallType, TxFunctionName};

#[derive(Clone, Debug)]
pub struct TxInput {
    pub from: VMAddress,
    pub to: VMAddress,
    pub rewa_value: BigUint,
    pub dct_values: Vec<TxTokenTransfer>,
    pub func_name: TxFunctionName,
    pub args: Vec<Vec<u8>>,
    pub call_type: CallType,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub tx_hash: H256,
    pub promise_callback_closure_data: Option<Vec<u8>>,
    pub callback_payments: CallbackPayments,
}

impl Default for TxInput {
    fn default() -> Self {
        TxInput {
            from: VMAddress::zero(),
            to: VMAddress::zero(),
            rewa_value: BigUint::zero(),
            dct_values: Vec::new(),
            func_name: TxFunctionName::EMPTY,
            args: Vec::new(),
            call_type: CallType::DirectCall,
            gas_limit: 0,
            gas_price: 0,
            tx_hash: H256::zero(),
            promise_callback_closure_data: None,
            callback_payments: Default::default(),
        }
    }
}

impl fmt::Display for TxInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TxInput {{ func: {}, args: {:?}, call_value: {}, dct_value: {:?}, from: 0x{}, to: 0x{}\n}}", 
            self.func_name.as_str(),
            self.args,
            self.rewa_value,
            self.dct_values,
            address_hex(&self.from),
            address_hex(&self.to))
    }
}

impl TxInput {
    pub fn add_arg(&mut self, arg: Vec<u8>) {
        self.args.push(arg);
    }

    pub fn func_name_from_arg_index(&self, arg_index: usize) -> TxFunctionName {
        if let Some(arg) = self.args.get(arg_index) {
            arg.into()
        } else {
            TxFunctionName::EMPTY
        }
    }
}

/// Models DCT transfers between accounts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TxTokenTransfer {
    pub token_identifier: Vec<u8>,
    pub nonce: u64,
    pub value: BigUint,
}

/// Signals to the callback that funds have been returned to it, without performing any transfer.
#[derive(Default, Clone, Debug)]
pub struct CallbackPayments {
    pub rewa_value: BigUint,
    pub dct_values: Vec<TxTokenTransfer>,
}

impl TxInput {
    /// The received REWA can come either from the original caller, or from an async call, during callback.
    pub fn received_rewa(&self) -> &BigUint {
        if !self.callback_payments.rewa_value.is_zero() {
            &self.callback_payments.rewa_value
        } else {
            &self.rewa_value
        }
    }

    /// The received DCT tokens can come either from the original caller, or from an async call, during callback.
    pub fn received_dct(&self) -> &[TxTokenTransfer] {
        if !self.callback_payments.dct_values.is_empty() {
            self.callback_payments.dct_values.as_slice()
        } else {
            self.dct_values.as_slice()
        }
    }

    pub fn get_argument_vec_u8(&self, arg_index: i32) -> Vec<u8> {
        let arg_idx_usize = arg_index as usize;
        assert!(arg_idx_usize < self.args.len(), "Tx arg index out of range");
        self.args[arg_idx_usize].clone()
    }
}
