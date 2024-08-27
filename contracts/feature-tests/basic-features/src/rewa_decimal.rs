drt_sc::imports!();

/// Used for testing the rewa_decimal function return type
#[drt_sc::module]
pub trait RewaDecimal {
    #[payable("REWA")]
    #[endpoint]
    fn returns_rewa_decimal(&self) -> ManagedDecimal<Self::Api, ConstDecimals<18>> {
        self.call_value().rewa_decimal()
    }
}
