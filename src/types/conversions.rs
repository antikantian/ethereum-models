use ethereum_types::U256;

pub enum Unit {
    Wei(U256),
    Gwei(f64),
    Ether(f64)
}