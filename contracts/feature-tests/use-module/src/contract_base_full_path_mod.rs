drt_sc::imports!();

#[drt_sc::module]
pub trait ContractBaseFullPathTestModule: drt_sc::contract_base::ContractBase {
    #[endpoint]
    fn call_contract_base_full_path_endpoint(&self) {}
}
