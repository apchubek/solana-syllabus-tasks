# Assignment 5 - Anchor Counter Program

## Goal

Build a simple Counter Program with Anchor and interact with it through a client.

## What this project should contain

- an Anchor workspace
- a counter account type that stores the current count
- instructions for `initialize`, `increment`, and optionally `decrement` or `reset`
- a client that calls the program and prints the count before and after each instruction
- tests that run against a local validator or Anchor test runner

## Recommended scope

This assignment should focus on Anchor conventions:

1. define program accounts with Anchor macros
2. derive and validate account constraints
3. serialize state into an Anchor account
4. write a client that derives the PDA or account address if needed
5. verify state changes through tests and logs

## Acceptance criteria

- the Anchor program builds successfully
- the counter account can be initialized
- the counter value can be incremented at least once
- the client shows the updated state
- tests pass on localnet or against the Anchor test validator

## Current structure

- `Anchor.toml` - Anchor workspace configuration
- `Cargo.toml` - Rust workspace manifest for the on-chain program
- `programs/counter/` - Anchor program with the counter account and instructions
- `client/` - TypeScript client that derives the PDA and sends instructions
- `client/tests/` - deterministic PDA test

## How the demo works

1. Build the Anchor workspace.
2. Deploy the counter program to localnet or Devnet.
3. Derive the counter PDA from `counter` + authority public key.
4. Call `initialize` to create the counter account.
5. Call `increment`, `decrement`, or `reset` as needed.
6. Read the account back through the client and confirm the count.

## Suggested folder contents

- `programs/counter/` - on-chain Anchor program
- `app/` or `client/` - client code that talks to the program
- `tests/` - integration tests
- `scripts/` - deployment or demo helpers

## Notes

If the first assignment is about raw Solana program mechanics, this one is about the Anchor development workflow and account constraints.

## Client commands

The client script understands the following commands:

- `demo` - initialize the counter and increment it once
- `init` - only create the counter account
- `increment` - increment the counter once
- `decrement` - decrement the counter once
- `reset` - reset the count to zero
- `show` - read and print the current counter state

Example:

```bash
cd assignment-5/client
npm install
RPC_URL=http://127.0.0.1:8899 PROGRAM_ID=<deployed-program-id> PAYER_KEYPAIR=~/.config/solana/id.json npm run start -- demo
```

## Test notes

- Run `npm test` inside `assignment-5/client` to verify PDA derivation.
- The Anchor program itself is intentionally small and can be extended with more account constraints or additional instructions if needed.
