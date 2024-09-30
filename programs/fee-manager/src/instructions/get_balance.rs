use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};

use crate::state::*;

#[derive(Accounts)]
pub struct GetBalance<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(seeds=[b"vault_signer", token_mint.key().as_ref()], bump = vault_signer.bump)]
    vault_signer: Account<'info, VaultSigner>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = vault_signer,
    )]
    pub token_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mint::token_program = token_program,
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    token_program: Program<'info, Token2022>
}