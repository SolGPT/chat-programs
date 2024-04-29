use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::pubkey::Pubkey;
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Message {
    pub sender: MessageType,
    pub text: String,
    pub image_url: Option<String>,
    pub image_description: Option<String>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum MessageType {
    User,
    Chat,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Conversation {
    pub wallet_address: Pubkey,
    pub date_created: u64,
    pub messages: Vec<Message>,
    pub content_summary: Vec<String>,
    pub description: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Conversations {
    pub conversations: Vec<Option<Conversation>>,
    pub max_conversations: usize,
    pub active_conversations: usize,
}

impl Conversations {
    pub fn new(max: usize) -> Self {
        Conversations {
            conversations: vec![None; max],
            max_conversations: max,
            active_conversations: 0,
        }
    }

    pub fn is_full(&self) -> bool {
        self.active_conversations == self.max_conversations
    }

    pub fn is_initialized(&self) -> bool {
        !self.conversations.is_empty()
    }

    pub fn from_account_info(account: &AccountInfo) -> Result<Self, ProgramError> {
        if account.data.borrow().len() == 0 {
            // Assuming a new account, initializing
            Ok(Self {
                conversations: Vec::new(),
                max_conversations: 3,
                active_conversations: 0,
            })
        } else {
            // Deserialize existing data
            Self::try_from_slice(&account.data.borrow())
                .map_err(|_| ProgramError::InvalidAccountData)
        }
    }

    /// Adds a new conversation if there is space available.
    pub fn add_conversation(&mut self, conversation: Conversation) -> Result<(), ProgramError> {
        if self.active_conversations < self.max_conversations {
            for slot in self.conversations.iter_mut() {
                if slot.is_none() {
                    *slot = Some(conversation);
                    self.active_conversations += 1;
                    return Ok(());
                }
            }
        }
        Err(ProgramError::Custom(0)) // No space available
    }

    pub fn remove_conversation(&mut self, index: usize) -> Result<(), ProgramError> {
        if index < self.conversations.len() && self.conversations[index].is_some() {
            self.conversations[index] = None;
            self.active_conversations -= 1; // Decrement the counter
            return Ok(());
        }
        Err(ProgramError::Custom(1)) // Invalid index or conversation does not exist
    }
}




// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_addition() {
        let mut conversations = Conversations::new(3);
        let conv = Conversation {
            wallet_address: Pubkey::new_unique(),
            date_created: 1625097600,
            messages: vec![],
            content_summary: vec!["test".into()],
            description: "Test description".into(),
        };

        assert_eq!(conversations.active_conversations, 0);
        assert!(conversations.add_conversation(conv.clone()).is_ok());
        assert_eq!(conversations.active_conversations, 1);
        assert!(conversations.is_initialized());
    }

    #[test]
    fn test_conversation_removal() {
        let mut conversations = Conversations::new(3);
        let conv = Conversation {
            wallet_address: Pubkey::new_unique(),
            date_created: 1625097600,
            messages: vec![],
            content_summary: vec!["test".into()],
            description: "Test description".into(),
        };

        conversations.add_conversation(conv).unwrap();
        assert_eq!(conversations.active_conversations, 1);

        assert!(conversations.remove_conversation(0).is_ok());
        assert_eq!(conversations.active_conversations, 0);
    }

    #[test]
    fn test_full_capacity() {
        let mut conversations = Conversations::new(1);
        let conv1 = Conversation {
            wallet_address: Pubkey::new_unique(),
            date_created: 1625097600,
            messages: vec![],
            content_summary: vec!["test1".into()],
            description: "Test description 1".into(),
        };

        let conv2 = Conversation {
            wallet_address: Pubkey::new_unique(),
            date_created: 1625097601,
            messages: vec![],
            content_summary: vec!["test2".into()],
            description: "Test description 2".into(),
        };

        conversations.add_conversation(conv1).unwrap();
        assert_eq!(conversations.add_conversation(conv2).is_err(), true);
        assert_eq!(conversations.active_conversations, 1);
    }
}

