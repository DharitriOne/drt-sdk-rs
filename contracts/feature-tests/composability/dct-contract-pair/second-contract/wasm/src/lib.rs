// Code generated by the drt-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            3
// Async Callback (empty):               1
// Total number of exported functions:   5

#![no_std]

drt_sc_wasm_adapter::allocator!();
drt_sc_wasm_adapter::panic_handler!();

drt_sc_wasm_adapter::endpoints! {
    second_contract
    (
        init => init
        acceptDctPayment => accept_dct_payment
        rejectDctPayment => reject_dct_payment
        getdctTokenName => get_contract_dct_token_identifier
    )
}

drt_sc_wasm_adapter::async_callback_empty! {}
