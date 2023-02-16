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
    applicant: Signer<'info>,
}

#[derive(Accounts)]
pub struct SignRequest<'info> {
    request: Account<'info, SigningRequest>,
    signer: Signer<'info>
}


// Accounts

#[account]
pub struct SigningRequest {
    applicant: Pubkey,
    authority: Pubkey,
    uri: String,
    signed: bool
}
