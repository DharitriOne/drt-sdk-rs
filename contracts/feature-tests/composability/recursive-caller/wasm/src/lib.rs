// Code generated by the drt-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            1
// Async Callback:                       1
// Total number of exported functions:   3

#![no_std]

drt_sc_wasm_adapter::allocator!();
drt_sc_wasm_adapter::panic_handler!();

drt_sc_wasm_adapter::endpoints! {
    recursive_caller
    (
        init => init
        recursive_send_funds => recursive_send_funds
    )
}

drt_sc_wasm_adapter::async_callback! { recursive_caller }
