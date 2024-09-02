#![no_std]

mod contract_base_full_path_mod;
mod contract_base_mod;
mod internal_mod_a;
mod internal_mod_b;
mod internal_mod_c;
mod internal_mod_d;
mod internal_mod_init;
mod ongoing_operation_mod_example;
mod only_admin_derived_mod;
mod only_admin_mod;
mod only_owner_derived_mod;
mod only_owner_mod;
pub mod token_merge_mod_impl;

drt_sc::imports!();

/// Contract that tests that using modules works correctly.
/// Also provides testing for the most common modules:
/// - DnsModule
/// - FeaturesModule
/// - DcdtModule
/// - GovernanceModule
/// - PauseModule
#[drt_sc::contract]
#[dcdt_attribute("TICKER1", BigUint)]
#[dcdt_attribute("TICKER2", ManagedBuffer)]
pub trait UseModule:
    ContractBase
    + contract_base_full_path_mod::ContractBaseFullPathTestModule
    + contract_base_mod::ContractBaseTestModule
    + internal_mod_a::InternalModuleA
    + internal_mod_b::InternalModuleB
    + internal_mod_c::InternalModuleC
    + internal_mod_init::InternalModuleInit
    + only_owner_mod::OnlyOwnerTestModule
    + only_owner_derived_mod::OnlyOwnerDerivedTestModule
    + only_admin_mod::OnlyAdminTestModule
    + only_admin_derived_mod::OnlyAdminDerivedTestModule
    + ongoing_operation_mod_example::OngoingOperationModExample
    + token_merge_mod_impl::TokenMergeModImpl
    + drt_sc_modules::claim_developer_rewards::ClaimDeveloperRewardsModule
    + drt_sc_modules::dns::DnsModule
    + drt_sc_modules::dcdt::DcdtModule
    + drt_sc_modules::features::FeaturesModule
    + drt_sc_modules::governance::GovernanceModule
    + drt_sc_modules::governance::governance_configurable::GovernanceConfigurablePropertiesModule
    + drt_sc_modules::governance::governance_events::GovernanceEventsModule
    + drt_sc_modules::pause::PauseModule
    + drt_sc_modules::staking::StakingModule
    + drt_sc_modules::token_merge::TokenMergeModule
    + drt_sc_modules::token_merge::merged_token_setup::MergedTokenSetupModule
    + drt_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + drt_sc_modules::only_admin::OnlyAdminModule
    + drt_sc_modules::ongoing_operation::OngoingOperationModule
{
    /// Validates that the "featureName" feature is on.
    /// Uses the `feature_guard!` macro.
    #[endpoint(checkFeatureGuard)]
    fn check_feature_guard(&self) {
        self.check_feature_on(b"featureName", true);
    }

    #[endpoint(checkPause)]
    fn check_pause(&self) -> bool {
        self.is_paused()
    }
}
