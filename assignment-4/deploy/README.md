# Deployment Notes

## Prerequisites

- Solana CLI installed
- a Devnet-funded payer keypair
- program and client crates built locally

## Suggested deploy flow

1. Set the cluster to Devnet.
2. Build the program as a Solana BPF artifact.
3. Deploy the program and save the returned program id.
4. Run the client demo with the deployed program id.
5. Copy the transaction signatures into the root README.

## Example commands

```bash
solana config set --url https://api.devnet.solana.com
solana address
cargo build-sbf --manifest-path program/Cargo.toml
solana program deploy target/deploy/assignment_4_program.so
cargo run -p assignment-4-client -- --program-id <DEPLOYED_PROGRAM_ID> demo
```

## Notes

- The program id used by the client must match the deployed program id.
- The demo command creates a new note account, initializes it, and optionally updates it in the same run.
