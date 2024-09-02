use drt_sc_modules::only_admin;

drt_sc::imports!();

#[drt_sc::module]
#[only_admin]
pub trait OnlyAdminTestModule:
    super::only_admin_derived_mod::OnlyAdminDerivedTestModule + only_admin::OnlyAdminModule
{
    #[endpoint]
    fn only_admin_mod_endpoint(&self) {}
}
