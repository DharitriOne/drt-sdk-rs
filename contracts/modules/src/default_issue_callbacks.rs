use drt_sc::{storage::StorageKey, storage_clear, storage_set};

drt_sc::imports!();
drt_sc::derive_imports!();

// Always keep in sync with the token-related storage mappers. Only modify if really necessary.
#[drt_sc::module]
pub trait DefaultIssueCallbacksModule {
    #[callback]
    fn default_issue_cb(
        &self,
        initial_caller: ManagedAddress,
        storage_key: ManagedBuffer,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        let key = StorageKey::from(storage_key);
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                storage_set(key.as_ref(), &TokenMapperState::Token(token_id));
            },
            ManagedAsyncCallResult::Err(_) => {
                self.return_failed_issue_funds(initial_caller);
                storage_clear(key.as_ref());
            },
        }
    }

    #[callback]
    fn default_issue_init_supply_cb(
        &self,
        initial_caller: ManagedAddress,
        storage_key: ManagedBuffer,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        let key = StorageKey::from(storage_key);
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let token_id = self.call_value().single_dcdt().token_identifier;
                storage_set(key.as_ref(), &TokenMapperState::Token(token_id));
            },
            ManagedAsyncCallResult::Err(_) => {
                self.return_failed_issue_funds(initial_caller);
                storage_clear(key.as_ref());
            },
        }
    }

    fn return_failed_issue_funds(&self, initial_caller: ManagedAddress) {
        let rewa_returned = self.call_value().rewa_value().to_u64().unwrap();
        if rewa_returned > 0u64 {
            self.tx().to(&initial_caller).rewa(rewa_returned).transfer();
        }
    }
}
