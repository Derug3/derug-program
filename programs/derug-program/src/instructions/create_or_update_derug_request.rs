use crate::errors::DerugError;
use crate::state::derug_data::ActiveRequest;
use crate::state::derug_request::DeruggerCreator;
use crate::state::{Action, RequestStatus, UtilityData};
use crate::utilities::calculate_new_suggestion_data_len;
use crate::{
    constants::DERUG_DATA_SEED,
    state::{DerugData, DerugRequest, UpdateUtilityDataDto},
    utilities::current_data_len,
};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token::Mint;
use itertools::Itertools;

#[derive(Accounts)]
#[instruction(utility_dtos:Vec<UpdateUtilityDataDto>)]
pub struct CreateOrUpdateDerugRequest<'info> {
    #[account(init_if_needed, seeds=[DERUG_DATA_SEED, derug_data.key().as_ref(), payer.key().as_ref()], bump, payer=payer, space =  current_data_len(derug_request,utility_dtos))]
    pub derug_request: Account<'info, DerugRequest>,
    #[account(mut, seeds =[DERUG_DATA_SEED, derug_data.collection.key().as_ref()], bump)]
    pub derug_data: Box<Account<'info, DerugData>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    ///CHECK
    #[account(mut, address="DRG3YRmurqpWQ1jEjK8DiWMuqPX9yL32LXLbuRdoiQwt".parse::<Pubkey>().unwrap())]
    pub fee_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_or_update_derug_request(
    ctx: Context<CreateOrUpdateDerugRequest>,
    utility_dtos: Vec<UpdateUtilityDataDto>,
    new_name: String,
    new_symbol: String,
    seller_fee_bps: u32,
    public_mint_price: Option<u64>,
    private_mint_duration: Option<i64>,
    wallet_limit: Option<u8>,
    creators: Vec<DeruggerCreator>,
) -> Result<()> {
    let derug_request = &mut ctx.accounts.derug_request;
    let derug_data = &mut ctx.accounts.derug_data;

    derug_request.created_at = Clock::get().unwrap().unix_timestamp;
    require!(
        ctx.accounts.payer.key() != derug_data.rug_update_authority,
        DerugError::RuggerSigner
    );

    derug_request.new_name = new_name;
    derug_request.new_symbol = new_symbol;

    derug_request.derugger = ctx.accounts.payer.key();

    require!(
        derug_data.period_end > derug_request.created_at,
        DerugError::TimeIsOut
    );

    if derug_request.request_status == RequestStatus::Initialized {
        derug_data.total_suggestion_count =
            derug_data.total_suggestion_count.checked_add(1).unwrap();

        derug_request.utility_data = utility_dtos
            .iter()
            .map(|item| UtilityData {
                description: item.description.clone(),
                title: item.title.clone(),
                is_active: item.action == Action::Add,
            })
            .collect();
        derug_request.request_status = RequestStatus::Voting;

        derug_request.wallet_limit = wallet_limit;
        let new_len = derug_data
            .to_account_info()
            .data_len()
            .checked_add(37)
            .unwrap();

        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: derug_data.to_account_info(),
                },
            ),
            Rent::default().minimum_balance(new_len),
        )?;

        derug_data.to_account_info().realloc(new_len, false)?;

        derug_data.active_requests.push(ActiveRequest {
            request: derug_request.key(),
            vote_count: 0,
            winning: false,
        });
    } else {
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
                        is_active: item.action == Action::Add,
                    });
                }
            });
        }
    }
    let remaining_accounts = &mut ctx.remaining_accounts.iter();
    if let Some(mint_currency) = remaining_accounts.next() {
        let _mint = Account::<Mint>::try_from(mint_currency).unwrap();

        derug_request.mint_currency = Some(mint_currency.key());
    }

    require!(creators.len() <= 5, DerugError::TooManyCreators);

    derug_request.vote_count = 0;
    derug_request.derug_data = ctx.accounts.derug_data.key();
    derug_request.private_mint_duration = private_mint_duration;
    derug_request.mint_price = public_mint_price;
    derug_request.seller_fee_bps = seller_fee_bps;
    derug_request.creators = creators;

    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.fee_wallet.to_account_info(),
            },
        ),
        9000000,
    )?;

    Ok(())
}
