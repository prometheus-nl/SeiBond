#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Addr, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Binary, Uint128, StdError};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg, BalanceResponse, TokenInfoResponse, Cw20QueryMsg};
use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use crate::state::{BALANCES, TOKEN_INFO, MINTER, TokenInfo};

// version info for migration
const CONTRACT_NAME: &str = "crates.io:seix-token";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Instantiate the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let total_supply = Uint128::new(msg.initial_supply);
    
    // Set initial balance to the instantiator
    BALANCES.save(deps.storage, &info.sender, &total_supply)?;

    // Initialize token info
    let token_info = TokenInfo {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        total_supply,
    };
    TOKEN_INFO.save(deps.storage, &token_info)?;

    // If a minter is provided, store it
    if let Some(minter) = msg.minter {
        MINTER.save(deps.storage, &Addr::unchecked(minter))?;
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender.to_string())
        .add_attribute("total_supply", total_supply.to_string()))
}

// Execute contract methods (e.g., Transfer, Mint, Burn)
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: Cw20ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        Cw20ExecuteMsg::Transfer { recipient, amount } => Ok(execute_transfer(deps, info, recipient, amount)?),
        Cw20ExecuteMsg::Mint { recipient, amount } => Ok(execute_mint(deps, info, recipient, amount)?),
        Cw20ExecuteMsg::Burn { amount } => Ok(execute_burn(deps, info, amount)?),
        _ => Err(ContractError::Unauthorized {}),
    }
}

fn execute_transfer(
    deps: DepsMut,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let sender_balance = BALANCES.load(deps.storage, &info.sender)?;
    if sender_balance < amount {
        return Err(ContractError::Std(StdError::generic_err("Insufficient funds")));
    }

    BALANCES.update(deps.storage, &info.sender, |balance| -> StdResult<_> {
        let balance = balance.unwrap_or(Uint128::zero());
        balance.checked_sub(amount).map_err(|_| StdError::generic_err("Overflow error"))
    })?;
    BALANCES.update(deps.storage, &Addr::unchecked(recipient.clone()), |balance| -> StdResult<_> {
        let balance = balance.unwrap_or(Uint128::zero());
        Ok(balance + amount)
    })?;

    Ok(Response::new()
        .add_attribute("method", "transfer")
        .add_attribute("from", info.sender.to_string())
        .add_attribute("to", recipient)
        .add_attribute("amount", amount.to_string()))
}

fn execute_mint(
    deps: DepsMut,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let minter = MINTER.load(deps.storage)?;
    if info.sender != minter {
        return Err(ContractError::Std(StdError::generic_err("Unauthorized: Only the minter can mint tokens")));
    }

    let recipient_balance = BALANCES.load(deps.storage, &Addr::unchecked(recipient.clone())).unwrap_or(Uint128::zero());
    BALANCES.save(deps.storage, &Addr::unchecked(recipient.clone()), &(recipient_balance + amount))?;

    let mut token_info = TOKEN_INFO.load(deps.storage)?;
    token_info.total_supply += amount;
    TOKEN_INFO.save(deps.storage, &token_info)?;

    Ok(Response::new()
        .add_attribute("method", "mint")
        .add_attribute("to", recipient)
        .add_attribute("amount", amount.to_string()))
}

fn execute_burn(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let sender_balance = BALANCES.load(deps.storage, &info.sender)?;
    if sender_balance < amount {
        return Err(ContractError::Std(StdError::generic_err("Insufficient funds")));
    }

    // Update sender balance
    BALANCES.update(deps.storage, &info.sender, |balance| -> StdResult<_> {
        let balance = balance.unwrap_or(Uint128::zero());
        balance.checked_sub(amount).map_err(|_| StdError::generic_err("Overflow error: cannot burn more than the balance"))
    })?;

    let mut token_info = TOKEN_INFO.load(deps.storage)?;
    token_info.total_supply = token_info.total_supply.checked_sub(amount)
        .map_err(|_| ContractError::Std(StdError::generic_err("Overflow error: total supply underflow")))?;
    TOKEN_INFO.save(deps.storage, &token_info)?;

    Ok(Response::new()
        .add_attribute("method", "burn")
        .add_attribute("from", info.sender.to_string())
        .add_attribute("amount", amount.to_string()))
}

// Query contract state (e.g., total supply, balance)
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: Cw20QueryMsg,
) -> StdResult<Binary> {
    match msg {
        Cw20QueryMsg::Balance { address } => to_json_binary(&query_balance(deps, address)?),
        Cw20QueryMsg::TokenInfo {} => to_json_binary(&query_token_info(deps)?),
        
        // For unsupported queries, handle them here
        _ => Err(StdError::generic_err("Unsupported query message")),
    }
}


fn query_balance(deps: Deps, address: String) -> StdResult<BalanceResponse> {
    let balance = BALANCES.load(deps.storage, &Addr::unchecked(address)).unwrap_or(Uint128::zero());
    Ok(BalanceResponse { balance })
}

fn query_token_info(deps: Deps) -> StdResult<TokenInfoResponse> {
    let token_info = TOKEN_INFO.load(deps.storage)?;
    Ok(TokenInfoResponse {
        name: token_info.name,
        symbol: token_info.symbol,
        decimals: token_info.decimals,
        total_supply: token_info.total_supply,
    })
}
