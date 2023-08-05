use anchor_lang::prelude::*;

use crate::{
    constants::PLATFORM_AUTHORITY,
    state::{DerugData, DerugStatus},
};

#[derive(Accounts)]
pub struct InitPublicMint<'info> {
    #[account(mut,address=PLATFORM_AUTHORITY.parse::<Pubkey>().unwrap())]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub derug_data: Account<'info, DerugData>,
}

pub fn init_public_mint(ctx: Context<InitPublicMint>) -> Result<()> {
    let derug_data = &mut ctx.accounts.derug_data;

    derug_data.derug_status = DerugStatus::PublicMint;
    Ok(())
}
