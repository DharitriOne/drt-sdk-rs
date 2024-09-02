drt_sc::imports!();

#[drt_sc::module]
pub trait OnlyAdminDerivedTestModule {
    #[view]
    fn call_derived_not_admin_only(&self) {}
}
