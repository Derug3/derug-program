use anchor_lang::prelude::*;
use derug_data::DerugStatus;

use crate::{
    constants::{ADMIN, DERUG_DATA_SEED},
    errors::DerugError,
    state::{
        derug_data,
        derug_request::{DerugRequest, RequestStatus},
        DerugData,
    },
};

#[derive(Accounts)]
pub struct SkipVote<'info> {
    #[account(mut,seeds=[DERUG_DATA_SEED,derug_data.collection.key().as_ref()],bump)]
    pub derug_data: Box<Account<'info, DerugData>>,
    #[account(mut,seeds=[DERUG_DATA_SEED, derug_data.key().as_ref(), derug_request.derugger.key().as_ref()],bump)]
    pub derug_request: Account<'info, DerugRequest>,
    #[account(mut, address = ADMIN.parse::<Pubkey>().unwrap() @ DerugError::WrongOwner)]
    pub payer: Signer<'info>,
}

pub fn skip_vote<'info>(ctx: Context<SkipVote<'info>>) -> Result<()> {
    let derug_data = &mut ctx.accounts.derug_data;
    let derug_request = &mut ctx.accounts.derug_request;

    derug_data.derug_status = DerugStatus::Reminting;
    derug_request.request_status = RequestStatus::Reminting;

    let vote_count = derug_data.threshold_denominator as i32 + 1;

    let mut request = derug_data
        .active_requests
        .iter_mut()
        .find(|ar| ar.request == derug_request.key())
        .unwrap();

    request.winning = true;
    request.vote_count = vote_count;

    derug_data.winning_request = Some(derug_request.key());

    Ok(())
}
