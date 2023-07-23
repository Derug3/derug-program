import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DerugProgram } from "../target/types/derug_program";
import { Metaplex, PublicKey } from "@metaplex-foundation/js";
import clubWallet from "../wallet/clubWallet.json";
import { Keypair, LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";
import {
  claimVictoryIx,
  createCm,
  createDerugRequest,
  initDerugData,
  insertItems,
  remintNft,
  sendTransaction,
} from "./helpers";
import assert from "assert";
import authority from "../wallet/authority.json";
import { NATIVE_MINT } from "@solana/spl-token";
import { BN } from "bn.js";
export const feeWallet = new PublicKey(
  "DRG3YRmurqpWQ1jEjK8DiWMuqPX9yL32LXLbuRdoiQwt"
);
export const metaplexProgram = new anchor.web3.PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);
describe("derug-program", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DerugProgram as Program<DerugProgram>;

  const derugger = Keypair.fromSecretKey(new Uint8Array(authority));

  const programId = new PublicKey(
    "DERUGwXJu3m1DG1VNq4gP7Ppkza95P7XbeujbtSNAebu"
  );

  const reminter = Keypair.fromSecretKey(new Uint8Array(clubWallet));

  const connection = anchor.getProvider().connection;

  before(async () => {
    const airdropIx = await anchor
      .getProvider()
      .connection.requestAirdrop(derugger.publicKey, 10 * LAMPORTS_PER_SOL);

    await anchor.getProvider().connection.confirmTransaction(airdropIx);
  });

  const mpx = new Metaplex(anchor.getProvider().connection);

  it("Is initialized!", async () => {
    const collectionExer = new PublicKey(
      "zsPM3UTNd2T1CeXsAxPiy2iaTq4ofRJSRFG2Auvxmk8"
    );

    const [derugData] = PublicKey.findProgramAddressSync(
      [Buffer.from("derug-data"), collectionExer.toBuffer()],
      programId
    );

    const [derugRequest] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("derug-data"),
        derugData.toBuffer(),
        derugger.publicKey.toBuffer(),
      ],
      programId
    );

    const cm = await createCm(collectionExer, derugger);

    console.log(cm.toString());

    await insertItems(derugger, mpx, new PublicKey(cm));
  });
});
