use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use cosmwasm_std::{Uint128, Addr};
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    pub denom : String,
    pub owner : Addr,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Stake { validator: String },
    Unstake { validator: String, amount: Uint128 },
    Claim {},
    Restake { from: String, to: String, amount: Uint128 },
    Withdraw { validator: String },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ConfigInfo {},
}

