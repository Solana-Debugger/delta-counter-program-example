use borsh::BorshSerialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::{msg, system_instruction, system_program};
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use crate::state::Counter;

pub fn create_counter(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {

    // Get accounts
    let account_info_iter = &mut accounts.iter();

    let user_info = next_account_info(account_info_iter)?;
    let counter_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    // Verify user
    assert!(!user_info.is_writable);
    if !user_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Verify system
    assert!(system_program::check_id(system_info.key));

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

    // Calculate rent-exempt balance
    let rent = Rent::get()?;
    let space = std::mem::size_of::<Counter>();
    msg!("Space: {}", space);
    let rent_lamports = rent.minimum_balance(space);
    msg!("Lamports: {}", rent_lamports);

    // Create the counter account
    invoke_signed(
        &system_instruction::create_account(
            payer_info.key,
            counter_info.key,
            rent_lamports,
            space as u64,
            program_id,
        ),
        &[
            payer_info.clone(),
            counter_info.clone(),
            system_info.clone()
        ],
        &[&[b"counter", user_info.key.as_ref(), &[bump_seed]]],
    )?;

    // Initialize counter data
    let counter = Counter { count: 0 };
    counter.serialize(&mut *counter_info.data.borrow_mut())?;

    msg!("Counter account created successfully!");
    Ok(())
}