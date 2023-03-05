use anchor_lang::{prelude::*, AccountsClose};

use crate::{
    constants::DERUG_DATA_SEED,
    errors::DerugError,
    state::{derug_data::DerugData, derug_request::DerugRequest},
};

#[derive(Accounts)]
pub struct CancelDerugRequest<'info> {
    #[account(mut, seeds=[DERUG_DATA_SEED, derug_data.key().as_ref(), payer.key().as_ref()], bump)]
    pub derug_request: Box<Account<'info, DerugRequest>>,
    #[account(mut, seeds =[DERUG_DATA_SEED, derug_data.collection.key().as_ref()], bump)]
    pub derug_data: Box<Account<'info, DerugData>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn cancel_derug_request(ctx: Context<CancelDerugRequest>) -> Result<()> {
    require!(
        ctx.accounts.derug_request.derugger.key() == ctx.accounts.payer.key(),
        DerugError::WrongDerugger
    );

    let derug_data = &mut ctx.accounts.derug_data;
    derug_data.total_suggestion_count = derug_data.total_suggestion_count.checked_sub(1).unwrap();
    derug_data
        .active_requests
        .iter_mut()
        .find(|request| request.request == ctx.accounts.derug_request.key())
        .unwrap()
        .vote_count = -1;

    ctx.accounts
        .derug_request
        .close(ctx.accounts.payer.to_account_info())?;

    Ok(())
}
