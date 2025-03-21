// Code generated by the drt-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           17
// Async Callback:                       1
// Total number of exported functions:  20

#![no_std]

drt_sc_wasm_adapter::allocator!();
drt_sc_wasm_adapter::panic_handler!();

drt_sc_wasm_adapter::endpoints! {
    crypto_zombies
    (
        init => init
        upgrade => upgrade
        set_crypto_kitties_sc_address => set_crypto_kitties_sc_address
        generate_random_dna => generate_random_dna
        create_random_zombie => create_random_zombie
        is_ready => is_ready
        feed_on_kitty => feed_on_kitty
        dna_digits => dna_digits
        zombie_last_index => zombie_last_index
        zombies => zombies
        zombie_owner => zombie_owner
        crypto_kitties_sc_address => crypto_kitties_sc_address
        cooldown_time => cooldown_time
        owned_zombies => owned_zombies
        level_up => level_up
        withdraw => withdraw
        change_name => change_name
        change_dna => change_dna
        attack => attack
    )
}

drt_sc_wasm_adapter::async_callback! { crypto_zombies }
