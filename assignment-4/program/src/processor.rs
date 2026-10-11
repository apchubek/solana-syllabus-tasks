use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{instruction::NoteInstruction, state::NoteAccount};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = NoteInstruction::unpack(instruction_data)?;

    match instruction {
        NoteInstruction::Initialize { message } => {
            msg!("instruction: initialize");
            process_initialize(program_id, accounts, &message)
        }
        NoteInstruction::Update { message } => {
            msg!("instruction: update");
            process_update(program_id, accounts, &message)
        }
    }
}

fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    message: &str,
) -> ProgramResult {
    let mut accounts_iter = accounts.iter();
    let authority = next_account_info(&mut accounts_iter)?;
    let note_account = next_account_info(&mut accounts_iter)?;

    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if note_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let state = NoteAccount::new(*authority.key, message)?;
    write_state(note_account, &state)
}

fn process_update(program_id: &Pubkey, accounts: &[AccountInfo], message: &str) -> ProgramResult {
    let mut accounts_iter = accounts.iter();
    let authority = next_account_info(&mut accounts_iter)?;
    let note_account = next_account_info(&mut accounts_iter)?;

    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if note_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut state = read_state(note_account)?;

    if !state.is_authority(authority.key) {
        return Err(ProgramError::IllegalOwner);
    }

    state.set_message(message)?;
    write_state(note_account, &state)
}

fn read_state(note_account: &AccountInfo) -> Result<NoteAccount, ProgramError> {
    let data = note_account.try_borrow_data()?;
    NoteAccount::try_from_slice(&data).map_err(|_| ProgramError::InvalidAccountData)
}

fn write_state(note_account: &AccountInfo, state: &NoteAccount) -> ProgramResult {
    let serialized = borsh::to_vec(state).map_err(|_| ProgramError::InvalidAccountData)?;
    let mut data = note_account.try_borrow_mut_data()?;

    if serialized.len() > data.len() {
        return Err(ProgramError::AccountDataTooSmall);
    }

    data[..serialized.len()].copy_from_slice(&serialized);
    Ok(())
}
