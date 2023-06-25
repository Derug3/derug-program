use std::ops::DerefMut;

use crate::{
    constants::VOTE_RECORD_SEED,
    errors::DerugError,
    state::{derug_data::DerugStatus, derug_request::RequestStatus},
};
use anchor_lang::{
    prelude::*,
    system_program::{create_account, transfer, CreateAccount, Transfer},
    Discriminator,
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
    #[account(mut)]
    pub derug_request: Box<Account<'info, DerugRequest>>,
    #[account(mut, seeds =[DERUG_DATA_SEED, derug_data.collection.key().as_ref()], bump)]
    pub derug_data: Box<Account<'info, DerugData>>,
    ///CHECK
    #[account(mut, address="DRG3YRmurqpWQ1jEjK8DiWMuqPX9yL32LXLbuRdoiQwt".parse::<Pubkey>().unwrap())]
    pub fee_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn vote<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, Vote<'info>>) -> Result<()> {
    let remaining_accounts = ctx.remaining_accounts;

    let derug_request = &mut ctx.accounts.derug_request;
    derug_request.request_status = RequestStatus::Voting;
    ctx.accounts.derug_data.derug_status = DerugStatus::Voting;

    for (vote_record_info, nft_mint_info, nft_metadata_info, nft_token_account_info) in
        remaining_accounts.iter().tuples()
    {
        let nft_mint = Account::<Mint>::try_from(nft_mint_info)?;
        let nft_metadata = try_from_slice_unchecked::<Metadata>(&nft_metadata_info.data.borrow())?;
        let nft_token_account = Account::<TokenAccount>::try_from(nft_token_account_info)?;

        let (vote_record_pubkey, vote_record_bump) =
            VoteRecord::get_seeds(nft_mint_info.key, ctx.program_id);

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

        require!(vote_record_info.data_is_empty(), DerugError::AlereadyVoted);

        let mut account_data: Vec<u8> = Vec::new();

        account_data.extend_from_slice(&VoteRecord::discriminator());

        ctx.accounts
            .derug_data
            .active_requests
            .iter_mut()
            .find(|req| req.request == derug_request.key())
            .unwrap()
            .vote_count += 1;

        let vote_record = VoteRecord { voted: true };
        let vote_record_length = 8 + vote_record.try_to_vec()?.len();

        create_account(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                CreateAccount {
                    from: ctx.accounts.payer.to_account_info(),
                    to: vote_record_info.clone(),
                },
                &[&[
                    DERUG_DATA_SEED,
                    nft_mint.key().as_ref(),
                    VOTE_RECORD_SEED,
                    &[vote_record_bump],
                ]],
            ),
            Rent::default().minimum_balance(vote_record_length),
            vote_record_length as u64,
            ctx.program_id,
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

        derug_request.vote_count = derug_request.vote_count.checked_add(1).unwrap();

        vote_record.try_serialize(vote_record_info.try_borrow_mut_data()?.deref_mut())?;
    }

    Ok(())
}
