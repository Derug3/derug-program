use crate::{
    constants::DERUG_DATA_SEED,
    errors::DerugError,
    state::{
        derug_data::{DerugData, DerugStatus},
        derug_request::{DerugRequest, RequestStatus},
    },
};
use anchor_lang::prelude::*;
use anchor_spl::token::{
    initialize_account, initialize_mint, mint_to, InitializeAccount, InitializeMint, Mint, MintTo,
    Token, TokenAccount,
};

use mpl_token_metadata::{
    instruction::{create_master_edition_v3, create_metadata_accounts_v3},
    state::{Collection, Creator, Metadata, TokenMetadataAccount, EDITION, PREFIX},
    ID as METADATA_PROGRAM_ID,
};
use solana_program::program::invoke;

#[derive(Accounts)]
pub struct RemintNft<'info> {
    #[account()]
    pub derug_request: Box<Account<'info, DerugRequest>>,
    #[account(mut,seeds=[DERUG_DATA_SEED,derug_data.collection.key().as_ref()],bump)]
    pub derug_data: Box<Account<'info, DerugData>>,
    #[account()]
    pub new_collection: Box<Account<'info, Mint>>,
    #[account()]
    pub old_collection: Box<Account<'info, Mint>>,
    ///CHECK
    #[account(mut, seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), old_collection.key().as_ref()], bump,seeds::program = METADATA_PROGRAM_ID)]
    pub old_collection_metadata: UncheckedAccount<'info>,
    #[account(mut)]
    pub old_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    ///CHECK
    pub new_mint: UncheckedAccount<'info>,
    //TODO: Require
    #[account(mut)]
    pub old_token: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    ///CHECK
    pub new_token: UncheckedAccount<'info>,

    ///CHECK
    #[account(mut, seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), old_mint.key().as_ref()], bump,seeds::program = METADATA_PROGRAM_ID)]
    pub old_metadata: UncheckedAccount<'info>,
    ///CHECK
    #[account(mut, seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), new_mint.key().as_ref()], bump,seeds::program = METADATA_PROGRAM_ID)]
    pub new_metadata: UncheckedAccount<'info>,

    ///CHECK
    #[account(mut, seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), old_mint.key().as_ref(), EDITION.as_ref()],bump, seeds::program = METADATA_PROGRAM_ID)]
    pub old_edition: UncheckedAccount<'info>,
    ///CHECK
    #[account(mut, seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), new_mint.key().as_ref(), EDITION.as_ref()],bump, seeds::program = METADATA_PROGRAM_ID)]
    pub new_edition: UncheckedAccount<'info>,

    #[account(mut)]
    pub temporary_authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    ///CHECK
    #[account(address = METADATA_PROGRAM_ID)]
    pub metadata_program: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}

pub fn remint_nft(ctx: Context<RemintNft>) -> Result<()> {
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

    initialize_mint(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            InitializeMint {
                mint: ctx.accounts.new_mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        0,
        ctx.accounts.payer.key,
        Some(ctx.accounts.payer.key),
    )?;

    initialize_account(CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        InitializeAccount {
            account: ctx.accounts.new_token.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
            mint: ctx.accounts.new_mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
    ))?;

    let old_metadata_account = Metadata::from_account_info(&ctx.accounts.old_metadata).unwrap();

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

    let create_metadata = create_metadata_accounts_v3(
        ctx.accounts.metadata_program.key(),
        ctx.accounts.new_metadata.key(),
        ctx.accounts.new_mint.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.temporary_authority.key(),
        old_metadata_account.data.name,
        old_metadata_account.data.symbol,
        old_metadata_account.data.uri,
        Some(vec![Creator {
            address: ctx.accounts.derug_request.derugger,
            share: 100,
            verified: false,
        }]),
        500,
        true,
        true,
        Some(Collection {
            key: ctx.accounts.new_collection.key(),
            verified: false,
        }),
        None,
        None,
    );

    invoke(
        &create_metadata,
        &[
            ctx.accounts.new_metadata.to_account_info(),
            ctx.accounts.new_mint.to_account_info(),
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.temporary_authority.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    let create_master_edition = create_master_edition_v3(
        ctx.accounts.metadata_program.key(),
        ctx.accounts.new_edition.key(),
        ctx.accounts.new_mint.key(),
        ctx.accounts.temporary_authority.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.new_metadata.key(),
        ctx.accounts.payer.key(),
        Some(0),
    );

    invoke(
        &create_master_edition,
        &[
            ctx.accounts.new_edition.to_account_info(),
            ctx.accounts.new_mint.to_account_info(),
            ctx.accounts.temporary_authority.to_account_info().clone(),
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.new_metadata.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
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