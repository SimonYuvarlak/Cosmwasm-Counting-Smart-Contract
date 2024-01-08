/// Imports
/// We import Item from cw_storage_plus which will allow us to store and load data from the state
use cw_storage_plus::Item;

/// This is where we define the variable we want to store in the state
pub const COUNTER: Item<u64> = Item::new("counter"); // We use "counter" as the key for the variable in the state
