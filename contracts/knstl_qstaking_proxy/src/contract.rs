
#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_binary, entry_point, Env, Deps, DepsMut, MessageInfo, Response, StdResult, Binary, Uint128, CosmosMsg, StakingMsg, Coin, BankMsg, DistributionMsg};
use cw2::set_contract_version;
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::error::ContractError;
use crate::state::{CONFIG, Config};
const CONTRACT_NAME: &str = "knstl_qstaking_proxy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &Config{
        admin: info.sender,
        denom: msg.denom,
        owner: msg.owner.clone(),
    })?;
    Ok(Response::new().add_attribute("owner", msg.owner))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Stake { validator } => exec_stake(deps, env, info, validator),
        ExecuteMsg::Unstake { validator, amount } => exec_unstake(deps, env, info, validator, amount),
        ExecuteMsg::Claim {} => exec_claim(deps, env, info),
        ExecuteMsg::Restake {from, to, amount} => exec_redelegate(deps, env, info, from, to, amount),
        ExecuteMsg::Withdraw {validator} => exec_withdraw(deps, info, validator),
    }
}

fn exec_stake(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    validator: String,
)->Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin {
        return Err(ContractError::UnknownUser {});
    }
    let received = info.funds.first().unwrap();
    if received.denom != config.denom {
        return Err(ContractError::UnstakeableTokenSent { denom: received.denom.clone() });
    }
    
    let res = Response::new()
    .add_message(CosmosMsg::Staking(
        StakingMsg::Delegate { 
            validator: validator.clone(), 
            amount: Coin { 
                denom: received.denom.clone(), 
                amount: received.amount,
            },
        }
    ))
    .add_attribute("action", "instantiate")
    .add_attribute("from", &info.sender)
    .add_attribute("to", &validator)
    ;
    Ok(res)
}

fn exec_unstake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    validator: String,
    amount: Uint128,
)->Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::UnknownUser {})
    }
    let res = Response::new()
    .add_message(CosmosMsg::Staking(
        StakingMsg::Undelegate { 
            validator: validator,
            amount : Coin {
                denom: config.denom,
                amount,
    }}))
    .add_attribute("action", "unstake")
    .add_attribute("from", &info.sender)
    .add_attribute("to", &env.contract.address);

    Ok(res)
}

fn exec_redelegate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    from: String,
    to: String,
    amount: Uint128,
)->Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::UnknownUser {})
    }
    let res = Response::new()
    .add_message(CosmosMsg::Staking(
        StakingMsg::Redelegate { 
         src_validator: from.clone(),
         dst_validator: to.clone(),
         amount: Coin { denom: config.denom, amount: amount },
     }))
    .add_attribute("action", "redelegate")
    .add_attribute("from", &from)
    .add_attribute("to", &to)
    .add_attribute("by", env.contract.address);
    Ok(res)
}
fn exec_claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
)->Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::UnknownUser {})
    }
    let amount = deps.querier.query_balance(env.contract.address, config.denom)?;
    let res = Response::new()
    .add_message(BankMsg::Send{
        amount: vec![amount],
        to_address: config.owner.to_string(),
    })
    .add_attribute("action", "claim")
    .add_attribute("from", &config.owner)
    .add_attribute("to", &info.sender)
    ;
    Ok(res)
}

fn exec_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    validator: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::UnknownUser {})
    }

    let res = Response::new()
    .add_message(CosmosMsg::Distribution(
        DistributionMsg::WithdrawDelegatorReward { validator: validator.clone() }
    ))
    .add_attribute("action", "withdraw")
    .add_attribute("from", &validator)
    .add_attribute("to", &info.sender);
    Ok(res)

}
#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::ConfigInfo {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps)-> StdResult<Config>{
    Ok(CONFIG.load(deps.storage)?)
}