
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use cw_storage_plus::Item;
use cosmwasm_std::{Addr, Uint128};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct TokenContract {
    pub contract: String,
    pub reward: Uint128,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct Config {
    pub owner : Addr,
    pub admin : Addr,
    pub denom : String,
}

pub const CONFIG : Item<Config> = Item::new("config");