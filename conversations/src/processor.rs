use borsh::{BorshSerialize};
use crate::state::{Conversation, Conversations};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    pubkey::Pubkey,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg
};

pub fn process_create_conversation(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    new_conversation: Conversation,
    user_wallet_address: &Pubkey,
) -> ProgramResult {

    // Iterator for handling accounts
    let account_info_iter = &mut accounts.iter();

    // Deriving PDA for storing the user's conversations
    let (user_pda, bump_seed) = Pubkey::find_program_address(&[&user_wallet_address.to_bytes()], program_id);

    // User's PDA Account
    let user_account = next_account_info(account_info_iter)?;
    if user_account.key != &user_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    // safety clause to ensure the account is owned by the program
    if user_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize or initialize user's conversation data
    let mut user_data: Conversations = Conversations::from_account_info(user_account)?;

    // Add the new conversation
    user_data.add_conversation(new_conversation)?;

    // Serialize and save the updated data back to the account
    user_data.serialize(&mut *user_account.data.borrow_mut())?;

    Ok(())
}

pub fn process_reset_conversation(
    accounts: &[AccountInfo],
    // Parameters for the reset
) -> ProgramResult {
    // Implement the logic for resetting a conversation here
    msg!("Instruction: ResetConversation");
    Ok(())
}

pub fn process_store_message(
    accounts: &[AccountInfo],
    // Parameters for the message
) -> ProgramResult {
    // Implement the logic for storing a message here
    msg!("Instruction: StoreMessage");
    Ok(())
}

pub fn process_update_content_summary(
    accounts: &[AccountInfo],
    // Parameters for the update
) -> ProgramResult {
    // Implement the logic for updating the content summary here
    msg!("Instruction: UpdateContentSummary");
    Ok(())
}


// TODO: implement tests
