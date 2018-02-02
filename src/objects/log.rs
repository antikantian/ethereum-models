use ethereum_types::{H160, H256, U256};
use rustc_serialize::hex::ToHex;
use web3::types::{Log as Web3Log};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub address: H160,
    pub topics: Vec<H256>,
    pub data: String,
    pub block_hash: Option<H256>,
    pub block_number: Option<U256>,
    pub transaction_hash: Option<H256>,
    pub transaction_index: Option<U256>,
    pub log_index: Option<U256>,
    pub transaction_log_index: Option<U256>,
    #[serde(rename="type")]
    pub log_type: String
}

impl From<Web3Log> for Log {
    fn from(log: Web3Log) -> Self {
        Log {
            address: H160::from(log.address.0),
            topics: log.topics.into_iter().map(|h| H256::from(h.0)).collect(),
            data: String::from("0x") + &log.data.0.to_hex(),
            block_hash: log.block_hash.map(|h| H256::from(h.0)),
            block_number: log.block_number.map(|idx| U256::from(idx.0)),
            transaction_hash: log.transaction_hash.map(|h| H256::from(h.0)),
            transaction_index: log.transaction_index.map(|idx| U256::from(idx.0)),
            log_index: log.log_index.map(|idx| U256::from(idx.0)),
            transaction_log_index: log.transaction_log_index.map(|idx| U256::from(idx.0)),
            log_type: log.log_type
        }
    }
}