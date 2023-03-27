use crate::{
    constants::{AUTHORITY_SEED, DERUG_DATA_SEED, REMINT_CONFIG_SEED},
    errors::DerugError,
    state::{
        derug_data::{DerugData, DerugStatus},
        derug_request::{DerugRequest, NftRemintedEvent, RemintConfig, RequestStatus},
    },
};
use anchor_lang::{prelude::*, system_program::transfer};
use anchor_spl::token::{
    initialize_account, initialize_mint, mint_to, InitializeAccount, InitializeMint, Mint, MintTo,
    Token, TokenAccount,
};

use mpl_token_metadata::{
    instruction::{create_master_edition_v3, create_metadata_accounts_v3},
    state::{Collection, Creator, Metadata, TokenMetadataAccount, EDITION, PREFIX},
    ID as METADATA_PROGRAM_ID,
};
use solana_program::program::{invoke, invoke_signed};

#[derive(Accounts)]
pub struct RemintNft<'info> {
    #[account()]
    pub derug_request: Box<Account<'info, DerugRequest>>,
    #[account(mut,seeds=[DERUG_DATA_SEED,derug_data.collection.key().as_ref()],bump)]
    pub derug_data: Box<Account<'info, DerugData>>,
    #[account()]
    pub new_collection: Box<Account<'info, Mint>>,
    #[account()]
    ///CHECK
    pub old_collection: UncheckedAccount<'info>,
    ///CHECK
    // #[account(mut, seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), old_collection.key().as_ref()], bump,seeds::program = METADATA_PROGRAM_ID)]
    // pub old_collection_metadata: UncheckedAccount<'info>,
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

    #[account(seeds=[REMINT_CONFIG_SEED,derug_data.key().as_ref()],bump)]
    pub remint_config: Account<'info, RemintConfig>,

    ///CHECK
    #[account(mut, seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), old_mint.key().as_ref(), EDITION.as_ref()],bump, seeds::program = METADATA_PROGRAM_ID)]
    pub old_edition: UncheckedAccount<'info>,
    ///CHECK
    #[account(mut, seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), new_mint.key().as_ref(), EDITION.as_ref()],bump, seeds::program = METADATA_PROGRAM_ID)]
    pub new_edition: UncheckedAccount<'info>,
    #[account(seeds=[DERUG_DATA_SEED,derug_request.key().as_ref(),AUTHORITY_SEED],bump)]
    ///CHECK
    pub pda_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    ///CHECK
    #[account(mut, address = "DRG3YRmurqpWQ1jEjK8DiWMuqPX9yL32LXLbuRdoiQwt".parse::<Pubkey>().unwrap())]
    pub fee_wallet: AccountInfo<'info>,
    ///CHECK
    #[account(address = METADATA_PROGRAM_ID)]
    pub metadata_program: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}

pub fn remint_nft<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, RemintNft<'info>>,
) -> Result<()> {
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

    if let Some(collection) = old_metadata_account.collection {
        require!(
            collection.key == ctx.accounts.derug_data.collection,
            DerugError::WrongCollection
        );
    } else {
        require!(
            old_metadata_account
                .data
                .creators
                .unwrap()
                .get(0)
                .unwrap()
                .address
                == ctx.accounts.derug_data.collection,
            DerugError::WrongCollection
        );
    }

    let mut burn_ix_accounts: Vec<AccountInfo> = vec![
        ctx.accounts.old_metadata.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.old_mint.to_account_info(),
        ctx.accounts.old_token.to_account_info(),
        ctx.accounts.old_edition.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
    ];

    let old_collection = if *ctx.accounts.old_collection.to_account_info().owner == spl_token::ID {
        let old_collection = ctx.remaining_accounts.iter().next().unwrap();
        burn_ix_accounts.push(old_collection.clone());
        Some(old_collection.key())
    } else {
        None
    };

    let burn_ix = mpl_token_metadata::instruction::burn_nft(
        METADATA_PROGRAM_ID,
        ctx.accounts.old_metadata.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.old_mint.key(),
        ctx.accounts.old_token.key(),
        ctx.accounts.old_edition.key(),
        ctx.accounts.token_program.key(),
        old_collection,
    );

    invoke(&burn_ix, &burn_ix_accounts)?;

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

    let derug_request = &ctx.accounts.derug_request;

    let len = ctx.accounts.derug_data.total_reminted.to_string().len();

    let formated_name = format!(
        "{} #{}{}",
        derug_request.new_name.to_uppercase(),
        "0".repeat(4 - len),
        ctx.accounts.derug_data.total_reminted + 1
    );

    let create_metadata = create_metadata_accounts_v3(
        ctx.accounts.metadata_program.key(),
        ctx.accounts.new_metadata.key(),
        ctx.accounts.new_mint.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.pda_authority.key(),
        formated_name,
        derug_request.new_symbol.clone(),
        old_metadata_account.data.uri,
        Some(vec![
            Creator {
                address: ctx.accounts.remint_config.candy_machine_key,
                share: 0,
                verified: false,
            },
            Creator {
                address: ctx.accounts.derug_request.derugger,
                share: 100,
                verified: false,
            },
        ]),
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

    invoke_signed(
        &create_metadata,
        &[
            ctx.accounts.new_metadata.to_account_info(),
            ctx.accounts.new_mint.to_account_info(),
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.pda_authority.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[&[
            DERUG_DATA_SEED,
            ctx.accounts.derug_request.key().as_ref(),
            AUTHORITY_SEED,
            &[*ctx.bumps.get(&"pda_authority".to_string()).unwrap()],
        ]],
    )?;

    let create_master_edition = create_master_edition_v3(
        ctx.accounts.metadata_program.key(),
        ctx.accounts.new_edition.key(),
        ctx.accounts.new_mint.key(),
        ctx.accounts.pda_authority.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.new_metadata.key(),
        ctx.accounts.payer.key(),
        Some(0),
    );

    invoke_signed(
        &create_master_edition,
        &[
            ctx.accounts.new_edition.to_account_info(),
            ctx.accounts.new_mint.to_account_info(),
            ctx.accounts.pda_authority.to_account_info().clone(),
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.new_metadata.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[&[
            DERUG_DATA_SEED,
            ctx.accounts.derug_request.key().as_ref(),
            AUTHORITY_SEED,
            &[*ctx.bumps.get(&"pda_authority".to_string()).unwrap()],
        ]],
    )?;

    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.fee_wallet.to_account_info(),
            },
        ),
        9000000,
    )?;

    ctx.accounts.derug_data.total_reminted = ctx
        .accounts
        .derug_data
        .total_reminted
        .checked_add(1)
        .unwrap();

    emit!(NftRemintedEvent {
        reminter: ctx.accounts.payer.key(),
        new_nft_mint: ctx.accounts.new_mint.key(),
        old_nft_mint: ctx.accounts.old_mint.key(),
        new_nft_metadata: ctx.accounts.new_metadata.key(),
        old_nft_metadata: ctx.accounts.old_metadata.key()
    });

    Ok(())
}
