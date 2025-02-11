use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
    pub contract_id: String,
    pub contract_to_call: String,
    pub a_or_b: u64,
    pub submsgid:u64
}

#[cw_serde]
pub struct ReplyMsg {
    
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
    FunctionA,
    FunctionB,
    Change_contract_to_call {new_contract_to_call:String},
}



#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
    #[returns(String)]
    GetContractId
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}

