use super::{DctTokenPayment, ManagedVec};
use crate::{
    api::ManagedTypeApi,
    codec::{
        self,
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    },
    types::BigUint,
};

use crate as drt_sc; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

/// Encodes any type of payment, which either:
/// - REWA (can be zero in case of no payment whatsoever);
/// - Multi-DCT (one or more DCT transfers).
#[derive(
    TopDecode, TopEncode, TypeAbi, NestedDecode, NestedEncode, Clone, PartialEq, Eq, Debug,
)]
pub enum RewaOrMultiDctPayment<M: ManagedTypeApi> {
    Rewa(BigUint<M>),
    MultiDct(ManagedVec<M, DctTokenPayment<M>>),
}

impl<M: ManagedTypeApi> RewaOrMultiDctPayment<M> {
    pub fn is_empty(&self) -> bool {
        match self {
            RewaOrMultiDctPayment::Rewa(rewa_value) => rewa_value == &0u32,
            RewaOrMultiDctPayment::MultiDct(dct_payments) => dct_payments.is_empty(),
        }
    }
}

/// The version of `RewaOrMultiDctPayment` that contains referrences instead of owned fields.
pub enum RewaOrMultiDctPaymentRefs<'a, M: ManagedTypeApi> {
    Rewa(&'a BigUint<M>),
    MultiDct(&'a ManagedVec<M, DctTokenPayment<M>>),
}

impl<M: ManagedTypeApi> RewaOrMultiDctPayment<M> {
    pub fn as_refs(&self) -> RewaOrMultiDctPaymentRefs<'_, M> {
        match self {
            RewaOrMultiDctPayment::Rewa(rewa_value) => {
                RewaOrMultiDctPaymentRefs::Rewa(rewa_value)
            },
            RewaOrMultiDctPayment::MultiDct(dct_payments) => {
                RewaOrMultiDctPaymentRefs::MultiDct(dct_payments)
            },
        }
    }
}

impl<'a, M: ManagedTypeApi> RewaOrMultiDctPaymentRefs<'a, M> {
    pub fn to_owned_payment(&self) -> RewaOrMultiDctPayment<M> {
        match self {
            RewaOrMultiDctPaymentRefs::Rewa(rewa_value) => {
                RewaOrMultiDctPayment::Rewa((*rewa_value).clone())
            },
            RewaOrMultiDctPaymentRefs::MultiDct(dct_payments) => {
                RewaOrMultiDctPayment::MultiDct((*dct_payments).clone())
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            RewaOrMultiDctPaymentRefs::Rewa(rewa_value) => *rewa_value == &0u32,
            RewaOrMultiDctPaymentRefs::MultiDct(dct_payments) => dct_payments.is_empty(),
        }
    }
}
