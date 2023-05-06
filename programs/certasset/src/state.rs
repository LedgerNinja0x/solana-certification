// Accounts

use anchor_lang::{prelude::*, solana_program::{program_pack::Pack}};

#[account]
pub struct SigningRequest {
    pub applicant: Pubkey, // 32
    pub authority: Pubkey, // 32
    pub uri: String, // 8 + 24 + 40 + 40
    pub signed: bool, // 1
    pub bump: u8 // 1
}

/// Mint Account
#[derive(Clone)]
pub struct Mint2022 (spl_token_2022::state::Mint);

impl Mint2022 {
    pub const LEN: usize = 82;
}

impl AccountDeserialize for Mint2022 {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
        msg!("Deserializing: {:?}", buf);
        let mint_native = spl_token_2022::state::Mint::unpack(buf);

        match mint_native {
            Ok(_) => msg!("Unpack Successfull"),
            Err(_) => msg!("Unpack failed")
        };

        msg!("Mapping");

        mint_native.map(Mint2022)
            .map_err(Into::into)    
    }
}

impl AccountSerialize for Mint2022 {
    fn try_serialize<W: std::io::Write>(&self, _writer: &mut W) -> Result<()> {
        msg!("Serializing");
        Ok(())
    }
}

impl Owner for Mint2022 {
    fn owner() -> Pubkey {
        spl_token_2022::ID
    }
}

impl Id for Mint2022 {
    fn id() -> Pubkey {
        spl_token_2022::ID
    }
}


impl SigningRequest {
    pub const MAXIMUM_SIZE: usize = 32+32+(40+40+24+8)+1+1;
}