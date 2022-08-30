import * as anchor from "@project-serum/anchor";
import * as splToken from "@solana/spl-token";
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

    const voteMint = anchor.web3.Keypair.generate();
    const shoeyMint = anchor.web3.Keypair.generate();

    const [voteMetadata, _voteMetadataBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          mplMd.PROGRAM_ID.toBuffer(),
          voteMint.publicKey.toBuffer(),
        ],
        mplMd.PROGRAM_ID
      );

    const [shoeyMetadata, _shoeyMetadataBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          mplMd.PROGRAM_ID.toBuffer(),
          shoeyMint.publicKey.toBuffer(),
        ],
        mplMd.PROGRAM_ID
      );

    const [shoeyMasterEdition, _shoeyMasterEditionBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          mplMd.PROGRAM_ID.toBuffer(),
          shoeyMint.publicKey.toBuffer(),
          Buffer.from("edition"),
        ],
        mplMd.PROGRAM_ID
      );

    const [manager, _managerBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [voteMint.publicKey.toBuffer()],
        program.programId
      );

    const shoeyMasterEditionVault = await splToken.getAssociatedTokenAddress(
      shoeyMint.publicKey,
      manager,
      true
    );

    const paymentVault = await splToken.getAssociatedTokenAddress(
      paymentMint.publicKey,
      manager,
      true
    );

    const setComputeBudgetIx = anchor.web3.ComputeBudgetProgram.requestUnits({
      units: 300_000,
      additionalFee: 0,
    });

    const initializeTxSig = await program.methods
      .initialize()
      .accounts({
        voteMint: voteMint.publicKey,
        voteMetadata: voteMetadata,
        shoeyMint: shoeyMint.publicKey,
        shoeyMetadata: shoeyMetadata,
        shoeyMasterEdition: shoeyMasterEdition,
        shoeyMasterEditionVault: shoeyMasterEditionVault,
        manager: manager,
        paymentMint: paymentMint.publicKey,
        paymentVault: paymentVault,
        admin: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        metadataProgram: mplMd.PROGRAM_ID,
        associatedTokenProgram: splToken.ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .preInstructions([setComputeBudgetIx])
      .signers([voteMint, shoeyMint])
      .rpc({ skipPreflight: true });
    console.log("initializeTxSig: %s", initializeTxSig);

    const shoeyEditionMint = anchor.web3.Keypair.generate();

    const [shoeyEditionMetadata, _newshoeyMetadataBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          mplMd.PROGRAM_ID.toBuffer(),
          shoeyEditionMint.publicKey.toBuffer(),
        ],
        mplMd.PROGRAM_ID
      );

    const [shoeyEditionEdition, _newshoeyEditionBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          mplMd.PROGRAM_ID.toBuffer(),
          shoeyEditionMint.publicKey.toBuffer(),
          Buffer.from("edition"),
        ],
        mplMd.PROGRAM_ID
      );

    const userVoteAta = await splToken.getAssociatedTokenAddress(
      voteMint.publicKey,
      provider.wallet.publicKey
    );
    const userShoeyEditionAta = await splToken.getAssociatedTokenAddress(
      shoeyEditionMint.publicKey,
      provider.wallet.publicKey
    );

    const shoeyName = "jack";
    const editionNumber = new anchor.BN(1);

    const [shoey, _shoeyBump] = await anchor.web3.PublicKey.findProgramAddress(
      [manager.toBuffer(), Buffer.from(shoeyName)],
      program.programId
    );

    const [shoeyEditionMarker, _shoeyEditionMarkerBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          mplMd.PROGRAM_ID.toBuffer(),
          shoeyMint.publicKey.toBuffer(),
          Buffer.from("edition"),
          //editionNumber.toArrayLike(Buffer, "le", 8),
          Buffer.from(
            Math.floor(
              editionNumber.toNumber() / mplMd.EditionMarker.byteSize
            ).toString()
          ),
        ],
        mplMd.PROGRAM_ID
      );

    const submitTxSig = await program.methods
      .submit(shoeyName, editionNumber)
      .accounts({
        voteMint: voteMint.publicKey,
        shoeyMasterEditionMint: shoeyMint.publicKey,
        shoeyMasterEditionMetadata: shoeyMetadata,
        shoeyMasterEdition: shoeyMasterEdition,
        shoeyMasterEditionVault: shoeyMasterEditionVault,
        shoeyEditionMint: shoeyEditionMint.publicKey,
        shoeyEditionMetadata: shoeyEditionMetadata,
        shoeyEdition: shoeyEditionEdition,
        shoeyEditionMarker: shoeyEditionMarker,
        manager: manager,
        paymentMint: paymentMint.publicKey,
        paymentVault: paymentVault,
        shoey: shoey,
        user: provider.wallet.publicKey,
        userVoteAta: userVoteAta,
        userShoeyEditionAta: userShoeyEditionAta,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        associatedTokenProgram: splToken.ASSOCIATED_TOKEN_PROGRAM_ID,
        metadataProgram: mplMd.PROGRAM_ID,
      })
      .signers([shoeyEditionMint])
      .preInstructions([setComputeBudgetIx])
      .rpc({ skipPreflight: true });
    console.log("submitTxSig: %s", submitTxSig);

    const voteTxSig = await program.methods
      .vote(shoeyName)
      .accounts({
        voteMint: voteMint.publicKey,
        manager: manager,
        paymentMint: paymentMint.publicKey,
        paymentVault: paymentVault,
        shoey: shoey,
        voter: provider.wallet.publicKey,
        voterVoteAta: userVoteAta,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        associatedTokenProgram: splToken.ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .rpc({skipPreflight: true});
    console.log("voteTxSig: %s", voteTxSig);
  });
});
