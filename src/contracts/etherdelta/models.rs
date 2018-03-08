use std::hash::{Hash, Hasher};

use twox_hash::XxHash;
use contracts::{ContractFunction, NamedFunction};
use super::constants::ETHERDELTA_ADDRESS;
use ::objects::*;
use ::types::*;

type Amount = U256;
type Balance = U256;
type Token = H160;
type User = H160;
type Sender = H160;

/// A transaction sent directly to the EtherDelta smart contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtherDeltaTransaction {
    /// The actual transaction.
    pub tx: Transaction,
    /// The transaction receipt.
    pub receipt: TransactionReceipt,
    /// The decoded Action specified by the transaction's input.
    pub action: EtherDeltaAction,
    /// The decoded logs/events (if any) that were generated after this transaction was executed
    /// in a block.  Note that this will be `None` if the transaction failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<EtherDeltaEvent>
}

impl EtherDeltaTransaction {
    /// Was this transactions successful?  If not, the transaction's event will be `None`
    pub fn is_success(&self) -> bool { self.event.is_some() }
}

/// Any contract transaction that, during the course of execution, interacts with the
/// EtherDelta smart contract, but was NOT sent directly to the contract address.
/// That is, another contract is being used as a proxy to call the EtherDelta
/// contract's methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtherDeltaProxyTransaction {
    /// The actual transaction.
    pub tx: Transaction,
    /// The transaction receipt.
    pub receipt: TransactionReceipt,
    /// The call chain produced by the transaction's trace.  The action is `None` in
    /// the case that the proxy contract calls methods outside of the EtherDelta contract.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<(ParityTrace, Option<EtherDeltaAction>)>,
    /// Any EtherDelta event generated after this transaction was executed.  This is not
    /// comprehensive, as a proxy contract might generate events external to EtherDelta.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<EtherDeltaEvent>
}

/// The action specified in a transaction's input.  This will either be a call to a constant
/// method, such as a balance check, or a call with change's to the EVM storage, like a trade.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EtherDeltaAction {
    /// A cancellation of an on-chain order.  Rarely seen.
    CancelOrder(OrderData),
    /// A deposit of ether to the smart contract.
    Deposit(Amount),
    /// A deposit of a compatible token to the smart contract.
    DepositToken(Token, Amount),
    /// A trade executed on-chain by an order's taker.
    Trade(OrderData, User, Amount),
    /// A withdrawal of ether from the smart contract.
    Withdraw(Amount),
    /// A withdrawal of any compatible token from the smart contract.
    WithdrawToken(Token, Amount),
    /// Constant call returning the amount of an order that has already been filled.
    AmountFilled(OrderData, User),
    /// Constant call returning the amount of an order that is available (has yet to be filled).
    AvailableVolume(OrderData, User),
    /// Constant call checking whether or not a trade will succeed upon on-chain execution.
    TestTrade(OrderData, User, Amount, Sender),
    /// Balance of any address with funds (either in ether or a compatible token) in the contract.
    /// If checking an address' ether balance, the token will be `0x0...0`.
    BalanceOf(Token, User)
}

