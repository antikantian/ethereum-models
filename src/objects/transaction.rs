use std::hash::{Hash, Hasher};

use types::{H160, H256, U128, U256};
use rustc_serialize::hex::ToHex;
use twox_hash::XxHash;
use web3::types::{
    Transaction as Web3Transaction,
    TransactionReceipt as Web3TransactionReceipt
};

use {u64_from_str, opt_u64_from_str};
use objects::Log;

/// A trait for all transaction-data-containing structures, but that are themselves
/// not (only) transactions.  Used in several of Etherswap's private repositories.
pub trait TransactionLike {
    fn get_tx(&self) -> &Transaction;

    fn tx_hash(&self) -> &H256 {
        &self.get_tx().hash
    }

    fn nonce(&self) -> &U256 {
        &self.get_tx().nonce
    }

    fn block_hash(&self) -> Option<&H256> {
        self.get_tx().block_hash.as_ref()
    }

    fn block_number(&self) -> Option<&U256> {
        self.get_tx().block_number.as_ref()
    }

    fn transaction_index(&self) -> Option<U128> {
        self.get_tx().transaction_index
    }

    fn tx_from(&self) -> &H160 {
        &self.get_tx().from
    }

    fn tx_to(&self) -> Option<&H160> {
        self.get_tx().to.as_ref()
    }

    fn tx_value(&self) -> &U256 {
        &self.get_tx().value
    }

    fn tx_gas_price(&self) -> &U256 {
        &self.get_tx().gas_price
    }

    fn tx_gas_limit(&self) -> &U256 {
        &self.get_tx().gas
    }

    fn tx_input(&self) -> &str {
        self.get_tx().input.as_str()
    }
}

/// An Ethereum transaction.
#[derive(Hash, Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub hash: H256,
    pub nonce: U256,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<H256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_number: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_index: Option<U128>,
    pub from: H160,
    pub to: Option<H160>,
    pub value: U256,
    pub gas_price: U256,
    pub gas: U256,
    pub input: String,
}

impl Transaction {
    pub fn to_hash(&self) -> u64 {
        let mut hasher = XxHash::default();
        self.hash(&mut hasher);
        hasher.finish()
    }

    pub fn hash_input(&self) -> u64 {
        let mut hasher = XxHash::default();
        let hash_string = format!("{:?}{:?}{}", &self.hash, &self.nonce, &self.input);
        hash_string.hash(&mut hasher);
        hasher.finish()
    }
}

impl From<Web3Transaction> for Transaction {
    fn from(tx: Web3Transaction) -> Self {
        Transaction {
            hash: tx.hash,
            nonce: tx.nonce,
            block_hash: tx.block_hash,
            block_number: tx.block_number,
            transaction_index: tx.transaction_index,
            from: tx.from,
            to: tx.to,
            value: tx.value,
            gas_price: tx.gas_price,
            gas: tx.gas,
            input: String::from("0x") + &tx.input.0.to_hex()
        }
    }
}

pub trait ReceiptLike {
    fn get_receipt(&self) -> &TransactionReceipt;

    fn gas_used(&self) -> &U256 {
        &self.get_receipt().gas_used
    }

    fn receipt_logs(&self) -> &Vec<Log> {
        self.get_receipt().logs.as_ref()
    }
}

/// A transaction's receipt.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceipt {
    pub transaction_hash: H256,
    pub transaction_index: U128,
    pub block_number: U256,
    pub block_hash: H256,
    pub cumulative_gas_used: U256,
    pub gas_used: U256,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<H160>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub logs: Vec<Log>
}

impl From<Web3TransactionReceipt> for TransactionReceipt {
    fn from(tr: Web3TransactionReceipt) -> Self {
        TransactionReceipt {
            transaction_hash: tr.transaction_hash,
            transaction_index: tr.transaction_index,
            block_number: tr.block_number,
            block_hash: tr.block_hash,
            cumulative_gas_used: tr.cumulative_gas_used,
            gas_used: tr.gas_used,
            contract_address: tr.contract_address,
            logs: tr.logs.into_iter().map(|web3_log| Log::from(web3_log)).collect()
        }
    }
}