use anchor_lang::prelude::*;

use anchor_lang::accounts::interface_account::InterfaceAccount;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use anchor_spl::associated_token::AssociatedToken; 
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(init, payer = creator, seeds=[b"vault_signer", token_mint.key().as_ref()], bump, space= 8 + VaultSigner::INIT_SPACE)]
    pub vault_signer: Account<'info, VaultSigner>,
    #[account(
        init,
        payer = creator,
        associated_token::mint = token_mint,
        associated_token::authority = vault_signer,
    )]
    pub token_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mint::token_program = token_program,
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,
    token_program: Program<'info, Token2022>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}