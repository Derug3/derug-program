import authority from "../wallet/authority.json";
import { Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { AnchorProvider, Program, Wallet } from "@project-serum/anchor";
import { IDL, DerugProgram } from "../target/types/derug_program";

const programId = new PublicKey("DERUGwXJu3m1DG1VNq4gP7Ppkza95P7XbeujbtSNAebu");
const connection = new Connection("https://api.devnet.solana.com");
const payer = Keypair.fromSecretKey(new Uint8Array(authority));
const program = new Program<DerugProgram>(
  IDL,
  programId,
  new AnchorProvider(connection, new Wallet(payer), {})
);

const bypassVoting = async () => {
  const ix = await program.methods
    .bypassVoting()
    .accounts({
      payer: payer.publicKey,
      derugRequest: new PublicKey(
        "DL4dT7det8T3gqAPqiCyeLHYngxsNzejrK92kTb3FXbJ"
      ),
      derugData: new PublicKey("AoSBBVWhH8c2VxpL3xXSLVoVpd4v68k7EJCXGcYjDx6c"),
    })
    .instruction();

  const tx = new Transaction({
    feePayer: payer.publicKey,
    blockhash: (await connection.getLatestBlockhash()).blockhash,
    lastValidBlockHeight: (await connection.getLatestBlockhash())
      .lastValidBlockHeight,
  });

  tx.add(ix);

  tx.sign(payer);

  const txRes = await connection.sendRawTransaction(tx.serialize());

  const confirmedTx = await connection.confirmTransaction(txRes);

  console.log(confirmedTx);
};

bypassVoting();
