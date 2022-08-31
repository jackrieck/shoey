import * as anchor from "@project-serum/anchor";
import * as mplMd from "@metaplex-foundation/mpl-token-metadata";
import * as splToken from "@solana/spl-token";
import { Shoey, IDL as ShoeyIDL } from "./shoey";

export const SHOEY_PROGRAM_ID = "EQTKRBAiJp6sRN9BbDcdb1ppwAZnQeBJ2h8uH9XuUwKg";

export class Client {
  readonly provider: anchor.AnchorProvider;
  private program: anchor.Program<Shoey>;

  constructor(provider: anchor.AnchorProvider) {
    const program: anchor.Program<Shoey> = new anchor.Program(
      ShoeyIDL,
      SHOEY_PROGRAM_ID,
      provider
    );

    this.provider = provider;
    this.program = program;
  }

  public async initialize(
    paymentMint: anchor.web3.PublicKey
  ): Promise<anchor.web3.PublicKey> {
    const voteMint = anchor.web3.Keypair.generate();
    const shoeyMasterEditionMint = anchor.web3.Keypair.generate();

    const [voteMetadata, _voteMetadataBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          mplMd.PROGRAM_ID.toBuffer(),
          voteMint.publicKey.toBuffer(),
        ],
        mplMd.PROGRAM_ID
      );

