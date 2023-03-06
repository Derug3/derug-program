import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { assert } from "chai";
import { DerugProgram } from "../target/types/derug_program";
import updateAuthorityWallet from "../wallet/keypair.json";
import * as mplTokenMetadata from "@metaplex-foundation/mpl-token-metadata";
import * as splToken from "@solana/spl-token";
import {
  AccountLayout,
  getMinimumBalanceForRentExemptAccount,
  MintLayout,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("derug-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DerugProgram as Program<DerugProgram>;
  const metaplexProgram = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  it("Is initialized!", async () => {
    const rugger = anchor.web3.Keypair.fromSecretKey(
      Buffer.from(updateAuthorityWallet)
    );
    const derugger0 = anchor.web3.Keypair.generate();
    const derugger = anchor.web3.Keypair.generate();

    const collectionKey = new anchor.web3.PublicKey(
      "5igf61dzqeaNCq3DjygoNr84QUd4KGNQMQ6A5vdHGYTM"
    );

    await anchor
      .getProvider()
      .connection.confirmTransaction(
        await anchor
          .getProvider()
          .connection.requestAirdrop(
            rugger.publicKey,
            anchor.web3.LAMPORTS_PER_SOL * 10
          )
      );
    await anchor
      .getProvider()
      .connection.confirmTransaction(
        await anchor
          .getProvider()
          .connection.requestAirdrop(
            derugger0.publicKey,
            anchor.web3.LAMPORTS_PER_SOL * 10
          )
      );
    await anchor
      .getProvider()
      .connection.confirmTransaction(
        await anchor
          .getProvider()
          .connection.requestAirdrop(
            derugger.publicKey,
            anchor.web3.LAMPORTS_PER_SOL * 10
          )
      );

    //Initialize derug

    const [collectionMetadata] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        metaplexProgram.toBuffer(),
        collectionKey.toBuffer(),
      ],
      metaplexProgram
    );

    const [derugData] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("derug-data"), collectionKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeDerug(100)
      .accounts({
        collectionKey,
        collectionMetadata,
        derugData,
        payer: rugger.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([rugger])
      .rpc();

    assert.equal(
      (await program.account.derugData.fetch(derugData)).collection.toString(),
      collectionKey.toString(),
      "Not initialized"
    );

    console.log("DERUG INITIALIZED");

    //Create derug request

    const [derugRequest0] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("derug-data"),
        derugData.toBuffer(),
        derugger0.publicKey.toBuffer(),
      ],
      program.programId
    );

    const utilityDto0 = {
      title: "First derug",
      description: "Derug numero uno",
      action: {
        add: {},
      },
    };

    await program.methods
      .createOrUpdateDerugRequest([utilityDto0])
      .accounts({
        derugData,
        derugRequest: derugRequest0,
        payer: derugger0.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([derugger0])
      .rpc();

    console.log("DERUG REQUEST CREATED");

    await program.methods
      .cancelDerugRequest()
      .accounts({
        derugData,
        derugRequest: derugRequest0,
        payer: derugger0.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([derugger0])
      .rpc();

    //Cancel request

    console.log("DERUG REQUEST CANCELLED");

    //Create derug request

    const [derugRequest] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("derug-data"),
        derugData.toBuffer(),
        derugger.publicKey.toBuffer(),
      ],
      program.programId
    );

    const utilityDto = {
      title: "First derug",
      description: "Derug numero uno",
      action: {
        add: {},
      },
    };

    await program.methods
      .createOrUpdateDerugRequest([utilityDto])
      .accounts({
        derugData,
        derugRequest,
        payer: derugger.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([derugger])
      .rpc();

    console.log("DERUG REQUEST CREATED");

    //Vote

    const nftMint = new anchor.web3.PublicKey(
      "4wHV9DgTrPh7nNU6LquYHYsT2u6iDBrMN47Cfrjh5e6R"
    );

    const [nftMetadata] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("metadata"), metaplexProgram.toBuffer(), nftMint.toBuffer()],
      metaplexProgram
    );

    const nftTokenAccount = new anchor.web3.PublicKey(
      "8CAjrv9CvvpfVCP4b8BQe14Wg3AuKQSC1sT3RsYpjNYr"
    );

    const [voteRecord] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("derug-data"),
        rugger.publicKey.toBuffer(),
        nftMint.toBuffer(),
        Buffer.from("vote-record"),
      ],
      program.programId
    );

    let remaining_accounts = [
      {
        pubkey: voteRecord,
        isWritable: true,
        isSigner: false,
      },
      {
        pubkey: nftMint,
        isWritable: false,
        isSigner: false,
      },
      {
        pubkey: nftMetadata,
        isWritable: false,
        isSigner: false,
      },
      {
        pubkey: nftTokenAccount,
        isWritable: false,
        isSigner: false,
      },
    ];

    await program.methods
      .vote()
      .accounts({
        derugData,
        derugRequest,
        payer: rugger.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .remainingAccounts(remaining_accounts)
      .signers([rugger])
      .rpc();

    assert.equal(
      (await program.account.derugRequest.fetch(derugRequest)).voteCount,
      1,
      "Didn't vote"
    );

    console.log("VOTED");

    await new Promise((resolve) => setTimeout(resolve, 2000));

    //Claim victory

    await program.methods
      .claimVictory()
      .accounts({
        derugData,
        derugRequest,
        payer: derugger.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([derugger])
      .rpc();

    assert.equal(
      (
        await program.account.derugData.fetch(derugData)
      ).winningRequest.toString(),
      derugRequest.toString(),
      "Winner isn't right"
    );

    console.log("VICTORY CLAIMED");

    //Initialize reminting

    const newCollectionMint = anchor.web3.Keypair.generate();
    const newCollectionTokenAccount = anchor.web3.Keypair.generate();
    const createMint = anchor.web3.SystemProgram.createAccount({
      fromPubkey: derugger.publicKey,
      lamports: await getMinimumBalanceForRentExemptAccount(
        anchor.getProvider().connection
      ),
      newAccountPubkey: newCollectionMint.publicKey,
      programId: TOKEN_PROGRAM_ID,
      space: MintLayout.span,
    });

    const createTa = anchor.web3.SystemProgram.createAccount({
      fromPubkey: derugger.publicKey,
      lamports: await getMinimumBalanceForRentExemptAccount(
        anchor.getProvider().connection
      ),
      newAccountPubkey: newCollectionTokenAccount.publicKey,
      programId: TOKEN_PROGRAM_ID,
      space: AccountLayout.span,
    });

    const txCreateAccs = new anchor.web3.Transaction({
      feePayer: derugger.publicKey,
      recentBlockhash: (
        await anchor.getProvider().connection.getLatestBlockhash()
      ).blockhash,
    });

    txCreateAccs.add(createMint);
    txCreateAccs.add(createTa);


    const txSig = await anchor
      .getProvider()
      .connection.sendTransaction(txCreateAccs, [
        derugger,
        newCollectionMint,
        newCollectionTokenAccount,
      ]);
    await anchor.getProvider().connection.confirmTransaction(txSig);

    let [newCollectionMetaplexMetadata] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          metaplexProgram.toBuffer(),
          newCollectionMint.publicKey.toBuffer(),
        ],
        metaplexProgram
      );

    const [newCollectionEdition] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          ,
          metaplexProgram.toBuffer(),
          newCollectionMint.publicKey.toBuffer(),
          Buffer.from("edition"),
        ],
        metaplexProgram
      );

    const [collectionAuthorityRecord] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("metadata"), metaplexProgram.toBuffer(), newCollectionMint.publicKey.toBuffer(), Buffer.from("collection_authority"), derugger.publicKey.toBuffer()], metaplexProgram);

    const ixInitRemint = await program.methods
      .initializeReminting()
      .accounts({
        derugData,
        derugRequest,
        newCollection: newCollectionMint.publicKey,
        metadataAccount: newCollectionMetaplexMetadata,
        tokenAccount: newCollectionTokenAccount.publicKey,
        masterEdition: newCollectionEdition,
        payer: derugger.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        metadataProgram: metaplexProgram,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        collectionAuthorityRecord
      })
      .signers([derugger])
      .preInstructions([
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
          units: 1400000000,
        }),
      ])
      .instruction();

    const txInitReminting = new anchor.web3.Transaction({
      feePayer: derugger.publicKey,
      recentBlockhash: (
        await anchor.getProvider().connection.getLatestBlockhash()
      ).blockhash,
    });
    txInitReminting.add(ixInitRemint);

    await anchor.getProvider().connection.sendTransaction(txInitReminting, [derugger]);

    console.log("INITIALIZED REMINTING");



  });
});
