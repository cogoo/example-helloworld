use borsh::{BorshDeserialize, BorshSerialize};
use std::io::{Error, Result as IoResult};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}

impl GreetingAccount {
    pub fn unpack(account: &[u8]) -> Result<Self, Error> {
        GreetingAccount::try_from_slice(&account)
    }

    pub fn pack(greeting_account: &Self, x: &mut &mut [u8]) -> IoResult<()> {
        greeting_account.serialize(x)
    }
}
