// Code generated by the drt-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Upgrade:                              1
// Endpoints:                            0
// Async Callback (empty):               1
// Total number of exported functions:   2

#![no_std]

drt_sc_wasm_adapter::allocator!();
drt_sc_wasm_adapter::panic_handler!();

drt_sc_wasm_adapter::endpoints! {
    vault
    (
        upgrade => upgrade
    )
}

drt_sc_wasm_adapter::async_callback_empty! {}
