use std::cmp::Ordering;

use anchor_lang::prelude::*;
use mpl_token_metadata::state::{MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URI_LENGTH};
#[account]
#[derive(Debug)]
pub struct DerugData {
    pub slug: String,
    pub collection: Pubkey,
    pub rug_update_authority: Pubkey,
    pub collection_metadata: Option<Pubkey>,
    pub total_supply: u32,
    pub new_collection: Option<Pubkey>,
    pub threshold_denominator:u8,
    pub date_added: i64,
    pub derug_status: DerugStatus,
    pub period_end: i64,
    pub total_reminted: u32,
    pub winning_request: Option<Pubkey>,
    pub total_suggestion_count: u8,
    pub collection_name: String,
    pub collection_symbol: String,
    pub collection_uri: String,
    pub active_requests: Vec<ActiveRequest>
}

impl DerugData {
    pub const LEN: usize = 
    64       //Max slug length   
    +2 * 32  //First two pubkeys
    +33  //Optional collection metadata account
    + 4 //total amount of nfts in rugged collection 
    + 33 //new_collection 
    + 1 //theshold denominator
    + 8 //timestamp of derug account creation 
    + 1 //status 
    + 8 //timestamp of voting started  
    + 4 //total reminted
    + 33 //winning request
    + 1 //suggestion count
    + MAX_NAME_LENGTH 
    + MAX_SYMBOL_LENGTH 
    + MAX_URI_LENGTH
    + 4; // vec<activerequest>
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, PartialEq,Debug)]
pub enum DerugStatus {
    Initialized,
    Voting,
    Succeeded,
    UploadingMetadata,
    Reminting,
    PublicMint,
    Completed,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, PartialEq, PartialOrd,Debug)]

pub struct ActiveRequest {
    pub request: Pubkey,
    pub vote_count: i32,
    pub winning: bool
}

impl Eq for ActiveRequest {
    fn assert_receiver_is_total_eq(&self) {
        todo!()
    }
}

impl Ord for ActiveRequest {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
       if self.vote_count >= other.vote_count {
            Ordering::Greater
       } else  {
            Ordering::Less
       } 

    }
}

impl ActiveRequest{
    pub fn get_winning(derug_data:&DerugData)->&Self{

        let  mut winning_request=derug_data.active_requests.get(0).unwrap();

        for request in derug_data.active_requests.iter(){
            if request.vote_count>winning_request.vote_count{
               winning_request=request;
            }
        }
        winning_request
    }
}