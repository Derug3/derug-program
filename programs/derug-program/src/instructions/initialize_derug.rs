use crate::{
    constants::{DERUG_DATA_SEED, METADATA_SEED},
    state::{DerugData, DerugStatus},
};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use mpl_candy_machine::state::CandyMachine;
use mpl_token_metadata::{state::Metadata, ID as METADATA_PROGRAM};
use solana_program::borsh::try_from_slice_unchecked;
#[derive(Accounts)]
pub struct InitializeDerug<'info> {
    #[account()]
    pub collection_key: Box<Account<'info, Mint>>,
    #[account(init,payer=payer,seeds=[DERUG_DATA_SEED,collection_key.key().as_ref()],bump,space=DerugData::LEN)]
    pub derug_data: Box<Account<'info, DerugData>>,
    #[account(seeds=[METADATA_SEED,collection_key.key().as_ref(),METADATA_PROGRAM.as_ref()],bump,seeds::program=METADATA_PROGRAM)]
    pub collection_metadata: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_derug(ctx: Context<InitializeDerug>, total_supply: u32) -> Result<()> {
    let collection_metadata =
        try_from_slice_unchecked::<Metadata>(&ctx.accounts.collection_metadata.data.borrow())
            .expect("Invalid collection metadata ");

    let derug_data = &mut ctx.accounts.derug_data;

    derug_data.date_added = Clock::get().unwrap().unix_timestamp;
    derug_data.collection = ctx.accounts.collection_key.key();

    if let Some(candy_machine) = &mut ctx.remaining_accounts.iter().peekable().peek() {
        let _candy_machine_account =
            try_from_slice_unchecked::<CandyMachine>(&candy_machine.data.borrow())
                .expect("Invalid candy machine");

        derug_data.candy_machine = Some(candy_machine.key());
    }

    derug_data.rug_update_authority = collection_metadata.update_authority;
    derug_data.collection_metadata = ctx.accounts.collection_metadata.key();
    derug_data.collection_name = collection_metadata.data.name;
    derug_data.collection_uri = collection_metadata.data.uri;
    derug_data.collection_symbol = collection_metadata.data.symbol;
    derug_data.derug_status = DerugStatus::Initialized;
    derug_data.total_supply = total_supply;

    Ok(())
}
