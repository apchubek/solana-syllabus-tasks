use assignment_4_program::{instruction::NoteInstruction, state::{NoteAccount, NOTE_MAX_LEN}};
use borsh::BorshDeserialize;
use solana_program::pubkey::Pubkey;

#[test]
fn note_account_roundtrips_through_borsh() {
    let authority = Pubkey::new_unique();
    let account = NoteAccount::new(authority, "hello solana").expect("state");
    let bytes = borsh::to_vec(&account).expect("serialize");
    let decoded = NoteAccount::try_from_slice(&bytes).expect("deserialize");

    assert_eq!(decoded.authority, authority);
    assert_eq!(decoded.message().expect("message"), "hello solana");
    assert_eq!(NoteAccount::LEN, 32 + 4 + NOTE_MAX_LEN);
}

#[test]
fn instruction_roundtrips_through_borsh() {
    let instruction = NoteInstruction::Update {
        message: "new note".to_string(),
    };

    let bytes = instruction.pack().expect("pack");
    let decoded = NoteInstruction::try_from_slice(&bytes).expect("unpack");

    assert_eq!(decoded, instruction);
}
