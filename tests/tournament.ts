import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Tournament } from "../target/types/tournament";

describe("tournament", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Tournament as Program<Tournament>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
