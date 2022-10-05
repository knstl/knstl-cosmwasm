#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_binary, entry_point, Env, Deps, DepsMut, MessageInfo, Response, StdResult, Binary, Uint128, CosmosMsg, StakingMsg, Coin, BankMsg, DistributionMsg, Decimal, Storage };
use cw2::set_contract_version;
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::error::ContractError;
use crate::state::{CONFIG, Config, UNBONDED, Unbonded, BONDED, COMPOUNDED};
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
        unbond_period: msg.unbond_period,
        community_pool : msg.community_pool,
        commission_rate: msg.commission_rate,
    })?;
    BONDED.save(deps.storage, &Uint128::zero())?;
    UNBONDED.save(deps.storage, &vec![])?;
    COMPOUNDED.save(deps.storage, &Uint128::zero())?;
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
        ExecuteMsg::Withdraw {} => exec_withdraw(deps, env, info),
        ExecuteMsg::Restake {from, to, amount} => exec_restake(deps, env, info, from, to, amount),
        ExecuteMsg::Claim {validator} => exec_claim(deps, info, validator),
        ExecuteMsg::Compound { validator, amount } => exec_compound(deps, info, validator, amount),
        ExecuteMsg::Decompound { validator, amount } => exec_decompound(deps, env, info, validator, amount),
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
    BONDED.update(deps.storage, |x| -> StdResult<Uint128> {
        Ok(x + received.amount)
    })?;
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
    .add_attribute("action", "stake")
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
    UNBONDED.update(deps.storage, |mut x| -> StdResult<Vec<Unbonded>> {
        x.push(Unbonded { amount: amount, date: env.block.time });
        Ok(x)
    })?;
    BONDED.update(deps.storage, |x| -> StdResult<Uint128> {
        Ok(x.saturating_sub(amount))
    })?;
    let res = Response::new()
    .add_message(CosmosMsg::Staking(
        StakingMsg::Undelegate { 
            validator: validator,
            amount : Coin {
                amount,
                denom: config.denom,
    }}))
    .add_attribute("action", "unstake")
    .add_attribute("from", &info.sender)
    .add_attribute("to", &env.contract.address);

    Ok(res)
}

fn exec_restake(
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
fn exec_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
)->Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::UnknownUser {})
    }
    let balance = deps.querier.query_balance(env.contract.address.clone(), config.denom.clone())?;
    let bonded = BONDED.load(deps.storage)?;
    let unbondings = resolve_unbondings(deps.storage, env.clone())?;
    let unbonded = get_unbonded_amount(deps.storage)?;
    // let compounded_unbondings = resolve_compounded_unbondings(deps.storage, env)?;
    if unbondings.is_zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }
    
    let reward_ratio: Decimal = Decimal::from_ratio(unbondings, bonded + unbonded);  
    let total_unbond = Coin {
        amount: unbondings + ((balance.amount - unbondings ) * reward_ratio * (Decimal::one() - config.commission_rate)),
        denom: config.denom.clone(),
    };

    let commission = Coin {
        amount : (balance.amount - unbondings ) * reward_ratio * config.commission_rate,
        denom: config.denom,
    };
    let res = Response::new()
    .add_message(BankMsg::Send{
        amount: vec![total_unbond],
        to_address: config.owner.to_string(),
    })
    .add_message(BankMsg::Send{
        amount: vec![commission],
        to_address: config.community_pool,
    })
    .add_attribute("action", "withdraw")
    .add_attribute("from", &config.owner)
    .add_attribute("to", &info.sender)
    ;
    Ok(res)
}

fn exec_claim(
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
    .add_attribute("action", "claim")
    .add_attribute("from", &validator)
    .add_attribute("to", &info.sender);
    Ok(res)
}

fn exec_compound (
    deps: DepsMut,
    info: MessageInfo,
    validator: String,
    amount: Uint128,
) -> Result<Response, ContractError> {

    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::UnknownUser {})
    }

    COMPOUNDED.update(deps.storage, |x| -> StdResult<Uint128> {
        Ok(x + amount)
    })?;

    
    let res = Response::new()
    .add_message(CosmosMsg::Staking(
        StakingMsg::Delegate { 
            validator: validator.clone(), 
            amount: Coin { 
                amount, 
                denom: config.denom.clone(),
    }}))
    .add_message(CosmosMsg::Bank(
        BankMsg::Send { 
            to_address: config.community_pool, 
            amount: vec![Coin{
                amount: amount * (config.commission_rate / (Decimal::one() - config.commission_rate)),
                denom: config.denom 
        }]
    }))
    .add_attribute("action", "compound")
    .add_attribute("to", &validator)
    ;
    Ok(res)
}

