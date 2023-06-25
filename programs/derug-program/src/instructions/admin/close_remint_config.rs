use anchor_lang::prelude::*;

use crate::state::derug_request::RemintConfig;

#[derive(Accounts)]
pub struct CloseRemintConfig<'info> {
    #[account(mut)]
    pub remint_confg: Account<'info, RemintConfig>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

pub fn close_remint_config(ctx: Context<CloseRemintConfig>) -> Result<()> {
    ctx.accounts
        .remint_confg
        .close(ctx.accounts.payer.to_account_info())
        .unwrap();

    Ok(())
}
