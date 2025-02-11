#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, CONTRACT_ID,  STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:project-name2";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
// const SUBMSG_ID; // This is a unique identifier so we can associate a reply with a specific submessage. It can be any numeric value.



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
        contract_to_call: msg.contract_to_call,
        a_or_b : msg.a_or_b,
        submsgid: msg.submsgid
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    println!(r#"hmm"#);

    let contract_id:String = msg.contract_id;
    
    STATE.save(deps.storage, &state)?;
    CONTRACT_ID.save(deps.storage, &contract_id)?;
    
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
    
}


#[entry_point]
pub fn reply(deps: DepsMut, env: Env, msg: cosmwasm_std::Reply) -> Result<Response, ContractError> {

    let contractid = CONTRACT_ID.load(deps.storage).unwrap();

    println!("the contract is {} ", contractid);

    match msg.id {
        1 => Ok(Response::new().add_attribute("reply", "contract_a")),
        2 => Ok(Response::new().add_attribute("reply", "contract_b")),
        3 => Ok(Response::new().add_attribute("reply", "contract_c")),
        _ => Err(ContractError::Unauthorized {  }  ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => execute::increment(deps),
        ExecuteMsg::Reset { count } => execute::reset(deps, info, count),
        ExecuteMsg::FunctionA => execute::function_a(deps),
        ExecuteMsg::FunctionB => execute::function_b(deps),
        ExecuteMsg::Change_contract_to_call { new_contract_to_call } => execute::change_contract_to_call(deps, info, new_contract_to_call)
    }
}


// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn reply(deps: DepsMut, env: Env, msg: cosmwasm_std::Reply) -> Result<Response, ContractError> {
//     println!("📩 Contract B received reply!");

//     if msg.id != SUBMSG_ID {
//         return Err(ContractError::WrongSubmsg {});
//     }

//     Ok(Response::new().add_attribute("action", "reply_received"))
// }

pub mod execute {
    
    use cosmwasm_std::WasmMsg;
    use cosmwasm_std::SubMsg;

    use super::*;

    pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.count += 1;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "increment"))
    }

    pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            state.count = count;
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("action", "reset"))
    }

    pub fn function_a(deps: DepsMut) -> Result<Response, ContractError> {
        // pass a smart contract address and have it executed with submsg and always reply
        let contract_id = CONTRACT_ID.load(deps.storage);
        println!("Contract ID {} Inside Function A ", contract_id.unwrap());

        let state = STATE.load(deps.storage).unwrap();

        let contract_addr_to_call = state.contract_to_call;
        let a_or_b = state.a_or_b;
       
        let mut msg_data:Binary = to_json_binary(&ExecuteMsg::FunctionB ).unwrap();

        if a_or_b == 0
        {
            // we call a if its 0
            msg_data = to_json_binary(&ExecuteMsg::FunctionA {}   ).unwrap();

        }

        let msg = WasmMsg::Execute {
            contract_addr: contract_addr_to_call,
            msg: msg_data,
            funds: vec![],
        };

        
     
        let submsg = SubMsg::reply_on_success(msg, state.submsgid);
        Ok(Response::new().add_submessage(submsg))

    }

    pub fn function_b(deps: DepsMut) -> Result<Response, ContractError> {
        let contract_id = CONTRACT_ID.load(deps.storage);
        println!("Contract ID {} Inside Function B ", contract_id.unwrap());
        Ok(Response::new())
    }
    
    pub fn change_contract_to_call(deps: DepsMut, info: MessageInfo, new_contract_to_call:String ) -> Result<Response, ContractError>
    {
        let mut state = STATE.load(deps.storage)?;
        let contract_to_call = state.contract_to_call;

        // assert!(info.sender == state.owner); // access control
        assert!(new_contract_to_call.is_empty() == false);

        if contract_to_call.is_empty() { // one time settable only
            state.contract_to_call = new_contract_to_call;
            STATE.save(deps.storage, &state)?;
        }

        Ok(Response::new())
    }

}



// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn reply(deps: DepsMut, env: Env, msg: cosmwasm_std::Reply) -> Result<Response, ContractError> {
    
//     // a conrtact calls only one other contract, so its safe to say it expects just one reply
//     let contract_id = CONTRACT_ID.load(deps.storage);

//     println!("The reply received Contract {} ", contract_id.unwrap() );
        
//     if msg.id != SUBMSG_ID {
//         return Err(ContractError::WrongSubmsg {} );
//         // return Err(ContractError::Unauthorized {});

//         // Err(ContractError::Unauthorized {});
//     }


//     // We received a message! From the contract we invoked earlier.
//     // println!("The reply received Contract {} ", contract_id.clone().unwrap() );
 
//     Ok(Response::default())
// }

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_json_binary(&query::count(deps)?),
        QueryMsg::GetContractId => to_json_binary(&query::contract_id(deps)?)
    }
}

pub mod query {
    use super::*;

    pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetCountResponse { count: state.count })
    }

    pub fn contract_id(deps:Deps) -> StdResult<String> {
        let state = CONTRACT_ID.load(deps.storage);
        state
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    // use cosmwasm_std::{coins, from_json};

    // #[test]
    // fn proper_initialization() {
    //     let mut deps = mock_dependencies();

    //     let msg = InstantiateMsg { count: 17 , contract_id: String::from("ASD")};
    //     let info = mock_info("creator", &coins(1000, "earth"));

    //     // we can just call .unwrap() to assert this was a success
    //     let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    //     assert_eq!(0, res.messages.len());

    //     // it worked, let's query the state
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: GetCountResponse = from_json(&res).unwrap();
    //     assert_eq!(17, value.count);
    // }

    // #[test]
    // fn increment() {
    //     let mut deps = mock_dependencies();

    //     let msg = InstantiateMsg { count: 17, contract_id: String::from("ASD") };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Increment {};
    //     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // should increase counter by 1
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: GetCountResponse = from_json(&res).unwrap();
    //     assert_eq!(18, value.count);
    // }

    // #[test]
    // fn reset() {
    //     let mut deps = mock_dependencies();

    //     let msg = InstantiateMsg { count: 17, contract_id: String::from("ASD") };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let unauth_info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
    //     match res {
    //         Err(ContractError::Unauthorized {}) => {}
    //         _ => panic!("Must return unauthorized error"),
    //     }

    //     // only the original creator can reset the counter
    //     let auth_info = mock_info("creator", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

    //     // should now be 5
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: GetCountResponse = from_json(&res).unwrap();
    //     assert_eq!(5, value.count);
    // }
}
