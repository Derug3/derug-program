use crate::{constants::VOTE_RECORD_SEED, errors::DerugError};
use anchor_lang::{
    prelude::*,
    system_program::{create_account, CreateAccount},
};
use anchor_spl::token::{Mint, TokenAccount};
use itertools::Itertools;
use mpl_token_metadata::state::Metadata;
use solana_program::borsh::try_from_slice_unchecked;

use crate::{
    constants::DERUG_DATA_SEED,
    state::{derug_data::DerugData, derug_request::DerugRequest, vote_record::VoteRecord},
};

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut, seeds= [DERUG_DATA_SEED, derug_data.key().as_ref(), payer.key().as_ref()], bump)]
    pub derug_request: Box<Account<'info, DerugRequest>>,
    #[account(mut, seeds =[DERUG_DATA_SEED, derug_data.collection.key().as_ref()], bump)]
    pub derug_data: Box<Account<'info, DerugData>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn vote<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, Vote<'info>>) -> Result<()> {
    let remaining_accounts = ctx.remaining_accounts;

    let derug_request = &mut ctx.accounts.derug_request;

    for (vote_record_info, nft_mint_info, nft_metadata_info, nft_token_account_info) in
        remaining_accounts.iter().tuples()
    {
        let nft_mint = Account::<Mint>::try_from(nft_mint_info)?;
        let nft_metadata = try_from_slice_unchecked::<Metadata>(&nft_metadata_info.data.borrow())?;
        let nft_token_account = Account::<TokenAccount>::try_from(nft_token_account_info)?;

        let (vote_record_pubkey, vote_record_bump) =
            VoteRecord::get_seeds(nft_mint_info.key, ctx.accounts.payer.key, ctx.program_id);

        require!(
            vote_record_info.key() == vote_record_pubkey,
            DerugError::InvalidVoteRecord
        );

        require!(
            nft_token_account.mint == nft_mint.key(),
            DerugError::InvalidTokenAccountMint
        );

        require!(
            nft_metadata.mint == nft_mint.key(),
            DerugError::InvalidMetadata
        );

        require!(nft_token_account.amount == 1, DerugError::EmptyTokenAccount);

        require!(
            nft_token_account.owner == ctx.accounts.payer.key(),
            DerugError::WrongOwner
        );

        let vote_record = VoteRecord { voted: true }.try_to_vec().unwrap();

        create_account(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                CreateAccount {
                    from: ctx.accounts.payer.to_account_info(),
                    to: vote_record_info.clone(),
                },
                &[&[
                    DERUG_DATA_SEED,
                    ctx.accounts.payer.key.as_ref(),
                    nft_mint.key().as_ref(),
                    VOTE_RECORD_SEED,
                    &[vote_record_bump],
                ]],
            ),
            Rent::default().minimum_balance(vote_record.len()),
            vote_record.len() as u64,
            ctx.program_id,
        )?;

        vote_record_info
            .data
            .borrow_mut()
            .copy_from_slice(&vote_record);
    }

    Ok(())
}
