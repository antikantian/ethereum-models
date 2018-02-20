mod block;
mod call;
mod log;
mod transaction;

pub mod trace;

pub use self::block::{Block, BlockNumber};
pub use self::call::TransactionCall;
pub use self::log::{Log, LogLike};
pub use self::trace::ParityTrace;
pub use self::transaction::{
    Transaction,
    TransactionLike,
    TransactionReceipt,
    ReceiptLike
};

#[derive(Debug, Serialize, Deserialize)]
pub enum AddressType {
    Address(::types::H160),
    Contract(::types::H160),
    Uncategorized(::types::H160)
}

impl AddressType {
    pub fn to_string_clean(&self) -> String {
        match *self {
            AddressType::Address(hash) => format!("{:?}", &hash),
            AddressType::Contract(hash) => format!("{:?}", &hash),
            AddressType::Uncategorized(hash) => format!("{:?}", &hash),
        }
    }

    pub fn to_string_0x(&self) -> String {
        String::from("0x") + &self.to_string_clean()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use types::H160;
    use super::AddressType;

    #[test]
    fn yields_0x_prefix() {
        let a = AddressType::Address(
            H160::from_str("a94f5374fce5edbc8e2a8697c15331677e6ebf0b").unwrap()
        );

        let a_0x = a.to_string_0x();

        assert_eq!(a_0x, "0xa94f5374fce5edbc8e2a8697c15331677e6ebf0b");
    }

    #[test]
    fn yields_no_0x_prefix() {
        let a = AddressType::Address(
            H160::from_str("a94f5374fce5edbc8e2a8697c15331677e6ebf0b").unwrap()
        );

        let a_no_0x = a.to_string_clean();

        assert_eq!(a_no_0x, "a94f5374fce5edbc8e2a8697c15331677e6ebf0b");
    }
}