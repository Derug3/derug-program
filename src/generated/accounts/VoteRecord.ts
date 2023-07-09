/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import * as beetSolana from '@metaplex-foundation/beet-solana'

/**
 * Arguments used to create {@link VoteRecord}
 * @category Accounts
 * @category generated
 */
export type VoteRecordArgs = {
  voted: boolean
}

export const voteRecordDiscriminator = [112, 9, 123, 165, 234, 9, 157, 167]
/**
 * Holds the data for the {@link VoteRecord} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class VoteRecord implements VoteRecordArgs {
  private constructor(readonly voted: boolean) {}

  /**
   * Creates a {@link VoteRecord} instance from the provided args.
   */
  static fromArgs(args: VoteRecordArgs) {
    return new VoteRecord(args.voted)
  }

  /**
   * Deserializes the {@link VoteRecord} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(
    accountInfo: web3.AccountInfo<Buffer>,
    offset = 0
  ): [VoteRecord, number] {
    return VoteRecord.deserialize(accountInfo.data, offset)
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link VoteRecord} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey,
    commitmentOrConfig?: web3.Commitment | web3.GetAccountInfoConfig
  ): Promise<VoteRecord> {
    const accountInfo = await connection.getAccountInfo(
      address,
      commitmentOrConfig
    )
    if (accountInfo == null) {
      throw new Error(`Unable to find VoteRecord account at ${address}`)
    }
    return VoteRecord.fromAccountInfo(accountInfo, 0)[0]
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
    return beetSolana.GpaBuilder.fromStruct(programId, voteRecordBeet)
  }

  /**
   * Deserializes the {@link VoteRecord} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [VoteRecord, number] {
    return voteRecordBeet.deserialize(buf, offset)
  }

  /**
   * Serializes the {@link VoteRecord} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return voteRecordBeet.serialize({
      accountDiscriminator: voteRecordDiscriminator,
      ...this,
    })
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link VoteRecord}
   */
  static get byteSize() {
    return voteRecordBeet.byteSize
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link VoteRecord} data from rent
   *
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    connection: web3.Connection,
    commitment?: web3.Commitment
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(
      VoteRecord.byteSize,
      commitment
    )
  }

  /**
   * Determines if the provided {@link Buffer} has the correct byte size to
   * hold {@link VoteRecord} data.
   */
  static hasCorrectByteSize(buf: Buffer, offset = 0) {
    return buf.byteLength - offset === VoteRecord.byteSize
  }

  /**
   * Returns a readable version of {@link VoteRecord} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      voted: this.voted,
    }
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const voteRecordBeet = new beet.BeetStruct<
  VoteRecord,
  VoteRecordArgs & {
    accountDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['accountDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['voted', beet.bool],
  ],
  VoteRecord.fromArgs,
  'VoteRecord'
)