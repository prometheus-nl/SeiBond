use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Track the balances of all users
pub const BALANCES: Map<&Addr, Uint128> = Map::new("balances");

// Track the total supply of tokens
pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");

// Optional minter (address that can mint more tokens)
pub const MINTER: Item<Addr> = Item::new("minter");

// Define a struct for token information
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
}
