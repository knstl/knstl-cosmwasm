
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use cw_storage_plus::Item;
use cosmwasm_std::{Addr, Uint128, Timestamp, Decimal};


#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct Config {
    pub owner : Addr,
    pub admin : Addr,
    pub denom : String,
    pub unbond_period: u64,
    pub commission_rate: Decimal,
}
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct Unbonded {
    pub amount: Uint128,
    pub date: Timestamp,
    pub validator: String,
}

pub const CONFIG : Item<Config> = Item::new("config");
pub const BONDED : Item<Uint128> = Item::new("bonded");
pub const UNBONDED : Item<Vec<Unbonded>> = Item::new("unbonded");
pub const COMPOUNDED : Item<Uint128> = Item::new("compounded");