impl NamedFunction for EtherDeltaAction {
    fn get_function(&self) -> ContractFunction {
        match *self {
            EtherDeltaAction::CancelOrder(_) => ContractFunction::Mutable(
                "EtherDelta.cancelOrder".to_string()
            ),
            EtherDeltaAction::Deposit(_) => ContractFunction::Mutable(
                "EtherDelta.deposit".to_string()
            ),
            EtherDeltaAction::DepositToken(_, _) => ContractFunction::Mutable(
                "EtherDelta.depositToken".to_string()
            ),
            EtherDeltaAction::Trade(_, _, _) => ContractFunction::Mutable(
                "EtherDelta.trade".to_string()
            ),
            EtherDeltaAction::Withdraw(_) => ContractFunction::Mutable(
                "EtherDelta.withdraw".to_string()
            ),
            EtherDeltaAction::WithdrawToken(_, _) => ContractFunction::Mutable(
                "EtherDelta.withdrawToken".to_string()
            ),
            EtherDeltaAction::AmountFilled(_, _) => ContractFunction::Immutable(
                "EtherDelta.amountFilled".to_string()
            ),
            EtherDeltaAction::AvailableVolume(_, _) => ContractFunction::Immutable(
                "EtherDelta.availableVolume".to_string()
            ),
            EtherDeltaAction::TestTrade(_, _, _, _) => ContractFunction::Immutable(
                "EtherDelta.testTrade".to_string()
            ),
            EtherDeltaAction::BalanceOf(_, _) => ContractFunction::Immutable(
                "EtherDelta.balanceOf".to_string()
            )
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EtherDeltaEvent {
    Cancel(OrderData, User),
    Deposit(Token, User, Amount, Balance),
    Order(OrderLog),
    Trade(TradeLog),
    Withdraw(Token, User, Amount, Balance),
}

/// The order data decoded either from a transaction's input, or from the transaction's log data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderData {
    /// The address of the token being received by the order's maker, given by the taker.
    /// Ether is specified by `0x0...0`.
    pub token_get: H160,
    /// The amount of ether/token received by the maker.
    pub amount_get: U256,
    /// The address of the token being received by the order's taker, given by the maker.
    pub token_give: H160,
    /// The amount of ether/token received by the taker.
    pub amount_give: U256,
    /// The block at which this order was set to expire.
    pub expires: U256,
    /// The order's nonce to disambiguate multiple identical orders.
    pub nonce: U256,
    /// `v` component of the signature data used in validation.  Used when the `trade`
    /// function calls `ecrecover`.
    pub v: U256,
    /// `r` component of the signature data used in validation.  Used when the `trade`
    /// function calls `ecrecover`.
    pub r: String,
    /// `s` component of the signature data used in validation.  Used when the `trade`
    /// function calls `ecrecover`.
    pub s: String,
}

impl OrderData {
    pub fn hash_order(&self) -> u64 {
        let mut hasher = XxHash::default();
        let hash_string = format!(
            "{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{}{}",
            &*ETHERDELTA_ADDRESS,
            &self.token_get,
            &self.amount_get,
            &self.token_give,
            &self.amount_give,
            &self.expires,
            &self.nonce,
            &self.v,
            &self.r,
            &self.s
        );
        hash_string.hash(&mut hasher);
        hasher.finish()
    }

    pub fn hash_trade(&self, tx_hash: &H256, maker: &H160, taker: &H160, amount: &U256) -> u64 {
        let mut hasher = XxHash::default();
        let hash_string = format!(
            // token_get, token_give, amount, maker, taker, tx_hash
            "{:?}{:?}{:?}{:?}{:?}{:?}",
            &self.token_get,
            &self.token_give,
            &amount,
            &maker,
            &taker,
            &tx_hash
        );
        hash_string.hash(&mut hasher);
        hasher.finish()
    }
}

/// The data decoded from successful on-chain calls to `order`.  Rarely seen.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLog {
    /// Same as in `OrderData`.
    pub token_get: H160,
    /// Same as in `OrderData`.
    pub amount_get: U256,
    /// Same as in `OrderData`.
    pub token_give: H160,
    /// Same as in `OrderData`.
    pub amount_give: U256,
    /// Same as in `OrderData`.
    pub expires: U256,
    /// Same as in `OrderData`.
    pub nonce: U256,
    /// The maker who placed the order either on- or off-chain.
    pub user: H160,
}

/// The data decoded from successful calls to `trade`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeLog {
    /// Same as in `OrderData`.
    pub token_get: H160,
    /// The amount received by the maker in this specific trade.  Could be less than the
    /// order amount in the case of partial fills.
    pub amount_get: U256,
    /// Same as in `OrderData`.
    pub token_give: H160,
    /// The amount received by the taker in this specific trade.  Could be less than the
    /// order amount in the case of partial fills.
    pub amount_give: U256,
    /// The order's maker.  `Get` as specified by the EtherDelta smart contract.  Renamed
    /// for clarity and normalization across all Etherswap exchange models.
    pub maker: H160,
    /// The order's taker.  `Give` as specified by the EtherDelta smart contract.  Renamed
    /// for clarity and normalization across all Etherswap exchange models.
    pub taker: H160,
    /// Not native to EtherDelta contract events.  This price is calculated during processing,
    /// since the calculation relies on knowledge of a token contract's `decimals` field.
    pub price: f64
}

impl TradeLog {
    pub fn hash_trade(&self, tx_hash: &H256) -> u64 {
        let mut hasher = XxHash::default();
        let hash_string = format!(
            // token_get, token_give, amount, maker, taker, tx_hash
            "{:?}{:?}{:?}{:?}{:?}{:?}",
            &self.token_get,
            &self.token_give,
            &self.amount_give,
            &self.maker,
            &self.taker,
            &tx_hash
        );
        hash_string.hash(&mut hasher);
        hasher.finish()
    }
}