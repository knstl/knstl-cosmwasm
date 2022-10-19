#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_binary, entry_point, Env, Deps, DepsMut, MessageInfo, Response, StdResult, Binary, Uint128, CosmosMsg, WasmMsg, Addr, SubMsg, ReplyOn, Reply, Decimal};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, Cw20InstantiateMsg, MinterResponse};
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::error::ContractError;
use crate::state::{STAKEINFO, StakeInfo, Config, CONFIG, Staked};
use qstaking_proxy::msg::{InstantiateMsg as ProxyInstantiateMsg, ExecuteMsg as ProxyExecuteMsg};

const CONTRACT_NAME: &str = "knstl_qstaking";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const TOKEN_INIT_ID : u64 = 1;
const STAKE_INIT_ID : u64 = 2;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,    
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(deps.storage, &Config{
        native_denom: msg.denom,
        cw20contract : String::new(),
        stake_contract_id : msg.proxy_id,
        stake_contract_label: msg.proxy_label,
        commission_rate: msg.commission_rate,
        unbond_period: msg.unbond_period,
    })?;

    let res = Response::new()
    .add_submessage(SubMsg { 
        id: TOKEN_INIT_ID, 
        msg: CosmosMsg::Wasm(WasmMsg::Instantiate { 
            admin: Some(env.contract.address.to_string()),
            code_id: msg.cw20_id, 
            msg: to_binary(&Cw20InstantiateMsg {
                name: msg.token_name,
                symbol: msg.token_symbol,
                mint: Some(
                    MinterResponse {
                    minter : env.contract.address.clone().to_string(),
                    cap : None,
                }),
                initial_balances: vec![],
                decimals: 6_u8,
                marketing : None,
            })?, 
            funds: vec![], 
            label: msg.cw20_label ,
        }),
        gas_limit : None,
        reply_on: ReplyOn::Success,
    })
    .add_attribute("action", "instantiate")
    .add_attribute("from", &info.sender)
    ;
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register {} => exec_register(deps, env, info),
        ExecuteMsg::Stake { validator } => exec_handle_stake(deps, env, info, validator),
        ExecuteMsg::Unstake { validator, amount } => exec_handle_unstake(deps, env, info, validator, amount),
        ExecuteMsg::Collect {validator} => exec_handle_collect(deps, info, validator),
        ExecuteMsg::CollectAll {} => exec_handle_collect_all(deps, info),
        ExecuteMsg::Restake { from, to, amount } => exec_handle_redelegation(deps, info, from, to, amount),
        ExecuteMsg::Withdraw {} => exec_handle_withdraw(deps, info),
        ExecuteMsg::Compound { validator, amount } => exec_handle_compound(deps, env, info, validator, amount),
    }
}
fn exec_register(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
)-> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let res = Response::new()
    .add_attribute("action", "register")
    .add_attribute("user", &info.sender)
    .add_submessage(SubMsg { 
        id: STAKE_INIT_ID, 
        msg: CosmosMsg::Wasm(WasmMsg::Instantiate { 
            admin: Some(env.contract.address.to_string()),
            code_id: config.stake_contract_id,
            msg: to_binary(&ProxyInstantiateMsg {
                denom: config.native_denom,
                owner: info.sender,
                commission_rate: config.commission_rate,
                unbond_period: config.unbond_period,
            })?, 
            funds: vec![], 
            label: config.stake_contract_label,
        }),
        gas_limit : None,
        reply_on: ReplyOn::Success ,
    })
    ;
    Ok(res)
}
fn exec_handle_stake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    validator: String,
)->Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if info.funds.len() > 1 {
        return Err(ContractError::InvalidMultipleTokens {})
    }
    let received = info.funds.first().unwrap();
    
    if received.denom != config.native_denom {
        return Err(ContractError::UnstakeableTokenSent { denom: received.denom.clone() });
    }
    
    let stake_info = STAKEINFO.load(deps.storage, &info.sender)?;
    let staked = stake_info.staked.iter().find(|x| x.validator == validator);

    STAKEINFO.update(deps.storage, &info.sender, |info| -> StdResult<_> {
        let mut ret = info.clone().unwrap();
        ret.minted += received.amount;
        Ok(match staked {
            Some(w) => {
                ret.staked.retain(|x| x.validator != validator);
                ret.staked.push(Staked{
                    amount: w.amount + received.amount,
                    validator: validator.clone(),
                });
                ret
            },
            None => {
                ret.staked.push(Staked { amount: received.amount, validator: validator.clone()});
                ret 
            },
    })})?;

    let res = Response::new()
    .add_message(CosmosMsg::Wasm(
        WasmMsg::Execute { 
            contract_addr: stake_info.stake_contract, 
            msg: to_binary(&ProxyExecuteMsg::Stake { validator: validator.clone() })?, 
            funds: info.funds.clone(), 
    }))
    .add_message(CosmosMsg::Wasm(WasmMsg::Execute { 
        contract_addr: config.cw20contract, 
        msg: to_binary(&Cw20ExecuteMsg::Mint { 
            recipient: info.sender.to_string(),
            amount: received.amount,
        })?, 
        funds: vec![],
    }))
    .add_attribute("action", "stakerequest")
    .add_attribute("from", &info.sender)
    .add_attribute("to", &env.contract.address)
    .add_attribute("validator", &validator);    
    Ok(res)
}
fn exec_handle_unstake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    validator: String,
    amount: Uint128,
)->Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let stake_info = STAKEINFO.load(deps.storage, &info.sender)?;
    let staked = stake_info.staked.iter().find(|x| x.validator == validator).unwrap();
    let compounded = stake_info.compounded.iter().find(|x| x.validator == validator);
    if amount > staked.amount {
        return Err(ContractError::InvalidUnstakeAmount {});
    }
    let redeem_rate = Decimal::from_ratio(amount, staked.amount);
    let res = 
    match compounded {
        Some(w) => {
            let decompound_amount = w.amount * redeem_rate;
            STAKEINFO.update(
                    deps.storage, 
                    &info.sender, 
                    |info| -> StdResult<_> {
                        let mut ret = info.clone().unwrap();
                        ret.staked.retain(|x| x.validator != validator );
                        ret.staked.push(Staked { amount: staked.amount.checked_sub(amount).unwrap(), validator: validator.clone() });
                        ret.minted -= amount;
                        ret.compounded.retain(|x| x.validator != validator);
                        ret.compounded.push(Staked {amount: w.amount.checked_sub(decompound_amount).unwrap(), validator: validator.clone() });
                        Ok(ret)
            })?;
            Response::new()
            .add_message(WasmMsg::Execute { 
                contract_addr: stake_info.stake_contract.clone(), 
                msg: to_binary(&ProxyExecuteMsg::Decompound { amount: decompound_amount, validator: validator.to_string() })?,
                funds: vec![],
            })
        },
        None => {
            STAKEINFO.update(
                    deps.storage, 
                    &info.sender, 
                    |info| -> StdResult<_> {
                        let mut ret = info.clone().unwrap();
                        ret.staked.retain(|x| x.validator != validator );
                        ret.staked.push(Staked { amount: staked.amount.checked_sub(amount).unwrap(), validator: validator.clone() });
                        ret.minted -= amount;
                        Ok(ret)
            })?;
            Response::new()
        },
    }            
    .add_message(WasmMsg::Execute { 
        contract_addr: stake_info.stake_contract.clone(), 
        msg: to_binary(&ProxyExecuteMsg::Unstake { amount: amount, validator: validator.to_string() })?,
        funds: vec![],
    })
    .add_message(WasmMsg::Execute { 
        contract_addr: config.cw20contract,
        msg: to_binary(&Cw20ExecuteMsg::BurnFrom { owner: info.sender.to_string(), amount })?,
        funds: vec![],
    })            
    .add_attribute("action", "unstake")
    .add_attribute("from", &info.sender)
    .add_attribute("to", &env.contract.address);
    Ok(res)
}
fn exec_handle_withdraw (
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let stake_info = STAKEINFO.load(deps.storage, &info.sender)?;
    // if stake_info.dayspent(env.block.time) > 21 {
    //     return Err(ContractError::OnUnbondingPeriod {});
    // }
    let res = Response::new()
    .add_message(CosmosMsg::Wasm({
        WasmMsg::Execute { 
            contract_addr: stake_info.stake_contract, 
            msg: to_binary(&ProxyExecuteMsg::Withdraw {})?, 
            funds: vec![],
    }}))
    .add_attribute("action", "withdraw")
    .add_attribute("from", &info.sender)
    ;
    Ok(res)
}
fn exec_handle_redelegation (
    deps: DepsMut,
    info: MessageInfo,
    from: String,
    to: String,
    amount: Uint128,
) -> Result<Response, ContractError> { 
    let stake_info = STAKEINFO.load(deps.storage, &info.sender)?;
    let from_info = stake_info.staked.iter().find(|x| x.validator == from);

    if from_info == None {
        return Err(ContractError::InvalidRequest { });
    } else if from_info.unwrap().amount < amount {
            return Err(ContractError::TooFewTokens {});
    }

    if let None = stake_info.staked.iter().find(|x| x.validator == to) {
        STAKEINFO.update(deps.storage, &info.sender, |info| -> StdResult<_> {
            let mut ret = info.clone().unwrap();
            ret.staked.push(Staked{ amount: Uint128::zero(), validator: to.clone()});
            Ok(ret)
        })?;
    }
    
    STAKEINFO.update(
        deps.storage, 
        &info.sender, 
        |info| -> StdResult<_> {
            let mut ret = info.clone().unwrap();
            ret.staked.retain(|x| x.validator != from );
            ret.staked.push(Staked { amount: from_info.unwrap().amount.checked_sub(amount).unwrap(), validator: from.clone() });
            let to_info = stake_info.staked.iter().find(|x| x.validator == to);
            ret.staked.retain(|x| x.validator != to );
            ret.staked.push(Staked { amount: to_info.unwrap().amount.checked_add(amount).unwrap(), validator: to.clone() });
            Ok(ret)
    })?;

    let res = Response::new()
    .add_message(CosmosMsg::Wasm(
        WasmMsg::Execute { 
            contract_addr: stake_info.stake_contract, 
            msg: to_binary(&ProxyExecuteMsg::Restake { from, to, amount })?,
            funds: vec![],
    }));
    Ok(res)
}
fn exec_handle_collect(
    deps: DepsMut, 
    info: MessageInfo,
    validator: String,
) -> Result<Response, ContractError> {
    let stake_info = STAKEINFO.load(deps.storage, &info.sender)?;

    let res = Response::new()
    .add_message(WasmMsg::Execute { 
        contract_addr: stake_info.stake_contract.clone(),
        msg: to_binary(&ProxyExecuteMsg::Collect { validator: validator.clone() })?, 
        funds: vec![],
    })
    .add_attribute("action", "collect_rewards")
    .add_attribute("from", &validator)
    .add_attribute("recipient", &info.sender)
    ;
    Ok(res)
}
fn exec_handle_collect_all(
    deps: DepsMut, 
    info: MessageInfo,
) -> Result<Response, ContractError> {

    let stake_info = STAKEINFO.load(deps.storage, &info.sender)?;
    let mut withdraw_msgs: Vec<CosmosMsg> = vec![];

    for staked in stake_info.staked.clone() {
        if !staked.amount.is_zero() {
        withdraw_msgs.push(CosmosMsg::Wasm({WasmMsg::Execute { 
            contract_addr: stake_info.stake_contract.clone(),
            msg: to_binary(&ProxyExecuteMsg::Collect { validator: staked.validator })?, 
            funds: vec![],
    }}))}}
    
    let res = Response::new()
    .add_messages(withdraw_msgs)
    .add_attribute("action", "collect_rewards")
    .add_attribute("from", &info.sender)
    ;
    Ok(res)
}
fn exec_handle_compound (
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    validator: String,
    amount: Uint128,
) -> Result<Response, ContractError> { 
    let stake_info = STAKEINFO.load(deps.storage, &info.sender)?;
    let compounded = stake_info.compounded.iter().find(|x| x.validator == validator);

    STAKEINFO.update(deps.storage, &info.sender, |info| -> StdResult<_> {
        let mut ret = info.clone().unwrap();
        Ok(match compounded {
            Some(w) => {
                ret.compounded.retain(|x| x.validator != validator);
                ret.compounded.push(Staked{
                    amount: w.amount + amount,
                    validator: validator.clone(),
                });
                ret
            },
            None => {
                ret.compounded.push(Staked { amount: amount, validator: validator.clone()});
                ret 
            },
    })})?;

    let res = Response::new()
    .add_message(CosmosMsg::Wasm(
        WasmMsg::Execute { 
            contract_addr: stake_info.stake_contract, 
            msg: to_binary(&ProxyExecuteMsg::Compound { validator: validator.clone(), amount })?, 
            funds: info.funds.clone(), 
    }))
    .add_attribute("action", "compound")
    .add_attribute("from", &info.sender)
    .add_attribute("to", &env.contract.address)
    .add_attribute("validator", &validator);
    Ok(res)
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut,
    _env: Env,
    msg: Reply,
) -> Result<Response, ContractError> {
    match msg.id {
        TOKEN_INIT_ID => handle_token_init (deps, msg),
        STAKE_INIT_ID => handle_stake_init (deps, msg),
        _ => Err(ContractError::Unauthorized {}),
    }
}
fn handle_token_init (
    deps: DepsMut,
    msg: Reply,
) -> Result<Response, ContractError> {
    CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
        config.cw20contract = cw_utils::parse_reply_instantiate_data(msg).unwrap().contract_address;
        Ok(config)
    })?;
    Ok(Response::default())
}

