use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use mpl_token_metadata::instruction::set_and_verify_sized_collection_item;
use mpl_token_metadata::state::EDITION;
use mpl_token_metadata::{instruction::update_metadata_accounts_v2, state::PREFIX};
use solana_program::program::invoke;

use crate::constants::DERUG_DATA_SEED;
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
    #[account(mut)]
    temp_authority: Signer<'info>,
    #[account(address=derug_data.new_collection.unwrap())]
    pub collection_mint: Box<Account<'info, Mint>>,
    #[account(mut,seeds=[PREFIX.as_bytes(),metadata_program.key().as_ref(),collection_mint.key().as_ref()],bump,seeds::program=metadata_program.key())]
    ///CHECK
    pub collection_metadata: UncheckedAccount<'info>,
    #[account(mut,seeds=[PREFIX.as_bytes(),metadata_program.key().as_ref(),collection_mint.key().as_ref(),EDITION.as_bytes()],bump,seeds::program=metadata_program.key())]
    ///CHECK
    pub collection_master_edition: UncheckedAccount<'info>,
    #[account(address=derug_request.derugger)]
    derugger: SystemAccount<'info>,
    ///CHECK
    metadata_program: UncheckedAccount<'info>,
}

pub fn update_verify_collection(ctx: Context<UpdateVerifyCollection>) -> Result<()> {
    require!(
        ctx.accounts.derug_request.request_status == RequestStatus::Reminting,
        DerugError::InvalidWinningRequest
    );

    let set_collection_ix = set_and_verify_sized_collection_item(
        ctx.accounts.metadata_program.key(),
        ctx.accounts.nft_metadata.key(),
        ctx.accounts.temp_authority.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.derugger.key(),
        ctx.accounts.collection_mint.key(),
        ctx.accounts.collection_metadata.key(),
        ctx.accounts.collection_master_edition.key(),
        None,
    );

    invoke(
        &set_collection_ix,
        &[
            ctx.accounts.nft_metadata.to_account_info(),
            ctx.accounts.temp_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.derug_request.to_account_info(),
            ctx.accounts.collection_mint.to_account_info(),
            ctx.accounts.collection_metadata.to_account_info(),
            ctx.accounts.collection_master_edition.to_account_info(),
        ],
    )?;

    let update_ix = update_metadata_accounts_v2(
        ctx.accounts.metadata_program.key(),
        ctx.accounts.nft_metadata.key(),
        ctx.accounts.temp_authority.key(),
        Some(ctx.accounts.derugger.key()),
        None,
        None,
        None,
    );

    invoke(
        &update_ix,
        &[
            ctx.accounts.nft_metadata.to_account_info(),
            ctx.accounts.temp_authority.to_account_info(),
        ],
    )?;

    Ok(())
}
