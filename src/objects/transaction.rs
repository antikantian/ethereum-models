use ethereum_types::{H160, H256, U256};
use rustc_serialize::hex::ToHex;
use web3::types::{
    Transaction as Web3Transaction
};

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