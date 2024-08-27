use crate::vault_proxy;

drt_sc::imports!();

const PERCENTAGE_TOTAL: u64 = 10_000; // 100%

#[drt_sc::module]
pub trait ForwarderSyncCallModule {
    #[endpoint]
    #[payable("*")]
    fn echo_arguments_sync(&self, to: ManagedAddress, args: MultiValueEncoded<ManagedBuffer>) {
        let half_gas = self.blockchain().get_gas_left() / 2;

        let result = self
            .tx()
            .to(&to)
            .gas(half_gas)
            .typed(vault_proxy::VaultProxy)
            .echo_arguments(args)
            .returns(ReturnsResult)
            .sync_call();

        self.execute_on_dest_context_result_event(&result.into_vec_of_buffers());
    }

    #[endpoint]
    #[payable("*")]
    fn echo_arguments_sync_twice(
        &self,
        to: ManagedAddress,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let one_third_gas = self.blockchain().get_gas_left() / 3;

        let result = self
            .tx()
            .to(&to)
            .gas(one_third_gas)
            .typed(vault_proxy::VaultProxy)
            .echo_arguments(args.clone())
            .returns(ReturnsResult)
            .sync_call();

        self.execute_on_dest_context_result_event(&result.into_vec_of_buffers());

        let result = self
            .tx()
            .to(&to)
            .gas(one_third_gas)
            .typed(vault_proxy::VaultProxy)
            .echo_arguments(args)
            .returns(ReturnsResult)
            .sync_call();

        self.execute_on_dest_context_result_event(&result.into_vec_of_buffers());
    }

    #[event("echo_arguments_sync_result")]
    fn execute_on_dest_context_result_event(&self, result: &ManagedVec<Self::Api, ManagedBuffer>);

    #[endpoint]
    #[payable("*")]
    fn forward_sync_accept_funds(&self, to: ManagedAddress) {
        let payment = self.call_value().rewa_or_single_dct();
        let half_gas = self.blockchain().get_gas_left() / 2;

        let result = self
            .tx()
            .to(&to)
            .gas(half_gas)
            .typed(vault_proxy::VaultProxy)
            .accept_funds_echo_payment()
            .payment(payment)
            .returns(ReturnsResult)
            .sync_call();

        let (rewa_value, dct_transfers_multi) = result.into_tuple();

        self.accept_funds_sync_result_event(&rewa_value, &dct_transfers_multi);
    }

    #[payable("*")]
    #[endpoint]
    fn forward_sync_accept_funds_with_fees(&self, to: ManagedAddress, percentage_fees: BigUint) {
        let (token_id, payment) = self.call_value().rewa_or_single_fungible_dct();
        let fees = &payment * &percentage_fees / PERCENTAGE_TOTAL;
        let amount_to_send = payment - fees;

        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .rewa_or_single_dct(&token_id, 0u64, &amount_to_send)
            .returns(ReturnsResult)
            .sync_call();
    }

    #[event("accept_funds_sync_result")]
    fn accept_funds_sync_result_event(
        &self,
        #[indexed] rewa_value: &BigUint,
        #[indexed] multi_dct: &MultiValueEncoded<DctTokenPaymentMultiValue>,
    );

    #[endpoint]
    #[payable("*")]
    fn forward_sync_accept_funds_then_read(&self, to: ManagedAddress) -> usize {
        let payment = self.call_value().rewa_or_single_dct();
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .payment(payment)
            .sync_call();

        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .call_counts(b"accept_funds")
            .returns(ReturnsResult)
            .sync_call()
    }

    #[endpoint]
    fn forward_sync_retrieve_funds(
        &self,
        to: ManagedAddress,
        token: RewaOrDctTokenIdentifier,
        token_nonce: u64,
        amount: BigUint,
    ) {
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds(token, token_nonce, amount)
            .sync_call();
    }

    #[payable("*")]
    #[endpoint]
    fn forward_sync_retrieve_funds_with_accept_func(
        &self,
        to: ManagedAddress,
        token: TokenIdentifier,
        amount: BigUint,
    ) {
        let payments = self.call_value().all_dct_transfers();

        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds_with_transfer_exec(
                token,
                amount,
                OptionalValue::<ManagedBuffer>::Some(b"accept_funds_func".into()),
            )
            .payment(payments)
            .sync_call();
    }

    #[payable("*")]
    #[endpoint]
    fn accept_funds_func(&self) {}

    #[endpoint]
    fn forward_sync_accept_funds_multi_transfer(
        &self,
        to: ManagedAddress,
        token_payments: MultiValueEncoded<MultiValue3<TokenIdentifier, u64, BigUint>>,
    ) {
        let mut all_token_payments = ManagedVec::new();

        for multi_arg in token_payments.into_iter() {
            let (token_identifier, token_nonce, amount) = multi_arg.into_tuple();
            let payment = DctTokenPayment::new(token_identifier, token_nonce, amount);
            all_token_payments.push(payment);
        }

        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .payment(all_token_payments)
            .sync_call();
    }
}
