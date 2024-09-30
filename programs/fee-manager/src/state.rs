use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VaultSigner {
    pub token_mint: Pubkey,
    pub bump: u8,
}