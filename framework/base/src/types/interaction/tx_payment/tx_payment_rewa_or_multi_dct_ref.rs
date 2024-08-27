use crate::types::{BigUint, RewaOrMultiDctPaymentRefs, ManagedAddress, TxFrom, TxToSpecified};

use super::{Rewa, FullPaymentData, FunctionCall, TxEnv, TxPayment};

impl<'a, Env> TxPayment<Env> for RewaOrMultiDctPaymentRefs<'a, Env::Api>
where
    Env: TxEnv,
{
    fn is_no_payment(&self, _env: &Env) -> bool {
        self.is_empty()
    }

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        match self {
            RewaOrMultiDctPaymentRefs::Rewa(rewa_amount) => {
                Rewa(rewa_amount).perform_transfer_execute(env, to, gas_limit, fc);
            },
            RewaOrMultiDctPaymentRefs::MultiDct(multi_dct_payment) => {
                multi_dct_payment.perform_transfer_execute(env, to, gas_limit, fc);
            },
        }
    }

    fn with_normalized<From, To, F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        From: TxFrom<Env>,
        To: TxToSpecified<Env>,
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, FunctionCall<Env::Api>) -> R,
    {
        match self {
            RewaOrMultiDctPaymentRefs::Rewa(rewa_amount) => {
                Rewa(rewa_amount).with_normalized(env, from, to, fc, f)
            },
            RewaOrMultiDctPaymentRefs::MultiDct(multi_dct_payment) => {
                multi_dct_payment.with_normalized(env, from, to, fc, f)
            },
        }
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        match self {
            RewaOrMultiDctPaymentRefs::Rewa(rewa_amount) => {
                Rewa(rewa_amount).into_full_payment_data(env)
            },
            RewaOrMultiDctPaymentRefs::MultiDct(multi_dct_payment) => {
                multi_dct_payment.into_full_payment_data(env)
            },
        }
    }
}
