// Code generated by the drt-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            0
// Async Callback:                       1
// Total number of exported functions:   2

#![no_std]

drt_sc_wasm_adapter::allocator!();
drt_sc_wasm_adapter::panic_handler!();

drt_sc_wasm_adapter::endpoints! {
    forwarder_raw
    (
        init => init_sync_call
    )
}

drt_sc_wasm_adapter::async_callback! { forwarder_raw }
