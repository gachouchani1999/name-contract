#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetNameResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:smart-test";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        name: msg.name.clone(),
        deployer: info.sender.clone().to_string()
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("creator", info.sender)
        .add_attribute("name", msg.name))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Update {new_name} => try_update(deps,info,new_name),
    }
}

pub fn try_update(deps: DepsMut, info: MessageInfo, new_name: String) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender.to_string() != state.deployer {
            return Err(ContractError::ChrisLoote {  })
        }
        state.name = new_name.clone();
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "update")
        .add_attribute("new_name", new_name))
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetName {} => to_binary(&query_name(deps)?),
    }
}

fn query_name(deps: Deps) -> StdResult<GetNameResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetNameResponse { name: state.name })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { name: "Georges".to_string() };
        let info = mock_info("georges", &vec![]);

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        let query_msg = QueryMsg::GetName {};
        // it worked, let's query the state
        let q_res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let value: GetNameResponse = from_binary(&q_res).unwrap();
        assert_eq!("Georges".to_string(), value.name);
    }

    #[test]
    fn proper_update() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg { name: "Georges".to_string() };
        let info = mock_info("georges", &vec![]);

        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        let ex_msg = ExecuteMsg::Update { new_name: "Chris".to_string() };
        let _ex = execute(deps.as_mut(), mock_env(), info.clone(), ex_msg).unwrap();

        let query_msg = QueryMsg::GetName {};
        // it worked, let's query the state
        let q_res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let value: GetNameResponse = from_binary(&q_res).unwrap();
        assert_eq!("Chris".to_string(), value.name);
    }
    #[test]
    #[should_panic]
    fn bad_update() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg { name: "Georges".to_string() };
        let info = mock_info("georges", &vec![]);

        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        let ex_msg = ExecuteMsg::Update { new_name: "Chris".to_string() };
        let info = mock_info("chris", &vec![]);
        let _ex = execute(deps.as_mut(), mock_env(), info.clone(), ex_msg).unwrap();

        let query_msg = QueryMsg::GetName {};
        // it worked, let's query the state
        let q_res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let value: GetNameResponse = from_binary(&q_res).unwrap();
        assert_eq!("Chris".to_string(), value.name);
    }
}
