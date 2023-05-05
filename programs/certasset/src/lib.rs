mod state;
mod instruction_context;

use anchor_lang::prelude::*;
use instruction_context::*;

//declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
declare_id!("spxGCXzMEKBuYAsCd5wcAUD2mz8745cYZD9D8xXVgtg");

#[program]
pub mod certasset {
    use anchor_spl::{token_2022::{self, InitializeMint2}};
    use state::Mint2022;

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
        msg!("CertAsset Program: Signing Request ...");
        ctx.accounts.request.signed = true;
        ctx.accounts.request.bump = ctx.bumps.get("mint").unwrap().clone();
        msg!("Generating NFT with PDA Bump: {}", ctx.accounts.request.bump);

        let token_2022 = ctx.accounts.token_program_2022.to_account_info();
        let init_instr = InitializeMint2 {
            mint: ctx.accounts.mint.to_account_info()
        };
        msg!("Mint: {}", ctx.accounts.mint.key().to_string());
        msg!("Mint Account Owner: {}", ctx.accounts.mint.to_account_info().owner.to_string());
        msg!("Token 2022 Program: {}", ctx.accounts.token_program_2022.key().to_string());

        let cpi_ctx = CpiContext::new(token_2022, init_instr);

        token_2022::initialize_mint2(cpi_ctx, 0, ctx.accounts.authority.key, Some(ctx.accounts.authority.key)).unwrap();

        // Checks if the created account is valid
        let _mint_check: Account<Mint2022> = Account::try_from(&ctx.accounts.mint).expect("Failed checks on the initialized mint");

        msg!("Request Signed Successfully!");

        Ok(())
    }
}
