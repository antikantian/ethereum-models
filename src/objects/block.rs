use std::u64;
use std::cmp::Ordering;

use ethereum_types::{H160, H256, U256};
use rustc_serialize::hex::ToHex;
use web3::types::{Block as Web3Block, Transaction as Web3Transaction};

use super::Transaction;
use opt_u64_from_str;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum BlockNumber<'a> {
    Name(&'a str),
    Number(u64)
}

/// An Ethereum block.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub hash: Option<H256>,
    pub parent_hash: H256,
    pub sha3_uncles: H256,
    pub author: H160,
    pub state_root: H256,
    pub transactions_root: H256,
    pub receipts_root: H256,
    #[serde(deserialize_with = "opt_u64_from_str")]
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

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.number
            .and_then(|bn_a| other.number.map(|bn_b| (bn_a, bn_b)))
            .map(|(bn_a, bn_b)| bn_a == bn_b)
            .unwrap_or(false)
    }
}


impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.number
            .and_then(|ba| other.number.map(|bb| (ba, bb)))
            .map(|(ba, bb)| ba.cmp(&bb))
    }
}

impl From<Web3Block<Web3Transaction>> for Block {
    fn from(block: Web3Block<Web3Transaction>) -> Self {
        Block {
            hash: block.hash,
            parent_hash: block.parent_hash,
            sha3_uncles: block.uncles_hash,
            author: block.author,
            state_root: block.state_root,
            transactions_root: block.transactions_root,
            receipts_root: block.receipts_root,
            number: block.number.map(|num| num.low_u64()),
            gas_used: block.gas_used,
            gas_limit: block.gas_limit,
            extra_data: String::from("0x") + &block.extra_data.0.to_hex(),
            timestamp: block.timestamp,
            difficulty: block.difficulty,
            total_difficulty: block.total_difficulty,
            transactions: block.transactions.into_iter().map(|tx| Transaction::from(tx)).collect(),
            size: block.size
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use serde_json;
    use super::Block;
    use types::{H160, H256, U256};

    #[test]
    fn block_deserializes_no_tx() {
        let block_json = include_str!("../../test_data/block_no_tx.json");
        serde_json::from_str::<Block>(&block_json).unwrap();
    }

    #[test]
    fn block_number_deserializes() {
        let block_json = include_str!("../../test_data/block_no_tx.json");
        let block = serde_json::from_str::<Block>(&block_json).unwrap();
        let actual_block_number = 5110738;
        let deserialized_block_number = block.number.unwrap();
        assert_eq!(deserialized_block_number, actual_block_number);
    }

    #[test]
    fn block_deserializes_with_tx() {
        let block_json = include_str!("../../test_data/block_with_tx.json");
        serde_json::from_str::<Block>(&block_json).unwrap();
    }

    #[test]
    fn blocks_have_partial_eq() {
        let some_h256_str = "32465f4e8fd8d0e3ab084396024e199344050ab33b993844ccb2229d648200d9";
        let some_h256 = H256::from_str(some_h256_str).unwrap();

        let some_h160_str = "dfc17b1d9263e729948016c43865a63d1462dae6";
        let some_h160 = H160::from_str(some_h160_str).unwrap();

        let some_u256 = U256::from(1000);

        let block_5000000 = Block {
            hash: Some(some_h256.clone()),
            parent_hash: some_h256.clone(),
            sha3_uncles: some_h256.clone(),
            author: some_h160.clone(),
            state_root: some_h256.clone(),
            transactions_root: some_h256.clone(),
            receipts_root: some_h256.clone(),
            number: Some(5000000_u64),
            gas_used: some_u256.clone(),
            gas_limit: some_u256.clone(),
            extra_data: "none".to_string(),
            timestamp: some_u256.clone(),
            difficulty: some_u256.clone(),
            total_difficulty: some_u256.clone(),
            transactions: Vec::new(),
            size: None
        };

        let block_5000001 = Block {
            hash: Some(some_h256.clone()),
            parent_hash: some_h256.clone(),
            sha3_uncles: some_h256.clone(),
            author: some_h160.clone(),
            state_root: some_h256.clone(),
            transactions_root: some_h256.clone(),
            receipts_root: some_h256.clone(),
            number: Some(5000001_u64),
            gas_used: some_u256.clone(),
            gas_limit: some_u256.clone(),
            extra_data: "none".to_string(),
            timestamp: some_u256.clone(),
            difficulty: some_u256.clone(),
            total_difficulty: some_u256.clone(),
            transactions: Vec::new(),
            size: None
        };

        assert_eq!(block_5000000, block_5000000);
        assert_ne!(block_5000001, block_5000000);
    }

    #[test]
    fn blocks_have_partial_ord() {
        let some_h256_str = "32465f4e8fd8d0e3ab084396024e199344050ab33b993844ccb2229d648200d9";
        let some_h256 = H256::from_str(some_h256_str).unwrap();

        let some_h160_str = "dfc17b1d9263e729948016c43865a63d1462dae6";
        let some_h160 = H160::from_str(some_h160_str).unwrap();

        let some_u256 = U256::from(1000);

        let block_5000000 = Block {
            hash: Some(some_h256.clone()),
            parent_hash: some_h256.clone(),
            sha3_uncles: some_h256.clone(),
            author: some_h160.clone(),
            state_root: some_h256.clone(),
            transactions_root: some_h256.clone(),
            receipts_root: some_h256.clone(),
            number: Some(5000000_u64),
            gas_used: some_u256.clone(),
            gas_limit: some_u256.clone(),
            extra_data: "none".to_string(),
            timestamp: some_u256.clone(),
            difficulty: some_u256.clone(),
            total_difficulty: some_u256.clone(),
            transactions: Vec::new(),
            size: None
        };

        let block_5000001 = Block {
            hash: Some(some_h256.clone()),
            parent_hash: some_h256.clone(),
            sha3_uncles: some_h256.clone(),
            author: some_h160.clone(),
            state_root: some_h256.clone(),
            transactions_root: some_h256.clone(),
            receipts_root: some_h256.clone(),
            number: Some(5000001_u64),
            gas_used: some_u256.clone(),
            gas_limit: some_u256.clone(),
            extra_data: "none".to_string(),
            timestamp: some_u256.clone(),
            difficulty: some_u256.clone(),
            total_difficulty: some_u256.clone(),
            transactions: Vec::new(),
            size: None
        };

        let block_5000002 = Block {
            hash: Some(some_h256.clone()),
            parent_hash: some_h256.clone(),
            sha3_uncles: some_h256.clone(),
            author: some_h160.clone(),
            state_root: some_h256.clone(),
            transactions_root: some_h256.clone(),
            receipts_root: some_h256.clone(),
            number: Some(5000002_u64),
            gas_used: some_u256.clone(),
            gas_limit: some_u256.clone(),
            extra_data: "none".to_string(),
            timestamp: some_u256.clone(),
            difficulty: some_u256.clone(),
            total_difficulty: some_u256.clone(),
            transactions: Vec::new(),
            size: None
        };

        let mut blocks = vec![block_5000000, block_5000002, block_5000001];
        blocks.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let block_seq = blocks.iter().map(|b| b.number).collect::<Vec<Option<u64>>>();

        assert_eq!(block_seq, vec![Some(5000000_u64), Some(5000001_u64), Some(5000002_u64)]);
    }
}