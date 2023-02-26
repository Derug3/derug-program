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
    state::{Metadata, EDITION, PREFIX},
    ID as METADATA_PROGRAM_ID,
};
use solana_program::{borsh::try_from_slice_unchecked, program::invoke};

#[derive(Accounts)]
pub struct InitializeReminting<'info> {
    #[account(seeds=[DERUG_DATA_SEED, derug_data.key().as_ref(), payer.key().as_ref()], bump)]
    pub derug_request: Box<Account<'info, DerugRequest>>,
    #[account(mut,seeds=[DERUG_DATA_SEED,derug_data.collection.key().as_ref()],bump)]
    pub derug_data: Box<Account<'info, DerugData>>,
    #[account()]
    pub new_collection: Box<Account<'info, Mint>>,
    #[account()]
    pub old_collection: Box<Account<'info, Mint>>,
    ///CHECK
    #[account(seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), old_collection.key().as_ref()], bump,seeds::program = METADATA_PROGRAM_ID)]
    pub old_collection_metadata: UncheckedAccount<'info>,
    #[account()]
    pub old_mint: Box<Account<'info, Mint>>,
    #[account(init, payer=payer, mint::decimals=0, mint::authority=payer, mint::freeze_authority=payer)]
    pub new_mint: Box<Account<'info, Mint>>,
    //TODO: Require
    #[account()]
    pub old_token: Box<Account<'info, TokenAccount>>,
    #[account(init, payer=payer, token::mint=new_mint, token::authority=payer)]
    pub new_token: Account<'info, TokenAccount>,

    ///CHECK
    #[account(seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), old_mint.key().as_ref()], bump,seeds::program = METADATA_PROGRAM_ID)]
    pub old_metadata: UncheckedAccount<'info>,
    ///CHECK
    #[account(seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), new_mint.key().as_ref()], bump,seeds::program = METADATA_PROGRAM_ID)]
    pub new_metadata: UncheckedAccount<'info>,

    ///CHECK
    #[account(seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), old_mint.key().as_ref(), EDITION.as_ref()],bump, seeds::program = METADATA_PROGRAM_ID)]
    pub old_edition: UncheckedAccount<'info>,
    ///CHECK
    #[account(seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), new_mint.key().as_ref(), EDITION.as_ref()],bump, seeds::program = METADATA_PROGRAM_ID)]
    pub new_edition: UncheckedAccount<'info>,

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
        ctx.accounts.old_collection.key() == ctx.accounts.derug_data.collection,
        DerugError::WrongCollection
    );

    require!(
        ctx.accounts.derug_data.derug_status == DerugStatus::Reminting,
        DerugError::InvalidStatus
    );

    require!(
        ctx.accounts.derug_request.request_status == RequestStatus::Reminting,
        DerugError::InvalidStatus
    );

    require!(
        ctx.accounts.derug_data.winning_request.unwrap() == ctx.accounts.derug_request.key(),
        DerugError::NoWinner
    );

    let burn_ix = mpl_token_metadata::instruction::burn_nft(
        METADATA_PROGRAM_ID,
        ctx.accounts.old_metadata.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.old_mint.key(),
        ctx.accounts.old_token.key(),
        ctx.accounts.old_edition.key(),
        ctx.accounts.token_program.key(),
        Some(ctx.accounts.old_collection_metadata.key()),
    );

    invoke(
        &burn_ix,
        &[
            ctx.accounts.old_metadata.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.old_mint.to_account_info(),
            ctx.accounts.old_token.to_account_info(),
            ctx.accounts.old_edition.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.old_collection_metadata.to_account_info(),
        ],
    )?;

    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.payer.to_account_info(),
                mint: ctx.accounts.new_mint.to_account_info(),
                to: ctx.accounts.new_token.to_account_info(),
            },
        ),
        1,
    )?;

    let old_metadata_account =
        try_from_slice_unchecked::<Metadata>(&ctx.accounts.old_metadata.data.borrow()).unwrap();

    let create_metadata = create_metadata_ix(
        &ctx.accounts.new_mint.key(),
        &ctx.accounts.derug_request.derugger.key(),
        Some(ctx.accounts.new_collection.key()),
        ctx.accounts.payer.key,
        &old_metadata_account.data.uri,
        &old_metadata_account.data.name,
        &old_metadata_account.data.symbol,
    );

    invoke(
        &create_metadata,
        &[
            ctx.accounts.new_metadata.to_account_info(),
            ctx.accounts.new_mint.to_account_info(),
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.metadata_program.to_account_info(),
        ],
    )?;

    let create_master_edition = create_master_edition_ix(
        &ctx.accounts.new_mint.key(),
        ctx.accounts.payer.key,
        ctx.accounts.payer.key,
    );

    invoke(
        &create_master_edition,
        &[
            ctx.accounts.new_metadata.to_account_info(),
            ctx.accounts.new_mint.to_account_info(),
            ctx.accounts.new_edition.to_account_info(),
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.metadata_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    ctx.accounts.derug_data.total_reminted = ctx
        .accounts
        .derug_data
        .total_reminted
        .checked_add(1)
        .unwrap();

    Ok(())
}
