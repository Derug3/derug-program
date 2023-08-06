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

pub fn init_private_mint(ctx: Context<InitPrivateMint>, total_supply: u32) -> Result<()> {
    let derug_data = &mut ctx.accounts.derug_data;
    let derug_request = &mut ctx.accounts.derug_request;

    derug_data.derug_status = DerugStatus::Reminting;
    derug_request.request_status = RequestStatus::Reminting;

    derug_data.total_supply = total_supply;

    derug_request.mint_config.remint_duration = derug_request
        .mint_config
        .remint_duration
        .checked_add(Clock::get().unwrap().unix_timestamp)
        .unwrap();

    emit!(PrivateMintStarted {
        derug_data: ctx.accounts.derug_data.key(),
        derug_request: ctx.accounts.derug_request.key()
    });

    Ok(())
}
