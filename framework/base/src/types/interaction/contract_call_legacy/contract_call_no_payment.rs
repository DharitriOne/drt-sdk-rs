use core::marker::PhantomData;

use crate::codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{
        BigUint, RewaOrDctTokenIdentifier, RewaOrDctTokenPayment, RewaOrMultiDctPayment,
        DctTokenPayment, FunctionCall, ManagedAddress, ManagedArgBuffer, ManagedBuffer,
        ManagedVec, TokenIdentifier, Tx, TxScEnv,
    },
};

use super::{
    contract_call_trait::ContractCallBase, contract_call_with_rewa::ContractCallWithRewa,
    contract_call_with_multi_dct::ContractCallWithMultiDct, ContractCall,
    ContractCallWithAnyPayment, ContractCallWithRewaOrSingleDct, UNSPECIFIED_GAS_LIMIT,
};

/// Holds metadata for calling another contract, without payments.
///
/// Proxies generally create contract calls of this type
/// (unless there are payment arguments in the endpoint - but these are mostly obsolete now).
///
/// It is also the basis for all other contract call types, all of them contain this one.
#[deprecated(
    since = "0.49.0",
    note = "Please use the unified transaction syntax instead."
)]
#[must_use]
pub struct ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(crate) _phantom: PhantomData<SA>,
    pub to: ManagedAddress<SA>,
    pub function_call: FunctionCall<SA>,
    pub explicit_gas_limit: u64,
    pub(crate) _return_type: PhantomData<OriginalResult>,
}

impl<SA, OriginalResult> ContractCallBase<SA> for ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    #[inline]
    fn into_normalized(self) -> ContractCallWithRewa<SA, Self::OriginalResult> {
        ContractCallWithRewa {
            basic: self,
            rewa_payment: BigUint::zero(),
        }
    }
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        self
    }

    fn transfer_execute(self) {
        self.transfer_execute_rewa(BigUint::zero());
    }
}

impl<SA, OriginalResult> ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub fn new<N: Into<ManagedBuffer<SA>>>(to: ManagedAddress<SA>, endpoint_name: N) -> Self {
        ContractCallNoPayment {
            _phantom: PhantomData,
            to,
            function_call: FunctionCall {
                function_name: endpoint_name.into(),
                arg_buffer: ManagedArgBuffer::new(),
            },
            explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
            _return_type: PhantomData,
        }
    }

    /// Sets payment to be REWA transfer.
    pub fn with_rewa_transfer(
        self,
        rewa_amount: BigUint<SA>,
    ) -> ContractCallWithRewa<SA, OriginalResult> {
        ContractCallWithRewa {
            basic: self,
            rewa_payment: rewa_amount,
        }
    }

    /// Adds a single DCT token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    pub fn with_dct_transfer<P: Into<DctTokenPayment<SA>>>(
        self,
        payment: P,
    ) -> ContractCallWithMultiDct<SA, OriginalResult> {
        let result = ContractCallWithMultiDct {
            basic: self,
            dct_payments: ManagedVec::new(),
        };
        result.with_dct_transfer(payment)
    }

    #[deprecated(
        since = "0.39.0",
        note = "Replace by `contract_call.with_dct_transfer((payment_token, payment_nonce, payment_amount))`. 
        The tuple argument will get automatically converted to DctTokenPayment."
    )]
    pub fn add_dct_token_transfer(
        self,
        payment_token: TokenIdentifier<SA>,
        payment_nonce: u64,
        payment_amount: BigUint<SA>,
    ) -> ContractCallWithMultiDct<SA, OriginalResult> {
        self.with_dct_transfer((payment_token, payment_nonce, payment_amount))
    }

    /// Sets payment to be a (potentially) multi-token transfer.
    #[inline]
    pub fn with_multi_token_transfer(
        self,
        payments: ManagedVec<SA, DctTokenPayment<SA>>,
    ) -> ContractCallWithMultiDct<SA, OriginalResult> {
        ContractCallWithMultiDct {
            basic: self,
            dct_payments: payments,
        }
    }

    /// Sets payment to be a (potentially) multi-token transfer.
    #[inline]
    pub fn with_any_payment(
        self,
        payment: RewaOrMultiDctPayment<SA>,
    ) -> ContractCallWithAnyPayment<SA, OriginalResult> {
        ContractCallWithAnyPayment {
            basic: self,
            payment,
        }
    }

    /// Sets payment to be either REWA or a single DCT transfer, as determined at runtime.
    pub fn with_rewa_or_single_dct_transfer<P: Into<RewaOrDctTokenPayment<SA>>>(
        self,
        payment: P,
    ) -> ContractCallWithRewaOrSingleDct<SA, OriginalResult> {
        ContractCallWithRewaOrSingleDct {
            basic: self,
            payment: payment.into(),
        }
    }

    #[deprecated(
        since = "0.39.0",
        note = "Replace by `contract_call.with_rewa_or_single_dct_transfer((payment_token, payment_nonce, payment_amount))`. "
    )]
    pub fn with_rewa_or_single_dct_token_transfer(
        self,
        payment_token: RewaOrDctTokenIdentifier<SA>,
        payment_nonce: u64,
        payment_amount: BigUint<SA>,
    ) -> ContractCallWithRewaOrSingleDct<SA, OriginalResult> {
        self.with_rewa_or_single_dct_transfer((payment_token, payment_nonce, payment_amount))
    }

    pub fn into_function_call(self) -> FunctionCall<SA> {
        self.function_call
    }

    pub fn tx(self) -> Tx<TxScEnv<SA>, (), (), (), (), FunctionCall<SA>, ()> {
        Tx::new_tx_from_sc().raw_data(self.function_call)
    }
}
