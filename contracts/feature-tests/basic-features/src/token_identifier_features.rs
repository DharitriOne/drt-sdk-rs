drt_sc::imports!();

#[drt_sc::module]
pub trait TokenIdentifierFeatures {
    #[endpoint]
    fn token_identifier_rewa(&self) -> RewaOrDctTokenIdentifier {
        RewaOrDctTokenIdentifier::rewa()
    }

    #[endpoint]
    fn token_identifier_is_valid_1(&self, token_id: RewaOrDctTokenIdentifier) -> bool {
        token_id.is_valid()
    }

    #[endpoint]
    fn token_identifier_is_valid_2(&self, bytes: ManagedBuffer) -> bool {
        TokenIdentifier::from(bytes).is_valid_dct_identifier()
    }
}
