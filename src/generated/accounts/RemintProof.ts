/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as web3 from '@solana/web3.js'
import * as beetSolana from '@metaplex-foundation/beet-solana'
import * as beet from '@metaplex-foundation/beet'

/**
 * Arguments used to create {@link RemintProof}
 * @category Accounts
 * @category generated
 */
export type RemintProofArgs = {
  derugData: web3.PublicKey
  reminter: web3.PublicKey
  oldMint: web3.PublicKey
  newMint: web3.PublicKey
}

export const remintProofDiscriminator = [248, 248, 215, 126, 94, 196, 34, 114]
/**
 * Holds the data for the {@link RemintProof} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class RemintProof implements RemintProofArgs {
  private constructor(
    readonly derugData: web3.PublicKey,
    readonly reminter: web3.PublicKey,
    readonly oldMint: web3.PublicKey,
    readonly newMint: web3.PublicKey
  ) {}

  /**
   * Creates a {@link RemintProof} instance from the provided args.
   */
  static fromArgs(args: RemintProofArgs) {
    return new RemintProof(
      args.derugData,
      args.reminter,
      args.oldMint,
      args.newMint
    )
  }

  /**
   * Deserializes the {@link RemintProof} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(
    accountInfo: web3.AccountInfo<Buffer>,
    offset = 0
  ): [RemintProof, number] {
    return RemintProof.deserialize(accountInfo.data, offset)
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link RemintProof} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey,
    commitmentOrConfig?: web3.Commitment | web3.GetAccountInfoConfig
  ): Promise<RemintProof> {
    const accountInfo = await connection.getAccountInfo(
      address,
      commitmentOrConfig
    )
    if (accountInfo == null) {
      throw new Error(`Unable to find RemintProof account at ${address}`)
    }
    return RemintProof.fromAccountInfo(accountInfo, 0)[0]
  }

  /**
   * Provides a {@link web3.Connection.getProgramAccounts} config builder,
   * to fetch accounts matching filters that can be specified via that builder.
   *
   * @param programId - the program that owns the accounts we are filtering
   */
  static gpaBuilder(
    programId: web3.PublicKey = new web3.PublicKey(
      'DERUGwXJu3m1DG1VNq4gP7Ppkza95P7XbeujbtSNAebu'
    )
  ) {
    return beetSolana.GpaBuilder.fromStruct(programId, remintProofBeet)
  }

  /**
   * Deserializes the {@link RemintProof} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [RemintProof, number] {
    return remintProofBeet.deserialize(buf, offset)
  }

  /**
   * Serializes the {@link RemintProof} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return remintProofBeet.serialize({
      accountDiscriminator: remintProofDiscriminator,
      ...this,
    })
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link RemintProof}
   */
  static get byteSize() {
    return remintProofBeet.byteSize
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link RemintProof} data from rent
   *
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    connection: web3.Connection,
    commitment?: web3.Commitment
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(
      RemintProof.byteSize,
      commitment
    )
  }

  /**
   * Determines if the provided {@link Buffer} has the correct byte size to
   * hold {@link RemintProof} data.
   */
  static hasCorrectByteSize(buf: Buffer, offset = 0) {
    return buf.byteLength - offset === RemintProof.byteSize
  }

  /**
   * Returns a readable version of {@link RemintProof} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      derugData: this.derugData.toBase58(),
      reminter: this.reminter.toBase58(),
      oldMint: this.oldMint.toBase58(),
      newMint: this.newMint.toBase58(),
    }
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const remintProofBeet = new beet.BeetStruct<
  RemintProof,
  RemintProofArgs & {
    accountDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['accountDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['derugData', beetSolana.publicKey],
    ['reminter', beetSolana.publicKey],
    ['oldMint', beetSolana.publicKey],
    ['newMint', beetSolana.publicKey],
  ],
  RemintProof.fromArgs,
  'RemintProof'
)