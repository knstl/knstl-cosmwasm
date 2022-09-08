use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use cosmwasm_std::{Addr, Uint128};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    pub tokencontract_id : u64,
    pub tokencontract_label : String,
    pub token_name : String,
    pub token_symbol : String,
    pub denom: String,
    pub reward_denom: String,
    pub reward_contract : String,
    pub stake_id: u64,
    pub stake_label: String,
}
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Register {},
    Stake { validator: String },
    Unstake { validator: String, amount : Uint128 },
    ChangeValidator {from: String, to: String, amount: Uint128},
    Claim {},
    GetReward {},
}
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ConfigInfo {},
    Staked {address: Addr},
    TokenInfo {address: Addr},
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct StakeInstantiateMsg {
    pub denom : String,
    pub owner : Addr,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StakeExecuteMsg {
    Stake { validator: String },
    Unstake { validator: String, amount: Uint128 },
    Claim {},
    Restake { from: String, to: String, amount: Uint128 },
    Withdraw { validator: String },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StakeQueryMsg {
    ConfigInfo {},
}

