drt_sc::imports!();
drt_sc::derive_imports!();

use crate::{
    common::{self, CallbackData},
    vault_proxy,
};
#[drt_sc::module]
pub trait CallPromisesBackTransfersModule: common::CommonModule {
    #[endpoint]
    fn forward_promise_retrieve_funds_back_transfers(
        &self,
        to: ManagedAddress,
        token: RewaOrDctTokenIdentifier,
        token_nonce: u64,
        amount: BigUint,
    ) {
        let gas_limit = self.blockchain().get_gas_left() - 20_000_000;
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds(token, token_nonce, amount)
            .gas(gas_limit)
            .callback(self.callbacks().retrieve_funds_back_transfers_callback())
            .gas_for_callback(10_000_000)
            .register_promise();
    }

    #[promises_callback]
    fn retrieve_funds_back_transfers_callback(&self) {
        let back_transfers = self.blockchain().get_back_transfers();
        let rewa_transfer = back_transfers.total_rewa_amount;

        if rewa_transfer != BigUint::zero() {
            let rewa_token_id = RewaOrDctTokenIdentifier::rewa();
            self.retrieve_funds_callback_event(&rewa_token_id, 0, &rewa_transfer);

            let _ = self.callback_data().push(&CallbackData {
                callback_name: ManagedBuffer::from(b"retrieve_funds_callback"),
                token_identifier: rewa_token_id,
                token_nonce: 0,
                token_amount: rewa_transfer,
                args: ManagedVec::new(),
            });
        }

        for dct_transfer in &back_transfers.dct_payments {
            let (token, nonce, payment) = dct_transfer.into_tuple();
            let dct_token_id = RewaOrDctTokenIdentifier::dct(token);
            self.retrieve_funds_callback_event(&dct_token_id, nonce, &payment);

            let _ = self.callback_data().push(&CallbackData {
                callback_name: ManagedBuffer::from(b"retrieve_funds_callback"),
                token_identifier: dct_token_id,
                token_nonce: nonce,
                token_amount: payment,
                args: ManagedVec::new(),
            });
        }
    }
}
