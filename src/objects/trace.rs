use std::hash::{Hash, Hasher};

use twox_hash::XxHash;

use types::{H160, H256, U256};

#[derive(Hash, Debug, Clone, Serialize, Deserialize)]
pub enum CallType {
    #[serde(rename = "call")]
    Call,
    #[serde(rename = "delegatecall")]
    DelegateCall,
    #[serde(rename = "staticcall")]
    StaticCall,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "suicide")]
    Suicide
}

#[derive(Hash, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ParityTrace {
    pub action: Action,
    pub block_hash: H256,
    pub block_number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ActionResult>,
    pub subtraces: u8,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub trace_address: Vec<u8>,
    pub transaction_hash: H256,
    pub transaction_position: u64,
    #[serde(rename = "type")]
    pub action_type: CallType
}

impl ParityTrace {
    pub fn to_hash(&self) -> u64 {
        let mut hasher = XxHash::default();
        self.hash(&mut hasher);
        hasher.finish()
    }

    pub fn hash_action(&self) -> u64 {
        let mut hasher = XxHash::default();
        let hash_string = format!(
            "{:?}{}{:?}{:?}",
            &self.transaction_hash, &self.subtraces, &self.trace_address, &self.action
        );
        hash_string.hash(&mut hasher);
        hasher.finish()
    }

    pub fn hash_action_input(&self) -> u64 {
        let mut hasher = XxHash::default();
        let hash_string = format!(
            "{:?}{}{:?}{}",
            &self.transaction_hash, &self.subtraces, &self.trace_address, &self.action.input
        );
        hash_string.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Hash, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Action {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_type: Option<CallType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<H160>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_address: Option<H160>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<U256>,
    #[serde(default)]
    pub input: String,
    #[serde(default)]
    pub init: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<H160>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<U256>
}

#[derive(Hash, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ActionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<H160>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub gas_used: U256,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>
}

#[cfg(test)]
mod tests {
    use serde_json;
    use super::ParityTrace;

    #[test]
    fn decodes_traces() {
        let traces = include_str!("../../test_data/parity_trace.json");
        let decoded_traces = serde_json::from_str::<Vec<ParityTrace>>(&traces);
    }
}