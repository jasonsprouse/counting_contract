use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use error::ContractError;
use msg::InstantiateMsg;

mod contract;
mod error;
pub mod msg;
mod state;
#[cfg(any(test, feature = "tests"))]
pub mod multitest;

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

// #[cfg(test)]
// mod test {
//     use cosmwasm_std::{coin, coins, Addr, Empty};
//     use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

//     use crate::error::ContractError;
//     use crate::msg::{ExecMsg, InstantiateMsg, QueryMsg, ValueResp};
//     use crate::{execute, instantiate, query};

//     fn counting_contract() -> Box<dyn Contract<Empty>> {
//         let contract = ContractWrapper::new(execute, instantiate, query);
//         Box::new(contract)
//     }

//     #[test]
//     fn query_value() {
//         let mut app = App::default();

//         let contract_id = app.store_code(counting_contract());

//         let contract_addr = app
//             .instantiate_contract(
//                 contract_id,
//                 Addr::unchecked("sender"),
//                 &InstantiateMsg {
//                     counter: 0,
//                     minimal_donation: coin(10, "atom"),
//                 },
//                 &[],
//                 "Counting contract",
//                 None,
//             )
//             .unwrap();

//         let resp: ValueResp = app
//             .wrap()
//             .query_wasm_smart(contract_addr, &QueryMsg::Value {})
//             .unwrap();

//         assert_eq!(resp, ValueResp { value: 0 });
//     }

//     // #[test]
//     // fn query_incremented() {
//     //     let mut app = App::default();

//     //     let contract_id = app.store_code(counting_contract());

//     //     let contract_addr = app
//     //         .instantiate_contract(
//     //             contract_id,
//     //             Addr::unchecked("sender"),
//     //             &InstantiateMsg {
//     //                 counter: 10,
//     //                 minimal_donation: coin(10, "atom"),
//     //             },
//     //             &[],
//     //             "Counting contract",
//     //             None,
//     //         )
//     //         .unwrap();

//     //     let resp: ValueResp = app
//     //         .wrap()
//     //         .query_wasm_smart(contract_addr, &QueryMsg::Incremented { value: 1 })
//     //         .unwrap();

//     //     assert_eq!(resp, ValueResp { value: 11 });
//     // }

//     #[test]
//     fn donate() {
//         let mut app = App::default();

//         let contract_id = app.store_code(counting_contract());

//         let contract_addr = app
//             .instantiate_contract(
//                 contract_id,
//                 Addr::unchecked("sender"),
//                 &InstantiateMsg {
//                     counter: 0,
//                     minimal_donation: coin(10, "atom"),
//                 },
//                 &[],
//                 "Counting contract",
//                 None,
//             )
//             .unwrap();

//         app.execute_contract(
//             Addr::unchecked("sender"),
//             contract_addr.clone(),
//             &ExecMsg::Donate {},
//             &[],
//         )
//         .unwrap();

//         let resp: ValueResp = app
//             .wrap()
//             .query_wasm_smart(contract_addr, &QueryMsg::Value {})
//             .unwrap();

//         assert_eq!(resp, ValueResp { value: 0 });
//     }

//     #[test]
//     fn donate_with_funds() {
//         let sender = Addr::unchecked("sender");

//         let mut app = AppBuilder::new().build(|router, _api, storage| {
//             router
//                 .bank
//                 .init_balance(storage, &sender, coins(100, "atom"))
//                 .unwrap();
//         });

//         let contract_id = app.store_code(counting_contract());

//         let contract_addr = app
//             .instantiate_contract(
//                 contract_id,
//                 Addr::unchecked("sender"),
//                 &InstantiateMsg {
//                     counter: 0,
//                     minimal_donation: coin(10, "atom"),
//                 },
//                 &[],
//                 "Counting contract",
//                 None,
//             )
//             .unwrap();

//         app.execute_contract(
//             Addr::unchecked("sender"),
//             contract_addr.clone(),
//             &ExecMsg::Donate {},
//             &coins(10, "atom"),
//         )
//         .unwrap();

//         let resp: ValueResp = app
//             .wrap()
//             .query_wasm_smart(contract_addr.clone(), &QueryMsg::Value {})
//             .unwrap();
//         assert_eq!(resp, ValueResp { value: 1 });
//         assert_eq!(
//             app.wrap().query_all_balances(sender).unwrap(),
//             coins(90, "atom")
//         );
//         assert_eq!(
//             app.wrap().query_all_balances(contract_addr).unwrap(),
//             coins(10, "atom")
//         )
//     }

//     #[test]
//     fn reset() {
//         let mut app = App::default();

//         let contract_id = app.store_code(counting_contract());

//         let contract_addr = app
//             .instantiate_contract(
//                 contract_id,
//                 Addr::unchecked("sender"),
//                 &InstantiateMsg {
//                     counter: 0,
//                     minimal_donation: coin(10, "atom"),
//                 },
//                 &[],
//                 "Counting contract",
//                 None,
//             )
//             .unwrap();

//         app.execute_contract(
//             Addr::unchecked("sender"),
//             contract_addr.clone(),
//             &ExecMsg::Reset { counter: 10 },
//             &[],
//         )
//         .unwrap();

//         let resp: ValueResp = app
//             .wrap()
//             .query_wasm_smart(contract_addr, &QueryMsg::Value {})
//             .unwrap();

//         assert_eq!(resp, ValueResp { value: 10 });
//     }

//     #[test]
//     fn withdraw() {
//         let owner = Addr::unchecked("owner");
//         let sender = Addr::unchecked("sender");

//         let mut app = App::new(|router, _api, storage| {
//             router
//                 .bank
//                 .init_balance(storage, &sender, coins(100, "atom"))
//                 .unwrap();
//         });

