import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Tournament } from "../target/types/tournament";
import { LocalNfts, IDL } from "../target/types/local_nfts";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";


describe("tournament", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Tournament as Program<Tournament>;
  const nftProgram = anchor.workspace.LocalNfts as Program<LocalNfts>;

  it("works as intended!", async () => {

    const tx = await program.methods.battle().rpc();
    console.log("Your transaction signature", tx);
  });

});
