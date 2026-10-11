# Assignment 6 - PDA On-Chain Profile

## Goal

Create a decentralized on-chain profile using a Program Derived Address and a client that can create, update, and read that profile.

## What this project should contain

- an on-chain profile account stored at a PDA
- deterministic PDA derivation from an authority pubkey and a seed prefix
- instructions for `initialize_profile` and `update_profile`
- a client that derives the PDA and calls the program
- tests that prove the same authority always maps to the same profile address

## Current structure

- `Anchor.toml` - Anchor workspace configuration
- `Cargo.toml` - Rust workspace manifest for the on-chain program
- `programs/profile/` - Anchor program with the profile account and PDA seeds
- `client/` - TypeScript client that derives the PDA and sends instructions
- `client/tests/` - deterministic PDA test
- `deploy/` - deployment notes and the PDA derivation scheme

## How the demo works

1. Build the Anchor workspace.
2. Deploy the profile program to localnet or Devnet.
3. Derive the PDA from the `profile` seed and the authority pubkey.
4. Call `initialize_profile` with handle, name, and bio.
5. Call `update_profile` to change one or more fields.
6. Read the profile back through the client and confirm the stored data.

## Recommended scope

Use this assignment to show why PDAs matter:

1. derive the profile address from seeds like `profile` + `authority`
2. create the account only through the program
3. store profile fields such as name, bio, avatar URL, or handle
4. allow the authority to update the profile
5. read the profile from the derived address in the client

## Acceptance criteria

- the profile address is deterministic and derived from seeds
- the program can create the profile account at the PDA
- the authority can update the stored data
- the client can read back the account state
- tests prove PDA derivation and state persistence

## Notes

This task is a natural continuation of Assignment 5, because it reuses Anchor accounts and adds PDA-based architecture.

## Current structure

- `Anchor.toml` - Anchor workspace configuration
- `Cargo.toml` - Rust workspace manifest
- `programs/profile/` - on-chain Anchor program for the profile logic
- `client/` - TypeScript client that derives the PDA and sends instructions
- `tests/` - deterministic PDA test scaffold

## How the demo is intended to work

1. Build and deploy the Anchor program to localnet or devnet.
2. Derive the profile PDA from the `profile` seed plus the authority pubkey.
3. Call `initialize_profile` to create the account.
4. Call `update_profile` to change handle, name, or bio.
5. Use the client `show` flow to read the PDA account back.

## Client commands

The client script understands the following commands:

- `demo` - initialize the profile and then update it once
- `init` - only create the profile account
- `update` - update the existing profile account
- `show` - read and print the current profile state

Example:

```bash
cd assignment-6/client
npm install
RPC_URL=http://127.0.0.1:8899 PROGRAM_ID=<deployed-program-id> PAYER_KEYPAIR=~/.config/solana/id.json npm run start -- demo
```

## Test notes

- Run `npm test` inside `assignment-6/client` to verify deterministic PDA derivation.
- The Rust program includes validation tests for handle, name, and bio constraints.
- The client validates the account owner and Anchor account discriminator before decoding profile data.
