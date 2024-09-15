use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

// Define the structure for each Bond
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Bond {
    pub issuer: Addr,           // Address of the issuer
    pub face_value: Uint128,     // Face value of the bond
    pub interest_rate: u64,      // Interest rate (in percentage)
    pub maturity: u64,           // Maturity period (in seconds or days)
    pub holder: Addr,            // Current holder of the bond
    pub issued_at: u64,          // When the bond was issued (timestamp)
}

// Store all bonds as a map of bond ID -> Bond struct
pub const BONDS: Map<String, Bond> = Map::new("bonds");

// Store a counter to track bond IDs
pub const BOND_COUNT: Item<u64> = Item::new("bond_count");

// Store the SeiX contract address
pub const SEIX_CONTRACT: Item<Addr> = Item::new("seix_contract");
