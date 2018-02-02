use ethereum_types::{H160, H256, U256};
use rustc_serialize::hex::ToHex;
use web3::types::{Block as Web3Block, Transaction as Web3Transaction};

use super::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub hash: Option<H256>,
    pub parent_hash: H256,
    pub uncles_hash: H256,
    pub author: H160,
    pub state_root: H256,
    pub transactions_root: H256,
    pub receipts_root: H256,
    pub number: Option<u64>,
    pub gas_used: U256,
    pub gas_limit: U256,
    pub extra_data: String,
    pub timestamp: U256,
    pub difficulty: U256,
    pub total_difficulty: U256,
    pub transactions: Vec<Transaction>,
    pub size: Option<U256>
}

impl From<Web3Block<Web3Transaction>> for Block {
    fn from(block: Web3Block<Web3Transaction>) -> Self {
        Block {
            hash: block.hash.map(|h| H256::from(h.0)),
            parent_hash: H256::from(block.parent_hash.0),
            uncles_hash: H256::from(block.uncles_hash.0),
            author: H160::from(block.author.0),
            state_root: H256::from(block.state_root.0),
            transactions_root: H256::from(block.transactions_root.0),
            receipts_root: H256::from(block.receipts_root.0),
            number: block.number.map(|num| num.low_u64()),
            gas_used: U256::from(block.gas_used.0),
            gas_limit: U256::from(block.gas_limit.0),
            extra_data: String::from("0x") + &block.extra_data.0.to_hex(),
            timestamp: U256::from(block.timestamp.0),
            difficulty: U256::from(block.difficulty.0),
            total_difficulty: U256::from(block.total_difficulty.0),
            transactions: block.transactions.into_iter().map(|tx| Transaction::from(tx)).collect(),
            size: block.size.map(|s| U256::from(s.0))
        }
    }
}