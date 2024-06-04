import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RateLimiter } from "../target/types/rate_limiter";

describe("rate-limiter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RateLimiter as Program<RateLimiter>;

  const key = anchor.AnchorProvider.env().wallet.publicKey;

  let priceOracleAccount;

  it("Is initialized!", async () => {
    priceOracleAccount = anchor.web3.Keypair.generate();


    const tx = await program.methods.initialize(new anchor.BN(120 * 10**9), new anchor.BN(30)).accounts({
      priceOracle: priceOracleAccount.publicKey,
      authority: key
    }).rpc();

    console.log("Your transaction signature", tx);
  });

  it("fetches the price", async () => {
    try {
      const tx = await program.methods.fetchPrice().accounts({
        priceOracle: priceOracleAccount.publicKey,
        authority: key
      }).rpc();
      console.log("price fetched", tx);
    } catch(e) {
      console.log("Error fetching price: ", e);
    }
  })

});
