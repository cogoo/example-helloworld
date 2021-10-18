use solana_program::program_error::ProgramError;
use std::convert::TryInto;

#[derive(Debug)]
pub enum HelloInstruction {
    SayHello,
    SayBye,
    Greet { amount: u8 },
}

impl HelloInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => HelloInstruction::SayHello,
            1 => HelloInstruction::SayBye,
            2 => {
                let amount = rest
                    .get(..rest.len())
                    .and_then(|slice| slice.try_into().ok())
                    .map(u8::from_le_bytes)
                    .ok_or(ProgramError::Custom(1))?;

                HelloInstruction::Greet { amount }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
