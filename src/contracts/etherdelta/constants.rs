use std::str::FromStr;

use ::types::{H160, H256, U256};
use serde_json;

lazy_static! {
    pub static ref MAKER_FEE: U256 = U256::from(0x0);

    pub static ref TAKER_FEE: U256 = U256::from_str("aa87bee538000")
        .expect("Static integer conversion should never fail");

    pub static ref ETHERDELTA_ADDRESS: H160 = {
        serde_json::from_str("8d12a197cb00d4747a1fe03395095ce2a5cc6819")
        .expect("Deserialization of static should never fail")
    };

    pub static ref ETHERDELTA_CANCEL_ORDER_TOPIC: H256 = {
        serde_json::from_str("1e0b760c386003e9cb9bcf4fcf3997886042859d9b6ed6320e804597fcdb28b0")
            .expect("Deserialization of static should never fail")
    };

    pub static ref ETHERDELTA_TRADE_TOPIC: H256 = {
        serde_json::from_str("6effdda786735d5033bfad5f53e5131abcced9e52be6c507b62d639685fbed6d")
            .expect("Deserialization of static should never fail")
    };

    pub static ref ETHERDELTA_DEPOSIT_TOPIC: H256 = {
        serde_json::from_str("dcbc1c05240f31ff3ad067ef1ee35ce4997762752e3a095284754544f4c709d7")
            .expect("Deserialization of static should never fail")
    };

    pub static ref ETHERDELTA_WITHDRAW_TOPIC: H256 = {
        serde_json::from_str("f341246adaac6f497bc2a656f546ab9e182111d630394f0c57c710a59a2cb567")
            .expect("Deserialization of static should never fail")
    };
}

pub const ETHERDELTA_CREATION_BLOCK: u64 = 3154196;

// Methods
pub const CANCEL_ORDER_ID: &'static str = "0x278b8c0e";
pub const DEPOSIT_ID: &'static str = "0xd0e30db0";
pub const DEPOSIT_TOKEN_ID: &'static str = "0x338b5dea";
pub const TRADE_ID: &'static str = "0x0a19b14a";
pub const WITHDRAW_ID: &'static str = "0x2e1a7d4d";
pub const WITHDRAW_TOKEN_ID: &'static str = "0x9e281a98";

// Static methods
pub const AMOUNT_FILLED_ID: &'static str = "0x46be96c3";
pub const AVAILABLE_VOLUME_ID: &'static str = "0xfb6e155f";
pub const TEST_TRADE_ID: &'static str = "0x6c86888b";
pub const BALANCE_OF_ID: &'static str = "0xf7888aec";


pub const ETHERDELTA_CANCEL_LOG: &'static str = "0x1e0b760c386003e9cb9bcf4fcf3997886042859d9b6ed6320e804597fcdb28b0";
pub const ETHERDELTA_DEPOSIT_LOG: &'static str = "0xdcbc1c05240f31ff3ad067ef1ee35ce4997762752e3a095284754544f4c709d7";
pub const ETHERDELTA_TRADE_LOG: &'static str = "0x6effdda786735d5033bfad5f53e5131abcced9e52be6c507b62d639685fbed6d";
pub const ETHERDELTA_WITHDRAW_LOG: &'static str = "0xf341246adaac6f497bc2a656f546ab9e182111d630394f0c57c710a59a2cb567";
