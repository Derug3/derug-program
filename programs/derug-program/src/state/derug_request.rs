use anchor_lang::prelude::*;

#[account]
pub struct DerugRequest {
    pub derug_data: Pubkey,
    pub new_name: String,
    pub new_symbol: String,
    pub derugger: Pubkey,
    pub created_at: i64,
    pub vote_count: u32,
    pub request_status: RequestStatus,
    pub creators: Vec<DeruggerCreator>,
    pub mint_config: MintConfig,
}

#[account]
pub struct RemintProof {
    pub derug_data: Pubkey,
    pub reminter: Pubkey,
    pub old_mint: Pubkey,
    pub new_mint: Pubkey,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub struct MintConfig {
    pub candy_machine_key: Pubkey,
    pub public_mint_price: u64,
    pub mint_currency: Pubkey,
    pub remint_duration: i64,
    pub seller_fee_bps: u16,
    pub whitelist_config: Option<WhitelistConfig>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub struct WhitelistConfig {
    pub price: u64,
    pub currency: Pubkey,
    pub duration: u32,
}
#[account]
pub struct RemintConfig {
    pub derug_request: Pubkey,
    pub new_name: String,
    pub new_symbol: String,
    pub authority: Pubkey,
    pub collection: Pubkey,
    pub public_mint_price: Option<u64>,
    pub mint_currency: Option<Pubkey>,
    pub mint_fee_treasury: Option<Pubkey>,
    pub private_mint_end: Option<i64>,
    pub creators: Vec<DeruggerCreator>,
    pub wallet_limit: Option<u8>,
    pub candy_machine_key: Pubkey,
    pub candy_machine_creator: Pubkey,
    pub seller_fee_bps: u32,
}

impl RemintConfig {
    pub const LEN: usize = 8 + 32 + 32 + 10 + 9 + 32 + 32 + 9 + 33 + 33 + 9 + 128 + 32 + 32 + 4;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct DeruggerCreator {
    pub address: Pubkey,
    pub share: u8,
}

impl Space for DeruggerCreator {
    const INIT_SPACE: usize = 33;
}

pub const FIXED_LEN: usize = 32 + 10 + 32 + 32 + 8 + 9 + 4 + 1 + 9 + 33 + 9 + 5 * 33 + 4 + 12;

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct UtilityData {
    pub title: String,
    pub description: String,
    pub is_active: bool,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, PartialEq)]
pub enum RequestStatus {
    Initialized,
    Voting,
    Succeeded,
    UploadingMetadata,
    Reminting,
    PublicMint,
    Completed,
}

pub trait AccountLen {
    fn length(&self) -> usize;
    fn current_data_len(&self) -> usize;
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

#[event]
pub struct NftRemintedEvent {
    pub reminter: Pubkey,
    pub new_nft_mint: Pubkey,
    pub new_nft_metadata: Pubkey,
    pub old_nft_mint: Pubkey,
    pub old_nft_metadata: Pubkey,
}

#[event]
pub struct PrivateMintStarted {
    pub derug_data: Pubkey,
}
