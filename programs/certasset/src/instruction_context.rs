use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{
        Token2022
    }
};

use crate::state::SigningRequest;
use crate::state::Mint2022;

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

    #[account(
        init, 
        payer=authority, 
        space=Mint2022::LEN+8, 
        owner=token_program_2022.key(),

        seeds=[b"certasset-rq", request.key().as_ref()], bump
    )]
    pub mint: Account<'info, Mint2022>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program_2022: Program<'info, Token2022>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>
}
