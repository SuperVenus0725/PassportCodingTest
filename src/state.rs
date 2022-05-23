use cosmwasm_std::{Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item,Map};

pub const CONFIG: Item<State> = Item::new("config_state");
pub const POINTER: Map<(&str, &str), Uint128> = Map::new("config_pointer");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner:String,
   
}
