use crate::types::{BigUint, Rewa, RewaOrDctTokenPayment, ManagedAddress, TxFrom, TxToSpecified};

use super::{FullPaymentData, FunctionCall, TxEnv, TxPayment};

impl<Env> TxPayment<Env> for &RewaOrDctTokenPayment<Env::Api>
where
    Env: TxEnv,
{
    fn is_no_payment(&self, _env: &Env) -> bool {
        self.amount == 0u32
    }

    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        self.map_ref_rewa_or_dct(
            (to, fc),
            |(to, fc), amount| Rewa(amount).perform_transfer_execute(env, to, gas_limit, fc),
            |(to, fc), dct_payment| dct_payment.perform_transfer_execute(env, to, gas_limit, fc),
        )
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
        self.map_ref_rewa_or_dct(
            (to, fc, f),
            |(to, fc, f), amount| Rewa(amount).with_normalized(env, from, to, fc, f),
            |(to, fc, f), dct_payment| dct_payment.with_normalized(env, from, to, fc, f),
        )
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        self.map_ref_rewa_or_dct(
            (),
            |(), amount| TxPayment::<Env>::into_full_payment_data(Rewa(amount), env),
            |(), dct_payment| TxPayment::<Env>::into_full_payment_data(dct_payment, env),
        )
    }
}

impl<Env> TxPayment<Env> for RewaOrDctTokenPayment<Env::Api>
where
    Env: TxEnv,
{
    #[inline]
    fn is_no_payment(&self, env: &Env) -> bool {
        (&self).is_no_payment(env)
    }

    #[inline]
    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        (&self).perform_transfer_execute(env, to, gas_limit, fc)
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
        self.map_rewa_or_dct(
            (to, fc, f),
            |(to, fc, f), amount| Rewa(amount).with_normalized(env, from, to, fc, f),
            |(to, fc, f), dct_payment| dct_payment.with_normalized(env, from, to, fc, f),
        )
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        self.map_rewa_or_dct(
            (),
            |(), amount| TxPayment::<Env>::into_full_payment_data(Rewa(amount), env),
            |(), dct_payment| TxPayment::<Env>::into_full_payment_data(dct_payment, env),
        )
    }
}
