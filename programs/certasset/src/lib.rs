use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod certasset {
    use super::*;

    /// Allows the user to create a certification request
    pub fn create_request(ctx: Context<CreateSR>, authority: Pubkey, uri: String) -> Result<()> {
        Ok(())
    }

    /// Allows the Signer Authority to Sign a Certification Request
    pub fn sign_certificate(ctx: Context<SignRequest>) -> Result<()> {
        Ok(())
    }
}

// Instructions

#[derive(Accounts)]
pub struct CreateSR<'info> {
    #[account(init, payer=applicant, space=8+SigningRequest::MAXIMUM_SIZE)]
    request: Account<'info, SigningRequest>,
    #[account(mut)]
    applicant: Signer<'info>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct SignRequest<'info> {
    #[account(mut)]
    request: Account<'info, SigningRequest>,
    signer: Signer<'info>
}


// Accounts

#[account]
pub struct SigningRequest {
    applicant: Pubkey, // 32
    authority: Pubkey, // 32
    uri: String, // 8 + 24 + 40 + 40
    signed: bool // 1
}

impl SigningRequest {
    const MAXIMUM_SIZE: usize = 32+32+(40+40+24+8)+1;
}
