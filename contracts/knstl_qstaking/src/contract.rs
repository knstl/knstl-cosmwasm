
#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_binary, entry_point, Env, Deps, DepsMut, MessageInfo, Response, StdResult, Binary, Uint128, CosmosMsg, WasmMsg, Addr, SubMsg, ReplyOn, Reply, Decimal};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, Cw20InstantiateMsg, MinterResponse};
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, QueryStaked};
use crate::error::ContractError;
use crate::state::{Config, CONFIG, PROXY, STAKES, Stakes};
use qstaking_proxy::msg::{InstantiateMsg as ProxyInstantiateMsg, ExecuteMsg as ProxyExecuteMsg};

const CONTRACT_NAME: &str = "knstl_qstaking_dev";
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
)-> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let proxy = PROXY.load(deps.storage, &info.sender)?;
    if info.funds.len() > 1 {
        return Err(ContractError::InvalidMultipleTokens {})
    }
    let received = info.funds.first().unwrap();
    if received.denom != config.native_denom {
        return Err(ContractError::UnstakeableTokenSent { denom: received.denom.clone() });
    }
    
    match STAKES.may_load(deps.storage, (&info.sender, validator.clone()))? {
        Some(w) => {
            STAKES.save(deps.storage, (&info.sender, validator.clone()), &Stakes {
                compounded: w.compounded,
                staked: w.staked + received.amount,
            })?;
        },
        None => {            
            STAKES.save(deps.storage, (&info.sender, validator.clone()), &Stakes {
                compounded: Uint128::zero(),
                staked: received.amount,
            })?;
        },
    }
    let res = Response::new()
    .add_message(CosmosMsg::Wasm(
        WasmMsg::Execute { 
            contract_addr: proxy, 
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
    .add_attribute("validator", &validator)
    ;    
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
    let stakes = STAKES.load(deps.storage, (&info.sender, validator.clone()))?;
    let proxy = PROXY.load(deps.storage, &info.sender)?;
    if amount > stakes.staked {
        return Err(ContractError::InvalidUnstakeAmount {});
    }
    let redeem_rate = Decimal::from_ratio(amount, stakes.staked);
    let res = 
    if stakes.compounded == Uint128::zero() {
        STAKES.update(
            deps.storage, (&info.sender, validator.clone()),
            |x| -> StdResult<_> {
                let mut ret = x.unwrap();
                ret.staked = ret.staked.checked_sub(amount).unwrap();
                Ok(ret)
            }
        )?;
        Response::new()
    } else {
        let decompound_amount = stakes.compounded * redeem_rate;
        STAKES.update(
            deps.storage, (&info.sender, validator.clone()),
            |x| -> StdResult<_> {
                let mut ret = x.unwrap();
                ret.staked = ret.staked.checked_sub(amount).unwrap();
                ret.compounded = ret.compounded.checked_sub(decompound_amount).unwrap();
                Ok(ret)
            }
        )?;
        Response::new()                
        .add_message(WasmMsg::Execute { 
            contract_addr: proxy.clone(),
            msg: to_binary(&ProxyExecuteMsg::Decompound { amount: decompound_amount, validator: validator.to_string() })?,
            funds: vec![],
        })      
    }            
    .add_message(WasmMsg::Execute { 
        contract_addr: proxy, 
        msg: to_binary(&ProxyExecuteMsg::Unstake { amount, validator })?,
        funds: vec![],
    })
    .add_message(WasmMsg::Execute { 
        contract_addr: config.cw20contract,
        msg: to_binary(&Cw20ExecuteMsg::BurnFrom { owner: info.sender.to_string(), amount })?,
        funds: vec![],
    })            
    .add_attribute("action", "unstake")
    .add_attribute("from", &info.sender)
    .add_attribute("to", &env.contract.address)
    ;
    Ok(res)
}

fn exec_handle_withdraw(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let proxy = PROXY.load(deps.storage, &info.sender)?;

    let res = Response::new()
    .add_message(CosmosMsg::Wasm({
        WasmMsg::Execute { 
            contract_addr: proxy,
            msg: to_binary(&ProxyExecuteMsg::Withdraw {})?, 
            funds: vec![],
        }
    }))
    .add_attribute("action", "withdraw")
    .add_attribute("from", &info.sender)
    ;
    Ok(res)
}

fn exec_handle_redelegation(
    deps: DepsMut,
    info: MessageInfo,
    from: String,
    to: String,
    amount: Uint128,
) -> Result<Response, ContractError> { 
    let from_stakes = STAKES.load(deps.storage, (&info.sender, from.clone()))?;
    let proxy = PROXY.load(deps.storage, &info.sender)?;
    if from_stakes.staked < amount {
            return Err(ContractError::NotEnoughTokens {});
    }
    STAKES.update(deps.storage, (&info.sender, from.clone()), |x| -> StdResult<_> {
        let mut ret = x.unwrap();
        ret.staked = ret.staked.checked_sub(amount).unwrap();
        Ok(ret)
    })?;
    match STAKES.has(deps.storage, (&info.sender, to.clone())) {
        true => {
            STAKES.update(deps.storage, (&info.sender, to.clone()), |x| -> StdResult<_> {
            let mut ret = x.unwrap();
            ret.staked = ret.staked.checked_sub(amount).unwrap();
            Ok(ret)
        })?;
        },
        false => {
            STAKES.save(deps.storage, (&info.sender, to.clone()), &Stakes { 
                compounded: Uint128::zero(), 
                staked: amount,
            })?;
        },
    }

    let res = Response::new()
    .add_message(CosmosMsg::Wasm(
        WasmMsg::Execute { 
            contract_addr: proxy,
            msg: to_binary(&ProxyExecuteMsg::Restake { from, to, amount })?,
            funds: vec![],
    }))
    ;
    Ok(res)
}

fn exec_handle_collect(
    deps: DepsMut, 
    info: MessageInfo,
    validator: String,
) -> Result<Response, ContractError> {
    let proxy = PROXY.load(deps.storage, &info.sender)?;

    let res = Response::new()
    .add_message(WasmMsg::Execute { 
        contract_addr: proxy,
        msg: to_binary(&ProxyExecuteMsg::Collect { validator: validator.clone() })?, 
        funds: vec![],
    })
    .add_attribute("action", "collect")
    .add_attribute("from", &validator)
    .add_attribute("recipient", &info.sender)
    ;
    Ok(res)
}

fn exec_handle_collect_all(
    deps: DepsMut, 
    info: MessageInfo,
) -> Result<Response, ContractError> {
    
    let proxy = PROXY.load(deps.storage, &info.sender)?;
    let mut withdraw_msgs: Vec<CosmosMsg> = vec![];

    let keys = STAKES
        .keys(deps.storage, None, None, cosmwasm_std::Order::Ascending);
    
    for key in keys {
        let v = key.unwrap().1;
        if !STAKES.load(deps.storage, (&info.sender, v.clone()))?.staked.is_zero() {
        withdraw_msgs.push(CosmosMsg::Wasm({WasmMsg::Execute { 
            contract_addr: proxy.clone(),
            msg: to_binary(&ProxyExecuteMsg::Collect { validator: v.clone() })?, 
            funds: vec![],
        }}))}
    }
    
    let res = Response::new()
    .add_messages(withdraw_msgs)
    .add_attribute("action", "collect_all")
    .add_attribute("from", &info.sender)
    ;
    Ok(res)
}

fn exec_handle_compound(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    validator: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let proxy = PROXY.load(deps.storage, &info.sender)?;

    STAKES.update(deps.storage, (&info.sender, validator.clone()), |x| -> StdResult<_> {
        let mut ret = x.unwrap();
        ret.compounded += amount;
        Ok(ret)
    })?;

    let res = Response::new()
    .add_message(CosmosMsg::Wasm(
        WasmMsg::Execute { 
            contract_addr: proxy,
            msg: to_binary(&ProxyExecuteMsg::Compound { validator: validator.clone(), amount })?, 
            funds: info.funds.clone(), 
    }))
    .add_attribute("action", "compound")
    .add_attribute("from", &info.sender)
    .add_attribute("to", &validator);
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

fn handle_stake_init(
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
    
    PROXY.save(deps.storage, 
        &deps.api.addr_validate(&owner).unwrap(), 
        &cw_utils::parse_reply_instantiate_data(msg).unwrap().contract_address
    )?;
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
fn query_stake_amount(deps: Deps, address: Addr)-> StdResult<Vec<QueryStaked>>{
    let iter  = STAKES.prefix(&address).range(deps.storage, None, None, cosmwasm_std::Order::Ascending).into_iter();
    let mut ret = vec![];
    for item in iter {
        let x = item.unwrap();
        ret.push(QueryStaked {  
            validator: x.0,
            staked: x.1.staked, 
            compounded: x.1.compounded,
        });
    }
    Ok(ret)
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
fn query_account_info(deps: Deps, address: Addr) -> StdResult<bool> {
    Ok(PROXY.has(deps.storage, &address))
}