export * from './DerugData'
export * from './DerugRequest'
export * from './RemintConfig'
export * from './RemintProof'
export * from './VoteRecord'

import { DerugData } from './DerugData'
import { DerugRequest } from './DerugRequest'
import { RemintProof } from './RemintProof'
import { RemintConfig } from './RemintConfig'
import { VoteRecord } from './VoteRecord'

export const accountProviders = {
  DerugData,
  DerugRequest,
  RemintProof,
  RemintConfig,
  VoteRecord,
}
