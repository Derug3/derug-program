use crate::state::{AccountLen, Action, UpdateUtilityDataDto, FIXED_LEN};
use anchor_lang::prelude::*;

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

pub fn current_data_len(account_info: &AccountInfo) -> usize {
    if account_info.data_is_empty() {
        FIXED_LEN
    } else {
        account_info.data_len().checked_sub(FIXED_LEN).unwrap()
    }
}
