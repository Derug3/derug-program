use anchor_lang::prelude::*;
use mpl_token_metadata::ID as METADATA_PROGRAM_ID;

use mpl_candy_machine::instruction::InitializeCandyMachine;

use crate::{
    constants::{DERUG_DATA_SEED, METADATA_SEED},
    errors::DerugError,
    state::{
        derug_data::{DerugData, DerugStatus},
        derug_request::{DerugRequest, RequestStatus},
    },
};

#[derive(Accounts)]
pub struct InitCandyMachine<'info> {
    #[account(mut, seeds=[DERUG_DATA_SEED, derug_data.key().as_ref(), payer.key().as_ref()], bump)]
    pub derug_request: Account<'info, DerugRequest>,
    #[account(mut,seeds=[DERUG_DATA_SEED,derug_data.collection.key().as_ref()],bump)]
    pub derug_data: Box<Account<'info, DerugData>>,
    ///CHECK: Checked in the instruction
    #[account(seeds = [METADATA_SEED, derug_data.collection.key().as_ref(), METADATA_PROGRAM_ID.as_ref()], seeds::program=METADATA_PROGRAM_ID, bump)]
    pub collection_metadata: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init_candy_machine(ctx: Context<InitCandyMachine>) -> Result<()> {
    require!(
        ctx.accounts.derug_data.derug_status == DerugStatus::Reminting,
        DerugError::NoWinner
    );

    require!(
        ctx.accounts.derug_request.request_status == RequestStatus::Succeeded,
        DerugError::NoWinner
    );

    Ok(())
}
