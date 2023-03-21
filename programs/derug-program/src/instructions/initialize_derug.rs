use crate::{
    constants::{DERUG_DATA_SEED, VOTING_TIME},
    state::{DerugData, DerugStatus},
    utilities::calculate_theshold_denominator,
};
use anchor_lang::prelude::*;
use mpl_token_metadata::state::Metadata;
use solana_program::borsh::try_from_slice_unchecked;
use spl_token::ID as TOKEN_PROGRAM_ID;
#[derive(Accounts)]
pub struct InitializeDerug<'info> {
    #[account()]
    ///CHECK
    pub collection_key: UncheckedAccount<'info>,
    #[account(init,payer=payer,seeds=[DERUG_DATA_SEED,collection_key.key().as_ref()], bump, space=DerugData::LEN)]
    pub derug_data: Box<Account<'info, DerugData>>,
    #[account()]
    ///CHECK
    pub collection_metadata: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_derug(
    ctx: Context<InitializeDerug>,
    total_supply: u32,
    slug: String,
) -> Result<()> {
    let collection_metadata =
        try_from_slice_unchecked::<Metadata>(&ctx.accounts.collection_metadata.data.borrow())
            .expect("Invalid collection metadata ");

    let derug_data = &mut ctx.accounts.derug_data;

    if *ctx.accounts.collection_key.to_account_info().owner == TOKEN_PROGRAM_ID {
        derug_data.collection_metadata = Some(ctx.accounts.collection_metadata.key());
    }

    derug_data.period_end = Clock::get().unwrap().unix_timestamp + VOTING_TIME;

    derug_data.date_added = Clock::get().unwrap().unix_timestamp;
    derug_data.collection = ctx.accounts.collection_key.key();
    derug_data.rug_update_authority = collection_metadata.update_authority;
    derug_data.collection_name = collection_metadata.data.name;
    derug_data.collection_uri = collection_metadata.data.uri;
    derug_data.collection_symbol = collection_metadata.data.symbol;
    derug_data.derug_status = DerugStatus::Initialized;
    derug_data.total_supply = total_supply;
    derug_data.total_suggestion_count = 0;
    derug_data.total_reminted = 0;
    derug_data.slug = slug;
    derug_data.threshold_denominator = calculate_theshold_denominator(total_supply);

    Ok(())
}
