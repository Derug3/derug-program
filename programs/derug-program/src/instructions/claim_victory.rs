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
    pub system_program: Program<'info, System>,
}

pub fn claim_victory(ctx: Context<ClaimVictory>) -> Result<()> {
    let derug_request = &mut ctx.accounts.derug_request;
    let derug_data = &mut ctx.accounts.derug_data;
    require!(
        ctx.accounts.payer.key() == derug_request.derugger.key(),
        DerugError::WrongDerugger
    );

    require!(
        Clock::get().unwrap().unix_timestamp > derug_data.period_end,
        DerugError::InvalidStatus
    );

    //Set the percentage
    let threshold = derug_data
        .total_supply
        .checked_div(derug_data.threshold_denominator as u32)
        .unwrap();

    derug_data.active_requests.sort();

    let first_request = derug_data.active_requests.get_mut(0).unwrap();

    require!(
        first_request.request == derug_request.key(),
        DerugError::NoWinner
    );

    require!(
        first_request.vote_count > threshold.try_into().unwrap(),
        DerugError::NoWinner
    );

    first_request.winning = true;

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
