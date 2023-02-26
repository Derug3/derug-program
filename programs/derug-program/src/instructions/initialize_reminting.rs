use crate::{
    constants::DERUG_DATA_SEED,
    errors::DerugError,
    state::{
        derug_data::{DerugData, DerugStatus},
        derug_request::{DerugRequest, RequestStatus},
    },
    utilities::{create_master_edition_ix, create_metadata_ix},
};
use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};

use mpl_token_metadata::{
    processor::verify_collection,
    state::{EDITION, PREFIX},
    ID as METADATA_PROGRAM_ID,
};
use solana_program::program::invoke;

#[derive(Accounts)]
pub struct InitializeReminting<'info> {
    #[account(mut, seeds=[DERUG_DATA_SEED, derug_data.key().as_ref(), payer.key().as_ref()], bump)]
    pub derug_request: Account<'info, DerugRequest>,
    #[account(mut,seeds=[DERUG_DATA_SEED,derug_data.collection.key().as_ref()],bump)]
    pub derug_data: Box<Account<'info, DerugData>>,
    #[account(init, payer=payer, mint::decimals=0, mint::authority=payer, mint::freeze_authority=payer)]
    pub new_collection: Account<'info, Mint>,
    #[account(init, payer=payer, token::mint=new_collection, token::authority=payer)]
    pub token_account: Account<'info, TokenAccount>,

    ///CHECK
    #[account(seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), new_collection.key().as_ref(), EDITION.as_ref()],bump, seeds::program = METADATA_PROGRAM_ID)]
    pub master_edition: UncheckedAccount<'info>,
    ///CHECK
    #[account(seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), new_collection.key().as_ref()], bump,seeds::program = METADATA_PROGRAM_ID)]
    pub metadata_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    ///CHECK
    #[account(address = METADATA_PROGRAM_ID)]
    pub metadata_program: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}

pub fn initialize_reminting(ctx: Context<InitializeReminting>) -> Result<()> {
    require!(
        ctx.accounts.derug_data.derug_status == DerugStatus::Succeeded,
        DerugError::NoWinner
    );

    require!(
        ctx.accounts.derug_request.request_status == RequestStatus::Succeeded,
        DerugError::NoWinner
    );

    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.payer.to_account_info(),
                mint: ctx.accounts.new_collection.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
            },
        ),
        1,
    )?;

    let create_metadata = create_metadata_ix(
        &ctx.accounts.new_collection.key(),
        ctx.accounts.payer.key,
        None,
        ctx.accounts.payer.key,
        &ctx.accounts.derug_data.collection_uri,
        &ctx.accounts.derug_data.collection_name,
        &ctx.accounts.derug_data.collection_symbol,
    );

    invoke(
        &create_metadata,
        &[
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.new_collection.to_account_info(),
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.metadata_program.to_account_info(),
        ],
    )?;

    let create_master_edition = create_master_edition_ix(
        &ctx.accounts.new_collection.key(),
        ctx.accounts.payer.key,
        ctx.accounts.payer.key,
    );

    invoke(
        &create_master_edition,
        &[
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.new_collection.to_account_info(),
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.metadata_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    //TODO: Check if this is right
    verify_collection(
        &METADATA_PROGRAM_ID,
        &[
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.new_collection.to_account_info(),
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.payer.to_account_info(),
        ],
    )?;

    Ok(())
}
