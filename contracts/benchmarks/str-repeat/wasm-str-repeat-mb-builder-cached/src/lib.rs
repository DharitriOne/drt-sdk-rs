// Code generated by the drt-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            1
// Async Callback (empty):               1
// Total number of exported functions:   3

#![no_std]

drt_sc_wasm_adapter::allocator!();
drt_sc_wasm_adapter::panic_handler!();

drt_sc_wasm_adapter::endpoints! {
    str_repeat
    (
        init => init
        mb_builder_benchmark => mb_builder_benchmark
    )
}

drt_sc_wasm_adapter::async_callback_empty! {}
