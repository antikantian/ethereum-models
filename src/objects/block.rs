use std::str;
use std::mem::transmute;

use ethereum_types::{H160, H256, U256};
use rustc_serialize::hex::ToHex;
use serde::ser::{Serialize, Serializer};
use web3::types::{Block as Web3Block, Transaction as Web3Transaction};

use super::Transaction;

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
            hash: block.hash,
            parent_hash: block.parent_hash,
            uncles_hash: block.uncles_hash,
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