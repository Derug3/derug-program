use anchor_lang::prelude::*;

use crate::{
    constants::DERUG_DATA_SEED,
    errors::DerugError,
    state::{
        derug_data::{DerugData, DerugStatus},
        derug_request::DerugRequest,
    },
};

#[derive(Accounts)]
pub struct ClaimVictory<'info> {
    #[account(mut, seeds=[DERUG_DATA_SEED, derug_data.key().as_ref(), payer.key().as_ref()], bump)]
    pub derug_request: Account<'info, DerugRequest>,
    #[account(mut, seeds =[DERUG_DATA_SEED, derug_data.collection.key().as_ref()], bump)]
    pub derug_data: Box<Account<'info, DerugData>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    ///CHECK
    #[account(mut, address="DRG3YRmurqpWQ1jEjK8DiWMuqPX9yL32LXLbuRdoiQwt".parse::<Pubkey>().unwrap())]
    pub fee_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn claim_victory<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, ClaimVictory<'info>>,
) -> Result<()> {
    let derug_request = &mut ctx.accounts.derug_request;
    let derug_data = &mut ctx.accounts.derug_data;
    require!(
        ctx.accounts.payer.key() == derug_request.derugger.key(),
        DerugError::WrongDerugger
    );

    derug_data
        .active_requests
        .iter_mut()
        .find(|req| req.request == derug_request.key())
        .unwrap()
        .winning = true;

    derug_data.winning_request = Some(derug_request.key());

    derug_data.derug_status = DerugStatus::Succeeded;

    Ok(())
}