fn exec_decompound (
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    validator: String,
    amount: Uint128,
) -> Result<Response, ContractError> {

    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::UnknownUser {})
    }

    COMPOUNDED.update(deps.storage, |x| -> StdResult<Uint128> {
        Ok(x - amount)
    })?;
    UNBONDED.update(deps.storage, |mut x| -> StdResult<Vec<Unbonded>> {
        x.push(Unbonded { amount: amount, date: env.block.time });
        Ok(x)
    })?;
    let res = Response::new()
    .add_message(CosmosMsg::Staking(
        StakingMsg::Undelegate { 
            validator: validator.clone(), 
            amount: Coin { 
                amount, 
                denom: config.denom 
    }}))
    .add_attribute("action", "compound")
    .add_attribute("to", &validator)
    ;
    Ok(res)
}

fn resolve_unbondings(
    storage: &mut dyn Storage,
    env: Env,
)-> StdResult<Uint128> {
    let config = CONFIG.load(storage)?;
    let unbondeds = UNBONDED.load(storage)?;
    let mut ret = Uint128::zero();
    let mut new_unbonded : Vec<Unbonded> = vec![];
    for unbonded in unbondeds.iter() {
        if env.block.time.seconds() - unbonded.date.seconds() >= config.unbond_period {
            ret += unbonded.amount;
        } 
        else { new_unbonded.push(Unbonded { amount: unbonded.amount, date: unbonded.date }) }
    }
    UNBONDED.update(storage, |_| -> StdResult<Vec<Unbonded>> {
        Ok(new_unbonded)
    })?;
    Ok(ret)
}
fn get_unbonded_amount(
    storage: &mut dyn Storage
)-> StdResult<Uint128> {
    let unbondeds = UNBONDED.load(storage)?;
    let mut ret = Uint128::zero();
    for unbonded in unbondeds.iter() {
        ret += unbonded.amount
    }
    Ok(ret)
}
// fn resolve_compounded_unbondings(
//     storage: &mut dyn Storage,
//     env: Env,
// )-> StdResult<Uint128> {
//     let config = CONFIG.load(storage)?;
//     let mut ret = Uint128::zero();
//     let mut new_unbonded : Vec<Unbonded> = vec![];
//     for unbonded in unbondeds.iter() {
//         if env.block.time.seconds() - unbonded.date.seconds() >= config.unbond_period {
//             ret += unbonded.amount;
//         } 
//         else { new_unbonded.push(Unbonded { amount: unbonded.amount, date: unbonded.date }) }
//     }
//     Ok(ret)
// }


#[entry_point]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::ConfigInfo {} => to_binary(&query_config(deps)?),
        QueryMsg::Unbondings {} => to_binary(&query_unbondings(deps)?),
        QueryMsg::Rewards {} => to_binary(&query_rewards(deps, env)?),
    }
}

fn query_config(deps: Deps)-> StdResult<Config>{
    Ok(CONFIG.load(deps.storage)?)
}
fn query_unbondings(deps: Deps)-> StdResult<Vec<Unbonded>>{
    Ok(UNBONDED.load(deps.storage)?)
}
fn query_rewards(deps: Deps, env: Env) -> StdResult<Uint128> {
    let config = CONFIG.load(deps.storage)?;
    let balance = deps.querier.query_balance(env.contract.address.clone(), config.denom.clone())?;
    let bonded = BONDED.load(deps.storage)?;
    let mut unbondings = Uint128::zero();
    let unbondeds = UNBONDED.load(deps.storage)?;
    for unbonded in unbondeds.iter() {
        if env.block.time.seconds() - unbonded.date.seconds() >= config.unbond_period {
            unbondings += unbonded.amount;
        } 
    }
    let mut unbonded = Uint128::zero();
    for unbond in unbondeds.iter() {
        unbonded += unbond.amount
    }
    unbonded -= unbondings;
    let reward_ratio: Decimal = Decimal::from_ratio(unbondings, bonded + unbonded);  
    
    Ok(unbondings + ((balance.amount - unbondings ) * reward_ratio * (Decimal::one() - config.commission_rate)))
}