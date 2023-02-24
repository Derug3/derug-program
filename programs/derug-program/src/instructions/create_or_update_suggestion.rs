use crate::errors::DerugError;
use crate::state::{Action, RequestStatus, UtilityData};
use crate::utilities::calculate_new_suggestion_data_len;
use crate::{
    constants::DERUG_DATA_SEED,
    state::{DerugData, DerugRequest, UpdateUtilityDataDto},
    utilities::current_data_len,
};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use itertools::Itertools;

#[derive(Accounts)]
pub struct CreateOrUpdateSuggestion<'info> {
    #[account(init_if_needed, seeds=[DERUG_DATA_SEED, derug_data.key().as_ref(), payer.key().as_ref()], bump, payer=payer, space =  current_data_len(suggestion_data))]
    pub suggestion_data: Account<'info, DerugRequest>,
    #[account(mut, seeds =[DERUG_DATA_SEED, derug_data.collection.key().as_ref()], bump)]
    pub derug_data: Box<Account<'info, DerugData>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_or_update_suggestion(
    ctx: Context<CreateOrUpdateSuggestion>,
    utility_dtos: Vec<UpdateUtilityDataDto>,
) -> Result<()> {
    let suggestion_data = &mut ctx.accounts.suggestion_data;

    suggestion_data.created_at = Clock::get().unwrap().unix_timestamp;
    require!(
        ctx.accounts.payer.key() != ctx.accounts.derug_data.rug_update_authority,
        DerugError::RuggerSigner
    );

    suggestion_data.derugger = ctx.accounts.payer.key();

    if suggestion_data.request_status == RequestStatus::Initialized {
        suggestion_data.utility_data = utility_dtos
            .iter()
            .map(|item| UtilityData {
                description: item.description.clone(),
                title: item.title.clone(),
            })
            .collect();
        suggestion_data.request_status = RequestStatus::Voting;
    } else {
        let new_data_len = calculate_new_suggestion_data_len(&utility_dtos, suggestion_data);
        if new_data_len > suggestion_data.to_account_info().data_len() {
            transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.payer.to_account_info(),
                        to: suggestion_data.to_account_info(),
                    },
                ),
                Rent::default().minimum_balance(
                    new_data_len
                        .checked_sub(suggestion_data.to_account_info().data_len())
                        .unwrap(),
                ),
            )?;
            suggestion_data
                .to_account_info()
                .realloc(new_data_len, false)?;
            let remaining_utilities: Vec<&UtilityData> = suggestion_data
                .utility_data
                .iter()
                .filter(|item| !utility_dtos.iter().any(|dto| dto.title == item.title))
                .collect_vec();
            //TODO: Check order
            suggestion_data.utility_data = remaining_utilities.into_iter().cloned().collect();

            utility_dtos.iter().for_each(|item| {
                if item.action == Action::Add {
                    suggestion_data.utility_data.push(UtilityData {
                        title: item.title.clone(),
                        description: item.description.clone(),
                    });
                }
            });
        }
    }
    suggestion_data.vote_count = 0;

    Ok(())
}
