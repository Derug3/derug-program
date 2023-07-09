import { Metaplex } from "@metaplex-foundation/js";
import { Metadata } from "@metaplex-foundation/mpl-token-metadata";
import {
  createAssociatedTokenAccount,
  createMint,
  NATIVE_MINT,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
  Connection,
  Keypair,
  Transaction,
  TransactionInstruction,
  TransactionMessage,
  sendAndConfirmRawTransaction,
  VersionedTransaction,
  PublicKey,
  LAMPORTS_PER_SOL,
  ComputeBudgetProgram,
} from "@solana/web3.js";
import assert from "assert";
import BN from "bn.js";
import {
  createBypassVotingInstruction,
  createClaimVictoryInstruction,
  createCreateOrUpdateDerugRequestInstruction,
  createInitializeDerugInstruction,
  createInitializeRemintingInstruction,
  createRemintNftInstruction,
  DerugData,
  DerugRequest,
} from "../src/generated";
import { feeWallet, metaplexProgram } from "./derug-program";

export const sendTransaction = async (
  connection: Connection,
  instructions: TransactionInstruction[],
  payer: Keypair,
  partialSigners?: Keypair[]
) => {
  try {
    const versionedMessage = new TransactionMessage({
      instructions,
      payerKey: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash,
    }).compileToV0Message();

    const versionedTx = new VersionedTransaction(versionedMessage);

    versionedTx.sign([payer]);

    if (partialSigners) versionedTx.sign(partialSigners);

    const txSig = await connection.sendRawTransaction(versionedTx.serialize());
    await connection.confirmTransaction(txSig);
    console.log(txSig);
  } catch (error) {
    console.log(error);
  }
};

export const initDerugData = async (
  programId: PublicKey,
  collectionExer: PublicKey,
  mpx: Metaplex,
  connection: Connection,
  derugger: Keypair
) => {
  const metadata = mpx.nfts().pdas().metadata({ mint: collectionExer });

  const [derugData] = PublicKey.findProgramAddressSync(
    [Buffer.from("derug-data"), collectionExer.toBuffer()],
    programId
  );

  const initDerugDataIx = createInitializeDerugInstruction(
    {
      collectionKey: collectionExer,
      collectionMetadata: metadata,
      payer: derugger.publicKey,
      derugData,
    },
    {
      slug: "degods",
      totalSupply: 150,
    }
  );
  console.log(initDerugDataIx);

  await sendTransaction(connection, [initDerugDataIx], derugger);

  const derugDataAccount = await DerugData.fromAccountAddress(
    connection,
    derugData
  );

  assert(derugDataAccount.slug === "degods");

  assert(derugDataAccount.totalSupply === 150);
};

export const createDerugRequest = async (
  derugData: PublicKey,
  derugger: Keypair,
  derugRequest: PublicKey,
  connection: Connection
) => {
  const candyMachine = Keypair.generate();

  const createRequestIx = createCreateOrUpdateDerugRequestInstruction(
    {
      derugData,
      feeWallet: derugger.publicKey,
      payer: derugger.publicKey,
      derugRequest,
      anchorRemainingAccounts: [
        {
          isSigner: false,
          isWritable: false,
          pubkey: NATIVE_MINT,
        },
      ],
    },
    {
      creators: [{ address: derugger.publicKey, share: 100 }],
      newName: "DeGods#0",
      newSymbol: "dgd",
      mintConfig: {
        candyMachineKey: candyMachine.publicKey,
        mintCurrency: NATIVE_MINT,
        publicMintPrice: new BN(1 * LAMPORTS_PER_SOL),
        remintDuration: new BN(60),
        sellerFeeBps: 500,
        whitelistConfig: null,
      },
    }
  );
  const bypassVoting = createBypassVotingInstruction({
    derugData,
    derugRequest,
    payer: derugger.publicKey,
  });

  await sendTransaction(connection, [createRequestIx, bypassVoting], derugger);
};

export const claimVictoryIx = async (
  payer: Keypair,
  derugData: PublicKey,
  derugRequest: PublicKey,
  connection: Connection,
  mpx: Metaplex
) => {
  const collectionMint = Keypair.generate();

  const metadata = mpx
    .nfts()
    .pdas()
    .metadata({ mint: collectionMint.publicKey });
  const masterEdition = mpx
    .nfts()
    .pdas()
    .masterEdition({ mint: collectionMint.publicKey });

  const tokenAccount = Keypair.generate();

  const initializeReminting = createInitializeRemintingInstruction({
    derugData,
    derugRequest,
    feeWallet: feeWallet,
    metadataAccount: metadata,
    masterEdition: masterEdition,
    metadataProgram: metaplexProgram,
    newCollection: collectionMint.publicKey,
    payer: payer.publicKey,
    tokenAccount: tokenAccount.publicKey,
  });

  const claimVictoryIx = createClaimVictoryInstruction({
    derugData,
    derugRequest,
    feeWallet,
    payer: payer.publicKey,
  });

  await sendTransaction(
    connection,
    [initializeReminting, claimVictoryIx],
    payer,
    [collectionMint, tokenAccount]
  );
};

