use crate::codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{
        BigUint, RewaOrDctTokenIdentifier, RewaOrDctTokenPayment, ManagedAddress, ManagedBuffer,
    },
};

use super::{
    contract_call_no_payment::ContractCallNoPayment, contract_call_trait::ContractCallBase,
    ContractCall, ContractCallWithRewa,
};

/// Holds data for calling another contract, with a single payment, either REWA or a single DCT token.
///
/// Gets created when chaining method `with_rewa_or_single_dct_transfer`.
#[deprecated(
    since = "0.49.0",
    note = "Please use the unified transaction syntax instead."
)]
#[must_use]
pub struct ContractCallWithRewaOrSingleDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(super) basic: ContractCallNoPayment<SA, OriginalResult>,
    pub payment: RewaOrDctTokenPayment<SA>,
}

impl<SA, OriginalResult> ContractCallWithRewaOrSingleDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    fn into_normalized_rewa(self) -> ContractCallWithRewa<SA, OriginalResult> {
        ContractCallWithRewa {
            basic: self.basic,
            rewa_payment: self.payment.amount,
        }
    }

    fn into_normalized_dct(self) -> ContractCallWithRewa<SA, OriginalResult> {
        self.basic
            .into_normalized()
            .convert_to_single_transfer_dct_call(self.payment.unwrap_dct())
    }
}

impl<SA, OriginalResult> ContractCallBase<SA>
    for ContractCallWithRewaOrSingleDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_normalized(self) -> ContractCallWithRewa<SA, Self::OriginalResult> {
        if self.payment.token_identifier.is_rewa() {
            self.into_normalized_rewa()
        } else {
            // Because we know that there can be at most one DCT payment,
            // there is no need to call the full `convert_to_dct_transfer_call`.
            self.into_normalized_dct()
        }
    }
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallWithRewaOrSingleDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
    }

    fn transfer_execute(self) {
        if self.payment.token_identifier.is_rewa() {
            self.basic.transfer_execute_rewa(self.payment.amount);
        } else {
            self.basic
                .transfer_execute_single_dct(self.payment.unwrap_dct());
        }
    }
}

impl<SA, OriginalResult> ContractCallWithRewaOrSingleDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    /// Creates a new instance directly.
    ///
    /// The constructor is mostly for hand-written proxies,
    /// the usual way of constructing this object is via the builder methods of other contract call types,
    /// especially `with_rewa_or_single_dct_transfer`.
    pub fn new<N: Into<ManagedBuffer<SA>>>(
        to: ManagedAddress<SA>,
        endpoint_name: N,
        token_identifier: RewaOrDctTokenIdentifier<SA>,
        token_nonce: u64,
        amount: BigUint<SA>,
    ) -> Self {
        ContractCallWithRewaOrSingleDct {
            basic: ContractCallNoPayment::new(to, endpoint_name),
            payment: RewaOrDctTokenPayment::new(token_identifier, token_nonce, amount),
        }
    }
}