//         let contract_id = app.store_code(counting_contract());

//         let contract_addr = app
//             .instantiate_contract(
//                 contract_id,
//                 owner.clone(),
//                 &InstantiateMsg {
//                     counter: 0,
//                     minimal_donation: coin(10, "atom"),
//                 },
//                 &[],
//                 "Counting contract",
//                 None,
//             )
//             .unwrap();

//         app.execute_contract(
//             sender.clone(),
//             contract_addr.clone(),
//             &ExecMsg::Donate {},
//             &coins(10, "atom"),
//         )
//         .unwrap();

//         assert_eq!(
//             app.wrap().query_all_balances(&contract_addr).unwrap(),
//             coins(10, "atom")
//         );

//         app.execute_contract(
//             owner.clone(),
//             contract_addr.clone(),
//             &ExecMsg::Withdraw {},
//             &[],
//         )
//         .unwrap();

//         assert_eq!(
//             app.wrap().query_all_balances(owner).unwrap(),
//             coins(10, "atom")
//         );
//         assert_eq!(
//             app.wrap().query_all_balances(sender).unwrap(),
//             coins(90, "atom")
//         );
//         assert_eq!(
//             app.wrap().query_all_balances(contract_addr).unwrap(),
//             vec![]
//         );
//     }

//     #[test]
//     fn withdraw_to() {
//         let owner = Addr::unchecked("owner");
//         let sender = Addr::unchecked("sender");
//         let receiver = Addr::unchecked("receiver");

//         let mut app = App::new(|router, _api, storage| {
//             router
//                 .bank
//                 .init_balance(storage, &sender, coins(100, "atom"))
//                 .unwrap();
//         });

//         let contract_id = app.store_code(counting_contract());

//         let contract_addr = app
//             .instantiate_contract(
//                 contract_id,
//                 owner.clone(),
//                 &InstantiateMsg {
//                     counter: 0,
//                     minimal_donation: coin(10, "atom"),
//                 },
//                 &[],
//                 "Counting contract",
//                 None,
//             )
//             .unwrap();

//         app.execute_contract(
//             sender.clone(),
//             contract_addr.clone(),
//             &ExecMsg::Donate {},
//             &coins(10, "atom"),
//         )
//         .unwrap();

//         app.execute_contract(
//             owner.clone(),
//             contract_addr.clone(),
//             &ExecMsg::WithdrawTo {
//                 receiver: receiver.to_string(),
//                 funds: coins(5, "atom"),
//             },
//             &[],
//         )
//         .unwrap();

//         assert_eq!(app.wrap().query_all_balances(owner).unwrap(), vec![]);
//         assert_eq!(
//             app.wrap().query_all_balances(sender).unwrap(),
//             coins(90, "atom")
//         );
//         assert_eq!(
//             app.wrap().query_all_balances(receiver).unwrap(),
//             coins(5, "atom")
//         );
//         assert_eq!(
//             app.wrap().query_all_balances(contract_addr).unwrap(),
//             coins(5, "atom")
//         );
//     }

//     #[test]
//     fn unauthorized_withdraw() {
//         let owner = Addr::unchecked("owner");
//         let member = Addr::unchecked("member");

//         let mut app = App::default();

//         let contract_id = app.store_code(counting_contract());

//         let contract_addr = app
//             .instantiate_contract(
//                 contract_id,
//                 owner.clone(),
//                 &InstantiateMsg {
//                     counter: 0,
//                     minimal_donation: coin(10, "atom"),
//                 },
//                 &[],
//                 "Counting contract",
//                 None,
//             )
//             .unwrap();

//         let err = app
//             .execute_contract(member, contract_addr, &ExecMsg::Withdraw {}, &[])
//             .unwrap_err();

//         assert_eq!(
//             ContractError::Unauthorized {
//                 owner: owner.into()
//             },
//             err.downcast().unwrap()
//         );
//     }

//     #[test]
//     fn unauthorized_withdraw_to() {
//         let owner = Addr::unchecked("owner");
//         let member = Addr::unchecked("member");

//         let mut app = App::default();

//         let contract_id = app.store_code(counting_contract());

//         let contract_addr = app
//             .instantiate_contract(
//                 contract_id,
//                 owner.clone(),
//                 &InstantiateMsg {
//                     counter: 0,
//                     minimal_donation: coin(10, "atom"),
//                 },
//                 &[],
//                 "Counting contract",
//                 None,
//             )
//             .unwrap();

//         let err = app
//             .execute_contract(
//                 member,
//                 contract_addr,
//                 &ExecMsg::WithdrawTo {
//                     receiver: owner.to_string(),
//                     funds: vec![],
//                 },
//                 &[],
//             )
//             .unwrap_err();

//         assert_eq!(
//             ContractError::Unauthorized {
//                 owner: owner.into()
//             },
//             err.downcast().unwrap()
//         );
//     }

//     #[test]
//     fn unauthorized_reset() {
//         let owner = Addr::unchecked("owner");
//         let member = Addr::unchecked("member");

//         let mut app = App::default();

//         let contract_id = app.store_code(counting_contract());

//         let contract_addr = app
//             .instantiate_contract(
//                 contract_id,
//                 owner.clone(),
//                 &InstantiateMsg {
//                     counter: 0,
//                     minimal_donation: coin(10, "atom"),
//                 },
//                 &[],
//                 "Counting contract",
//                 None,
//             )
//             .unwrap();

//         let err = app
//             .execute_contract(member, contract_addr, &ExecMsg::Reset { counter: 10 }, &[])
//             .unwrap_err();

//         assert_eq!(
//             ContractError::Unauthorized {
//                 owner: owner.into()
//             },
//             err.downcast().unwrap()
//         );
//     }
// }