export const createMintIx = async (payer: Keypair, connection: Connection) => {
  const ix = await createMint(
    connection,
    payer,
    payer.publicKey,
    payer.publicKey,
    0
  );

  return ix;
};

export const createTokenAccount = async (
  payer: Keypair,
  connection: Connection,
  mint: PublicKey
) => {
  const ta = await createAssociatedTokenAccount(
    connection,
    payer,
    mint,
    payer.publicKey
  );

  return ta;
};

export const remintNft = async (
  connection: Connection,
  derugRequest: PublicKey,
  derugData: PublicKey,
  mpx: Metaplex,
  payer: Keypair,
  programId: PublicKey
) => {
  const newToken = Keypair.generate();
  const newMint = Keypair.generate();
  const derugDataAccount = await DerugData.fromAccountAddress(
    connection,
    derugData
  );
  const allTokens = await connection.getParsedTokenAccountsByOwner(
    payer.publicKey,
    { programId: TOKEN_PROGRAM_ID }
  );
  console.log(allTokens.value);

  const nfts = await (
    await mpx.nfts().findAllByOwner({ owner: payer.publicKey })
  ).filter(
    (nft) =>
      nft.collection &&
      nft.collection.address &&
      nft.collection.address.toString() ===
        derugDataAccount.collection.toString()
  );

  const nft = nfts[0];

  const newMetadata = mpx.nfts().pdas().metadata({ mint: newMint.publicKey });
  const newMasterEdition = mpx
    .nfts()
    .pdas()
    .masterEdition({ mint: newMint.publicKey });

  const oldMetadata = await Metadata.fromAccountAddress(
    connection,
    nft.address
  );

  const oldMint = oldMetadata.mint;

  const collectionMint = derugDataAccount.collection;

  const collectionMetadata = mpx
    .nfts()
    .pdas()
    .metadata({ mint: collectionMint });

  const collectionMasterEdition = mpx
    .nfts()
    .pdas()
    .masterEdition({ mint: collectionMint });

  const derugRequestAccount = await DerugRequest.fromAccountAddress(
    connection,
    derugRequest
  );

  const tokenAccs = (
    await connection.getParsedTokenAccountsByOwner(payer.publicKey, {
      programId: TOKEN_PROGRAM_ID,
    })
  ).value.filter(
    (ta) => ta.account.data.parsed.info.mint.toString() === oldMint.toString()
  );

  const oldMasterEdition = mpx.nfts().pdas().masterEdition({ mint: oldMint });

  const [firstCreator] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("derug"),
      derugRequestAccount.mintConfig.candyMachineKey.toBuffer(),
    ],
    programId
  );

  const [pdaAuthority] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("derug-data"),
      derugRequest.toBuffer(),
      Buffer.from("authority"),
    ],
    programId
  );

  const [remintProof] = PublicKey.findProgramAddressSync(
    [Buffer.from("derug"), oldMint.toBuffer()],
    programId
  );

  const oldCollectionMetadata = mpx
    .nfts()
    .pdas()
    .metadata({ mint: derugDataAccount.collection });

  const remintIx = createRemintNftInstruction(
    {
      collectionMasterEdition,
      collectionMetadata,
      collectionMint,
      derugData,
      derugRequest,
      feeWallet: feeWallet,
      firstCreator,
      metadataProgram: metaplexProgram,
      newCollection: derugDataAccount.newCollection,
      newEdition: newMasterEdition,
      newMetadata: newMetadata,
      newMint: newMint.publicKey,
      newToken: newToken.publicKey,
      oldCollection: derugDataAccount.collection,
      oldEdition: oldMasterEdition,
      oldMetadata: nft.address,
      oldMint: oldMint,
      oldToken: tokenAccs[0].pubkey,
      payer: payer.publicKey,
      pdaAuthority,
      remintProof,
      anchorRemainingAccounts: [
        {
          isSigner: false,
          isWritable: true,
          pubkey: oldCollectionMetadata,
        },
      ],
    },
    {
      newName: nft.name,
      newUri: nft.uri,
    }
  );

  await sendTransaction(
    connection,
    [
      ComputeBudgetProgram.setComputeUnitLimit({
        units: 1400000,
      }),
      remintIx,
    ],
    payer,
    [newToken, newMint]
  );
};
