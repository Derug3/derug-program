use anchor_lang::prelude::*;

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
    let derug_data = &mut ctx.accounts.derug_data;
    derug_data.total_suggestion_count = derug_data.total_suggestion_count.checked_sub(1).unwrap();

    require!(
        ctx.accounts.derug_request.derugger.key() == ctx.accounts.payer.key(),
        DerugError::WrongDerugger
    );

    let derug_request_info = ctx.accounts.derug_request.to_account_info();

    **ctx.accounts.payer.lamports.borrow_mut() = ctx
        .accounts
        .payer
        .lamports()
        .checked_add(derug_request_info.lamports())
        .unwrap();

    **derug_request_info.lamports.borrow_mut() = 0;

    let mut derug_request_data = derug_request_info.data.borrow_mut();
    derug_request_data.fill(0);

    Ok(())
}
