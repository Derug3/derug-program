import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DerugProgram } from "../target/types/derug_program";

describe("derug-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DerugProgram as Program<DerugProgram>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
