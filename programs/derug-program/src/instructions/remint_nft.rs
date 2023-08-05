use crate::{
    constants::{AUTHORITY_SEED, DERUG, DERUG_DATA_SEED},
    errors::DerugError,
    state::{
        derug_data::DerugData,
        derug_request::{DerugRequest, NftRemintedEvent, RemintProof},
    },
    utilities::extract_name,
};
use anchor_lang::{prelude::*, system_program::transfer};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use mpl_token_metadata::{
    instruction::{
        builders::{BurnBuilder, CreateBuilder, MintBuilder, VerifyBuilder},
        BurnArgs, CreateArgs, InstructionBuilder, MintArgs, VerificationArgs,
    },
    state::AssetData,
};
use std::mem::size_of;

use mpl_token_metadata::{
    state::{Collection, Creator, Metadata, TokenMetadataAccount, TokenStandard, EDITION, PREFIX},
    ID as METADATA_PROGRAM_ID,
};
use solana_program::program::{invoke, invoke_signed};

#[derive(Accounts)]
pub struct RemintNft<'info> {
    #[account()]
    pub derug_request: Box<Account<'info, DerugRequest>>,
    #[account(mut,seeds=[DERUG_DATA_SEED,derug_data.collection.key().as_ref()],bump)]
    pub derug_data: Box<Account<'info, DerugData>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account()]
    pub new_collection: Box<Account<'info, Mint>>,
    ///CHECK:address checekd
    #[account(address=derug_request.derugger)]
    pub derugger: UncheckedAccount<'info>,
    #[account(init,payer=payer,seeds=[DERUG,old_mint.key().as_ref()],bump,space=8+size_of::<RemintProof>())]
    pub remint_proof: Box<Account<'info, RemintProof>>,
    #[account()]
    ///CHECK
    pub old_collection: UncheckedAccount<'info>,
    #[account(mut)]
    pub old_mint: Box<Account<'info, Mint>>,
    #[account()]
    ///CHECK:initialized by mpl-program
    pub new_mint: UncheckedAccount<'info>,
    //TODO: Require
    #[account(mut)]
    pub old_token: Box<Account<'info, TokenAccount>>,
    #[account()]
    ///CHECK:initialized by mpl-program
    pub new_token: UncheckedAccount<'info>,
    #[account(mut, seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), old_mint.key().as_ref()], bump,seeds::program = METADATA_PROGRAM_ID)]
    ///CHECK:seeds checked
    pub old_metadata: UncheckedAccount<'info>,
    ///CHECK
    #[account(mut, seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), new_mint.key().as_ref()], bump,seeds::program = METADATA_PROGRAM_ID)]
    pub new_metadata: UncheckedAccount<'info>,
    ///CHECK
    #[account(seeds=[DERUG,derug_request.mint_config.candy_machine_key.as_ref()],bump)]
    pub first_creator: UncheckedAccount<'info>,
    ///CHECK
    #[account(mut, seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), old_mint.key().as_ref(), EDITION.as_ref()],bump, seeds::program = METADATA_PROGRAM_ID)]
    pub old_edition: UncheckedAccount<'info>,
    ///CHECK
    #[account(mut, seeds=[PREFIX.as_ref(), METADATA_PROGRAM_ID.as_ref(), new_mint.key().as_ref(), EDITION.as_ref()],bump, seeds::program = METADATA_PROGRAM_ID)]
    pub new_edition: UncheckedAccount<'info>,
    #[account(seeds=[DERUG_DATA_SEED,derug_request.key().as_ref(),AUTHORITY_SEED],bump)]
    ///CHECK
    pub pda_authority: UncheckedAccount<'info>,
    #[account()]
    pub collection_mint: Account<'info, Mint>,
    #[account()]
    ///CHECK
    pub collection_metadata: UncheckedAccount<'info>,
    ///CHECK
    pub collection_master_edition: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    ///CHECK
    #[account(mut, address = "DRG3YRmurqpWQ1jEjK8DiWMuqPX9yL32LXLbuRdoiQwt".parse::<Pubkey>().unwrap())]
    pub fee_wallet: AccountInfo<'info>,
    #[account()]
    ///CHECK
    pub metaplex_foundation_ruleset: UncheckedAccount<'info>,
    #[account()]
    ///CHECK
    pub metaplex_authorization_rules: UncheckedAccount<'info>,
    ///CHECK
    #[account(address = METADATA_PROGRAM_ID)]
    pub metadata_program: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    ///CHECK:checked by mpl_token_metadata
    pub sysvar_instructions: UncheckedAccount<'info>,
    ///CHECK:checked by mpl_token_metadata
    pub spl_ata_program: Program<'info, AssociatedToken>,
}

