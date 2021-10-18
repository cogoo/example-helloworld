use crate::instruction::HelloInstruction;
use crate::state::GreetingAccount;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = HelloInstruction::unpack(instruction_data)?;

    msg!("Instruction: {:?}", instruction);

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut greeting_account = GreetingAccount::unpack(&account.data.borrow())?;

    match instruction {
        HelloInstruction::SayHello => {
            greeting_account.counter += 1;
        }
        HelloInstruction::SayBye => {
            greeting_account.counter -= 1;
        }
        HelloInstruction::Greet { amount } => {
            greeting_account.counter += amount as u32;
        }
    }

    let mut data = &mut account.data.borrow_mut()[..];
    GreetingAccount::pack(&greeting_account, &mut data)?;

    msg!("Greeted {} time(s)!", greeting_account.counter);

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use solana_program::clock::Epoch;
    use std::mem;

    use super::*;

    #[test]
    fn test_hello() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data = vec![0];
        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::unpack(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );

        process(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::unpack(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );

        process(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::unpack(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
    }

    #[test]
    fn test_goodbye() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let hello_instruction_data = vec![0];
        let bye_instruction_data = vec![1];
        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::unpack(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );

        process(&program_id, &accounts, &hello_instruction_data).unwrap();

        process(&program_id, &accounts, &bye_instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::unpack(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
    }
}
