
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
    pub compounded: Vec<Staked>,
    pub staked : Vec<Staked>,
    pub stake_contract : String,
    pub minted : Uint128,
}
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq,  )]
pub struct Staked {
    pub amount: Uint128,
    pub validator: String,

}

pub const STAKEINFO : Map<&Addr, StakeInfo> = Map::new("stakeinfo");
pub const CONFIG : Item<Config> = Item::new("delegateinfo");