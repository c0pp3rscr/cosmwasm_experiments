use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,

    pub contract_to_call:String,
    pub a_or_b:u64,
    pub submsgid:u64
}

pub const STATE: Item<State> = Item::new("state");
pub const CONTRACT_ID: Item<String> = Item::new("contract_id");

