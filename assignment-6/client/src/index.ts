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

const PROGRAM_ID = new PublicKey(process.env.PROGRAM_ID ?? "Fg6PaFpoGXkYsidMpWxTWqk1zXg7Z4jzq7N4Qx6gsJx");
const CONNECTION = new Connection(process.env.RPC_URL ?? "http://127.0.0.1:8899", "confirmed");

type Command = "demo" | "init" | "update" | "show";

async function main() {
  const command = (process.argv[2] as Command | undefined) ?? "demo";
  const payer = readKeypair(process.env.PAYER_KEYPAIR ?? "~/.config/solana/id.json");
  const authority = payer.publicKey;
  const [profilePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), authority.toBuffer()],
    PROGRAM_ID,
  );

  if (command === "show") {
    await showProfile(profilePda);
    return;
  }

  if (command === "init") {
    await sendInstruction(payer, buildInitializeInstruction(authority, profilePda, defaultProfile()));
    console.log(`initialized profile PDA: ${profilePda.toBase58()}`);
    await showProfile(profilePda);
    return;
  }

  if (command === "update") {
    await sendInstruction(payer, buildUpdateInstruction(authority, profilePda, updatedProfile()));
    console.log(`updated profile PDA: ${profilePda.toBase58()}`);
    await showProfile(profilePda);
    return;
  }

  await sendInstruction(payer, buildInitializeInstruction(authority, profilePda, defaultProfile()));
  console.log(`initialized profile PDA: ${profilePda.toBase58()}`);
  await sendInstruction(payer, buildUpdateInstruction(authority, profilePda, updatedProfile()));
  console.log("updated profile once");
  await showProfile(profilePda);
}

function defaultProfile() {
  return {
    handle: "student",
    name: "Web3 Student",
    bio: "First version of the on-chain profile",
  };
}

function updatedProfile() {
  return {
    handle: "builder",
    name: "Web3 Builder",
    bio: "Updated from the client through the PDA account",
  };
}

function buildInitializeInstruction(
  authority: PublicKey,
  profilePda: PublicKey,
  profile: { handle: string; name: string; bio: string },
) {
  return new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
      { pubkey: authority, isSigner: true, isWritable: true },
      { pubkey: profilePda, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    data: encodeInstruction("initialize_profile", profile.handle, profile.name, profile.bio),
  });
}

function buildUpdateInstruction(
  authority: PublicKey,
  profilePda: PublicKey,
  profile: { handle: string; name: string; bio: string },
) {
  return new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
      { pubkey: authority, isSigner: true, isWritable: true },
      { pubkey: profilePda, isSigner: false, isWritable: true },
    ],
    data: encodeInstruction("update_profile", profile.handle, profile.name, profile.bio),
  });
}

async function sendInstruction(payer: Keypair, instruction: TransactionInstruction) {
  const tx = new Transaction().add(instruction);
  tx.feePayer = payer.publicKey;
  tx.recentBlockhash = (await CONNECTION.getLatestBlockhash()).blockhash;
  await sendAndConfirmTransaction(CONNECTION, tx, [payer]);
}

async function showProfile(profilePda: PublicKey) {
  const account = await CONNECTION.getAccountInfo(profilePda);

  if (!account) {
    console.log(`profile not found: ${profilePda.toBase58()}`);
    return;
  }

  if (!account.owner.equals(PROGRAM_ID)) {
    throw new Error(`unexpected owner: ${account.owner.toBase58()}`);
  }

  const parsed = decodeProfile(account.data);
  console.log(`profile PDA: ${profilePda.toBase58()}`);
  console.log(`authority: ${parsed.authority.toBase58()}`);
  console.log(`handle: ${parsed.handle}`);
  console.log(`name: ${parsed.name}`);
  console.log(`bio: ${parsed.bio}`);
}

function decodeProfile(data: Buffer) {
  const discriminator = anchorAccountDiscriminator("ProfileAccount");

  if (data.length < discriminator.length + 32 + 1 + 4) {
    throw new Error("profile account data is too short");
  }

  if (!data.subarray(0, discriminator.length).equals(discriminator)) {
    throw new Error("invalid profile account discriminator");
  }

  let offset = 8;
  const authority = new PublicKey(data.subarray(offset, offset + 32));
  offset += 32;
  const bump = data.readUInt8(offset);
  offset += 1;

  const handle = readString(data, offset);
  offset = handle.nextOffset;
  const name = readString(data, offset);
  offset = name.nextOffset;
  const bio = readString(data, offset);

  return {
    authority,
    bump,
    handle: handle.value,
    name: name.value,
    bio: bio.value,
  };
}

function readString(data: Buffer, offset: number) {
  const length = data.readUInt32LE(offset);
  const start = offset + 4;
  const end = start + length;

  if (end > data.length) {
    throw new Error("profile account contains truncated string data");
  }

  return {
    value: data.subarray(start, end).toString("utf8"),
    nextOffset: end,
  };
}

function encodeInstruction(name: string, handle: string, profileName: string, bio: string) {
  return Buffer.concat([
    anchorDiscriminator(name),
    encodeString(handle),
    encodeString(profileName),
    encodeString(bio),
  ]);
}

function encodeString(value: string) {
  const bytes = Buffer.from(value, "utf8");
  const length = Buffer.alloc(4);
  length.writeUInt32LE(bytes.length, 0);
  return Buffer.concat([length, bytes]);
}

function anchorDiscriminator(name: string) {
  return crypto.createHash("sha256").update(`global:${name}`).digest().subarray(0, 8);
}

function anchorAccountDiscriminator(name: string) {
  return crypto.createHash("sha256").update(`account:${name}`).digest().subarray(0, 8);
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
