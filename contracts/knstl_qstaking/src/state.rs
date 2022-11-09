
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use cw_storage_plus::{Map, Item};
use cosmwasm_std::{Addr, Uint128, Decimal};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct Config {
    pub native_denom: String,
    pub cw20contract : String,
    pub stake_contract_id : u64,
    pub stake_contract_label: String,
    pub commission_rate: Decimal,
    pub unbond_period: u64,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct StakeInfo {
    pub compounded: Uint128,
    pub staked : Uint128,
}
pub const CONFIG : Item<Config> = Item::new("delegateinfo");

pub const STAKEINFO : Map<(&Addr, String), StakeInfo> = Map::new("stakeinfo");
pub const PROXY : Map<&Addr, String> = Map::new("proxyaddr");