use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utilities;
use instructions::*;
use state::*;
declare_id!("DERUGwXJu3m1DG1VNq4gP7Ppkza95P7XbeujbtSNAebu");

#[program]
pub mod derug_program {
    use super::*;

    pub fn initialize_derug(
        ctx: Context<InitializeDerug>,
        total_supply: u32,
        slug: String,
    ) -> Result<()> {
        instructions::initialize_derug(ctx, total_supply, slug)
    }

    pub fn create_or_update_derug_request(
        ctx: Context<CreateOrUpdateDerugRequest>,
        utility_dtos: Vec<UpdateUtilityDataDto>,
        seller_fee_bps: u32,
        public_mint_price: Option<u64>,
        private_mint_duration: Option<i64>,
        new_name: String,
        new_symbol: String,
        wallet_limit: Option<u8>,
        creators: Vec<DeruggerCreator>,
    ) -> Result<()> {
        instructions::create_or_update_derug_request(
            ctx,
            utility_dtos,
            new_name,
            new_symbol,
            seller_fee_bps,
            public_mint_price,
            private_mint_duration,
            wallet_limit,
            creators,
        )
    }

    pub fn vote<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, Vote<'info>>) -> Result<()> {
        instructions::vote(ctx)
    }

    pub fn cancel_derug_request(ctx: Context<CancelDerugRequest>) -> Result<()> {
        instructions::cancel_derug_request(ctx)
    }

    pub fn claim_victory<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ClaimVictory<'info>>,
    ) -> Result<()> {
        instructions::claim_victory(ctx)
    }

    pub fn initialize_reminting(ctx: Context<InitializeReminting>) -> Result<()> {
        instructions::initialize_reminting(ctx)
    }

    pub fn remint_nft<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, RemintNft<'info>>,
        new_name: String,
        new_uri: String,
    ) -> Result<()> {
        instructions::remint_nft(ctx, new_name, new_uri)
    }

    pub fn update_verify_collection(ctx: Context<UpdateVerifyCollection>) -> Result<()> {
        instructions::update_verify_collection(ctx)
    }

    pub fn close_program_account<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CloseProgramAccount<'info>>,
    ) -> Result<()> {
        instructions::close_program_account(ctx)
    }
    pub fn close_single_request(ctx: Context<CloseSingleRequest>) -> Result<()> {
        instructions::close_single_request(ctx)
    }

    pub fn close_remint_config(ctx: Context<CloseRemintConfig>) -> Result<()> {
        instructions::close_remint_config(ctx)
    }

    pub fn freeze_nft(ctx: Context<FreezeNft>) -> Result<()> {
        instructions::freeze_nft(ctx)
    }

    pub fn init_private_mint(ctx: Context<InitPrivateMint>) -> Result<()> {
        instructions::init_private_mint(ctx)
    }

    pub fn bypass_voting(ctx: Context<BypassVoting>) -> Result<()> {
        instructions::bypass_voting(ctx)
    }
}
