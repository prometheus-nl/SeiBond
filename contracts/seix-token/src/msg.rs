use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_supply: u128,
    pub minter: Option<String>,  // Optional minter address
}

#[cw_serde]
pub enum ExecuteMsg {
    Transfer { recipient: String, amount: u128 },
    Burn { amount: u128 },
    Mint { recipient: String, amount: u128 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // Get balance of a specific address
    #[returns(BalanceResponse)]
    Balance { address: String },

    // Get token info
    #[returns(TokenInfoResponse)]
    TokenInfo {},
}

// Response for Balance query
#[cw_serde]
pub struct BalanceResponse {
    pub balance: u128,
}

// Response for TokenInfo query
#[cw_serde]
pub struct TokenInfoResponse {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
}
