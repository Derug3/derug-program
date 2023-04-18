use anchor_lang::prelude::*;

use crate::{
    constants::{DERUG_DATA_SEED, METADATA_UPLOADER, REMINT_CONFIG_SEED},
    state::{
        derug_request::{DerugRequest, PrivateMintStarted, RemintConfig, RequestStatus},
        DerugData, DerugStatus,
    },
};

#[derive(Accounts)]
pub struct InitPrivateMint<'info> {
    #[account(mut)]
    pub derug_data: Account<'info, DerugData>,
    #[account(mut,seeds=[DERUG_DATA_SEED, derug_data.key().as_ref(), derug_request.derugger.key().as_ref()],bump)]
    pub derug_request: Account<'info, DerugRequest>,
    #[account(mut,seeds=[REMINT_CONFIG_SEED,derug_data.key().as_ref()],bump)]
    pub remint_config: Account<'info, RemintConfig>,
    #[account(address=METADATA_UPLOADER.parse::<Pubkey>().unwrap())]
    pub payer: Signer<'info>,
}

pub fn init_private_mint(ctx: Context<InitPrivateMint>) -> Result<()> {
    let derug_data = &mut ctx.accounts.derug_data;
    let derug_request = &mut ctx.accounts.derug_request;
    let remint_config = &mut ctx.accounts.remint_config;

    derug_data.derug_status = DerugStatus::Reminting;
    derug_request.request_status = RequestStatus::Reminting;

    if let Some(private_mint_end) = derug_request.private_mint_duration {
        remint_config.private_mint_end = Some(
            Clock::get()
                .unwrap()
                .unix_timestamp
                .checked_add(private_mint_end)
                .unwrap(),
        );
    }

    emit!(PrivateMintStarted {
        derug_data: ctx.accounts.derug_data.key(),
        remint_config: ctx.accounts.remint_config.key()
    });

    Ok(())
}
