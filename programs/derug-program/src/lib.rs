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
        new_name: String,
        new_symbol: String,
        creators: Vec<DeruggerCreator>,
        mint_config: MintConfig,
    ) -> Result<()> {
        instructions::create_or_update_derug_request(
            ctx,
            new_name,
            new_symbol,
            mint_config,
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
    ) -> Result<()> {
        instructions::remint_nft(ctx)
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

    pub fn init_private_mint(ctx: Context<InitPrivateMint>, total_supply: u32) -> Result<()> {
        instructions::init_private_mint(ctx, total_supply)
    }

    pub fn bypass_voting(ctx: Context<BypassVoting>) -> Result<()> {
        instructions::bypass_voting(ctx)
    }

    pub fn init_public_mint(ctx: Context<InitPublicMint>) -> Result<()> {
        instructions::init_public_mint(ctx)
    }
}
