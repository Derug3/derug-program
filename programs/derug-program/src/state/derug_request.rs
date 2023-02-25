use anchor_lang::prelude::*;
#[account]
pub struct DerugRequest {
    pub derug_data: Pubkey,
    pub derugger: Pubkey,
    pub created_at: i64,
    pub vote_count: u32,
    pub request_status: RequestStatus,
    pub utility_data: Vec<UtilityData>,
}

pub const FIXED_LEN: usize = 32 + 8 + 4 + 1;

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct UtilityData {
    pub title: String,
    pub description: String,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, PartialEq)]
pub enum RequestStatus {
    Initialized,
    Voting,
    Succeeded,
    Finished,
}

pub trait AccountLen {
    fn length(&self) -> usize;
    fn current_data_len(&self) -> usize;
}

impl AccountLen for Account<'_, DerugRequest> {
    fn length(&self) -> usize {
        self.try_to_vec().unwrap().len()
    }

    fn current_data_len(&self) -> usize {
        self.utility_data.try_to_vec().unwrap().len() + FIXED_LEN
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct UpdateUtilityDataDto {
    pub title: String,
    pub description: String,
    pub action: Action,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, PartialEq)]
pub enum Action {
    Add,
    Remove,
}
