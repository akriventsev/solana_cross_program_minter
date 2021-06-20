use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use spl_token::instruction::mint_to;

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the accounts to mint
    let token_program_id = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let recipient_account = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    msg!("Creating mint instruction");
    let mint_ix = mint_to(
        &token_program_id.key,
        &mint.key,
        &recipient_account.key,
        &pda_account.key,
        &[],
        100000000,
    )?;

    let res = invoke_signed(
        &mint_ix,
        &[
            mint.clone(),
            recipient_account.clone(),
            pda_account.clone(),
            token_program_id.clone(),
        ],
        &[&[&b"superminter"[..]]],
    );

    match res {
        Err(unwrapped_error) => {
            msg!("Error: {:}", unwrapped_error);
        }
        _ => {}
    }

    Ok(())
}
