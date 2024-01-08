/// Imports
// Cosmwasm Standard Library
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
// Messages
use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
// Contract Logic
use contract::{exec, query};
// State variables
use state::COUNTER;
// Modules
mod contract; // This modules is private because it has the contract logic in it
pub mod msg; // This module is public because we want to be able to call these messages outside of the contract, like other contracts.
mod state; // This modules holds the state of the contract

/// Entry point for contract instantiation
/// Instantiate method run once when the contract is instantiated
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    COUNTER.save(deps.storage, &msg.counter_value)?; // Save the value 0 to the COUNTER variable in the state

    Ok(Response::new()) // Return empty response
}

/// Entry point for contract execution
/// This is where contract logic for execution is called
/// Execute functions alter the state of the contract
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        // Call functions from the exec module depending on the message received
        ExecuteMsg::Increment {} => exec::increment(deps, info),
        ExecuteMsg::Reset { counter_value } => exec::reset(deps, info, counter_value),
    }
}

/// Entry point for contract query
/// This is where contract logic for queries is called
/// Query functions do not alter the state of the contract
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Value {} => to_json_binary(&query::value(deps)?),
    }
}

/// Tests
/// Tests are run with cargo test
/// Testing is particularly important
/// We can test in the same or seperate files
#[cfg(test)]
mod test {
    // Imports
    use crate::{
        execute, instantiate,
        msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ValueResp},
        query,
    };
    use cosmwasm_std::{Addr, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    // This function will create a new contract for our local tests
    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    /// This function will test whether we can instantiate the contract and query the correct value
    /// In general it is better to test instantiation and query function seperately
    /// Since this is a small example, I will test them in the same place
    #[test]
    fn test_query_value() {
        // Create default app instance
        let mut app = App::default();

        // This simulates storing the contract code in the blockchain even though we store nothing here
        let contract_id = app.store_code(counting_contract());

        // This function will instantiate the contract and return the contract address
        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg { counter_value: 1 },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        // This function will query the contract for the value
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        // Here we check whether the value is equal to 1 which is what we initialized it to
        assert_eq!(resp, ValueResp { value: 1 });
    }

    /// This function will test whether we can increment the value of the counter
    #[test]
    fn test_increment() {
        // Create default app instance
        let mut app = App::default();

        // This simulates storing the contract code in the blockchain even though we store nothing here
        let contract_id = app.store_code(counting_contract());

        // This function will instantiate the contract and return the contract address
        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg { counter_value: 0 },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        // This function will call the increment function from the contract which is an execute function
        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(),
            &ExecuteMsg::Increment {},
            &[],
        )
        .unwrap();

        // This function will query the contract for the value
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        // Here we check whether the value is equal to 1
        // We initialized the value to 0 and incremented it by 1
        assert_eq!(resp, ValueResp { value: 1 });
    }

    /// This function will test whether we can reset the value of the counter
    /// In the reset function we are resetting the contract to a certain value
    #[test]
    fn test_reset() {
        // Create default app instance
        let mut app = App::default();

        // This simulates storing the contract code in the blockchain even though we store nothing here
        let contract_id = app.store_code(counting_contract());

        // This function will instantiate the contract and return the contract address
        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg { counter_value: 0 },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        // This function will call the reset function from the contract which is an execute function
        // We are setting the value to 5
        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(),
            &ExecuteMsg::Reset { counter_value: 5 },
            &[],
        )
        .unwrap();

        // This function will query the contract for the value
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        // Here we check whether the value is equal to 5
        assert_eq!(resp, ValueResp { value: 5 });
    }
}
