use std::{cmp::Ordering, vec};

use crate::state::{AccountLen, Action, UpdateUtilityDataDto, FIXED_LEN};
use anchor_lang::prelude::*;

use mpl_token_metadata::{
    instruction::{create_master_edition_v3, create_metadata_accounts_v3},
    pda::{find_edition_account, find_metadata_account},
    state::{Collection, CollectionDetails, Creator},
};
use solana_program::instruction::Instruction;

use mpl_token_metadata::ID as METADATA_PROGRAM_ID;

pub fn calculate_new_suggestion_data_len<T>(
    utility_data: &Vec<UpdateUtilityDataDto>,
    account: &T,
) -> usize
where
    T: AccountLen,
{
    let mut remove_data_len: usize = 0;
    let mut add_data_len: usize = 0;

    utility_data.iter().for_each(|item| {
        if item.action == Action::Add {
            add_data_len = add_data_len
                .checked_add(item.try_to_vec().unwrap().len())
                .unwrap();
        } else {
            remove_data_len = remove_data_len
                .checked_add(item.try_to_vec().unwrap().len())
                .unwrap();
        }
    });

    let current_len = T::length(account);

    current_len + add_data_len - remove_data_len
}

pub fn current_data_len(
    account_info: &AccountInfo,
    derug_request_dtos: Vec<UpdateUtilityDataDto>,
) -> usize {
    let new_dtos_size = derug_request_dtos.try_to_vec().unwrap().len();
    if account_info.data_is_empty() {
        FIXED_LEN.checked_add(new_dtos_size).unwrap()
    } else {
        account_info.data_len().checked_sub(FIXED_LEN).unwrap()
    }
}

pub fn create_metadata_ix(
    nft_mint: &Pubkey,
    update_authority: &Pubkey,
    mint_authority: &Pubkey,
    collection: Option<Pubkey>,
    payer: &Pubkey,
    uri: &String,
    name: &String,
    symbol: &String,
) -> Instruction {
    let metadata_account = find_metadata_account(nft_mint);

    let mut collection_data: Option<Collection> = None;
    let mut collection_details: Option<CollectionDetails> = None;

    if let Some(collection_key) = collection {
        collection_data = Some(Collection {
            verified: false,
            key: collection_key.key(),
        });

        collection_details = Some(CollectionDetails::V1 { size: 0 });
    }

    msg!("Update authority: {:?}", update_authority);
    msg!("Payer: {:?}", payer);

    create_metadata_accounts_v3(
        METADATA_PROGRAM_ID,
        metadata_account.0,
        nft_mint.clone(),
        mint_authority.clone(),
        mint_authority.clone(),
        update_authority.clone(),
        name.clone(),
        symbol.clone(),
        uri.clone(),
        Some(vec![Creator {
            address: update_authority.clone(),
            share: 100,
            verified: false,
        }]),
        //TODO: Change this
        500,
        update_authority == payer,
        true,
        collection_data,
        None,
        collection_details,
    )
}

pub fn create_master_edition_ix(
    nft_mint: &Pubkey,
    update_authority: &Pubkey,
    mint_authority: &Pubkey,
    payer: &Pubkey,
) -> Instruction {
    let edition = find_edition_account(nft_mint, "0".to_string());
    let metadata = find_metadata_account(nft_mint);

    create_master_edition_v3(
        METADATA_PROGRAM_ID,
        edition.0,
        nft_mint.clone(),
        update_authority.clone(),
        mint_authority.clone(),
        metadata.0.clone(),
        payer.clone(),
        Some(0),
    )
}
