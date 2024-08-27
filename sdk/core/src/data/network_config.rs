use serde::{Deserialize, Serialize};

// NetworkConfig holds the network configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    #[serde(rename = "moa_chain_id")]
    pub chain_id: String,
    #[serde(rename = "moa_denomination")]
    pub denomination: i32,
    #[serde(rename = "moa_gas_per_data_byte")]
    pub gas_per_data_byte: u64,
    #[serde(rename = "moa_latest_tag_software_version")]
    pub latest_tag_software_version: String,
    #[serde(rename = "moa_meta_consensus_group_size")]
    pub meta_consensus_group_size: u64,
    #[serde(rename = "moa_min_gas_limit")]
    pub min_gas_limit: u64,
    #[serde(rename = "moa_min_gas_price")]
    pub min_gas_price: u64,
    #[serde(rename = "moa_min_transaction_version")]
    pub min_transaction_version: u32,
    #[serde(rename = "moa_num_metachain_nodes")]
    pub num_metachain_nodes: u64,
    #[serde(rename = "moa_num_nodes_in_shard")]
    pub num_nodes_in_shard: u64,
    #[serde(rename = "moa_num_shards_without_meta")]
    pub num_shards_without_meta: u32,
    #[serde(rename = "moa_round_duration")]
    pub round_duration: i64,
    #[serde(rename = "moa_shard_consensus_group_size")]
    pub shard_consensus_group_size: u64,
    #[serde(rename = "moa_start_time")]
    pub start_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfigData {
    pub config: NetworkConfig,
}

// NetworkConfigResponse holds the network config endpoint response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfigResponse {
    pub error: String,
    pub code: String,
    pub data: Option<NetworkConfigData>,
}
