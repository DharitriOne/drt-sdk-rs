use drt_sdk::{
    data::address::Address,
    gateway::{GatewayProxy, DEVNET_GATEWAY},
};

#[tokio::main]
async fn main() {
    let addr = Address::from_bech32_string(
        "moa1932eft30w753xyvme8d49qejgkjc09n5e49w4mwdjtm0neld797s3hts63",
    )
    .unwrap();

    let blockchain = GatewayProxy::new(DEVNET_GATEWAY.to_string());
    let account_storage = blockchain.get_account_storage_keys(&addr).await.unwrap();

    println!("Account Storage: {account_storage:#?}");
}
