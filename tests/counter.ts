import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { PublicKey } from "@solana/web3.js";

describe("counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Counter as Program<Counter>;

  const [counterPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("counter")],
    program.programId
  );

  console.log("PDA: ", counterPDA);

  it("Is initialized!", async () => {
    try {
      // Invoke the initialize instruction
      const transactionSignature = await program.methods
        .initialize()
        .accounts({
          counter: counterPDA,
        })
        .rpc();

      // Fetch the counter account data
      const accountData = await program.account.counter.fetch(counterPDA);

      console.log(`Transaction Signature: ${transactionSignature}`);
      console.log(`Count: ${accountData.count}`);
    } catch (error) {
      // If PDA Account already created, then we expect an error
      console.log(error);
    }
  });

  it("Increment", async () => {
    // Invoke the increment instruction
    const transactionSignature = await program.methods
      .increment()
      .accounts({
        counter: counterPDA,
      })
      .rpc();

    // Fetch the counter account data
    const accountData = await program.account.counter.fetch(counterPDA);

    console.log(`Transaction Signature: ${transactionSignature}`);
    console.log(`Count: ${accountData.count}`);
  });
});
