use anchor_lang::prelude::*;

use crate::constants::{DERUG_DATA_SEED, VOTE_RECORD_SEED};
#[account]
pub struct VoteRecord {
    pub voted: bool,
}

impl VoteRecord {
    pub fn get_seeds(nft_mint: &Pubkey, payer: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                DERUG_DATA_SEED,
                payer.as_ref(),
                nft_mint.as_ref(),
                VOTE_RECORD_SEED,
            ],
            program_id,
        )
    }
}
