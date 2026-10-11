use std::path::PathBuf;

use anyhow::{Context, Result};
use borsh::BorshDeserialize;
use assignment_4_program::{instruction::NoteInstruction, state::NoteAccount};
use clap::{Parser, Subcommand};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

#[derive(Parser, Debug)]
#[command(name = "assignment-4-client", about = "Client for the first Solana program")]
struct Cli {
    #[arg(long, default_value = "https://api.devnet.solana.com")]
    rpc_url: String,

    #[arg(long)]
    program_id: Pubkey,

    #[arg(long, default_value = "~/.config/solana/id.json")]
    payer: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Demo {
        #[arg(long, default_value = "Hello from assignment 4")]
        message: String,

        #[arg(long)]
        update_message: Option<String>,
    },
    Update {
        #[arg(long)]
        note_account: Pubkey,

        #[arg(long)]
        message: String,
    },
    Show {
        #[arg(long)]
        note_account: Pubkey,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let rpc = RpcClient::new_with_commitment(cli.rpc_url.clone(), CommitmentConfig::confirmed());
    let payer = read_keypair(&cli.payer)?;

    match cli.command {
        Command::Demo { message, update_message } => run_demo(&rpc, &payer, cli.program_id, &message, update_message.as_deref()),
        Command::Update { note_account, message } => run_update(&rpc, &payer, cli.program_id, note_account, &message),
        Command::Show { note_account } => run_show(&rpc, note_account),
    }
}

fn run_demo(
    rpc: &RpcClient,
    payer: &Keypair,
    program_id: Pubkey,
    message: &str,
    update_message: Option<&str>,
) -> Result<()> {
    let note_account = Keypair::new();
    let rent = rpc.get_minimum_balance_for_rent_exemption(NoteAccount::LEN)?;
    let create_account = system_instruction::create_account(
        &payer.pubkey(),
        &note_account.pubkey(),
        rent,
        NoteAccount::LEN as u64,
        &program_id,
    );
    let initialize = build_program_instruction(
        program_id,
        payer.pubkey(),
        note_account.pubkey(),
        NoteInstruction::Initialize {
            message: message.to_string(),
        },
    )?;

    let recent_blockhash = rpc.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[create_account, initialize],
        Some(&payer.pubkey()),
        &[payer, &note_account],
        recent_blockhash,
    );

    let init_sig = rpc.send_and_confirm_transaction(&tx)?;
    println!("Initialized note account: {}", note_account.pubkey());
    println!("Initialization signature: {}", init_sig);

    if let Some(update_message) = update_message {
        run_update(rpc, payer, program_id, note_account.pubkey(), update_message)?;
    }

    run_show(rpc, note_account.pubkey())
}

fn run_update(
    rpc: &RpcClient,
    payer: &Keypair,
    program_id: Pubkey,
    note_account: Pubkey,
    message: &str,
) -> Result<()> {
    let instruction = build_program_instruction(
        program_id,
        payer.pubkey(),
        note_account,
        NoteInstruction::Update {
            message: message.to_string(),
        },
    )?;

    let recent_blockhash = rpc.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );

    let signature = rpc.send_and_confirm_transaction(&tx)?;
    println!("Updated note account: {}", note_account);
    println!("Update signature: {}", signature);
    run_show(rpc, note_account)
}

fn run_show(rpc: &RpcClient, note_account: Pubkey) -> Result<()> {
    let account = rpc.get_account(&note_account)?;
    let state = NoteAccount::try_from_slice(&account.data)?;
    let message = state.message()?;

    println!("Account: {}", note_account);
    println!("Authority: {}", state.authority);
    println!("Message: {}", message);
    println!("Lamports: {}", account.lamports);
    println!("Owner: {}", account.owner);

    Ok(())
}

fn build_program_instruction(
    program_id: Pubkey,
    authority: Pubkey,
    note_account: Pubkey,
    instruction: NoteInstruction,
) -> Result<solana_sdk::instruction::Instruction> {
    let data = instruction.pack()?;
    Ok(solana_sdk::instruction::Instruction {
        program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new_readonly(authority, true),
            solana_sdk::instruction::AccountMeta::new(note_account, false),
        ],
        data,
    })
}

fn read_keypair(path: &str) -> Result<Keypair> {
    let expanded = path
        .strip_prefix('~')
        .map(|suffix| {
            let home = std::env::var("HOME").context("HOME is not set")?;
            Ok::<String, anyhow::Error>(format!("{}{}", home, suffix))
        })
        .transpose()? 
        .unwrap_or_else(|| path.to_string());

    read_keypair_file(&PathBuf::from(expanded)).context("failed to read payer keypair")
}
