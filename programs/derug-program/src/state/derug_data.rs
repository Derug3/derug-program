use anchor_lang::prelude::*;
use mpl_token_metadata::state::{MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URI_LENGTH};
#[account]
pub struct DerugData {
    pub collection: Pubkey,
    pub rug_update_authority: Pubkey,
    pub collection_metadata: Pubkey,
    pub total_supply: u32,
    pub candy_machine: Option<Pubkey>,
    pub date_added: i64,
    pub derug_status: DerugStatus,
    pub total_suggestion_count: u8,
    pub collection_name: String,
    pub collection_symbol: String,
    pub collection_uri: String,
}

impl DerugData {
    pub const LEN: usize = 3 * 32  //First three pubkeys
    + 4 //total amount of nfts in rugged collection 
    + 33 //candy machine key(if collection is minted via candy machine) 
    + 8 //timestamp of derug account creation 
    + 1 //status 
    + MAX_NAME_LENGTH 
    + MAX_SYMBOL_LENGTH 
    + MAX_URI_LENGTH; 

}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub enum DerugStatus {
    Initialized,
    Voting,
    Reminting,
    Completed,
}
