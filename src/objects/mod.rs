mod block;
mod log;
mod transaction;

pub mod trace;

pub use self::block::Block;
pub use self::log::{Log, LogLike};
pub use self::trace::ParityTrace;
pub use self::transaction::{
    Transaction,
    TransactionLike,
    TransactionReceipt,
    ReceiptLike
};

pub enum AddressType {
    Address(::types::H160),
    Contract(::types::H160)
}

impl AddressType {
    pub fn to_string_clean(&self) -> String {
        match *self {
            AddressType::Address(hash) => format!("{:?}", &hash),
            AddressType::Contract(hash) => format!("{:?}", &hash)
        }
    }

    pub fn to_string_0x(&self) -> String {
        String::from("0x") + &self.to_string_clean()
    }
}