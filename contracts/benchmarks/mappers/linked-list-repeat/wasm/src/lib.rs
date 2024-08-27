// Code generated by the drt-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback (empty):               1
// Total number of exported functions:  10

#![no_std]

drt_sc_wasm_adapter::allocator!();
drt_sc_wasm_adapter::panic_handler!();

drt_sc_wasm_adapter::endpoints! {
    linked_list_repeat
    (
        init => init
        add => add
        count => count
        remove => remove
        bench => bench
        add_struct => add_struct
        count_struct => count_struct
        remove_struct => remove_struct
        bench_struct => bench_struct
    )
}

drt_sc_wasm_adapter::async_callback_empty! {}
