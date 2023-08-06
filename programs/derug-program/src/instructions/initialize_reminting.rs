use crate::{
    constants::DERUG_DATA_SEED,
    state::{derug_data::DerugData, derug_request::DerugRequest},
};
use anchor_lang::{prelude::*, system_program::Transfer};
use anchor_spl::{associated_token::AssociatedToken, token::Token};

use mpl_token_metadata::{
    instruction::{
        builders::{CreateBuilder, MintBuilder},
        CreateArgs, InstructionBuilder, MintArgs,
    },
    state::{AssetData, Creator},
};
use solana_program::program::invoke;

#[derive(Accounts)]
pub struct InitializeReminting<'info> {
    #[account(mut, seeds=[DERUG_DATA_SEED, derug_data.key().as_ref(), payer.key().as_ref()], bump)]
    pub derug_request: Account<'info, DerugRequest>,
    #[account(mut,seeds=[DERUG_DATA_SEED,derug_data.collection.key().as_ref()],bump)]
    pub derug_data: Box<Account<'info, DerugData>>,
    #[account()]
    ///CHECK
    pub authority: Signer<'info>,
    #[account(mut)]
    ///CHECK
    pub new_collection: Signer<'info>,
    ///CHECK
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    ///CHECK
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    ///CHECK
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    ///CHECK
    #[account(mut, address="DRG3YRmurqpWQ1jEjK8DiWMuqPX9yL32LXLbuRdoiQwt".parse::<Pubkey>().unwrap())]
    pub fee_wallet: AccountInfo<'info>,
    ///CHECK
    #[account()]
    pub metadata_program: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    #[account()]
    ///CHECK
    pub sysvar_instructions: UncheckedAccount<'info>,
    #[account()]
    ///CHECK
    pub metaplex_foundation_ruleset: UncheckedAccount<'info>,
    #[account()]
    ///CHECK
    pub metaplex_authorization_rules: UncheckedAccount<'info>,
    pub spl_ata_program: Program<'info, AssociatedToken>,
    #[account(mut)]
    ///CHECK
    pub token_record: UncheckedAccount<'info>,
}

pub fn initialize_reminting(ctx: Context<InitializeReminting>) -> Result<()> {
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

    let asset = AssetData {
        collection: None,
        collection_details: Some(mpl_token_metadata::state::CollectionDetails::V1 {
            size: ctx.accounts.derug_data.total_supply.into(),
        }),
        creators: Some(vec![
            Creator {
                address: ctx.accounts.authority.key(),
                share: 0,
                verified: false,
            },
            Creator {
                address: ctx.accounts.payer.key(),
                share: 100,
                verified: false,
            },
        ]),
        is_mutable: true,
        name: ctx.accounts.derug_request.new_name.clone(),
        primary_sale_happened: false,
        seller_fee_basis_points: ctx.accounts.derug_request.mint_config.seller_fee_bps,
        symbol: ctx.accounts.derug_request.new_symbol.clone(),
        token_standard: mpl_token_metadata::state::TokenStandard::ProgrammableNonFungible,
        uri: ctx.accounts.derug_data.collection_uri.clone(),
        uses: None,
        rule_set: None,
    };

    let create_ix = CreateBuilder::new()
        .authority(ctx.accounts.authority.key())
        .initialize_mint(true)
        .master_edition(ctx.accounts.master_edition.key())
        .metadata(ctx.accounts.metadata_account.key())
        .mint(ctx.accounts.new_collection.key())
        .payer(ctx.accounts.payer.key())
        .update_authority(ctx.accounts.authority.key())
        .update_authority_as_signer(true)
        .build(CreateArgs::V1 {
            asset_data: asset,
            decimals: Some(0),
            print_supply: Some(mpl_token_metadata::state::PrintSupply::Zero),
        })
        .unwrap()
        .instruction();

    invoke(
        &create_ix,
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.new_collection.to_account_info(),
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
        .master_edition(ctx.accounts.master_edition.key())
        .metadata(ctx.accounts.metadata_account.key())
        .mint(ctx.accounts.new_collection.key())
        .token_record(ctx.accounts.token_record.key())
        .payer(ctx.accounts.payer.key())
        .spl_token_program(ctx.accounts.token_program.key())
        .token(ctx.accounts.token_account.key())
        .spl_ata_program(ctx.accounts.spl_ata_program.key())
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
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.new_collection.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.token_record.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.sysvar_instructions.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.metaplex_authorization_rules.to_account_info(),
            ctx.accounts.metaplex_foundation_ruleset.to_account_info(),
            ctx.accounts.spl_ata_program.to_account_info(),
        ],
    )?;

    Ok(())
}
