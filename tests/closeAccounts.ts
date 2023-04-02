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

  const [derugData] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("derug-data"),
      new PublicKey("C8v1JF55UrBDdoAmCYAeaYRuxXKMkkqWGsCJs1suU3WC").toBuffer(),
    ],
    program.programId
  );

  const [derugRequest] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("derug-data"),
      derugData.toBuffer(),
      new PublicKey("A6DHb3s8VKSKV3cC58xYzLooyVsLuKCrWwQEe2ZdbEZg").toBuffer(),
    ],
    program.programId
  );

  const [remintConfig] = PublicKey.findProgramAddressSync(
    [Buffer.from("remint-config"), derugData.toBuffer()],
    program.programId
  );

  const remainingAccounts: AccountMeta[] = voteRecords.map((vr) => {
    return {
      isSigner: false,
      isWritable: true,
      pubkey: vr.publicKey,
    };
  });

  console.log(payer.publicKey.toString());

  const ix = program.instruction.closeProgramAccount({
    accounts: {
      derugData,
      derugRequest,
      payer: payer.publicKey,
      remintConfig,
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
