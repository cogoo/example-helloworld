use borsh::{BorshDeserialize, BorshSerialize};
use std::cell::Ref;
use std::io::Error;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}
