use cosmwasm_std::{
    entry_point,  Deps, DepsMut, Env, MessageInfo, Response,Binary,to_binary,
    StdResult, Uint128};

use cw2::set_contract_version;

use crate::error::{ContractError};
use crate::msg::{ ExecuteMsg, InstantiateMsg,QueryMsg};
use crate::state::{State,CONFIG,POINTER};


const CONTRACT_NAME: &str = "home_coding_test";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
   
    let state = State {
        owner : msg.owner
    };

    CONFIG.save(deps.storage,&state)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
    ExecuteMsg::SetPointer{address,token_type,pointer} =>execute_set_pointer(deps,env,info,address,token_type,pointer),
    ExecuteMsg::ChangeOwner { address }=>execute_change_owner(deps,env,info,address)
    }
}


fn execute_set_pointer(
    deps: DepsMut,
    _env:Env,
    info: MessageInfo,
    address:String,
    token_type:String,
    pointer:Uint128
) -> Result<Response, ContractError> {
 let  state = CONFIG.load(deps.storage)?;
 
 deps.api.addr_validate(&address)?;

 if state.owner !=info.sender.to_string(){
     return Err(ContractError::Unauthorized {  })
 }

 let key = (address.as_str(),token_type.as_str());

 let origin_pointer = POINTER.may_load(deps.storage,key)?;

 if origin_pointer == None{
     POINTER.save(deps.storage,key,&pointer)?;
 }
 else{
     let new_pointer = origin_pointer.unwrap()+pointer;
     POINTER.save(deps.storage,key,&new_pointer)?;
 }

 Ok(Response::default())

}



fn execute_change_owner(
    deps: DepsMut,
    _env:Env,
    info: MessageInfo,
    address:String
) -> Result<Response, ContractError> {
    deps.api.addr_validate(&address)?;
    let state = CONFIG.load(deps.storage)?;
    if info.sender.to_string() != state.owner{
        return Err(ContractError::Unauthorized {})
    }

    CONFIG.update(deps.storage,
        |mut state|->StdResult<_>{
            state.owner =  address;
            Ok(state)
        }
    )?;

    Ok(Response::default())
}


#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStateInfo {} => to_binary(&query_get_state(deps)?),
        QueryMsg::GetPointer { address, token_type } => to_binary(&query_get_pointer(deps,address,token_type)?),
    }   
}

pub fn query_get_state(deps:Deps) -> StdResult<State>{
    let state = CONFIG.load(deps.storage)?;
    Ok(state)
}

pub fn query_get_pointer(deps:Deps,address:String,token_type:String) -> StdResult<Uint128>{
    let key = (address.as_str(),token_type.as_str());
    let pointer = POINTER.may_load(deps.storage,key)?;
    if pointer ==None{
        Ok(Uint128::new(0))
    }
    else{
    Ok(pointer.unwrap())
}
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
   
    #[test]
    fn instantiate_contract() {
        let mut deps = mock_dependencies(&[]);
    
        let instantiate_msg = InstantiateMsg {
            owner:"owner".to_string(),
        };
        
        let info = mock_info("creator", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());
        
        let state = query_get_state(deps.as_ref()).unwrap();
        assert_eq!(state.owner,"owner".to_string());
    }
    
     #[test]
     fn change_owner() {
        let mut deps = mock_dependencies(&[]);
    
        let instantiate_msg = InstantiateMsg {
            owner:"owner".to_string(),
        };
        
        let info = mock_info("creator", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());
        
        let state = query_get_state(deps.as_ref()).unwrap();
        assert_eq!(state.owner,"owner".to_string());

         
        let info = mock_info("owner", &[]);
        let message = ExecuteMsg::ChangeOwner { address: "creator".to_string() };
        let res = execute(deps.as_mut(),mock_env(),info,message).unwrap();
        assert_eq!(0, res.messages.len());
        
        let state = query_get_state(deps.as_ref()).unwrap();
        assert_eq!(state.owner,"creator".to_string());

    }

    #[test]
     fn set_pointer() {
        let mut deps = mock_dependencies(&[]);
    
        let instantiate_msg = InstantiateMsg {
            owner:"owner".to_string(),
        };
        
        let info = mock_info("creator", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());
        
        let state = query_get_state(deps.as_ref()).unwrap();
        assert_eq!(state.owner,"owner".to_string());

         
        let info = mock_info("owner", &[]);
        let message = ExecuteMsg::SetPointer { address: "user1".to_string(), token_type: "luna".to_string(), pointer: Uint128::new(100) };
        execute(deps.as_mut(),mock_env(),info,message).unwrap();
        
        let pointer = query_get_pointer(deps.as_ref(),"user1".to_string(),"luna".to_string()).unwrap();
        assert_eq!(pointer,Uint128::new(100));

              
        let info = mock_info("owner", &[]);
        let message = ExecuteMsg::SetPointer { address: "user1".to_string(), token_type: "luna".to_string(), pointer: Uint128::new(120) };
        execute(deps.as_mut(),mock_env(),info,message).unwrap();

        let pointer = query_get_pointer(deps.as_ref(),"user1".to_string(),"luna".to_string()).unwrap();
        assert_eq!(pointer,Uint128::new(220));

        let info = mock_info("owner", &[]);
        let message = ExecuteMsg::SetPointer { address: "user1".to_string(), token_type: "uusd".to_string(), pointer: Uint128::new(120) };
        execute(deps.as_mut(),mock_env(),info,message).unwrap();

        let pointer = query_get_pointer(deps.as_ref(),"user1".to_string(),"uusd".to_string()).unwrap();
        assert_eq!(pointer,Uint128::new(120));

        let pointer = query_get_pointer(deps.as_ref(),"user3".to_string(),"uusd".to_string()).unwrap();
        assert_eq!(pointer,Uint128::new(0));
    }

}
 