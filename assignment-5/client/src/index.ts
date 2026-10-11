import crypto from "node:crypto";
import fs from "node:fs";

import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";

const COUNTER_PROGRAM_ID = new PublicKey(
  process.env.PROGRAM_ID ?? "Fg6PaFpoGXkYsidMpWxTWqk1zXg7Z4jzq7N4Qx6gsJx",
);
const CONNECTION = new Connection(process.env.RPC_URL ?? "http://127.0.0.1:8899", "confirmed");

type Command = "demo" | "init" | "increment" | "decrement" | "reset" | "show";

async function main() {
  const command = (process.argv[2] as Command | undefined) ?? "demo";
  const payer = readKeypair(process.env.PAYER_KEYPAIR ?? "~/.config/solana/id.json");
  const authority = payer.publicKey;
  const [counterPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("counter"), authority.toBuffer()],
    COUNTER_PROGRAM_ID,
  );

  if (command === "show") {
    await showCounter(counterPda);
    return;
  }

  if (command === "init") {
    await sendInstruction(payer, buildInitializeInstruction(authority, counterPda));
    console.log(`initialized counter PDA: ${counterPda.toBase58()}`);
    await showCounter(counterPda);
    return;
  }

  if (command === "increment") {
    await sendInstruction(payer, buildModifyInstruction("increment", authority, counterPda));
    console.log(`incremented counter PDA: ${counterPda.toBase58()}`);
    await showCounter(counterPda);
    return;
  }

  if (command === "decrement") {
    await sendInstruction(payer, buildModifyInstruction("decrement", authority, counterPda));
    console.log(`decremented counter PDA: ${counterPda.toBase58()}`);
    await showCounter(counterPda);
    return;
  }

  if (command === "reset") {
    await sendInstruction(payer, buildModifyInstruction("reset", authority, counterPda));
    console.log(`reset counter PDA: ${counterPda.toBase58()}`);
    await showCounter(counterPda);
    return;
  }

  await sendInstruction(payer, buildInitializeInstruction(authority, counterPda));
  console.log(`initialized counter PDA: ${counterPda.toBase58()}`);
  await sendInstruction(payer, buildModifyInstruction("increment", authority, counterPda));
  console.log("incremented counter once");
  await showCounter(counterPda);
}

function buildInitializeInstruction(authority: PublicKey, counterPda: PublicKey) {
  return new TransactionInstruction({
    programId: COUNTER_PROGRAM_ID,
    keys: [
      { pubkey: authority, isSigner: true, isWritable: true },
      { pubkey: counterPda, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    data: anchorDiscriminator("initialize"),
  });
}

function buildModifyInstruction(name: "increment" | "decrement" | "reset", authority: PublicKey, counterPda: PublicKey) {
  return new TransactionInstruction({
    programId: COUNTER_PROGRAM_ID,
    keys: [
      { pubkey: authority, isSigner: true, isWritable: true },
      { pubkey: counterPda, isSigner: false, isWritable: true },
    ],
    data: anchorDiscriminator(name),
  });
}

async function sendInstruction(payer: Keypair, instruction: TransactionInstruction) {
  const tx = new Transaction().add(instruction);
  tx.feePayer = payer.publicKey;
  tx.recentBlockhash = (await CONNECTION.getLatestBlockhash()).blockhash;
  await sendAndConfirmTransaction(CONNECTION, tx, [payer]);
}

async function showCounter(counterPda: PublicKey) {
  const account = await CONNECTION.getAccountInfo(counterPda);

  if (!account) {
    console.log(`counter account not found: ${counterPda.toBase58()}`);
    return;
  }

  const authority = new PublicKey(account.data.subarray(8, 40));
  const count = Number(account.data.readBigUInt64LE(40));

  console.log(`counter PDA: ${counterPda.toBase58()}`);
  console.log(`authority: ${authority.toBase58()}`);
  console.log(`count: ${count}`);
}

function anchorDiscriminator(name: string) {
  const hash = crypto.createHash("sha256").update(`global:${name}`).digest();
  return hash.subarray(0, 8);
}

function readKeypair(path: string) {
  const resolvedPath = path.startsWith("~")
    ? `${process.env.HOME ?? ""}${path.slice(1)}`
    : path;

  const secretKey = JSON.parse(fs.readFileSync(resolvedPath, "utf8")) as number[];
  return Keypair.fromSecretKey(Uint8Array.from(secretKey));
}

void main().catch((error) => {
  console.error(error);
  process.exit(1);
});
