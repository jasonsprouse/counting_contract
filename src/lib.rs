use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use error::ContractError;
use msg::InstantiateMsg;

mod contract;
mod error;
pub mod msg;
#[cfg(any(test, feature = "tests"))]
pub mod multitest;
mod state;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(_deps, _info, _msg)
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: msg::ExecMsg,
) -> Result<Response, ContractError> {
    use contract::exec;
    use msg::ExecMsg::*;

    match msg {
        Donate {} => exec::donate(deps, info),
        Reset { counter } => exec::reset(deps, info, counter),
        Withdraw {} => exec::withdraw(deps, _env, info),
        WithdrawTo { receiver, funds } => exec::withdraw_to(deps, _env, info, receiver, funds),
    }
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use contract::query;
    use msg::QueryMsg::*;

    match msg {
        Value {} => to_binary(&query::value(_deps)?),
    }
}
