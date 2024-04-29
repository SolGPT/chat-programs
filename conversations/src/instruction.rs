// instruction.rs

use crate::state::{Conversation, Message};
use solana_program::{
    account_info::AccountInfo,
    pubkey::Pubkey,
    program_error::ProgramError,
    entrypoint::ProgramResult,
};
use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::account_info::next_account_info;
use solana_program::sysvar::Sysvar;
use crate::processor::process_create_conversation;

// create a new enum to represent the sender of a message
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum ChatMessageUser {
    User,
    Chat,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum ChatInstruction {
    /// Starts a new conversation
    CreateConversation {
        wallet_address: Pubkey,
        description: String,
        content_summary: Vec<String>,
        initial_messages: Option<Vec<Message>>,
    },
    /// Stores a new message in a conversation
    StoreMessage {
        user: ChatMessageUser,
        text: String,
        image_url: Option<String>,
        image_description: Option<String>,
    },
    ResetConversation {
        wallet_address: Pubkey,
        id: u8,
    },
    // Updates the content summary of a conversation
    // UpdateContentSummary {
    //     content_summary: Vec<String>,
    // },
    // Add more instructions as needed...
}

// Function to process instructions (moved from lib.rs)
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let _account = next_account_info(account_iter)?;

    let instruction = ChatInstruction::try_from_slice(_instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // You will add logic here to store and retrieve conversation data
    match instruction {
        ChatInstruction::CreateConversation { description, content_summary, wallet_address, initial_messages } => {
            let new_conversation = Conversation {
                wallet_address,
                date_created: solana_program::clock::Clock::get().unwrap().unix_timestamp as u64,
                messages: initial_messages.unwrap_or_default(),
                content_summary,
                description,
            };

            return process_create_conversation(
                program_id,
                accounts,
                new_conversation,
                &wallet_address,
            );
        },
        ChatInstruction::StoreMessage { text, image_url, image_description, user } => {
            // Handle store message logic...
        },
        // ChatInstruction::UpdateContentSummary { content_summary } => {
            // Handle update content summary logic...
        // },
        // Handle other instructions...
        ChatInstruction::ResetConversation { wallet_address, id } => {
            // Handle reset conversation logic...
        },
    }

    Ok(())
}