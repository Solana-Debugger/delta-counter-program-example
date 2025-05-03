mod process_create_counter;

mod process_increase_counter;

use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use {
    process_create_counter::*,
    process_increase_counter::*,
};

use crate::instruction::CounterInstruction;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let instruction = CounterInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        CounterInstruction::CreateCounter => {
            create_counter(program_id, accounts)
        },
        CounterInstruction::IncreaseCounter { delta } => {
            increase_counter(program_id, accounts, delta)
        },
    }
}