    const [shoeyMasterEditionMetadata, _shoeyMasterEditionMetadataBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          mplMd.PROGRAM_ID.toBuffer(),
          shoeyMasterEditionMint.publicKey.toBuffer(),
        ],
        mplMd.PROGRAM_ID
      );

    const [shoeyMasterEdition, _shoeyMasterEditionBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          mplMd.PROGRAM_ID.toBuffer(),
          shoeyMasterEditionMint.publicKey.toBuffer(),
          Buffer.from("edition"),
        ],
        mplMd.PROGRAM_ID
      );

    const [manager, _managerBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [voteMint.publicKey.toBuffer()],
        this.program.programId
      );

    const shoeyMasterEditionVault = await splToken.getAssociatedTokenAddress(
      shoeyMasterEditionMint.publicKey,
      manager,
      true
    );

    const paymentVault = await splToken.getAssociatedTokenAddress(
      paymentMint,
      manager,
      true
    );

    const setComputeBudgetIx = anchor.web3.ComputeBudgetProgram.requestUnits({
      units: 300_000,
      additionalFee: 0,
    });

    const initializeTxSig = await this.program.methods
      .initialize()
      .accounts({
        voteMint: voteMint.publicKey,
        voteMetadata: voteMetadata,
        shoeyMasterEditionMint: shoeyMasterEditionMint.publicKey,
        shoeyMasterEditionMetadata: shoeyMasterEditionMetadata,
        shoeyMasterEdition: shoeyMasterEdition,
        shoeyMasterEditionVault: shoeyMasterEditionVault,
        manager: manager,
        paymentMint: paymentMint,
        paymentVault: paymentVault,
        admin: this.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        metadataProgram: mplMd.PROGRAM_ID,
        associatedTokenProgram: splToken.ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .preInstructions([setComputeBudgetIx])
      .signers([voteMint, shoeyMasterEditionMint])
      .rpc({ skipPreflight: true });
    console.log("initializeTxSig: %s", initializeTxSig);

    return manager;
  }

  public async submit(
    managerAddress: anchor.web3.PublicKey,
    shoeyName: string
  ) {
    const manager = await this.fetchManager(managerAddress);

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

    const [shoeyEdition, _newshoeyEditionBump] =
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
      manager.voteMint,
      this.provider.wallet.publicKey
    );
    const userShoeyEditionAta = await splToken.getAssociatedTokenAddress(
      shoeyEditionMint.publicKey,
      this.provider.wallet.publicKey
    );

    // TODO: how to get this
    const editionNumber = new anchor.BN(1);

    const [shoey, _shoeyBump] = await anchor.web3.PublicKey.findProgramAddress(
      [managerAddress.toBuffer(), Buffer.from(shoeyName)],
      this.program.programId
    );

    const shoeyPaymentVault = await splToken.getAssociatedTokenAddress(
      manager.paymentMint,
      shoey,
      true
    );

    const userPaymentAta = await splToken.getAssociatedTokenAddress(
      manager.paymentMint,
      this.provider.wallet.publicKey
    );

    const [shoeyEditionMarker, _shoeyEditionMarkerBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          mplMd.PROGRAM_ID.toBuffer(),
          manager.shoeyMasterEditionMint.toBuffer(),
          Buffer.from("edition"),
          Buffer.from(
            Math.floor(
              editionNumber.toNumber() / mplMd.EditionMarker.byteSize
            ).toString()
          ),
        ],
        mplMd.PROGRAM_ID
      );

    const setComputeBudgetIx = anchor.web3.ComputeBudgetProgram.requestUnits({
      units: 300_000,
      additionalFee: 0,
    });

    const submitTxSig = await this.program.methods
      .submit(shoeyName, editionNumber)
      .accounts({
        shoeyMasterEditionMint: manager.shoeyMasterEditionMint,
        shoeyMasterEditionMetadata: manager.shoeyMasterEditionMetadata,
        shoeyMasterEdition: manager.shoeyMasterEdition,
        shoeyMasterEditionVault: manager.shoeyMasterEditionVault,
        shoeyEditionMint: shoeyEditionMint.publicKey,
        shoeyEditionMetadata: shoeyEditionMetadata,
        shoeyEdition: shoeyEdition,
        shoeyEditionMarker: shoeyEditionMarker,
        manager: managerAddress,
        voteMint: manager.voteMint,
        paymentMint: manager.paymentMint,
        paymentVault: manager.paymentVault,
        shoey: shoey,
        shoeyPaymentVault: shoeyPaymentVault,
        user: this.provider.wallet.publicKey,
        userPaymentAta: userPaymentAta,
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
  }

  public async vote(managerAddress: anchor.web3.PublicKey, shoeyName: string) {
    const manager = await this.fetchManager(managerAddress);

    const voterVoteAta = await splToken.getAssociatedTokenAddress(
      manager.voteMint,
      this.provider.wallet.publicKey
    );

    const voterPaymentAta = await splToken.getAssociatedTokenAddress(
      manager.paymentMint,
      this.provider.wallet.publicKey
    );

    const [shoey, _shoeyBump] = await anchor.web3.PublicKey.findProgramAddress(
      [managerAddress.toBuffer(), Buffer.from(shoeyName)],
      this.program.programId
    );

    const shoeyPaymentVault = await splToken.getAssociatedTokenAddress(
      manager.paymentMint,
      shoey,
      true
    );

    const voteTxSig = await this.program.methods
      .vote(shoeyName)
      .accounts({
        voteMint: manager.voteMint,
        manager: managerAddress,
        paymentMint: manager.paymentMint,
        paymentVault: manager.paymentVault,
        shoey: shoey,
        shoeyPaymentVault: shoeyPaymentVault,
        voter: this.provider.wallet.publicKey,
        voterPaymentAta: voterPaymentAta,
        voterVoteAta: voterVoteAta,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        associatedTokenProgram: splToken.ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .rpc({ skipPreflight: true });
    console.log("voteTxSig: %s", voteTxSig);
  }

  public async fetchManager(
    managerAddress: anchor.web3.PublicKey
  ): Promise<Manager> {
    const managerData = await this.program.account.manager.fetch(
      managerAddress,
      "processed"
    );

    return {
      voteMint: managerData.voteMint,
      voteMetadata: managerData.voteMetadata,
      shoeyMasterEditionMint: managerData.shoeyMasterEditionMint,
      shoeyMasterEditionMetadata: managerData.shoeyMasterEditionMetadata,
      shoeyMasterEdition: managerData.shoeyMasterEdition,
      shoeyMasterEditionVault: managerData.shoeyMasterEditionVault,
      paymentMint: managerData.paymentMint,
      paymentVault: managerData.paymentVault,
      admin: managerData.admin,
    };
  }
}

export interface Manager {
  voteMint: anchor.web3.PublicKey;
  voteMetadata: anchor.web3.PublicKey;
  shoeyMasterEditionMint: anchor.web3.PublicKey;
  shoeyMasterEditionMetadata: anchor.web3.PublicKey;
  shoeyMasterEdition: anchor.web3.PublicKey;
  shoeyMasterEditionVault: anchor.web3.PublicKey;
  paymentMint: anchor.web3.PublicKey;
  paymentVault: anchor.web3.PublicKey;
  admin: anchor.web3.PublicKey;
}
