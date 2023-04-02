use anchor_lang::prelude::*;

use crate::state::{
    derug_data::DerugData,
    derug_request::{DerugRequest, RemintConfig},
};

#[derive(Accounts)]
pub struct CloseProgramAccount<'info> {
    #[account(mut)]
    pub derug_data: Account<'info, DerugData>,
    #[account(mut)]
    pub derug_request: Account<'info, DerugRequest>,
    #[account()]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub remint_config: Account<'info, RemintConfig>,
}

pub fn close_program_account<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, CloseProgramAccount<'info>>,
) -> Result<()> {
    // ctx.accounts
    //     .derug_data
    //     .close(ctx.accounts.payer.to_account_info())
    //     .unwrap();

    // ctx.accounts
    //     .derug_request
    //     .close(ctx.accounts.payer.to_account_info())
    //     .unwrap();

    ctx.accounts
        .remint_config
        .close(ctx.accounts.payer.to_account_info())
        .unwrap();

    // let remaining_accounts = &mut ctx.remaining_accounts.iter();

    // for rem_acc in remaining_accounts {
    //     let vote_record = Account::<VoteRecord>::try_from(&rem_acc).unwrap();

    //     vote_record
    //         .close(ctx.accounts.payer.to_account_info())
    //         .unwrap();
    // }

    Ok(())
}
