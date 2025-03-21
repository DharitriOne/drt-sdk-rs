// Code generated by the drt-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            4
// Async Callback:                       1
// Total number of exported functions:   6

#![no_std]

drt_sc_wasm_adapter::allocator!();
drt_sc_wasm_adapter::panic_handler!();

drt_sc_wasm_adapter::endpoints! {
    fractional_nfts
    (
        init => init
        claimRoyaltiesFromMarketplace => claim_royalties_from_marketplace
        fractionalizeNFT => fractionalize_nft
        unFractionalizeNFT => unfractionalize_nft
        getFractionalToken => fractional_token
    )
}

drt_sc_wasm_adapter::async_callback! { fractional_nfts }
