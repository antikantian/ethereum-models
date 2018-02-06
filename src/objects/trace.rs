use types::{H160, H256, U256};

#[derive(Serialize, Deserialize)]
pub enum CallType {
    #[serde(rename = "call")]
    Call,
    #[serde(rename = "staticcall")]
    StaticCall
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ParityTrace {
    pub action: Action,
    pub block_hash: H256,
    pub block_number: u64,
    pub result: Option<ActionResult>,
    pub subtraces: u8,
    pub trace_address: Vec<u8>,
    pub transaction_hash: H256,
    pub transaction_position: u64,
    #[serde(rename = "type")]
    pub action_type: CallType
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Action {
    pub call_type: CallType,
    pub from: H160,
    pub gas: U256,
    pub input: String,
    pub to: H160,
    pub value: U256
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ActionResult {
    pub gas_used: U256,
    pub output: String
}