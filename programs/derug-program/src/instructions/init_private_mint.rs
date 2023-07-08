use anchor_lang::prelude::*;

use crate::{
    constants::{DERUG_DATA_SEED, METADATA_UPLOADER},
    state::{
        derug_request::{DerugRequest, PrivateMintStarted, RequestStatus},
        DerugData, DerugStatus,
    },
};

#[derive(Accounts)]
pub struct InitPrivateMint<'info> {
    #[account(mut)]
    pub derug_data: Account<'info, DerugData>,
    #[account(mut,seeds=[DERUG_DATA_SEED, derug_data.key().as_ref(), derug_request.derugger.key().as_ref()],bump)]
    pub derug_request: Account<'info, DerugRequest>,
    #[account(address=METADATA_UPLOADER.parse::<Pubkey>().unwrap())]
    pub payer: Signer<'info>,
}

pub fn init_private_mint(ctx: Context<InitPrivateMint>) -> Result<()> {
    let derug_data = &mut ctx.accounts.derug_data;
    let derug_request = &mut ctx.accounts.derug_request;

    derug_data.derug_status = DerugStatus::Reminting;
    derug_request.request_status = RequestStatus::Reminting;

    emit!(PrivateMintStarted {
        derug_data: ctx.accounts.derug_data.key(),
    });

    Ok(())
}