fn handle_stake_init (
    deps: DepsMut,
    msg: Reply,
) -> Result<Response, ContractError> {
    let res = msg.clone().result.into_result().unwrap();
    
    let mut owner = String::new();
    for event in res.events {
        for attr in event.attributes {
            if attr.key == "owner" {
                owner = attr.value;
                break
    }}}

    if owner.is_empty() {
        return Err(ContractError::InvalidSubmsg {});
    }
    
    STAKEINFO.save(deps.storage, &deps.api.addr_validate(&owner).unwrap(), &StakeInfo { 
        staked: vec![],
        compounded: vec![],
        stake_contract: cw_utils::parse_reply_instantiate_data(msg).unwrap().contract_address,
        minted: Uint128::zero(),
    })?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::AccountInfo { address } => to_binary(&query_account_info(deps, address)?),
        QueryMsg::ConfigInfo {} => to_binary(&query_config(deps)?),
        QueryMsg::Staked { address } => to_binary(&query_stake_amount(deps, address)?),
        QueryMsg::TokenInfo { address } => to_binary(&query_reward_token_amount(deps, address)?),
    }
}
fn query_stake_amount(deps: Deps, address: Addr)-> StdResult<StakeInfo>{
    match STAKEINFO.has(deps.storage, &address) {
        true => { Ok(STAKEINFO.load(deps.storage, &address).unwrap() ) }
        false => { Ok(StakeInfo { compounded: vec![], staked: vec![], stake_contract: String::new(), minted: Uint128::zero() })}
    }
}
fn query_reward_token_amount(deps: Deps, address: Addr) -> StdResult<String> {
    let config = CONFIG.load(deps.storage)?;
    Ok(deps.querier.query_wasm_smart(
        config.cw20contract,
        &Cw20QueryMsg::Balance { address: address.into() },
    )?)
}
fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config)
}
fn query_account_info (deps: Deps, address: Addr) -> StdResult<bool> {
    Ok(STAKEINFO.has(deps.storage, &address))
}