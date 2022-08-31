import * as anchor from "@project-serum/anchor";
import * as splToken from "@solana/spl-token";
import * as shoeySdk from "../sdk/index";

describe("shoey", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider() as anchor.AnchorProvider;

  it("smoke", async () => {
    // create payer for setup
    const payer = await createPayer(provider.connection);

    // create payment mint, mimic $DUST
    const paymentMint = await splToken.createMint(
      provider.connection,
      payer,
      payer.publicKey,
      payer.publicKey,
      9
    );

    // create ATA for provider and seed with payment tokens
    const providerPaymentAta = await splToken.getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      paymentMint,
      provider.wallet.publicKey
    );

    await splToken.mintTo(
      provider.connection,
      payer,
      paymentMint,
      providerPaymentAta.address,
      payer,
      100 * 10 ** 9
    );

    console.log("setup complete");

    const shoeyClient = new shoeySdk.Client(provider);

    const managerAddress = await shoeyClient.initialize(paymentMint);

    await shoeyClient.submit(managerAddress, "jack");

    await shoeyClient.vote(managerAddress, "jack");

    await shoeyClient.claim(managerAddress, "jack");
  });
});

async function createPayer(
  connection: anchor.web3.Connection
): Promise<anchor.web3.Keypair> {
  const payer = anchor.web3.Keypair.generate();

  const requestAirdropTxSig = await connection.requestAirdrop(
    payer.publicKey,
    10 * anchor.web3.LAMPORTS_PER_SOL
  );
  await connection.confirmTransaction(requestAirdropTxSig, "confirmed");

  return payer;
}

async function delayMs(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
