use crate::codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{RewaOrMultiDctPayment, ManagedAddress, ManagedBuffer},
};

use super::{
    contract_call_no_payment::ContractCallNoPayment, contract_call_trait::ContractCallBase,
    ContractCall, ContractCallWithRewa,
};

/// Holds data for calling another contract, with any type of payment: none, REWA, Multi-DCT.
///
/// Gets created when chaining method `with_any_payment`.
#[deprecated(
    since = "0.49.0",
    note = "Please use the unified transaction syntax instead."
)]
#[must_use]
pub struct ContractCallWithAnyPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub basic: ContractCallNoPayment<SA, OriginalResult>,
    pub payment: RewaOrMultiDctPayment<SA>,
}

impl<SA, OriginalResult> ContractCallBase<SA> for ContractCallWithAnyPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_normalized(self) -> ContractCallWithRewa<SA, Self::OriginalResult> {
        match self.payment {
            RewaOrMultiDctPayment::Rewa(rewa_amount) => self.basic.with_rewa_transfer(rewa_amount),
            RewaOrMultiDctPayment::MultiDct(multi_dct_payment) => self
                .basic
                .into_normalized()
                .convert_to_dct_transfer_call(multi_dct_payment),
        }
    }
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallWithAnyPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
    }

    fn transfer_execute(self) {
        match self.payment {
            RewaOrMultiDctPayment::Rewa(rewa_amount) => {
                self.basic.transfer_execute_rewa(rewa_amount);
            },
            RewaOrMultiDctPayment::MultiDct(multi_dct_payment) => {
                self.basic.transfer_execute_dct(multi_dct_payment);
            },
        }
    }
}

impl<SA, OriginalResult> ContractCallWithAnyPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    /// Creates a new instance directly.
    pub fn new<N: Into<ManagedBuffer<SA>>>(
        to: ManagedAddress<SA>,
        endpoint_name: N,
        payment: RewaOrMultiDctPayment<SA>,
    ) -> Self {
        ContractCallWithAnyPayment {
            basic: ContractCallNoPayment::new(to, endpoint_name),
            payment,
        }
    }
}
