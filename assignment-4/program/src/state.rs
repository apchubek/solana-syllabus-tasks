use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

pub const NOTE_MAX_LEN: usize = 128;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct NoteAccount {
    pub authority: Pubkey,
    pub message_len: u32,
    pub message: [u8; NOTE_MAX_LEN],
}

impl NoteAccount {
    pub const LEN: usize = 32 + 4 + NOTE_MAX_LEN;

    pub fn new(authority: Pubkey, message: &str) -> Result<Self, ProgramError> {
        let mut account = Self {
            authority,
            message_len: 0,
            message: [0; NOTE_MAX_LEN],
        };

        account.set_message(message)?;
        Ok(account)
    }

    pub fn set_message(&mut self, message: &str) -> Result<(), ProgramError> {
        let bytes = message.as_bytes();

        if bytes.len() > NOTE_MAX_LEN {
            return Err(ProgramError::InvalidInstructionData);
        }

        self.message = [0; NOTE_MAX_LEN];
        self.message[..bytes.len()].copy_from_slice(bytes);
        self.message_len = bytes.len() as u32;

        Ok(())
    }

    pub fn message(&self) -> Result<String, ProgramError> {
        let message_len = self.message_len as usize;

        if message_len > NOTE_MAX_LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        String::from_utf8(self.message[..message_len].to_vec())
            .map_err(|_| ProgramError::InvalidAccountData)
    }

    pub fn is_authority(&self, authority: &Pubkey) -> bool {
        &self.authority == authority
    }
}
