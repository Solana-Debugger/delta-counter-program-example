use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use solana_program::account_info::next_account_info;
use solana_program::msg;
use {
    solana_program::{
        pubkey::Pubkey,
        account_info::AccountInfo,
        entrypoint::ProgramResult,
    },
};
use crate::state::Counter;

pub fn increase_counter(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    delta: u8,
) -> ProgramResult {

    // Get accounts
    let account_info_iter = &mut accounts.iter();

    let user_info = next_account_info(account_info_iter)?;
    let counter_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;

    // Verify user
    assert!(!user_info.is_writable);
    if !user_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Verify payer
    assert!(payer_info.is_signer);
    assert!(payer_info.is_writable);

    // Verify counter
    assert!(!counter_info.is_signer);
    assert!(counter_info.is_writable);

    let (expected_counter_pubkey, bump_seed) = Pubkey::find_program_address(
        &[b"counter", user_info.key.as_ref()],
        program_id,
    );

    if expected_counter_pubkey != *counter_info.key {
        msg!("Error: Counter account address does not match derived PDA");
        return Err(ProgramError::InvalidArgument);
    }

    let mut counter_data = counter_info.try_borrow_mut_data()?;
    let mut counter = Counter::try_from_slice(&counter_data)?;

    // Increase counter
    counter.count = counter.count.checked_add(delta)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    msg!("Counter incremented by {}, new value: {}", delta, counter.count);

    // Save updated counter data
    counter.serialize(&mut *counter_data)?;

    Ok(())
}