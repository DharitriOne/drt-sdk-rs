use drt_sc::imports::*;

use crate::deposit_info::*;

#[drt_sc::module]
pub trait StorageModule {
    #[view]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<DepositInfo<Self::Api>>;

    #[storage_mapper("fee")]
    fn fee(&self, token: &RewaOrDctTokenIdentifier) -> SingleValueMapper<BigUint>;

    #[storage_mapper("collectedFees")]
    fn collected_fees(&self, token: &RewaOrDctTokenIdentifier) -> SingleValueMapper<BigUint>;

    #[storage_mapper("whitelistedFeeTokens")]
    fn whitelisted_fee_tokens(&self) -> UnorderedSetMapper<RewaOrDctTokenIdentifier>;

    #[storage_mapper("allTimeFeeTokens")]
    fn all_time_fee_tokens(&self) -> UnorderedSetMapper<RewaOrDctTokenIdentifier>;
}
