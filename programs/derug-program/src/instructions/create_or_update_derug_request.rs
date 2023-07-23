use crate::errors::DerugError;
use crate::state::derug_data::ActiveRequest;
use crate::state::derug_request::{DeruggerCreator, MintConfig};
use crate::state::RequestStatus;

use crate::{
    constants::DERUG_DATA_SEED,
    state::{DerugData, DerugRequest},
    utilities::current_data_len,
};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct CreateOrUpdateDerugRequest<'info> {
    #[account(init_if_needed, seeds=[DERUG_DATA_SEED, derug_data.key().as_ref(), payer.key().as_ref()], bump, payer=payer, space =  current_data_len(derug_request,vec![]))]
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

pub fn create_or_update_derug_request(
    ctx: Context<CreateOrUpdateDerugRequest>,
    new_name: String,
    new_symbol: String,
    mint_config: MintConfig,
    creators: Vec<DeruggerCreator>,
) -> Result<()> {
    let derug_request = &mut ctx.accounts.derug_request;
    let derug_data = &mut ctx.accounts.derug_data;

    derug_request.created_at = Clock::get().unwrap().unix_timestamp;
    require!(
        ctx.accounts.payer.key() != derug_data.rug_update_authority,
        DerugError::RuggerSigner
    );

    derug_request.new_name = new_name;
    derug_request.new_symbol = new_symbol;

    derug_request.derugger = ctx.accounts.payer.key();

    derug_data.total_suggestion_count = derug_data.total_suggestion_count.checked_add(1).unwrap();

    derug_request.request_status = RequestStatus::Voting;

    let new_len = derug_data
        .to_account_info()
        .data_len()
        .checked_add(37)
        .unwrap();

    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: derug_data.to_account_info(),
            },
        ),
        Rent::default().minimum_balance(new_len),
    )?;

    derug_data.to_account_info().realloc(new_len, false)?;

    derug_data.active_requests.push(ActiveRequest {
        request: derug_request.key(),
        vote_count: 0,
        winning: false,
    });

    let remaining_accounts = &mut ctx.remaining_accounts.iter();

    let mint_currency_info = remaining_accounts.next().unwrap();

    require!(
        mint_currency_info.key() == mint_config.mint_currency,
        DerugError::InvalidMintCurrency
    );

    Account::<Mint>::try_from(mint_currency_info).expect("Invalid mint!");

    derug_request.mint_config = mint_config;

    require!(creators.len() <= 5, DerugError::TooManyCreators);

    derug_request.vote_count = 0;
    derug_request.derug_data = ctx.accounts.derug_data.key();
    derug_request.creators = creators;

    Ok(())
}
