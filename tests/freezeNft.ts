import { Program, AnchorProvider } from "@project-serum/anchor";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { DerugProgram, IDL } from "../target/types/derug_program";
import { PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
export const freezeNft = async () => {
  const connection = new Connection("https://api.devnet.solana.com ");
  const payer = Keypair.fromSecretKey(
    new Uint8Array([
      5, 162, 133, 24, 28, 203, 27, 186, 141, 11, 118, 154, 40, 26, 140, 47,
      155, 81, 26, 225, 88, 32, 219, 199, 71, 7, 86, 101, 47, 242, 221, 9, 88,
      98, 14, 146, 253, 127, 160, 6, 136, 229, 199, 58, 91, 169, 247, 183, 225,
      209, 228, 24, 22, 37, 92, 223, 190, 10, 59, 148, 157, 243, 86, 36,
    ])
  );

  const program = new Program<DerugProgram>(
    IDL,
    new PublicKey("8spRpt6yfwWjE8BAyR9jX1xFkVLjQcmijVha6hqQPVMU"),
    new AnchorProvider(connection, new NodeWallet(payer), {})
  );

  const allNfts = await connection.getParsedTokenAccountsByOwner(
    payer.publicKey,
    { programId: TOKEN_PROGRAM_ID }
  );

  for (const [index, nft] of allNfts.value.entries()) {
    console.log("FREEZING NFT:", index + 1);

    const mint = new PublicKey(nft.account.data.parsed.info.mint);

    const [delegate] = PublicKey.findProgramAddressSync(
      [payer.publicKey.toBuffer(), mint.toBuffer()],
      program.programId
    );

    const [nftMasterEdition] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
        Buffer.from("edition"),
      ],
      PROGRAM_ID
    );

    console.log(delegate.toString());

    const ix = await program.methods
      .freezeNft()
      .accounts({
        nftMint: mint,
        nftTokenAccount: nft.pubkey,
        delegate,
        metaplexProgram: PROGRAM_ID,
        payer: payer.publicKey,
        systemProgram: SystemProgram.programId,
        nftMasterEdition,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .instruction();
    const tx = new Transaction({
      feePayer: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash,
    });
    tx.add(ix);
    tx.sign(payer);
    await connection.sendRawTransaction(tx.serialize());
    console.log("NFT SUCCESSFULLY FREEZED");
  }
};

freezeNft();
