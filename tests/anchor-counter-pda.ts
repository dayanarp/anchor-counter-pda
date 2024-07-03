import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorCounterPda } from "../target/types/anchor_counter_pda";

describe("anchor-counter-pda", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorCounterPda as Program<AnchorCounterPda>;

  let counterPk: anchor.web3.PublicKey;
  let bump: number;

  before(async () => {
     // counter PDA
     [counterPk, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("counter", "utf-8"), // seed "counter"
        provider.wallet.publicKey.toBuffer() // counter authority
      ],
      program.programId
    );

    console.log("Counter PDA Address: ", counterPk.toBase58());
    console.log("Counter canonical bump: ", bump);

  });

  it("Create counter", async () => {
    
    const tx = await program.methods.createCounter()
    .accounts({
      authority: provider.wallet.publicKey,
      counter:counterPk,
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Increment counter", async () => {
    
    const oldCount = await program.account.counter.fetch(counterPk);
    console.log("Old count: ", oldCount.count.toString())

    const tx = await program.methods.incrementCounter()
    .accounts({
      authority: provider.wallet.publicKey,
      counter:counterPk
    })
    .rpc();

    const newCounter = await program.account.counter.fetch(counterPk);
    console.log("New count: ", newCounter.count.toString());
    console.log("Your transaction signature", tx);
  });

  it("Delete counter", async () => {
    
    const beforeCounter = await program.account.counter.fetch(counterPk);
    console.log("Counter before:", beforeCounter);

    const tx = await program.methods.deleteCounter()
    .accounts({
      authority: provider.wallet.publicKey,
      counter:counterPk
    })
    .rpc();

    const afterCounter = await program.account.counter.fetchNullable(counterPk);
    console.log("Counter after:", afterCounter)
    console.log("Your transaction signature", tx);
  });
});
