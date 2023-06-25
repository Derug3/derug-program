use crate::constants::{DERUG_DATA_SEED, PLATFORM_AUTHORITY};
use crate::state::*;
use anchor_lang::prelude::*;
#[derive(Accounts)]

pub struct BypassVoting<'info> {
    #[account(mut)]
    pub derug_request: Box<Account<'info, DerugRequest>>,
    #[account(mut,seeds=[DERUG_DATA_SEED,derug_data.collection.key().as_ref()],bump)]
    pub derug_data: Box<Account<'info, DerugData>>,
    #[account(address=PLATFORM_AUTHORITY.parse::<Pubkey>().unwrap())]
    pub payer: Signer<'info>,
}

pub fn bypass_voting(ctx: Context<BypassVoting>) -> Result<()> {
    let derug_data = &mut ctx.accounts.derug_data;
    let derug_request = &mut ctx.accounts.derug_request;

    let total_supply = derug_data.total_supply as i32;

    derug_data.winning_request = Some(derug_request.key());

    derug_data.active_requests.push(ActiveRequest {
        request: derug_request.key(),
        winning: true,
        vote_count: total_supply,
    });

    derug_request.vote_count = total_supply as u32;

    Ok(())
}
