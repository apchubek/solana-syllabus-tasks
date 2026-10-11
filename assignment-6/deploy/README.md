# Deployment Notes

## Prerequisites

- Solana CLI installed
- Anchor installed
- a funded payer keypair
- localnet or devnet access

## Suggested deploy flow

1. Set the desired cluster.
2. Build and deploy the Anchor program.
3. Save the deployed program id in this directory.
4. Run the client demo to initialize the profile PDA.
5. Run the update flow and confirm the changed fields.

## Example commands

```bash
anchor build
anchor deploy
cd client
npm install
RPC_URL=http://127.0.0.1:8899 PAYER_KEYPAIR=~/.config/solana/id.json npm run start -- demo
```

## PDA scheme

- seed prefix: `profile`
- authority seed: the wallet public key bytes
- account address: derived through `findProgramAddressSync`
