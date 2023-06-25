use anchor_lang::prelude::*;
use anchor_spl::token::{Approve, Mint, Token, TokenAccount};
use mpl_token_metadata::{instruction::freeze_delegated_account, ID};
use solana_program::program::invoke_signed;

#[derive(Accounts)]
pub struct FreezeNft<'info> {
    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,
    #[account(mut)]
    ///CHECK
    pub nft_master_edition: UncheckedAccount<'info>,
    #[account(init,seeds=[payer.key().as_ref(),nft_mint.key().as_ref()],bump,space=0,payer=payer)]
    /// CHECK
    pub delegate: UncheckedAccount<'info>,
    #[account(mut)]
    pub nft_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    ///CHECK
    pub metaplex_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn freeze_nft(ctx: Context<FreezeNft>) -> Result<()> {
    anchor_spl::token::approve(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Approve {
                authority: ctx.accounts.payer.to_account_info(),
                delegate: ctx.accounts.delegate.to_account_info(),
                to: ctx.accounts.nft_token_account.to_account_info(),
            },
        ),
        1,
    )?;

    let ix = freeze_delegated_account(
        ID,
        ctx.accounts.delegate.key(),
        ctx.accounts.nft_token_account.key(),
        ctx.accounts.nft_master_edition.key(),
        ctx.accounts.nft_mint.key(),
    );

    invoke_signed(
        &ix,
        &[
            ctx.accounts.delegate.to_account_info(),
            ctx.accounts.nft_token_account.to_account_info(),
            ctx.accounts.nft_master_edition.to_account_info(),
            ctx.accounts.nft_mint.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ],
        &[&[
            ctx.accounts.payer.key().as_ref(),
            ctx.accounts.nft_mint.key().as_ref(),
            &[*ctx.bumps.get(&"delegate".to_string()).unwrap()],
        ]],
    )?;

    Ok(())
}
