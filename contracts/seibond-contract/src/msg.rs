use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Uint128, Addr};

// InstantiateMsg defines the parameters for initializing the contract
#[cw_serde]
pub struct InstantiateMsg {
    pub seix_contract: String,  // The CW20 contract address for the SeiX token
}

// ExecuteMsg defines the different actions the contract can perform
#[cw_serde]
pub enum ExecuteMsg {
    MintBond {
        face_value: Uint128,
        interest_rate: u64,
        maturity: u64,
    },
    TransferBond {
        bond_id: String,
        new_holder: Addr,
    },
    RedeemBond {
        bond_id: String,
    },
}

// QueryMsg defines the queries that can be made to the contract
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetBond returns the details of a bond given its ID
    #[returns(BondResponse)]
    GetBond {
        bond_id: String,
    },
}

// Define the response structure for bond details
#[cw_serde]
pub struct BondResponse {
    pub issuer: Addr,
    pub face_value: Uint128,
    pub interest_rate: u64,
    pub maturity: u64,
    pub holder: Addr,
    pub issued_at: u64,
}
