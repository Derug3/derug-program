import { chunk } from "@metaplex-foundation/js";
import { AnchorProvider, Program, Wallet } from "@project-serum/anchor";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import {
  AccountMeta,
  Connection,
  Keypair,
  PublicKey,
  Transaction,
} from "@solana/web3.js";
import { IDL, DerugProgram } from "../target/types/derug_program";
import kp from "../wallet/keypair.json";
async function closeAccounts() {
  const connection = new Connection("https://api.devnet.solana.com ");
  const payer = Keypair.fromSecretKey(new Uint8Array(kp));

  const program = new Program<DerugProgram>(
    IDL,
    new PublicKey("8spRpt6yfwWjE8BAyR9jX1xFkVLjQcmijVha6hqQPVMU"),
    new AnchorProvider(connection, new NodeWallet(payer), {})
  );

  const voteRecords = await program.account.voteRecord.all();

  const chunkedVoteRecords = chunk(voteRecords, 25);

  const [derugData] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("derug-data"),
      new PublicKey("9P2aidVgTfSfKwMJwEUP7rTSTgYPmCj9eAHN1yccUL3U").toBuffer(),
    ],
    program.programId
  );

  //6smu36j5E6AfW4NM2RQPpDbdzEpp9tKZvg7ZTE2KKgcL
  const [derugRequest] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("derug-data"),
      derugData.toBuffer(),
      new PublicKey("31hGE85HasBGpXRz1xxeczAJZmeW4HDS8VRCY5odqZVg").toBuffer(),
    ],
    program.programId
  );

  const [remintConfig] = PublicKey.findProgramAddressSync(
    [Buffer.from("remint-config"), derugData.toBuffer()],
    program.programId
  );

  const ix = program.instruction.closeSingleRequest({
    accounts: {
      derugRequest,
      payer: payer.publicKey,
      // remintConfig,
    },
  });

  try {
    const tx = new Transaction({
      feePayer: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash,
    });

    tx.add(ix);

    await connection.sendTransaction(tx, [payer]);
  } catch (error) {
    console.log(error);
  }
}

closeAccounts();
