use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utilities;
use instructions::*;
use state::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod derug_program {
    use super::*;

    pub fn initialize_derug(ctx: Context<InitializeDerug>, total_supply: u32) -> Result<()> {
        instructions::initialize_derug(ctx, total_supply)
    }
    pub fn create_or_update_suggestion(
        ctx: Context<CreateOrUpdateDerugRequest>,
        utility_dtos: Vec<UpdateUtilityDataDto>,
    ) -> Result<()> {
        instructions::create_or_update_derug_request(ctx, utility_dtos)
    }
}
