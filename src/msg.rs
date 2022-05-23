

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{ Uint128};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner : String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetPointer{address:String,token_type:String,pointer:Uint128},
    ChangeOwner{address:String}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg  {
    GetStateInfo{},
    GetPointer{address:String,token_type:String}
}