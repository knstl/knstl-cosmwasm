use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use cosmwasm_std::{Uint128, Addr, Decimal};
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    pub denom : String,
    pub owner : Addr,
    pub unbond_period: u64,
    pub community_pool: String,
    pub commission_rate: Decimal,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Stake { validator: String },
    Unstake { validator: String, amount: Uint128 },
    Collect {validator: String },
    Restake { from: String, to: String, amount: Uint128 },
    Withdraw { },
    Compound { validator: String, amount: Uint128},
    Decompound { validator: String, amount: Uint128},
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ConfigInfo {},
    Unbondings {},
    Rewards {},
}

