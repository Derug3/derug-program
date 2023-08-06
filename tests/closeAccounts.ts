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

  console.log(payer.publicKey.toString());

  const program = new Program<DerugProgram>(
    IDL,
    new PublicKey("DERUGwXJu3m1DG1VNq4gP7Ppkza95P7XbeujbtSNAebu"),
    new AnchorProvider(connection, new NodeWallet(payer), {})
  );

  const [derugData] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("derug-data"),
      new PublicKey("DokxHuiYjAcnWzRzq2BqE3i8CeHMCpZJbpTdDNHZ99ei").toBuffer(),
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

  const ix = program.instruction.closeProgramAccount({
    accounts: {
      derugData,
      derugRequest,
      payer: payer.publicKey,
    },
  });

  try {
    const tx = new Transaction({
      feePayer: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash,
    });
    tx.add(ix);
    const txSim = await connection.simulateTransaction(tx);
    console.log(txSim.value.logs);

    // tx.add(ix);

    const txSig = await connection.sendTransaction(tx, [payer]);
    await connection.confirmTransaction(txSig);
    console.log(txSig);
  } catch (error) {
    console.log(error);
  }
  // }
}

closeAccounts();
