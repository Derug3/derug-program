use anchor_lang::prelude::*;

use itertools::Itertools;

use crate::{
    constants::DERUG_DATA_SEED,
    errors::DerugError,
    state::{
        derug_data::{DerugData, DerugStatus},
        derug_request::{DerugRequest, RequestStatus},
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
    let active_requests = derug_data
        .active_requests
        .iter()
        .filter(|request| request.vote_count > 0 && request.winning == true)
        .collect_vec();

    let winner = active_requests.get(0).unwrap();

    require!(
        winner.request == derug_request.key(),
        DerugError::InvalidWinningRequest
    );

    let multiple_winners = derug_data
        .active_requests
        .iter()
        .filter(|request| request.vote_count == winner.vote_count)
        .collect_vec();

    if multiple_winners.len() > 1 {
        panic!("There are multiple winners");
    }

    derug_data.derug_status = DerugStatus::Succeeded;
    derug_request.request_status = RequestStatus::Succeeded;

    Ok(())
}
