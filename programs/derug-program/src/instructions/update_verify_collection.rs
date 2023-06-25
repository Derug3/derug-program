use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token::Mint;
use mpl_token_metadata::instruction::verify_sized_collection_item;
use mpl_token_metadata::state::EDITION;
use mpl_token_metadata::{instruction::update_metadata_accounts_v2, state::PREFIX};
use solana_program::program::invoke_signed;

use crate::constants::{AUTHORITY_SEED, DERUG_DATA_SEED};
use crate::errors::DerugError;
use crate::state::derug_data::DerugData;
use crate::state::derug_request::{DerugRequest, RequestStatus};
#[derive(Accounts)]
pub struct UpdateVerifyCollection<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    #[account()]
    nft_mint: Account<'info, Mint>,
    #[account(mut,seeds=[PREFIX.as_bytes(),metadata_program.key().as_ref(),nft_mint.key().as_ref()],bump,seeds::program=metadata_program.key())]
    ///CHECK
    nft_metadata: UncheckedAccount<'info>,
    #[account()]
    pub derug_request: Box<Account<'info, DerugRequest>>,
    #[account()]
    pub derug_data: Box<Account<'info, DerugData>>,
    #[account(seeds=[DERUG_DATA_SEED,derug_request.key().as_ref(),AUTHORITY_SEED],bump)]
    ///CHECK
    pub pda_authority: UncheckedAccount<'info>,
    #[account(mut,address=derug_data.new_collection.unwrap())]
    pub collection_mint: Box<Account<'info, Mint>>,
    #[account(mut,seeds=[PREFIX.as_bytes(),metadata_program.key().as_ref(),collection_mint.key().as_ref()],bump,seeds::program=metadata_program.key())]
    ///CHECK
    pub collection_metadata: UncheckedAccount<'info>,
    #[account(mut,seeds=[PREFIX.as_bytes(),metadata_program.key().as_ref(),collection_mint.key().as_ref(),EDITION.as_bytes()],bump,seeds::program=metadata_program.key())]
    ///CHECK
    pub collection_master_edition: UncheckedAccount<'info>,
    #[account(mut,address=derug_request.derugger)]
    derugger: SystemAccount<'info>,
    #[account()]
    ///CHECK
    pub collection_authority: UncheckedAccount<'info>,
    ///CHECK
    metadata_program: UncheckedAccount<'info>,
    ///CHECK
    #[account(mut, address="DRG3YRmurqpWQ1jEjK8DiWMuqPX9yL32LXLbuRdoiQwt".parse::<Pubkey>().unwrap())]
    pub fee_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn update_verify_collection(ctx: Context<UpdateVerifyCollection>) -> Result<()> {
    require!(
        ctx.accounts.derug_request.request_status == RequestStatus::Reminting,
        DerugError::InvalidWinningRequest
    );

    //Temporarily commented while we find out how to enable collection in candy machine

    let set_collection_ix = verify_sized_collection_item(
        ctx.accounts.metadata_program.key(),
        ctx.accounts.nft_metadata.key(),
        ctx.accounts.pda_authority.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.collection_mint.key(),
        ctx.accounts.collection_metadata.key(),
        ctx.accounts.collection_master_edition.key(),
        Some(ctx.accounts.collection_authority.key()),
    );

    invoke_signed(
        &set_collection_ix,
        &[
            ctx.accounts.nft_metadata.to_account_info(),
            ctx.accounts.pda_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.pda_authority.to_account_info(),
            ctx.accounts.collection_mint.to_account_info(),
            ctx.accounts.collection_metadata.to_account_info(),
            ctx.accounts.collection_master_edition.to_account_info(),
            ctx.accounts.collection_authority.to_account_info(),
        ],
        &[&[
            DERUG_DATA_SEED,
            ctx.accounts.derug_request.key().as_ref(),
            AUTHORITY_SEED,
            &[*ctx.bumps.get(&"pda_authority".to_string()).unwrap()],
        ]],
    )?;

    let update_ix = update_metadata_accounts_v2(
        ctx.accounts.metadata_program.key(),
        ctx.accounts.nft_metadata.key(),
        ctx.accounts.pda_authority.key(),
        Some(ctx.accounts.derugger.key()),
        None,
        None,
        None,
    );

    invoke_signed(
        &update_ix,
        &[
            ctx.accounts.nft_metadata.to_account_info(),
            ctx.accounts.pda_authority.to_account_info(),
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
            Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.fee_wallet.to_account_info(),
            },
        ),
        9000000,
    )?;

    Ok(())
}
