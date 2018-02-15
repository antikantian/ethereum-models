use std::str::FromStr;

use bigdecimal::{BigDecimal, ParseBigDecimalError};
use ethereum_types::U256;
use num::Num;

/// Used to convert between canonical Ethereum units of measure.
pub enum EthereumUnit {
    Wei(BigDecimal),
    Kwei(BigDecimal),
    Mwei(BigDecimal),
    Gwei(BigDecimal),
    Szabo(BigDecimal),
    Finney(BigDecimal),
    Ether(BigDecimal),
    Kether(BigDecimal),
    Mether(BigDecimal),
    Gether(BigDecimal),
    Tether(BigDecimal)
}

impl EthereumUnit {
    pub fn wei_factor(&self) -> BigDecimal {
        let factor = match *self {
            EthereumUnit::Wei(_) => 0,
            EthereumUnit::Kwei(_) => 3,
            EthereumUnit::Mwei(_) => 6,
            EthereumUnit::Gwei(_) => 9,
            EthereumUnit::Szabo(_) => 12,
            EthereumUnit::Finney(_) => 15,
            EthereumUnit::Ether(_) => 18,
            EthereumUnit::Kether(_) => 21,
            EthereumUnit::Mether(_) => 24,
            EthereumUnit::Gether(_) => 27,
            EthereumUnit::Tether(_) => 30
        };
        BigDecimal::from(1.0 * 10f32.powi(factor as i32))
    }

    pub fn value(&self) -> &BigDecimal {
        match *self {
            EthereumUnit::Wei(ref n) => n,
            EthereumUnit::Kwei(ref n) => n,
            EthereumUnit::Mwei(ref n) => n,
            EthereumUnit::Gwei(ref n) => n,
            EthereumUnit::Szabo(ref n) => n,
            EthereumUnit::Finney(ref n) => n,
            EthereumUnit::Ether(ref n) => n,
            EthereumUnit::Kether(ref n) => n,
            EthereumUnit::Mether(ref n) => n,
            EthereumUnit::Gether(ref n) => n,
            EthereumUnit::Tether(ref n) => n
        }
    }

    pub fn from_wei(amount: &U256) -> Result<EthereumUnit, ParseBigDecimalError> {
        BigDecimal::from_str(&format!("{:?}", &amount))
            .map(|n| EthereumUnit::Wei(n))
    }

    pub fn to_wei(&self) -> EthereumUnit {
        EthereumUnit::Wei(self.value() * self.wei_factor())
    }

    pub fn to_kwei(&self) -> EthereumUnit {
        match *self {
            EthereumUnit::Kwei(ref n) => EthereumUnit::Kwei(n.clone()),
            _ => {
                EthereumUnit::Kwei(
                    self.to_wei().value() / BigDecimal::from(1.0 * 10_f32.powi(3i32))
                )
            }
        }
    }

    pub fn to_mwei(&self) -> EthereumUnit {
        match *self {
            EthereumUnit::Mwei(ref n) => EthereumUnit::Mwei(n.clone()),
            _ => {
                EthereumUnit::Mwei(
                    self.to_wei().value() / BigDecimal::from(1.0 * 10_f32.powi(6i32))
                )
            }
        }
    }

    pub fn to_gwei(&self) -> EthereumUnit {
        match *self {
            EthereumUnit::Gwei(ref n) => EthereumUnit::Gwei(n.clone()),
            _ => {
                EthereumUnit::Gwei(
                    self.to_wei().value() / BigDecimal::from(1.0 * 10_f32.powi(9i32))
                )
            }
        }
    }

    pub fn to_szabo(&self) -> EthereumUnit {
        match *self {
            EthereumUnit::Szabo(ref n) => EthereumUnit::Szabo(n.clone()),
            _ => {
                EthereumUnit::Szabo(
                    self.to_wei().value() / BigDecimal::from(1.0 * 10_f32.powi(12i32))
                )
            }
        }
    }

