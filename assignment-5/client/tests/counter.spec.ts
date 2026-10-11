import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { PublicKey } from "@solana/web3.js";

const PROGRAM_ID = new PublicKey("Fg6PaFpoGXkYsidMpWxTWqk1zXg7Z4jzq7N4Qx6gsJx");

describe("counter PDA", () => {
  it("derives the same address for the same authority", () => {
    const authority = new PublicKey("11111111111111111111111111111111");
    const first = PublicKey.findProgramAddressSync(
      [Buffer.from("counter"), authority.toBuffer()],
      PROGRAM_ID,
    );
    const second = PublicKey.findProgramAddressSync(
      [Buffer.from("counter"), authority.toBuffer()],
      PROGRAM_ID,
    );

    assert.equal(first[0].toBase58(), second[0].toBase58());
    assert.equal(first[1], second[1]);
  });
});