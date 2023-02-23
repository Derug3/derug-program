use anchor_lang::prelude::*;

#[account]
pub struct DerugRequst {
    pub derugger: Pubkey,
    pub created_at: i64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct UtilityData {
    pub title: String,
    pub description: String,
}
