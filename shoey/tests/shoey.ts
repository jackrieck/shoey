import * as anchor from "@project-serum/anchor";
import * as splToken from "@solana/spl-token";
import * as shoeySdk from "../sdk/index";
import { Program } from "@project-serum/anchor";
import { Shoey } from "../target/types/shoey";
import * as mplMd from "@metaplex-foundation/mpl-token-metadata";

describe("shoey", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Shoey as Program<Shoey>;
  const provider = program.provider as anchor.AnchorProvider;

  it("smoke", async () => {
    // setup payment mint, this will be $DUST
    const paymentMint = anchor.web3.Keypair.generate();
    const paymentMintInitAccountIx = anchor.web3.SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: paymentMint.publicKey,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(
        splToken.MINT_SIZE
      ),
      space: splToken.MINT_SIZE,
      programId: splToken.TOKEN_PROGRAM_ID,
    });
    const paymentMintIx = splToken.createInitializeMintInstruction(
      paymentMint.publicKey,
      9,
      provider.wallet.publicKey,
      provider.wallet.publicKey
    );

    const initMintTxSig = await provider.sendAndConfirm(
      new anchor.web3.Transaction().add(
        paymentMintInitAccountIx,
        paymentMintIx
      ),
      [paymentMint]
    );
    console.log("initMintTxSig: %s", initMintTxSig);

    const shoeyClient = new shoeySdk.Client(provider);

    const managerAddress = await shoeyClient.initialize(paymentMint.publicKey);

    await shoeyClient.submit(managerAddress, "jack");

    await shoeyClient.vote(managerAddress, "jack");
  });
});
