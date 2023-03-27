use std::mem::size_of;

use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::token::TokenAccount;
use itertools::Itertools;

use crate::{
    constants::{DERUG_DATA_SEED, REMINT_CONFIG_SEED},
    errors::DerugError,
    state::{
        derug_data::{ActiveRequest, DerugData, DerugStatus},
        derug_request::{DerugRequest, RemintConfig, RequestStatus},
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
    #[account(init,seeds=[REMINT_CONFIG_SEED,derug_data.key().as_ref()],payer=payer,bump,space=size_of::<RemintConfig>())]
    pub remint_config: Account<'info, RemintConfig>,
    ///CHECK
    #[account(mut, address="DRG3YRmurqpWQ1jEjK8DiWMuqPX9yL32LXLbuRdoiQwt".parse::<Pubkey>().unwrap())]
    pub fee_wallet: AccountInfo<'info>,
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

    let remint_config = &mut ctx.accounts.remint_config;

    remint_config.authority = ctx.accounts.payer.key();

    let remaining_accounts = &mut ctx.remaining_accounts.iter();

    let candy_machine = remaining_accounts.next().unwrap();

    remint_config.candy_machine_key = candy_machine.key();

    remint_config.public_mint_price = derug_request.mint_price;

    if let Some(private_mint_end) = derug_request.private_mint_duration {
        remint_config.private_mint_end = Some(
            Clock::get()
                .unwrap()
                .unix_timestamp
                .checked_add(private_mint_end)
                .unwrap(),
        );
    }

    remint_config.seller_fee_bps = derug_request.seller_fee_bps;

    //Set the percentage
    let threshold = derug_data
        .total_supply
        .checked_div(derug_data.threshold_denominator as u32)
        .unwrap();

    let winning_request = ActiveRequest::get_winning(derug_data);

    require!(
        winning_request.request == derug_request.key(),
        DerugError::NoWinner
    );

    require!(
        winning_request.vote_count > threshold.try_into().unwrap(),
        DerugError::NoWinner
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

    if let Some(mint_currency) = derug_request.mint_currency {
        let token_info = remaining_accounts.next().unwrap();
        let token = Account::<TokenAccount>::try_from(token_info).unwrap();
        require!(
            token.mint == mint_currency,
            DerugError::InvalidTokenAccountMint
        );
        remint_config.mint_currency = Some(mint_currency);
    }

    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.fee_wallet.to_account_info(),
            },
        ),
        9000000,
    )?;

    Ok(())
}