    pub fn to_finney(&self) -> EthereumUnit {
        match *self {
            EthereumUnit::Finney(ref n) => EthereumUnit::Finney(n.clone()),
            _ => {
                EthereumUnit::Finney(
                    self.to_wei().value() / BigDecimal::from(1.0 * 10_f32.powi(15i32))
                )
            }
        }
    }

    pub fn to_ether(&self) -> EthereumUnit {
        match *self {
            EthereumUnit::Ether(ref n) => EthereumUnit::Ether(n.clone()),
            _ => {
                EthereumUnit::Ether(
                    self.to_wei().value() / BigDecimal::from(1.0 * 10_f32.powi(18i32))
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use bigdecimal::{BigDecimal, ParseBigDecimalError};
    use ethereum_types::U256;

    use super::EthereumUnit;

    #[test]
    fn u256_into_wei() {
        let wei_u256 = U256::from(1000000000000000000u64);
        let eu_wei = EthereumUnit::from_wei(&wei_u256).unwrap();
        assert_eq!(*eu_wei.value(), BigDecimal::from(1000000000000000000u64))
    }

    #[test]
    fn converts_to_wei() {
        let one_ether_in_wei = EthereumUnit::Wei(BigDecimal::from(1000000000000000000u64));
        let one_ether_in_ether = EthereumUnit::Ether(BigDecimal::from(1));
        assert_eq!(one_ether_in_ether.to_wei().value(), one_ether_in_wei.value())
    }

    #[test]
    fn converts_to_kwei() {
        let one_ether_in_wei = EthereumUnit::Wei(BigDecimal::from(1000000000000000000u64));
        let one_ether_in_kwei = EthereumUnit::Kwei(BigDecimal::from(1000000000000000u64));
        assert_eq!(one_ether_in_wei.to_kwei().value(), one_ether_in_kwei.value())
    }

    #[test]
    fn converts_to_mwei() {
        let one_ether_in_wei = EthereumUnit::Wei(BigDecimal::from(1000000000000000000u64));
        let one_ether_in_mwei = EthereumUnit::Mwei(BigDecimal::from(1000000000000u64));
        assert_eq!(one_ether_in_wei.to_mwei().value(), one_ether_in_mwei.value())
    }

    #[test]
    fn converts_to_gwei() {
        let one_ether_in_wei = EthereumUnit::Wei(BigDecimal::from(1000000000000000000u64));
        let one_ether_in_gwei = EthereumUnit::Gwei(BigDecimal::from(1000000000u64));
        assert_eq!(one_ether_in_wei.to_gwei().value(), one_ether_in_gwei.value())
    }

    #[test]
    fn converts_to_szabo() {
        let one_ether_in_wei = EthereumUnit::Wei(BigDecimal::from(1000000000000000000u64));
        let one_ether_in_szabo = EthereumUnit::Szabo(BigDecimal::from(1000000u64));
        assert_eq!(one_ether_in_wei.to_szabo().value(), one_ether_in_szabo.value())
    }

    #[test]
    fn converts_to_finney() {
        let one_ether_in_wei = EthereumUnit::Wei(BigDecimal::from(1000000000000000000u64));
        let one_ether_in_finney = EthereumUnit::Finney(BigDecimal::from(1000u64));
        assert_eq!(one_ether_in_wei.to_finney().value(), one_ether_in_finney.value())
    }

    #[test]
    fn converts_to_ether() {
        let one_ether_in_wei = EthereumUnit::Wei(BigDecimal::from(1000000000000000000u64));
        let one_ether_in_ether = EthereumUnit::Ether(BigDecimal::from(1));
        assert_eq!(one_ether_in_wei.to_ether().value(), one_ether_in_ether.value())
    }

    #[test]
    fn converts_to_kwei_with_decimals() {
        let value_in_wei = EthereumUnit::Wei(BigDecimal::from_str("189432878943289").unwrap());
        let value_in_kwei = EthereumUnit::Kwei(BigDecimal::from_str("189432878943.289").unwrap());
        assert_eq!(value_in_wei.to_kwei().value(), value_in_kwei.value())
    }

    #[test]
    fn converts_to_mwei_with_decimals() {
        let value_in_wei = EthereumUnit::Wei(BigDecimal::from_str("189432878943289").unwrap());
        let value_in_mwei = EthereumUnit::Mwei(BigDecimal::from_str("189432878.943289").unwrap());
        assert_eq!(value_in_wei.to_mwei().value(), value_in_mwei.value())
    }

    #[test]
    fn converts_to_gwei_with_decimals() {
        let value_in_wei = EthereumUnit::Wei(BigDecimal::from_str("189432878943289").unwrap());
        let value_in_gwei = EthereumUnit::Gwei(BigDecimal::from_str("189432.878943289").unwrap());
        assert_eq!(value_in_wei.to_gwei().value(), value_in_gwei.value())
    }

    #[test]
    fn converts_to_szabo_with_decimals() {
        let value_in_wei = EthereumUnit::Wei(BigDecimal::from_str("189432878943289").unwrap());
        let value_in_szabo = EthereumUnit::Szabo(BigDecimal::from_str("189.432878943289").unwrap());
        assert_eq!(value_in_wei.to_szabo().value(), value_in_szabo.value())
    }

    #[test]
    fn converts_to_finney_with_decimals() {
        let value_in_wei = EthereumUnit::Wei(BigDecimal::from_str("189432878943289").unwrap());
        let value_in_finney = EthereumUnit::Finney(BigDecimal::from_str("0.189432878943289").unwrap());
        assert_eq!(value_in_wei.to_finney().value(), value_in_finney.value())
    }

    #[test]
    fn converts_to_ether_with_decimals() {
        let value_in_wei = EthereumUnit::Wei(BigDecimal::from_str("189432878943289").unwrap());
        let value_in_ether = EthereumUnit::Ether(BigDecimal::from_str("0.000189432878943289").unwrap());
        assert_eq!(value_in_wei.to_ether().value(), value_in_ether.value())
    }

    #[test]
    fn one_wei_converts_to_kwei() {
        let one_wei = EthereumUnit::Wei(BigDecimal::from(1u64));
        let one_wei_in_kwei = EthereumUnit::Kwei(BigDecimal::from_str("0.001").unwrap());
        assert_eq!(one_wei.to_kwei().value(), one_wei_in_kwei.value());
    }

    #[test]
    fn one_wei_converts_to_mwei() {
        let one_wei = EthereumUnit::Wei(BigDecimal::from(1u64));
        let one_wei_in_mwei = EthereumUnit::Mwei(BigDecimal::from_str("0.000001").unwrap());
        assert_eq!(one_wei.to_mwei().value(), one_wei_in_mwei.value());
    }

    #[test]
    fn one_wei_converts_to_gwei() {
        let one_wei = EthereumUnit::Wei(BigDecimal::from(1u64));
        let one_wei_in_gwei = EthereumUnit::Gwei(BigDecimal::from_str("0.000000001").unwrap());
        assert_eq!(one_wei.to_gwei().value(), one_wei_in_gwei.value());
    }

    #[test]
    fn one_wei_converts_to_szabo() {
        let one_wei = EthereumUnit::Wei(BigDecimal::from(1u64));
        let one_wei_in_szabo = EthereumUnit::Szabo(BigDecimal::from_str("0.000000000001").unwrap());
        assert_eq!(one_wei.to_szabo().value(), one_wei_in_szabo.value());
    }

    #[test]
    fn one_wei_converts_to_finney() {
        let one_wei = EthereumUnit::Wei(BigDecimal::from(1u64));
        let one_wei_in_finney = EthereumUnit::Finney(BigDecimal::from_str("0.000000000000001").unwrap());
        assert_eq!(one_wei.to_finney().value(), one_wei_in_finney.value());
    }

    #[test]
    fn one_wei_converts_to_ether() {
        let one_wei = EthereumUnit::Wei(BigDecimal::from(1u64));
        let one_wei_in_ether = EthereumUnit::Ether(BigDecimal::from_str("0.000000000000000001").unwrap());
        assert_eq!(one_wei.to_ether().value(), one_wei_in_ether.value());
    }

}