// Accounts

use anchor_lang::prelude::*;

#[account]
pub struct SigningRequest {
    pub applicant: Pubkey, // 32
    pub authority: Pubkey, // 32
    pub uri: String, // 8 + 24 + 40 + 40
    pub signed: bool // 1
}


impl SigningRequest {
    pub const MAXIMUM_SIZE: usize = 32+32+(40+40+24+8)+1;
}