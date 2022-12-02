pub mod query {
    use crate::msg::ValueResp;
    use crate::state::COUNTER;
    use cosmwasm_std::{Deps, StdResult};

    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        let value = COUNTER.load(deps.storage)?;
        Ok(ValueResp { value })
    }

    pub fn incremented(value: u64) -> ValueResp {
        ValueResp { value: value + 1 }
    }
}
