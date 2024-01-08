/// This module includes the contracts execution logic  
pub mod exec {
    // Necessary imports
    use crate::state::COUNTER;
    use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};

    /// This function increments the counter
    pub fn increment(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        // Load and increment the counter
        let counter = COUNTER
            .load(deps.storage)? // load the counter value
            .checked_add(1u64) // add one, this will return a Result
            .unwrap_or(COUNTER.load(deps.storage)?); // if you cannot unwrap the result then return the value from the state

        // Save the counter
        COUNTER.save(deps.storage, &counter)?;

        // Return a response
        // Response includes attributes which are key value pairs
        // Attributes lets the chain know what happened in this operation
        Ok(Response::new()
            .add_attribute("action", "poke")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", counter.to_string()))
    }

    /// This function resets the counter
    pub fn reset(deps: DepsMut, info: MessageInfo, counter_value: u64) -> StdResult<Response> {
        // Save the counter with the given value
        COUNTER.save(deps.storage, &counter_value)?;

        // Return a response
        // Response includes attributes which are key value pairs
        // Attributes lets the chain know what happened in this operation
        Ok(Response::new()
            .add_attribute("action", "reset")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", counter_value.to_string()))
    }
}

/// This module includes the contracts query logic
pub mod query {
    // Necessary imports
    use crate::{msg::ValueResp, state::COUNTER};
    use cosmwasm_std::{Deps, StdResult};

    /// This function returns the counter value
    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        // Load the counter value
        let value = COUNTER.load(deps.storage)?;

        // Return the value
        Ok(ValueResp { value })
    }
}
