# Counting Contract
This contract is very similar to the contract example on cosmwasm academy https://academy.cosmwasm.com/learn/smart-contracts.

That being said, contract have some differences in the code and also the code has documented and commented for you to understand better.

## Project Structure
We can think cosmwasm smart contract in three parts.

### 1st State
State is where we store our data. 
In the example you can find it under `state.rs`.

### 2nd Messages
Messages are how cosmwasm smart contracts get executed.
First we have the state, and using messages we can call function either to retirieve or manipulate data which we store in the state of the smart contract.
You can find messages under the `msg.rs` file. 

### 3rd Contract Logic
As the third part, we have contract logic.
This is where the magic happens. 
We have our state and with the messages we have possible scenerios that may happen.
With this contract logic part we are saying how this state alteration or data retrieval should work. 
You can find the contract logic under the contract.rs

### lib.rs
In the lib.rs file we have 3 entry points. Since we do not have a `main.rs` file, we need some entry points where the smart contract will run.
We have three different entry points:
- `instantiate` This is where we instantiate the smart contract. This only runs once when we deploy the contract.
- `execute` This is the part where we call function that will alter the state based on the message we have received.
- `query` This entry point is for data retrieval. We query the contract state and return the data based on the messages provided.

Finally, you will find `tests` in the `lib.rs`.
Testing is a crucial part of software development and even more crucial part of smart contract development. This is a whole other topic that needs more attention and time, so if you are new to smart contract and blockchain development, I suggest you search the importance of testing.

In our contract, we created tests on the lib.rs file since the project is small but you can also use have a seperate file.

## Install
To run the project, you are going to need:
- `Rust`
- `Cargo`
- `wasm32-unknown-unknown`
- `cosmwasm-check`

For installing Rust and cargo, I suggest you to go to the official site.
For `wasm32-unknown-unknown` you can use the command: `rustup target add wasm32-unknown-unknown`.
For `cosmwasm-check`, you can install it using the command `cargo install cosmwasm-check`.

## Build
You can build the project with the command `cargo build`.

## Wasm
To build wasm, you can use the command: `cargo wasm`.
After this command, you may want to check if this Rust project is a propoer `cosmwasm smart contract`. To do that, in the root of the project run the command: `cosmwasm-check ./target/wasm32-unknown-unknown/release/counting_contract.wasm`.

## Testing
You can test the contract using the command `cargo test`.

_I hope this repo works both as an introduction and as a template_

