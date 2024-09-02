// Code generated by the drt-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           16
// Async Callback (empty):               1
// Total number of exported functions:  18

#![no_std]

drt_sc_wasm_adapter::allocator!();
drt_sc_wasm_adapter::panic_handler!();

drt_sc_wasm_adapter::endpoints! {
    vault
    (
        init => init
        echo_arguments => echo_arguments
        echo_arguments_without_storage => echo_arguments_without_storage
        echo_caller => echo_caller
        accept_funds => accept_funds
        accept_funds_echo_payment => accept_funds_echo_payment
        accept_funds_single_dcdt_transfer => accept_funds_single_dcdt_transfer
        reject_funds => reject_funds
        retrieve_funds_with_transfer_exec => retrieve_funds_with_transfer_exec
        retrieve_funds => retrieve_funds
        retrieve_multi_funds_async => retrieve_multi_funds_async
        burn_and_create_retrieve_async => burn_and_create_retrieve_async
        get_owner_address => get_owner_address
        call_counts => call_counts
        num_called_retrieve_funds_promises => num_called_retrieve_funds_promises
        num_async_calls_sent_from_child => num_async_calls_sent_from_child
        retrieve_funds_promises => retrieve_funds_promises
    )
}

drt_sc_wasm_adapter::async_callback_empty! {}
