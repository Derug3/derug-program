import { Program, AnchorProvider } from "@project-serum/anchor";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionMessage,
  VersionedTransaction,
} from "@solana/web3.js";
import { DerugProgram, IDL } from "../target/types/derug_program";
import kp from "../wallet/keypair.json";

export const closeDerugData = async () => {
  try {
    const connection = new Connection("https://api.devnet.solana.com ");
    const payer = Keypair.fromSecretKey(new Uint8Array(kp));

    console.log(payer.publicKey.toString());

    const program = new Program<DerugProgram>(
      IDL,
      new PublicKey("8spRpt6yfwWjE8BAyR9jX1xFkVLjQcmijVha6hqQPVMU"),
      new AnchorProvider(connection, new NodeWallet(payer), {})
    );

    const requests = await program.account.derugRequest.all();
  } catch (error) {
    console.log(error);
  }
};

closeDerugData();
