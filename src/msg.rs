/// This file contains the messages that can be sent to the contract.
// Necessary imports
use serde::{Deserialize, Serialize};

/// This message is sent when the contract is instantiated
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub counter_value: u64,
}

/// This message is sent when the contract is executed
/// We are storing different possible messages that will invoke different functions in an enum
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {},                 // This message will invoke the increment function
    Reset { counter_value: u64 }, // This message will invoke the reset function
}

/// This message is sent when the contract is queried
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Value {}, // This message will invoke the value function
}

/// This is a custom response that we return for the query
/// We use custom return types for the queries because we may want to manipulate or transform the data before returning it
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ValueResp {
    pub value: u64, // This is the value we want to return
}
