use types::{H160, U256};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionCall {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<H160>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<H160>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>
}

impl TransactionCall {
    pub fn empty() -> Self {
        TransactionCall {
            from: None,
            to: None,
            gas: None,
            gas_price: None,
            value: None,
            data: None
        }
    }

    pub fn from(mut self, v: H160) -> Self {
        self.from = Some(v);
        self
    }

    pub fn to(mut self, v: H160) -> Self {
        self.to = Some(v);
        self
    }

    pub fn gas(mut self, v: U256) -> Self {
        self.gas = Some(v);
        self
    }

    pub fn gas_price(mut self, v: U256) -> Self {
        self.gas_price = Some(v);
        self
    }

    pub fn value(mut self, v: U256) -> Self {
        self.value = Some(v);
        self
    }

    pub fn data(mut self, v: &str) -> Self {
        self.data = Some(v.to_string());
        self
    }

    pub fn done(self) -> Self {
        self
    }

}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use types::{H160, U256};
    use super::TransactionCall;

    #[test]
    fn transaction_call_builds() {
        let tc = TransactionCall::empty()
            .from(H160::from_str("a94f5374fce5edbc8e2a8697c15331677e6ebf0b").unwrap())
            .to(H160::from_str("a94f5374fce5edbc8e2a8697c15331677e6ebf0b").unwrap())
            .gas(U256::from(100_u64))
            .gas_price(U256::from(100_u64))
            .value(U256::from(100_u64))
            .data("0x")
            .done();

        assert_eq!(
            tc.from, Some(H160::from_str("a94f5374fce5edbc8e2a8697c15331677e6ebf0b").unwrap())
        );

        assert_eq!(
            tc.to, Some(H160::from_str("a94f5374fce5edbc8e2a8697c15331677e6ebf0b").unwrap())
        );

        assert_eq!(
            tc.gas, Some(U256::from(100_u64))
        );

        assert_eq!(
            tc.gas_price, Some(U256::from(100_u64))
        );

        assert_eq!(
            tc.data, Some("0x".to_string())
        );
    }
}