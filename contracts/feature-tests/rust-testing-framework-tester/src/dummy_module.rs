drt_sc::imports!();

#[drt_sc::module]
pub trait DummyModule {
    fn some_function(&self) -> BigUint {
        BigUint::zero()
    }
}