pub fn remint_nft<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, RemintNft<'info>>,
) -> Result<()> {
    let derug_request = &ctx.accounts.derug_request;
    require!(
        ctx.accounts.old_collection.key() == ctx.accounts.derug_data.collection,
        DerugError::WrongCollection
    );

    let remint_proof = &mut ctx.accounts.remint_proof;

    remint_proof.derug_data = ctx.accounts.derug_data.key();
    remint_proof.new_mint = ctx.accounts.new_mint.key();
    remint_proof.reminter = ctx.accounts.payer.key();
    remint_proof.old_mint = ctx.accounts.old_mint.key();

    //TODO:comment in

    // require!(
    //     ctx.accounts.derug_data.derug_status == DerugStatus::Reminting,
    //     DerugError::InvalidStatus
    // );

    // require!(
    //     ctx.accounts.derug_request.request_status == RequestStatus::Reminting,
    //     DerugError::InvalidStatus
    // );

    //TODO:Comment in

    // require!(
    //     ctx.accounts.derug_data.winning_request.unwrap() == ctx.accounts.derug_request.key(),
    //     DerugError::NoWinner
    // );

    // require!(
    //     Clock::get().unwrap().unix_timestamp <= derug_request.mint_config.remint_duration,
    //     DerugError::PrivateMintEnded
    // );

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

    let mut creators_vec: Vec<Creator> = ctx
        .accounts
        .derug_request
        .creators
        .iter()
        .map(|c| Creator {
            address: c.address,
            share: c.share,
            verified: false,
        })
        .collect();

    creators_vec.insert(
        0,
        Creator {
            address: ctx.accounts.first_creator.key(),
            verified: false,
            share: 0,
        },
    );

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

    let old_collection = if *ctx.accounts.old_collection.to_account_info().owner == spl_token::ID {
        let old_collection = ctx.remaining_accounts.iter().next().unwrap();

        Some(old_collection.key())
    } else {
        None
    };

    let mut burn_builder = BurnBuilder::new();
    burn_builder
        .authority(ctx.accounts.payer.key())
        .collection_metadata(ctx.accounts.collection_metadata.key())
        .master_edition(ctx.accounts.old_edition.key())
        .metadata(ctx.accounts.old_metadata.key())
        .mint(ctx.accounts.old_mint.key())
        .token(ctx.accounts.old_token.key());
    if old_collection.is_some() {
        burn_builder.collection_metadata(ctx.accounts.collection_metadata.key());
    }

    let burn_ix = burn_builder
        .build(BurnArgs::V1 { amount: 1 })
        .unwrap()
        .instruction();

    invoke(
        &burn_ix,
        &[
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.collection_metadata.to_account_info(),
            ctx.accounts.old_metadata.to_account_info(),
            ctx.accounts.old_edition.to_account_info(),
            ctx.accounts.old_mint.to_account_info(),
            ctx.accounts.old_token.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.sysvar_instructions.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ],
    )?;

    let asset = AssetData {
        collection: Some(Collection {
            verified: false,
            key: ctx.accounts.collection_mint.key(),
        }),
        is_mutable: true,
        primary_sale_happened: false,
        uses: None,
        uri: old_metadata_account.data.uri,
        collection_details: None,
        name: format!(
            "${} #${}",
            derug_request.new_name,
            extract_name(&old_metadata_account.data.name)
        ),
        symbol: ctx.accounts.derug_request.new_symbol.clone(),
        rule_set: Some(ctx.accounts.metaplex_foundation_ruleset.key()),
        seller_fee_basis_points: ctx.accounts.derug_request.mint_config.seller_fee_bps,
        token_standard: TokenStandard::ProgrammableNonFungible,
        creators: Some(creators_vec),
    };

    let create_ix = CreateBuilder::new()
        .authority(ctx.accounts.authority.key())
        .initialize_mint(true)
        .master_edition(ctx.accounts.new_edition.key())
        .metadata(ctx.accounts.new_metadata.key())
        .mint(ctx.accounts.new_mint.key())
        .payer(ctx.accounts.payer.key())
        .update_authority(ctx.accounts.authority.key())
        .update_authority_as_signer(true)
        .build(CreateArgs::V1 {
            asset_data: asset,
            decimals: Some(0),
            print_supply: None,
        })
        .unwrap()
        .instruction();

    invoke(
        &create_ix,
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.new_edition.to_account_info(),
            ctx.accounts.new_metadata.to_account_info(),
            ctx.accounts.new_mint.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.sysvar_instructions.to_account_info(),
        ],
    )?;

    let mint_ix = MintBuilder::new()
        .authority(ctx.accounts.authority.key())
        .authorization_rules(ctx.accounts.metaplex_foundation_ruleset.key())
        .authorization_rules_program(ctx.accounts.metaplex_authorization_rules.key())
        .master_edition(ctx.accounts.new_edition.key())
        .metadata(ctx.accounts.new_metadata.key())
        .mint(ctx.accounts.new_mint.key())
        .payer(ctx.accounts.payer.key())
        .spl_token_program(ctx.accounts.token_program.key())
        .token(ctx.accounts.new_token.key())
        .token_owner(ctx.accounts.payer.key())
        .build(MintArgs::V1 {
            amount: 1,
            authorization_data: None,
        })
        .unwrap()
        .instruction();

    invoke(
        &mint_ix,
        &[
            ctx.accounts.new_token.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.new_metadata.to_account_info(),
            ctx.accounts.new_edition.to_account_info(),
            ctx.accounts.new_mint.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.sysvar_instructions.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.metaplex_authorization_rules.to_account_info(),
            ctx.accounts.spl_ata_program.to_account_info(),
        ],
    )?;

    let verify_collection_ix = VerifyBuilder::new()
        .authority(ctx.accounts.authority.key())
        .collection_master_edition(ctx.accounts.collection_master_edition.key())
        .collection_metadata(ctx.accounts.collection_metadata.key())
        .collection_mint(ctx.accounts.collection_mint.key())
        .metadata(ctx.accounts.new_metadata.key())
        .build(VerificationArgs::CollectionV1 {})
        .unwrap()
        .instruction();

    invoke(
        &verify_collection_ix,
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.new_mint.to_account_info(),
            ctx.accounts.collection_mint.to_account_info(),
            ctx.accounts.collection_metadata.to_account_info(),
            ctx.accounts.collection_master_edition.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.sysvar_instructions.to_account_info(),
        ],
    )?;

    let verify_creator = VerifyBuilder::new()
        .authority(ctx.accounts.first_creator.key())
        .collection_master_edition(ctx.accounts.collection_master_edition.key())
        .collection_metadata(ctx.accounts.collection_metadata.key())
        .collection_mint(ctx.accounts.collection_mint.key())
        .metadata(ctx.accounts.new_metadata.key())
        .build(VerificationArgs::CreatorV1 {})
        .unwrap()
        .instruction();

    invoke_signed(
        &verify_creator,
        &[
            ctx.accounts.first_creator.to_account_info(),
            ctx.accounts.new_mint.to_account_info(),
            ctx.accounts.collection_mint.to_account_info(),
            ctx.accounts.collection_metadata.to_account_info(),
            ctx.accounts.collection_master_edition.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.sysvar_instructions.to_account_info(),
        ],
        &[&[
            DERUG,
            ctx.accounts
                .derug_request
                .mint_config
                .candy_machine_key
                .as_ref(),
            &[*ctx.bumps.get(&"first_creator".to_string()).unwrap()],
        ]],
    )?;

    emit!(NftRemintedEvent {
        reminter: ctx.accounts.payer.key(),
        new_nft_mint: ctx.accounts.new_mint.key(),
        old_nft_mint: ctx.accounts.old_mint.key(),
        new_nft_metadata: ctx.accounts.new_metadata.key(),
        old_nft_metadata: ctx.accounts.old_metadata.key()
    });

    Ok(())
}
