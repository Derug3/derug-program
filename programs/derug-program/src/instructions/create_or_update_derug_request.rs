use crate::errors::DerugError;
use crate::state::derug_data::ActiveRequest;
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
pub struct CreateOrUpdateDerugRequest<'info> {
    #[account(init_if_needed, seeds=[DERUG_DATA_SEED, derug_data.key().as_ref(), payer.key().as_ref()], bump, payer=payer, space =  current_data_len(derug_request))]
    pub derug_request: Account<'info, DerugRequest>,
    #[account(mut, seeds =[DERUG_DATA_SEED, derug_data.collection.key().as_ref()], bump)]
    pub derug_data: Box<Account<'info, DerugData>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_or_update_derug_request(
    ctx: Context<CreateOrUpdateDerugRequest>,
    utility_dtos: Vec<UpdateUtilityDataDto>,
) -> Result<()> {
    let derug_request = &mut ctx.accounts.derug_request;
    let derug_data = &mut ctx.accounts.derug_data;

    derug_request.created_at = Clock::get().unwrap().unix_timestamp;
    require!(
        ctx.accounts.payer.key() != derug_data.rug_update_authority,
        DerugError::RuggerSigner
    );

    derug_request.derugger = ctx.accounts.payer.key();

    if derug_request.request_status == RequestStatus::Initialized {
        derug_request.utility_data = utility_dtos
            .iter()
            .map(|item| UtilityData {
                description: item.description.clone(),
                title: item.title.clone(),
            })
            .collect();
        derug_request.request_status = RequestStatus::Voting;

        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: derug_data.to_account_info(),
                },
            ),
            Rent::default().minimum_balance(36),
        )?;

        derug_data
            .to_account_info()
            .realloc(derug_data.to_account_info().data_len() + 36, false)?;

        derug_data.active_requests.push(ActiveRequest {
            request: derug_request.key(),
            vote_count: 0,
        });
    } else {
        ctx.accounts.derug_data.total_suggestion_count = ctx
            .accounts
            .derug_data
            .total_suggestion_count
            .checked_add(1)
            .unwrap();

        let new_data_len = calculate_new_suggestion_data_len(&utility_dtos, derug_request);
        if new_data_len > derug_request.to_account_info().data_len() {
            transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.payer.to_account_info(),
                        to: derug_request.to_account_info(),
                    },
                ),
                Rent::default().minimum_balance(
                    new_data_len
                        .checked_sub(derug_request.to_account_info().data_len())
                        .unwrap(),
                ),
            )?;
            derug_request
                .to_account_info()
                .realloc(new_data_len, false)?;
            let remaining_utilities: Vec<&UtilityData> = derug_request
                .utility_data
                .iter()
                .filter(|item| !utility_dtos.iter().any(|dto| dto.title == item.title))
                .collect_vec();

            derug_request.utility_data = remaining_utilities.into_iter().cloned().collect();

            utility_dtos.iter().for_each(|item| {
                if item.action == Action::Add {
                    derug_request.utility_data.push(UtilityData {
                        title: item.title.clone(),
                        description: item.description.clone(),
                    });
                }
            });
        }
    }
    derug_request.vote_count = 0;
    derug_request.derug_data = ctx.accounts.derug_data.key();

    Ok(())
}
