use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use cosmwasm_std::{Addr, Uint128, Decimal};


#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    pub denom: String,
    pub cw20_id: u64,
    pub cw20_label: String,
    pub token_name: String,
    pub token_symbol: String,
    pub proxy_id: u64,
    pub proxy_label: String,
    pub commission_rate: Decimal,
    pub unbond_period: u64,
}
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Register {},
    Stake { validator: String },
    Unstake { validator: String, amount : Uint128 },
    Restake {from: String, to: String, amount: Uint128},
    Collect {validator: String},
    CollectAll {},
    Withdraw {},
    Compound {validator: String, amount: Uint128},
    // Decompound {validator: String, amount: Uint128},
}
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ConfigInfo {},
    AccountInfo {address: Addr},
    Staked {address: Addr},
    TokenInfo {address: Addr},
}


#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct QueryStaked {
    pub validator: String,
    pub staked: Uint128,
    pub compounded: Uint128,
}