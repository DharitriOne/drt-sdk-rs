use crate::{
    abi::TypeAbiFrom,
    api::ManagedTypeApi,
    types::{BigUint, RewaOrDctTokenIdentifier},
};

use crate::codec::{
    self,
    derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
};

use crate as drt_sc; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

use super::{DctTokenPayment, DctTokenPaymentRefs};

#[derive(
    TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Clone, PartialEq, Eq, Debug,
)]
pub struct RewaOrDctTokenPayment<M: ManagedTypeApi> {
    pub token_identifier: RewaOrDctTokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
}

impl<M: ManagedTypeApi> RewaOrDctTokenPayment<M> {
    pub fn no_payment() -> Self {
        RewaOrDctTokenPayment {
            token_identifier: RewaOrDctTokenIdentifier::rewa(),
            token_nonce: 0,
            amount: BigUint::zero(),
        }
    }

    pub fn new(
        token_identifier: RewaOrDctTokenIdentifier<M>,
        token_nonce: u64,
        amount: BigUint<M>,
    ) -> Self {
        RewaOrDctTokenPayment {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    /// Will convert to just DCT or terminate execution if the token is REWA.
    pub fn unwrap_dct(self) -> DctTokenPayment<M> {
        DctTokenPayment::new(
            self.token_identifier.unwrap_dct(),
            self.token_nonce,
            self.amount,
        )
    }

    /// Equivalent to a `match { Rewa | Dct }`.
    ///
    /// Context passed on from function to closures, to avoid ownership issues.
    /// More precisely, since only one of the two closures `for_rewa` and `for_dct` is called,
    /// it is ok for them to have fully owned access to anything from the environment.
    /// The compiler doesn't know that only one of them can ever be called,
    /// so if we pass context to both closures, it will complain that they are moved twice.
    pub fn map_rewa_or_dct<Context, D, F, U>(self, context: Context, for_rewa: D, for_dct: F) -> U
    where
        D: FnOnce(Context, BigUint<M>) -> U,
        F: FnOnce(Context, DctTokenPayment<M>) -> U,
    {
        self.token_identifier.map_or_else(
            (context, self.amount),
            |(context, amount)| for_rewa(context, amount),
            |(context, amount), token_identifier| {
                for_dct(
                    context,
                    DctTokenPayment::new(token_identifier, self.token_nonce, amount),
                )
            },
        )
    }

    /// Same as `map_rewa_or_dct`, but only takes a reference,
    /// and consequently, the closures also only get references.
    pub fn map_ref_rewa_or_dct<Context, D, F, U>(
        &self,
        context: Context,
        for_rewa: D,
        for_dct: F,
    ) -> U
    where
        D: FnOnce(Context, &BigUint<M>) -> U,
        F: FnOnce(Context, DctTokenPaymentRefs<'_, M>) -> U,
    {
        self.token_identifier.map_ref_or_else(
            context,
            |context| for_rewa(context, &self.amount),
            |context, token_identifier| {
                for_dct(
                    context,
                    DctTokenPaymentRefs::new(token_identifier, self.token_nonce, &self.amount),
                )
            },
        )
    }

    pub fn into_tuple(self) -> (RewaOrDctTokenIdentifier<M>, u64, BigUint<M>) {
        (self.token_identifier, self.token_nonce, self.amount)
    }
}

impl<M: ManagedTypeApi> From<(RewaOrDctTokenIdentifier<M>, u64, BigUint<M>)>
    for RewaOrDctTokenPayment<M>
{
    #[inline]
    fn from(value: (RewaOrDctTokenIdentifier<M>, u64, BigUint<M>)) -> Self {
        let (token_identifier, token_nonce, amount) = value;
        Self::new(token_identifier, token_nonce, amount)
    }
}

impl<M: ManagedTypeApi> From<DctTokenPayment<M>> for RewaOrDctTokenPayment<M> {
    fn from(dct_payment: DctTokenPayment<M>) -> Self {
        RewaOrDctTokenPayment {
            token_identifier: RewaOrDctTokenIdentifier::dct(dct_payment.token_identifier),
            token_nonce: dct_payment.token_nonce,
            amount: dct_payment.amount,
        }
    }
}

impl<M> TypeAbiFrom<&[u8]> for RewaOrDctTokenPayment<M> where M: ManagedTypeApi {}

impl<M: ManagedTypeApi> RewaOrDctTokenPayment<M> {
    pub fn as_refs(&self) -> RewaOrDctTokenPaymentRefs<'_, M> {
        RewaOrDctTokenPaymentRefs::new(&self.token_identifier, self.token_nonce, &self.amount)
    }
}

/// Similar to `RewaOrDctTokenPayment`, but only contains references.
pub struct RewaOrDctTokenPaymentRefs<'a, M: ManagedTypeApi> {
    pub token_identifier: &'a RewaOrDctTokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: &'a BigUint<M>,
}

impl<'a, M: ManagedTypeApi> RewaOrDctTokenPaymentRefs<'a, M> {
    pub fn new(
        token_identifier: &'a RewaOrDctTokenIdentifier<M>,
        token_nonce: u64,
        amount: &'a BigUint<M>,
    ) -> RewaOrDctTokenPaymentRefs<'a, M> {
        RewaOrDctTokenPaymentRefs {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    pub fn to_owned_payment(&self) -> RewaOrDctTokenPayment<M> {
        RewaOrDctTokenPayment {
            token_identifier: self.token_identifier.clone(),
            token_nonce: self.token_nonce,
            amount: self.amount.clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.amount == &0u32
    }

    pub fn map_rewa_or_dct<Context, D, F, U>(self, context: Context, for_rewa: D, for_dct: F) -> U
    where
        D: FnOnce(Context, &BigUint<M>) -> U,
        F: FnOnce(Context, DctTokenPaymentRefs<M>) -> U,
    {
        self.token_identifier.map_ref_or_else(
            context,
            |context| for_rewa(context, self.amount),
            |context, token_identifier| {
                for_dct(
                    context,
                    DctTokenPaymentRefs::new(token_identifier, self.token_nonce, self.amount),
                )
            },
        )
    }
}
