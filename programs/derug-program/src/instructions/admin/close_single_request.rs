use anchor_lang::prelude::*;

use crate::state::derug_request::DerugRequest;

#[derive(Accounts)]
pub struct CloseSingleRequest<'info> {
    #[account(mut)]
    pub derug_request: Account<'info, DerugRequest>,
    #[account()]
    pub payer: Signer<'info>,
}

pub fn close_single_request<'info>(ctx: Context<CloseSingleRequest<'info>>) -> Result<()> {
    ctx.accounts
        .derug_request
        .close(ctx.accounts.payer.to_account_info())
        .unwrap();

    Ok(())
}
