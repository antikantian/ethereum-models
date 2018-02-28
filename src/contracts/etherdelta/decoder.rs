use std::str::{self, FromStr};

use fixed_hash::clean_0x;
use serde_json;

use contracts::normalize_data;
use error::{Error, ErrorKind};
use objects::{Log, ParityTrace, Transaction};
use types::{H160, H256, U256};

use super::models::*;
use super::constants::*;

/// Decoder for all actions, events, and traces to/from the EtherDelta smart contract.
pub struct EtherDeltaDecoder;

impl EtherDeltaDecoder {
    pub fn decode_action(tx: &Transaction) -> Result<EtherDeltaAction, Error> {
        let input = &tx.input;
        if input.len() < 10 {
            Err(ErrorKind::Decoder("Transaction input length < 10".to_string()).into())
        } else {
            let method_id = &input[..10];
            match &*method_id {
                CANCEL_ORDER_ID => EtherDeltaDecoder::decode_cancel_order(&tx),
                DEPOSIT_ID => Ok(EtherDeltaAction::Deposit(tx.value.clone())),
                DEPOSIT_TOKEN_ID => EtherDeltaDecoder::decode_deposit_token(&tx),
                TRADE_ID => EtherDeltaDecoder::decode_trade(&tx),
                WITHDRAW_ID => EtherDeltaDecoder::decode_withdraw(&tx),
                WITHDRAW_TOKEN_ID => EtherDeltaDecoder::decode_withdraw_token(&tx),
                _ => Err(ErrorKind::Decoder(format!("Invalid method: {}", &method_id)).into())
            }
        }
    }

    pub fn decode_trace_action(trace: &ParityTrace) -> Result<EtherDeltaAction, Error> {
        let input = &trace.action.input;
        if input.len() < 10 {
            Err(ErrorKind::Decoder("Trace action input length < 10".to_string()).into())
        } else {
            let method_id = &input[..10];
            match &*method_id {
                CANCEL_ORDER_ID => EtherDeltaDecoder::decode_cancel_order_id(&input),
                DEPOSIT_ID => Ok(EtherDeltaAction::Deposit(trace.action.value.unwrap().clone())),
                DEPOSIT_TOKEN_ID => EtherDeltaDecoder::decode_deposit_token_id(&input),
                TRADE_ID => EtherDeltaDecoder::decode_trade_id(&input),
                WITHDRAW_ID => EtherDeltaDecoder::decode_withdraw_id(&input),
                WITHDRAW_TOKEN_ID => EtherDeltaDecoder::decode_withdraw_token_id(&input),
                AMOUNT_FILLED_ID => EtherDeltaDecoder::decode_amount_filled(&trace),
                AVAILABLE_VOLUME_ID => EtherDeltaDecoder::decode_available_volume(&trace),
                TEST_TRADE_ID => EtherDeltaDecoder::decode_test_trade(&trace),
                BALANCE_OF_ID => EtherDeltaDecoder::decode_balance_of(&trace),
                _ => Err(ErrorKind::Decoder(
                    format!("[Tx: {:?}] Unknown trace input: {}", &trace.transaction_hash, &method_id)
                ).into())
            }
        }
    }

