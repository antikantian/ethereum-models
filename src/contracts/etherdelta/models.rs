use contracts::{ContractFunction, NamedFunction};
use ::objects::*;
use ::types::*;

type Amount = U256;
type Balance = U256;
type Token = H160;
type User = H160;
type Sender = H160;

pub struct EtherDeltaTransaction {
    pub tx: Transaction,
    pub receipt: TransactionReceipt,
    pub action: EtherDeltaAction,
    pub event: Option<EtherDeltaEvent>
}

impl EtherDeltaTransaction {
    pub fn is_success(&self) -> bool { self.event.is_some() }
}

pub struct EtherDeltaProxyTransaction {
    pub tx: Transaction,
    pub receipt: TransactionReceipt,
    pub actions: Vec<(ParityTrace, Option<EtherDeltaAction>)>,
    pub events: Vec<EtherDeltaEvent>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EtherDeltaAction {
    CancelOrder(OrderData),
    Deposit(Amount),
    DepositToken(Token, Amount),
    Trade(OrderData, User, Amount),
    Withdraw(Amount),
    WithdrawToken(Token, Amount),
    AmountFilled(OrderData, User),
    AvailableVolume(OrderData, User),
    TestTrade(OrderData, User, Amount, Sender),
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
                "EtherDelta.withdrawToken".to_string()),
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

#[derive(Debug, Serialize, Deserialize)]
pub enum EtherDeltaEvent {
    Cancel(OrderData, User),
    Deposit(Token, User, Amount, Balance),
    Order(OrderLog),
    Trade(TradeLog),
    Withdraw(Token, User, Amount, Balance),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderData {
    pub token_get: H160,
    pub amount_get: U256,
    pub token_give: H160,
    pub amount_give: U256,
    pub expires: U256,
    pub nonce: U256,
    pub v: U256,
    pub r: String,
    pub s: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderLog {
    pub token_get: H160,
    pub amount_get: U256,
    pub token_give: H160,
    pub amount_give: U256,
    pub expires: U256,
    pub nonce: U256,
    pub user: H160,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeLog {
    pub token_get: H160,
    pub amount_get: U256,
    pub token_give: H160,
    pub amount_give: U256,
    pub maker: H160,
    pub taker: H160,
    pub price: f64
}