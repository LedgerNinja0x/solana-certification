mod state;
mod instruction_context;

use anchor_lang::prelude::*;
use instruction_context::*;

//declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
declare_id!("spxGCXzMEKBuYAsCd5wcAUD2mz8745cYZD9D8xXVgtg");

#[program]
pub mod certasset {
    use super::*;

    /// Used for testing purposes
    pub fn ping(_ctx: Context<Void>) -> Result<()>{
        msg!("PING! PING!");
        Ok(())
    }

    /// Allows the user to create a certification request
    pub fn create_request(ctx: Context<CreateSR>, authority: Pubkey, uri: String) -> Result<()> {
        msg!("CertAsset Program: Creating Signing Request");
        ctx.accounts.request.applicant = ctx.accounts.applicant.key();
        ctx.accounts.request.authority = authority;
        ctx.accounts.request.uri = uri;
        ctx.accounts.request.signed = false;
        msg!("CertAsset Program: Signing Request Created");

        msg!("Applicant: {}", ctx.accounts.request.applicant.key().to_string());
        msg!("Authority: {}", ctx.accounts.request.authority.key().to_string());
        msg!("URI: {}", ctx.accounts.request.uri);
        msg!("Signed: {}", ctx.accounts.request.signed);

        Ok(())
    }

    /// Allows the Signer Authority to Sign a Certification Request
    pub fn sign_certificate(ctx: Context<SignRequest>) -> Result<()> {
        ctx.accounts.request.signed = true;

        Ok(())
    }
}
