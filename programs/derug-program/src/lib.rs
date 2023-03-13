use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utilities;
use instructions::*;
use state::*;
declare_id!("drgnPyduLEuNqAcsSRfhYDndd9qUSLfHUZdwDxzAXF4");

#[program]
pub mod derug_program {
    use super::*;

    pub fn initialize_derug(ctx: Context<InitializeDerug>, total_supply: u32) -> Result<()> {
        instructions::initialize_derug(ctx, total_supply)
    }

    pub fn create_or_update_derug_request(
        ctx: Context<CreateOrUpdateDerugRequest>,
        utility_dtos: Vec<UpdateUtilityDataDto>,
    ) -> Result<()> {
        instructions::create_or_update_derug_request(ctx, utility_dtos)
    }

    pub fn vote<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, Vote<'info>>) -> Result<()> {
        instructions::vote(ctx)
    }

    pub fn cancel_derug_request(ctx: Context<CancelDerugRequest>) -> Result<()> {
        instructions::cancel_derug_request(ctx)
    }

    pub fn claim_victory(ctx: Context<ClaimVictory>) -> Result<()> {
        instructions::claim_victory(ctx)
    }

    pub fn initialize_reminting(ctx: Context<InitializeReminting>) -> Result<()> {
        instructions::initialize_reminting(ctx)
    }

    pub fn remint_nft<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, RemintNft<'info>>,
    ) -> Result<()> {
        instructions::remint_nft(ctx)
    }

    pub fn update_verify_collection(ctx: Context<UpdateVerifyCollection>) -> Result<()> {
        instructions::update_verify_collection(ctx)
    }
}
