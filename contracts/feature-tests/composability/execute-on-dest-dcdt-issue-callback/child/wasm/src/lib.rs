// Code generated by the drt-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            2
// Async Callback:                       1
// Total number of exported functions:   4

#![no_std]

drt_sc_wasm_adapter::allocator!();
drt_sc_wasm_adapter::panic_handler!();

drt_sc_wasm_adapter::endpoints! {
    child
    (
        init => init
        issueWrappedRewa => issue_wrapped_rewa
        getWrappedRewaTokenIdentifier => wrapped_rewa_token_identifier
    )
}

drt_sc_wasm_adapter::async_callback! { child }
