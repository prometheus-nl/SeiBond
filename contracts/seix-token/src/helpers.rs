use cosmwasm_std::{
    to_json_binary, Addr, CosmosMsg, CustomQuery, Querier, QuerierWrapper, StdResult, WasmMsg,
    WasmQuery,
};
use cw20::{Cw20ExecuteMsg, BalanceResponse, Cw20QueryMsg};

/// Cw20Contract is a wrapper around Addr that provides helpers for CW20 operations
pub struct Cw20Contract(pub Addr);

impl Cw20Contract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<Cw20ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_json_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    /// Get balance of a specific address
    pub fn balance<Q, T, CQ>(&self, querier: &Q, address: T) -> StdResult<BalanceResponse>
    where
        Q: Querier,
        T: Into<String>,
        CQ: CustomQuery,
    {
        let msg = Cw20QueryMsg::Balance {
            address: address.into(),
        };
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_json_binary(&msg)?,
        }
        .into();
        let res: BalanceResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
        Ok(res)
    }

    /// Get token info
    pub fn token_info<Q, CQ>(&self, querier: &Q) -> StdResult<cw20::TokenInfoResponse>
    where
        Q: Querier,
        CQ: CustomQuery,
    {
        let msg = Cw20QueryMsg::TokenInfo {};
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_json_binary(&msg)?,
        }
        .into();
        let res: cw20::TokenInfoResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
        Ok(res)
    }
}
