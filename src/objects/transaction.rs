use ethereum_types::{H160, H256, U256};
use rustc_serialize::hex::ToHex;
use web3::types::{
    Transaction as Web3Transaction,
    TransactionReceipt as Web3TransactionReceipt
};

use objects::Log;

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub hash: H256,
    pub nonce: U256,
    pub block_hash: Option<H256>,
    pub block_number: Option<U256>,
    pub transaction_index: Option<u64>,
    pub from: H160,
    pub to: Option<H160>,
    pub value: U256,
    pub gas_price: U256,
    pub gas: U256,
    pub input: String,
}

impl From<Web3Transaction> for Transaction {
    fn from(tx: Web3Transaction) -> Self {
        Transaction {
            hash: H256::from(tx.hash.0),
            nonce: U256::from(tx.nonce.0),
            block_hash: tx.block_hash.map(|h| H256::from(h.0)),
            block_number: tx.block_number.map(|bn| U256::from(bn.0)),
            transaction_index: tx.transaction_index.map(|i| i.low_u64()),
            from: H160::from(tx.from.0),
            to: tx.to.map(|h| H160::from(h.0)),
            value: U256::from(tx.value.0),
            gas_price: U256::from(tx.gas_price.0),
            gas: U256::from(tx.gas.0),
            input: String::from("0x") + &tx.input.0.to_hex()
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceipt {
    pub transaction_hash: H256,
    pub transaction_index: u64,
    pub block_number: U256,
    pub block_hash: H256,
    pub cumulative_gas_used: U256,
    pub gas_used: U256,
    pub contract_address: Option<H160>,
    pub logs: Vec<Log>
}

impl From<Web3TransactionReceipt> for TransactionReceipt {
    fn from(tr: Web3TransactionReceipt) -> Self {
        TransactionReceipt {
            transaction_hash: H256::from(tr.transaction_hash.0),
            transaction_index: tr.transaction_index.low_u64(),
            block_number: U256::from(tr.block_number.0),
            block_hash: H256::from(tr.block_hash.0),
            cumulative_gas_used: U256::from(tr.cumulative_gas_used.0),
            gas_used: U256::from(tr.gas_used.0),
            contract_address: tr.contract_address.map(|ca| H160::from(ca.0)),
            logs: tr.logs.into_iter().map(|web3_log| Log::from(web3_log)).collect()
        }
    }
}