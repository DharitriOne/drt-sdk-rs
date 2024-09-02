pub mod cli;
pub mod cmd;
pub mod folder_structure;

pub use drt_sc_meta_lib::abi_json;
pub use drt_sc_meta_lib::ei;
pub use drt_sc_meta_lib::ei_check_json;
pub use drt_sc_meta_lib::version;
pub use drt_sc_meta_lib::version_history;

/// Backwards compatibility, please use `drt_sc_meta_lib::cli_main::<AbiObj>()`.
///
/// Failure to do so will result in slower build time.
#[deprecated(
    since = "0.41.0",
    note = "Backwards compatibility only, please use `cli_main` from crate `drt-sc-meta-lib` instead."
)]
pub fn cli_main<AbiObj: drt_sc::contract_base::ContractAbiProvider>() {
    drt_sc_meta_lib::cli_main::<AbiObj>()
}

/// Backwards compatibility, please use `drt_sc_meta_lib::multi_contract_config::<AbiObj>(contract_crate_path)`.
#[deprecated(
    since = "0.41.0",
    note = "Backwards compatibility only, please use `multi_contract_config` from crate `drt-sc-meta-lib` instead."
)]
pub fn multi_contract_config<AbiObj: drt_sc::contract_base::ContractAbiProvider>(
    contract_crate_path: &std::path::Path,
) -> drt_sc_meta_lib::contract::sc_config::ScConfig {
    drt_sc_meta_lib::multi_contract_config::<AbiObj>(contract_crate_path)
}
