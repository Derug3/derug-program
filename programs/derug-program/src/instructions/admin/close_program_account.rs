use anchor_lang::prelude::*;

use crate::state::{
    derug_data::DerugData,
    derug_request::{DerugRequest, RemintConfig},
    vote_record::VoteRecord,
};

#[derive(Accounts)]
pub struct CloseProgramAccount<'info> {
    #[account(mut)]
    ///CHECK
    pub derug_data: UncheckedAccount<'info>,
    #[account(mut)]
    ///CHECK
    pub derug_request: UncheckedAccount<'info>,
    #[account()]
    pub payer: Signer<'info>,
    #[account(mut)]
    ///CHECK
    pub remint_config: UncheckedAccount<'info>,
}

pub fn close_program_account<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, CloseProgramAccount<'info>>,
) -> Result<()> {
    if ctx.accounts.derug_data.data_len() > 8 {
        let derug_data = Account::<DerugData>::try_from(&ctx.accounts.derug_data).unwrap();

        derug_data.close(ctx.accounts.payer.to_account_info())?;
    }
    if ctx.accounts.derug_request.data_len() > 8 {
        let derug_request = Account::<DerugRequest>::try_from(&ctx.accounts.derug_request).unwrap();

        derug_request.close(ctx.accounts.payer.to_account_info())?;
    }

    if ctx.accounts.remint_config.data_len() > 8 {
        let remint_config = Account::<RemintConfig>::try_from(&ctx.accounts.remint_config).unwrap();

        remint_config.close(ctx.accounts.payer.to_account_info())?;
    }

    let remaining_accounts = &mut ctx.remaining_accounts.iter();

    for rem_acc in remaining_accounts {
        let vote_record = Account::<VoteRecord>::try_from(&rem_acc).unwrap();

        vote_record
            .close(ctx.accounts.payer.to_account_info())
            .unwrap();
    }

    Ok(())
}
