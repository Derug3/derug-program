use crate::{
    constants::{AUTHORITY_SEED, DERUG_DATA_SEED, REMINT_CONFIG_SEED},
    errors::DerugError,
    state::{
        derug_data::{DerugData, DerugStatus},
        derug_request::{DerugRequest, RemintConfig, RequestStatus},
    },
};
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::token::{
    initialize_account, initialize_mint, mint_to, InitializeAccount, InitializeMint, MintTo, Token,
};

use mpl_token_metadata::{
    instruction::{
        approve_collection_authority, create_master_edition_v3, create_metadata_accounts_v3,
    },
    state::{Creator, EDITION, PREFIX},
    ID as METADATA_PROGRAM_ID,
};
use solana_program::program::invoke;

#[derive(Accounts)]
pub struct InitializeReminting<'info> {
    #[account(mut, seeds=[DERUG_DATA_SEED, derug_data.key().as_ref(), payer.key().as_ref()], bump)]
    pub derug_request: Account<'info, DerugRequest>,
    #[account(mut,seeds=[DERUG_DATA_SEED,derug_data.collection.key().as_ref()],bump)]
    pub derug_data: Box<Account<'info, DerugData>>,
    #[account(mut)]
    ///CHECK
    pub new_collection: UncheckedAccount<'info>,
    ///CHECK
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    ///CHECK
    #[account(mut,seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), new_collection.key().as_ref(), EDITION.as_ref()],bump, seeds::program = METADATA_PROGRAM_ID)]
    pub master_edition: UncheckedAccount<'info>,
    ///CHECK
    #[account(mut,seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), new_collection.key().as_ref()], bump,seeds::program = METADATA_PROGRAM_ID)]
    pub metadata_account: UncheckedAccount<'info>,

    #[account(seeds=[DERUG_DATA_SEED,derug_request.key().as_ref(),AUTHORITY_SEED],bump)]
    ///CHECK
    pub pda_authority: UncheckedAccount<'info>,
    #[account(mut,seeds=[REMINT_CONFIG_SEED,derug_data.key().as_ref()],bump)]
    pub remint_config: Account<'info, RemintConfig>,

    #[account(mut)]
    ///CHECK
    pub collection_authority_record: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    ///CHECK
    #[account(mut, address="DRG3YRmurqpWQ1jEjK8DiWMuqPX9yL32LXLbuRdoiQwt".parse::<Pubkey>().unwrap())]
    pub fee_wallet: AccountInfo<'info>,

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

    ctx.accounts.remint_config.collection = ctx.accounts.new_collection.key();

    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.derug_data.to_account_info(),
            },
        ),
        Rent::default().minimum_balance(32),
    )?;

    ctx.accounts.derug_data.to_account_info().realloc(
        ctx.accounts
            .derug_data
            .to_account_info()
            .data_len()
            .checked_add(33)
            .unwrap(),
        false,
    )?;

    ctx.accounts.derug_data.new_collection = Some(ctx.accounts.new_collection.key());

    initialize_mint(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            InitializeMint {
                mint: ctx.accounts.new_collection.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        0,
        ctx.accounts.payer.key,
        Some(ctx.accounts.payer.key),
    )?;

    initialize_account(CpiContext::new(
        ctx.accounts.token_account.to_account_info(),
        InitializeAccount {
            account: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
            mint: ctx.accounts.new_collection.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
    ))?;

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

    let create_metadata = create_metadata_accounts_v3(
        ctx.accounts.metadata_program.key(),
        ctx.accounts.metadata_account.key(),
        ctx.accounts.new_collection.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.derug_request.new_name.clone(),
        ctx.accounts.derug_request.new_symbol.clone(),
        ctx.accounts.derug_data.collection_uri.clone(),
        Some(vec![Creator {
            address: ctx.accounts.payer.key(),
            share: 100,
            verified: true,
        }]),
        500,
        true,
        true,
        None,
        None,
        Some(mpl_token_metadata::state::CollectionDetails::V1 { size: 0 }),
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

    let create_master_edition = create_master_edition_v3(
        ctx.accounts.metadata_program.key(),
        ctx.accounts.master_edition.key(),
        ctx.accounts.new_collection.key(),
        ctx.accounts.derug_request.derugger,
        ctx.accounts.payer.key(),
        ctx.accounts.metadata_account.key(),
        ctx.accounts.payer.key(),
        Some(0),
    );

    invoke(
        &create_master_edition,
        &[
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.new_collection.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
    )?;

    let approve_collection_authority_ix = approve_collection_authority(
        ctx.accounts.metadata_program.key(),
        ctx.accounts.collection_authority_record.key(),
        ctx.accounts.pda_authority.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.metadata_account.key(),
        ctx.accounts.new_collection.key(),
    );

    let approve_accounts = vec![
        ctx.accounts.metadata_account.to_account_info(),
        ctx.accounts.collection_authority_record.to_account_info(),
        ctx.accounts.pda_authority.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.metadata_account.to_account_info(),
        ctx.accounts.new_collection.to_account_info(),
    ];

    invoke(&approve_collection_authority_ix, &approve_accounts)?;

    ctx.accounts.derug_request.request_status = RequestStatus::UploadingMetadata;
    ctx.accounts.derug_data.derug_status = DerugStatus::UploadingMetadata;

    // transfer(
    //     CpiContext::new(
    //         ctx.accounts.system_program.to_account_info(),
    //         Transfer {
    //             from: ctx.accounts.payer.to_account_info(),
    //             to: ctx.accounts.fee_wallet.to_account_info(),
    //         },
    //     ),
    //     9000000,
    // )?;

    Ok(())
}
