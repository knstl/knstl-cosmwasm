#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Env, DepsMut, MessageInfo, Response, BankMsg };
use cw2::set_contract_version;
use crate::msg::{InstantiateMsg, ExecuteMsg};
use crate::ContractError;
const CONTRACT_NAME: &str = "knstl_qstaking_proxy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    Ok(Response::default())
}

#[entry_point]
pub fn execute (
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,    
) -> Result<Response, ContractError> {

    match msg {
        ExecuteMsg::BurnMyDarc {} => exec_burn (info),
    }

}

fn exec_burn (
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let res = Response::new()
    .add_message(BankMsg::Burn { amount: info.funds})
    .add_attribute("action", "burn")
    ;
    Ok(res)
}