# Assignment 4 - First Solana Program

## Goal

Build the first real Solana program without Anchor, deploy it to Devnet, and prove that an external client can call it.

## What this project should contain

- a standalone Solana program written in Rust with `solana-program`
- an instruction enum with at least one meaningful action, for example `initialize`, `greet`, or `set_message`
- account validation logic for the accounts required by the instruction
- program ID management and deploy instructions for Devnet
- a small client or CLI script that sends the instruction and reads the result
- a short README section with the deployed program address and the transaction signature

## Current structure

- `program/` - the on-chain Rust program written with `solana-program`
- `client/` - a command-line client that builds and submits transactions
- `deploy/` - deployment notes, commands, and the deployed program id
- `program/tests/` - Rust tests for serialization and account state behavior

## How the demo works

1. Build the program.
2. Deploy the program to Devnet.
3. Create a note account owned by the deployed program.
4. Send an `initialize` instruction with an initial message.
5. Send an `update` instruction to change the stored message.
6. Read the account back and confirm the stored data.

## Recommended scope

Keep the program intentionally small. The point of this task is to understand the full deployment flow, not to build a large product.

Suggested implementation:

1. Define one instruction.
2. Parse instruction data manually.
3. Read and/or modify one account.
4. Deploy to Devnet.
5. Call the program from a client.
6. Verify the result in Solana Explorer.

## Acceptance criteria

- the program compiles successfully
- the program is deployed to Devnet
- the client can invoke the program at least once
- the transaction is visible in Explorer
- the README explains how to reproduce the result

## Client commands

The client supports a simple demo flow plus separate read and update actions.

- `demo` - create the account, initialize it, and optionally update it in the same run
- `update` - update an existing note account
- `show` - print the current on-chain state

## Suggested folder contents

- `program/` - Solana program source
- `client/` - script or app used to call the program
- `deploy/` - notes, IDs, and commands
- `tests/` - integration notes or validator tests

## Test notes

- `program/tests/` contains Rust tests for the instruction and account model.
- The client is designed to work against Devnet or a local validator once the program id is set correctly.
- The deployed program id should be recorded in `deploy/README.md` after the first successful deployment.