    pub fn decode_event(log: &Log) -> Result<EtherDeltaEvent, Error> {
        log.topics.get(0)
            .ok_or(ErrorKind::Decoder("Expected log topic vec of length (1)".to_string()).into())
            .and_then(|topic| {
                serde_json::to_string(&topic)
                    .map_err(|e| ErrorKind::Decoder("Couldn't decode log topic".to_string()).into())
                    .and_then(|hash| {
                        let h = hash.replace(r#"""#, "");
                        match h.as_ref() {
                            ETHERDELTA_CANCEL_LOG => EtherDeltaDecoder::decode_cancel_log(&log),
                            ETHERDELTA_TRADE_LOG => EtherDeltaDecoder::decode_trade_log(&log),
                            ETHERDELTA_DEPOSIT_LOG => EtherDeltaDecoder::decode_transfer_log(&topic, &log),
                            ETHERDELTA_WITHDRAW_LOG => EtherDeltaDecoder::decode_transfer_log(&topic, &log),
                            _ => Err(ErrorKind::Decoder(format!("Invalid log topic: {:?}", &topic)).into())
                        }
                    })
            })
    }

    fn decode_cancel_log(log: &Log) -> Result<EtherDeltaEvent, Error> {
        let data = log.data.to_owned();
        let data = clean_0x(&data);
        let expected_length = 64 * 10;
        if data.len() != expected_length {
            Err(ErrorKind::Decoder(format!("Couldn't decode cancel log: {}", &data)).into())
        } else {
            let fields = data.as_bytes()
                .chunks(64 as usize)
                .map(|buf| unsafe { str::from_utf8_unchecked(buf).to_string() })
                .collect::<Vec<String>>();

            Ok(EtherDeltaEvent::Cancel(
                OrderData {
                    token_get: H160::from_str(&fields[0][24..]).unwrap(),
                    amount_get: U256::from_str(&fields[1]).unwrap(),
                    token_give: H160::from_str(&fields[2][24..]).unwrap(),
                    amount_give: U256::from_str(&fields[3]).unwrap(),
                    expires: U256::from_str(&fields[4]).unwrap(),
                    nonce: U256::from_str(&fields[5]).unwrap(),
                    v: U256::from_str(&fields[7]).unwrap(),
                    r: fields[8].to_string(),
                    s: fields[9].to_string()
                },
                H160::from_str(&fields[6][24..]).unwrap()
            ))
        }
    }

    fn decode_transfer_log(topic: &H256, log: &Log) -> Result<EtherDeltaEvent, Error> {
        let data = log.data.to_owned();
        let data = clean_0x(&data);
        let expected_length = 64 * 4;
        if data.len() != expected_length {
            Err(ErrorKind::Decoder(format!("Couldn't decode transfer log: {}", &data)).into())
        } else {
            let fields = data.as_bytes()
                .chunks(64 as usize)
                .map(|buf| unsafe { str::from_utf8_unchecked(buf).to_string() })
                .collect::<Vec<String>>();

            let token = H160::from_str(&fields[0][24..]).unwrap();
            let user = H160::from_str(&fields[1][24..]).unwrap();
            let amount = U256::from_str(&fields[2]).unwrap();
            let balance = U256::from_str(&fields[3]).unwrap();

            if topic == &*ETHERDELTA_DEPOSIT_TOPIC {
                Ok(EtherDeltaEvent::Deposit(token, user, amount, balance))
            } else if topic == &*ETHERDELTA_WITHDRAW_TOPIC {
                Ok(EtherDeltaEvent::Withdraw(token, user, amount, balance))
            } else {
                Err(ErrorKind::Decoder(format!("Couldn't decode transfer log: {}", &data)).into())
            }
        }
    }

    fn decode_order_log(log: &Log) -> Result<EtherDeltaEvent, Error> {
        let data = log.data.to_owned();
        let data = clean_0x(&data);
        let expected_length = 64 * 7;
        if data.len() != expected_length {
            Err(ErrorKind::Decoder(format!("Couldn't decode order log: {}", &data)).into())
        } else {
            let fields = data.as_bytes()
                .chunks(64 as usize)
                .map(|buf| unsafe { str::from_utf8_unchecked(buf).to_string() })
                .collect::<Vec<String>>();

            Ok(EtherDeltaEvent::Order(
                OrderLog {
                    token_get: H160::from_str(&fields[0][24..]).unwrap(),
                    amount_get: U256::from_str(&fields[1]).unwrap(),
                    token_give: H160::from_str(&fields[2][24..]).unwrap(),
                    amount_give: U256::from_str(&fields[3]).unwrap(),
                    expires: U256::from_str(&fields[4]).unwrap(),
                    nonce: U256::from_str(&fields[5]).unwrap(),
                    user: H160::from_str(&fields[6][24..]).unwrap()
                }
            ))
        }
    }

    fn decode_trade_log(log: &Log) -> Result<EtherDeltaEvent, Error> {
        let data = log.data.to_owned();
        let data = clean_0x(&data);
        let expected_length = 64 * 6;
        if data.len() != expected_length {
            Err(ErrorKind::Decoder(format!("Couldn't decode trade log: {}", &data)).into())
        } else {
            let fields = data.as_bytes()
                .chunks(64 as usize)
                .map(|buf| unsafe { str::from_utf8_unchecked(buf).to_string() })
                .collect::<Vec<String>>();

            Ok(EtherDeltaEvent::Trade(
                TradeLog {
                    token_get: H160::from_str(&fields[0][24..]).unwrap(),
                    amount_get: U256::from_str(&fields[1]).unwrap(),
                    token_give: H160::from_str(&fields[2][24..]).unwrap(),
                    amount_give: U256::from_str(&fields[3]).unwrap(),
                    maker: H160::from_str(&fields[4][24..]).unwrap(),
                    taker: H160::from_str(&fields[5][24..]).unwrap(),
                    price: 0.0
                }
            ))
        }
    }

    fn decode_cancel_order(tx: &Transaction) -> Result<EtherDeltaAction, Error> {
        EtherDeltaDecoder::decode_cancel_order_id(&tx.input)
    }

    fn decode_cancel_order_id(input: &str) -> Result<EtherDeltaAction, Error> {
        match normalize_data(&input, 9) {
            Ok(fields) => {
                Ok(EtherDeltaAction::CancelOrder(
                    OrderData {
                        token_get: H160::from_str(&fields[0][24..]).unwrap(),
                        amount_get: U256::from_str(&fields[1]).unwrap(),
                        token_give: H160::from_str(&fields[2][24..]).unwrap(),
                        amount_give: U256::from_str(&fields[3]).unwrap(),
                        expires: U256::from_str(&fields[4]).unwrap(),
                        nonce: U256::from_str(&fields[5]).unwrap(),
                        v: U256::from_str(&fields[6]).unwrap(),
                        r: fields[7].to_string(),
                        s: fields[8].to_string()
                    }
                ))
            },
            Err(e) => Err(e)
        }
    }

    fn decode_withdraw(tx: &Transaction) -> Result<EtherDeltaAction, Error> {
        EtherDeltaDecoder::decode_withdraw_id(&tx.input)
    }

    fn decode_withdraw_id(input: &str) -> Result<EtherDeltaAction, Error> {
        match normalize_data(&input, 1) {
            Ok(fields) => Ok(EtherDeltaAction::Withdraw(U256::from_str(&fields[0]).unwrap())),
            Err(e) => Err(e)
        }
    }

    fn decode_deposit_token(tx: &Transaction) -> Result<EtherDeltaAction, Error> {
        EtherDeltaDecoder::decode_deposit_token_id(&tx.input)
    }

    fn decode_deposit_token_id(input: &str) -> Result<EtherDeltaAction, Error> {
        match normalize_data(&input, 2) {
            Ok(fields) => {
                Ok(EtherDeltaAction::DepositToken(
                    H160::from_str(&fields[0][24..]).unwrap(),
                    U256::from_str(&fields[1]).unwrap()
                ))
            },
            Err(e) => Err(e)
        }
    }

    fn decode_withdraw_token(tx: &Transaction) -> Result<EtherDeltaAction, Error> {
        EtherDeltaDecoder::decode_withdraw_token_id(&tx.input)
    }

    fn decode_withdraw_token_id(input: &str) -> Result<EtherDeltaAction, Error> {
        match normalize_data(&input, 2) {
            Ok(fields) => {
                Ok(EtherDeltaAction::WithdrawToken(
                    H160::from_str(&fields[0][24..]).unwrap(),
                    U256::from_str(&fields[1]).unwrap()
                ))
            },
            Err(e) => Err(e)
        }
    }

    fn decode_trade(tx: &Transaction) -> Result<EtherDeltaAction, Error> {
        EtherDeltaDecoder::decode_trade_id(&tx.input)
    }

    fn decode_trade_id(input: &str) -> Result<EtherDeltaAction, Error> {
        match normalize_data(&input, 11) {
            Ok(fields) => {
                Ok(EtherDeltaAction::Trade(
                    OrderData {
                        token_get: H160::from_str(&fields[0][24..]).unwrap(),
                        amount_get: U256::from_str(&fields[1]).unwrap(),
                        token_give: H160::from_str(&fields[2][24..]).unwrap(),
                        amount_give: U256::from_str(&fields[3]).unwrap(),
                        expires: U256::from_str(&fields[4]).unwrap(),
                        nonce: U256::from_str(&fields[5]).unwrap(),
                        v: U256::from_str(&fields[7]).unwrap(),
                        r: fields[8].to_string(),
                        s: fields[9].to_string(),
                    },
                    H160::from_str(&fields[6][24..]).unwrap(),
                    U256::from_str(&fields[10]).unwrap()
                ))
            },
            Err(e) => Err(e)
        }
    }

    fn decode_amount_filled(trace: &ParityTrace) -> Result<EtherDeltaAction, Error> {
        match normalize_data(&trace.action.input, 10) {
            Ok(fields) => {
                Ok(EtherDeltaAction::AmountFilled(
                    OrderData {
                        token_get: H160::from_str(&fields[0][24..]).unwrap(),
                        amount_get: U256::from_str(&fields[1]).unwrap(),
                        token_give: H160::from_str(&fields[2][24..]).unwrap(),
                        amount_give: U256::from_str(&fields[3]).unwrap(),
                        expires: U256::from_str(&fields[4]).unwrap(),
                        nonce: U256::from_str(&fields[5]).unwrap(),
                        v: U256::from_str(&fields[7]).unwrap(),
                        r: fields[8].to_string(),
                        s: fields[9].to_string(),
                    },
                    H160::from_str(&fields[6][24..]).unwrap()
                ))
            },
            Err(e) => Err(e)
        }
    }

    fn decode_available_volume(trace: &ParityTrace) -> Result<EtherDeltaAction, Error> {
        match EtherDeltaDecoder::decode_amount_filled(&trace) {
            Ok(EtherDeltaAction::AmountFilled(data, user)) => {
                Ok(EtherDeltaAction::AvailableVolume(data, user))
            },
            Err(e) => Err(e),
            _ => Err(ErrorKind::Decoder(format!("Invalid input: {}", &trace.action.input)).into())
        }
    }

    fn decode_test_trade(trace: &ParityTrace) -> Result<EtherDeltaAction, Error> {
        match normalize_data(&trace.action.input, 12) {
            Ok(fields) => {
                Ok(EtherDeltaAction::TestTrade(
                    OrderData {
                        token_get: H160::from_str(&fields[0][24..]).unwrap(),
                        amount_get: U256::from_str(&fields[1]).unwrap(),
                        token_give: H160::from_str(&fields[2][24..]).unwrap(),
                        amount_give: U256::from_str(&fields[3]).unwrap(),
                        expires: U256::from_str(&fields[4]).unwrap(),
                        nonce: U256::from_str(&fields[5]).unwrap(),
                        v: U256::from_str(&fields[7]).unwrap(),
                        r: fields[8].to_string(),
                        s: fields[9].to_string(),
                    },
                    H160::from_str(&fields[6][24..]).unwrap(),
                    U256::from_str(&fields[10]).unwrap(),
                    H160::from_str(&fields[11][24..]).unwrap()
                ))
            },
            Err(e) => Err(e)
        }
    }

    fn decode_balance_of(trace: &ParityTrace) -> Result<EtherDeltaAction, Error> {
        match normalize_data(&trace.action.input, 2) {
            Ok(fields) => {
                Ok(EtherDeltaAction::BalanceOf(
                    H160::from_str(&fields[0][24..]).unwrap(),
                    H160::from_str(&fields[1][24..]).unwrap()
                ))
            },
            Err(e) => Err(e)
        }
    }

}

