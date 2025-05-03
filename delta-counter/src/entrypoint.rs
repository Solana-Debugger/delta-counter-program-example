#![cfg(all(target_os = "solana", not(feature = "no-entrypoint")))]

use {
    crate::processor,
    solana_program::{
        pubkey::Pubkey,
        account_info::AccountInfo,
        entrypoint::ProgramResult,
    },
};

solana_program::entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    processor::process_instruction(program_id, accounts, instruction_data)
}
