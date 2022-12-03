use cosmwasm_std::{DepsMut, Response, StdResult, MessageInfo};
use crate::state::{COUNTER, MINIMAL_DONATION, OWNER};

use crate::msg::{InstantiateMsg};

pub fn instantiate(deps: DepsMut, info: MessageInfo, msg: InstantiateMsg) -> StdResult<Response> {
    COUNTER.save(deps.storage, &0)?;
    MINIMAL_DONATION.save(deps.storage, &msg.minimal_donation)?;
    OWNER.save(deps.storage, &info.sender)?;
    Ok(Response::new())
}


pub mod query {
    use crate::msg::ValueResp;
    use crate::state::COUNTER;
    use cosmwasm_std::{Deps, StdResult};

    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        let value = COUNTER.load(deps.storage)?;
        Ok(ValueResp { value })
    }

    // pub fn incremented(value: u64) -> ValueResp {
    //     ValueResp { value: value + 1 }
    // }
}

pub mod exec {
    use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult, BankMsg, StdError, Env};

    use crate::state::{COUNTER, MINIMAL_DONATION, OWNER};

    // pub fn poke(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    //     let counter = COUNTER.load(deps.storage)? + 1;
    //     COUNTER.save(deps.storage, &counter)?;

    //     let resp = Response::new()
    //         .add_attribute("action", "poke")
    //         .add_attribute("sender", info.sender.as_str())
    //         .add_attribute("counter", counter.to_string());

    //     Ok(resp)
    // }

    pub fn reset(deps: DepsMut, info: MessageInfo, counter: u64) -> StdResult<Response> {
        let owner = OWNER.load(deps.storage)?;
        if info.sender != owner {
            return Err(StdError::generic_err("Unauthorized"));
        }

        COUNTER.save(deps.storage, &counter)?;

        let resp = Response::new()
            .add_attribute("action", "reset")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", counter.to_string());

        Ok(resp)
    }
    pub fn donate(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        let counter = COUNTER.load(deps.storage)?;
        let minimal_donation = MINIMAL_DONATION.load(deps.storage)?;

        if info.funds.iter().any(|coin| {
            coin.denom == minimal_donation.denom && coin.amount >= minimal_donation.amount
        }) {
            let mut counter = COUNTER.load(deps.storage)?;
            counter += 1;
            COUNTER.save(deps.storage, &counter)?;

        }

        let resp = Response::new()
            .add_attribute("action", "donate")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("minimal_donation", minimal_donation.to_string())
            .add_attribute("counter", counter.to_string());
        Ok(resp)
    }

    pub fn withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
        let owner = OWNER.load(deps.storage)?;
        if info.sender != owner {
            return Err(StdError::generic_err("Unauthorized"));
        }
    
        let balance = deps.querier.query_all_balances(&env.contract.address)?;
        let bank_msg = BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: balance,
        };
    
        let resp = Response::new()
            .add_message(bank_msg)
            .add_attribute("action", "withdraw")
            .add_attribute("sender", info.sender.as_str());
    
        Ok(resp)
    }

    pub fn withdraw_to(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        receiver: String,
        funds: Vec<Coin>,
    ) -> StdResult<Response> {
        let owner = OWNER.load(deps.storage)?;
        if info.sender != owner {
            return Err(StdError::generic_err("Unauthorized"));
        }

        let mut balance = deps.querier.query_all_balances(&env.contract.address)?;

        if !funds.is_empty() {
            for coin in &mut balance {
                let limit = funds
                    .iter()
                    .find(|c| c.denom == coin.denom)
                    .map(|c| c.amount)
                    .unwrap_or(Uint128::zero());

                coin.amount = std::cmp::min(coin.amount, limit);
            }
        }

        let bank_msg = BankMsg::Send {
            to_address: receiver,
            amount: balance,
        };

        let resp = Response::new()
            .add_message(bank_msg)
            .add_attribute("action", "withdraw")
            .add_attribute("sender", info.sender.as_str());

        Ok(resp)
    }
}
