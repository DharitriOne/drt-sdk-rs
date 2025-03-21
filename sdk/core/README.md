# Dharitri SDK for Rust

[![Crates.io](https://img.shields.io/crates/v/drt-sdk)](https://crates.io/crates/drt-sdk)

General purpose collection of tools & SDKs to interact with the Dharitri blockchain from Rust projects.

## Example

```rust
use drt_sdk::blockchain::rpc::{CommunicationProxy, DEVNET_GATEWAY};

#[tokio::test]
async fn get_network_config() {
    let blockchain = CommunicationProxy::new(DEVNET_GATEWAY.to_string());
    let network_config = blockchain.get_network_config().await.unwrap();

    println!("network_config: {:?}", network_config)
}
```

More examples in `./examples`.

## Acknowledgements

Project originally started by [Bicarus labs](https://github.com/bicarus-labs/numbat-sdk-moars), later integrated into the Dharitri official codebase.
