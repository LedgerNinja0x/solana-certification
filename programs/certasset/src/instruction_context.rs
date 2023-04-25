use anchor_lang::prelude::*;
use anchor_spl::token_interface::Token2022;

use crate::state::SigningRequest;

// Instructions

#[derive(Accounts)]
/// Void Context for Testing Transactions
pub struct Void {}

#[derive(Accounts)]
pub struct CreateSR<'info> {
    #[account(init, payer=applicant, space=8+SigningRequest::MAXIMUM_SIZE)]
    pub request: Account<'info, SigningRequest>,
    #[account(mut)]
    pub applicant: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct SignRequest<'info> {
    #[account(mut, has_one = authority)]
    pub request: Account<'info, SigningRequest>,
    pub authority: Signer<'info>,
    pub token_program_2022: Program<'info, Token2022>
}
