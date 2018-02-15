use types::U256;

/// ERC20 model for contracts implementing this standard
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ERC20 {
    /// Token name
    pub name: String,
    /// Token symbol
    pub symbol: String,
    /// Token decimals
    pub decimals: u8,
    /// Token total supply
    pub total_supply: U256,
    /// Link to the token's icon (not part of ERC20 standard.
    /// Used by Etherswap internally.
    pub icon_url: Option<String>
}