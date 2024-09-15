#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdResult, Binary, Uint128, WasmMsg, Addr};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{BOND_COUNT, BONDS, SEIX_CONTRACT, Bond};
use cw2::set_contract_version;
use cw20::Cw20ExecuteMsg;

// version info for migration
const CONTRACT_NAME: &str = "crates.io:seibond-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Instantiate the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // Store the SeiX token contract address provided during instantiation
    SEIX_CONTRACT.save(deps.storage, &Addr::unchecked(msg.seix_contract))?;

    // Initialize the bond count
    BOND_COUNT.save(deps.storage, &0)?;

    // Set contract version info
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("seix_contract", msg.seix_contract))
}

// Execute contract methods
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::MintBond { face_value, interest_rate, maturity } => {
            mint_bond(deps, env, info, face_value, interest_rate, maturity)
        }
        ExecuteMsg::TransferBond { bond_id, new_holder } => {
            transfer_bond(deps, info, bond_id, new_holder)
        }
        ExecuteMsg::RedeemBond { bond_id } => {
            redeem_bond(deps, env, info, bond_id)
        }
    }
}

// Mint a new bond
pub fn mint_bond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    face_value: Uint128,
    interest_rate: u64,
    maturity: u64,
) -> StdResult<Response> {
    let bond_count = BOND_COUNT.load(deps.storage)?;
    let new_bond_id = bond_count + 1;

    // Load the SeiX contract address from state
    let seix_contract = SEIX_CONTRACT.load(deps.storage)?;

    // Collect platform fee in SeiX token using CW20 transfer
    let platform_fee = Uint128::new(10);  // Example fee amount

    let transfer_msg = Cw20ExecuteMsg::Transfer {
        recipient: "platform_treasury_address".to_string(),
        amount: platform_fee,
    };

    // Encode the CW20 transfer message to be sent
    let wasm_msg = WasmMsg::Execute {
        contract_addr: seix_contract.to_string(),
        msg: to_binary(&transfer_msg)?,
        funds: vec![],
    };

    let bond = Bond {
        issuer: info.sender.clone(),
        face_value,
        interest_rate,
        maturity,
        holder: info.sender.clone(),
        issued_at: env.block.time.seconds(),
    };

    // Save the new bond in storage
    BONDS.save(deps.storage, &new_bond_id.to_string(), &bond)?;

    // Increment bond count
    BOND_COUNT.save(deps.storage, &new_bond_id)?;

    Ok(Response::new()
        .add_message(wasm_msg)  // Send the CW20 transfer message
        .add_attribute("method", "mint_bond")
        .add_attribute("bond_id", new_bond_id.to_string()))
}

// Transfer an existing bond to a new holder
pub fn transfer_bond(
    deps: DepsMut,
    info: MessageInfo,
    bond_id: String,
    new_holder: Addr,
) -> StdResult<Response> {
    let mut bond = BONDS.load(deps.storage, &bond_id)?;

    // Ensure that the sender is the current bond holder
    if bond.holder != info.sender {
        return Err(StdError::generic_err("Unauthorized: Only the bond holder can transfer the bond"));
    }

    // Load the SeiX contract address from state
    let seix_contract = SEIX_CONTRACT.load(deps.storage)?;

    // Collect platform fee in SeiX token using CW20 transfer
    let platform_fee = Uint128::new(5);  // Example fee amount

    let transfer_msg = Cw20ExecuteMsg::Transfer {
        recipient: "platform_treasury_address".to_string(),
        amount: platform_fee,
    };

    // Encode the CW20 transfer message to be sent
    let wasm_msg = WasmMsg::Execute {
        contract_addr: seix_contract.to_string(),
        msg: to_binary(&transfer_msg)?,
        funds: vec![],
    };

    // Update the bond holder
    bond.holder = new_holder;
    BONDS.save(deps.storage, &bond_id, &bond)?;

    Ok(Response::new()
        .add_message(wasm_msg)  // Send the CW20 transfer message
        .add_attribute("method", "transfer_bond")
        .add_attribute("bond_id", bond_id))
}

// Redeem a bond
pub fn redeem_bond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    bond_id: String,
) -> StdResult<Response> {
    let bond = BONDS.load(deps.storage, &bond_id)?;

    // Ensure that the sender is the bond holder
    if bond.holder != info.sender {
        return Err(StdError::generic_err("Unauthorized: Only the bond holder can redeem the bond"));
    }

    // Ensure that the bond has matured
    let current_time = env.block.time.seconds();
    if current_time < bond.issued_at + bond.maturity {
        return Err(StdError::generic_err("Bond has not matured yet"));
    }

    // Load the SeiX contract address from state
    let seix_contract = SEIX_CONTRACT.load(deps.storage)?;

    // Collect platform fee in SeiX token using CW20 transfer
    let platform_fee = Uint128::new(15);  // Example fee amount

    let transfer_msg = Cw20ExecuteMsg::Transfer {
        recipient: "platform_treasury_address".to_string(),
        amount: platform_fee,
    };

    // Encode the CW20 transfer message to be sent
    let wasm_msg = WasmMsg::Execute {
        contract_addr: seix_contract.to_string(),
        msg: to_binary(&transfer_msg)?,
        funds: vec![],
    };

    // Optionally, transfer face value + interest to bond holder (in stablecoin)
    BONDS.remove(deps.storage, &bond_id);

    Ok(Response::new()
        .add_message(wasm_msg)  // Send the CW20 transfer message
        .add_attribute("method", "redeem_bond")
        .add_attribute("bond_id", bond_id))
}

// Query contract state
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBond { bond_id } => {
            let bond = BONDS.load(deps.storage, &bond_id)?;
            to_binary(&bond)
        }
    }
}

// Helper function to convert to binary for queries
fn to_binary<T: serde::Serialize>(obj: &T) -> StdResult<Binary> {
    cosmwasm_std::to_binary(obj)
}
