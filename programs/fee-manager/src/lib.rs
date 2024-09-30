use anchor_lang::prelude::*;

pub mod state;
pub mod error;
pub mod instructions;

use instructions::*;

declare_id!("HYYEkC1SitwZMBAGFgGf3e9NLA2bWNbzHbfhmnjDKfDR");

#[program]
pub mod fee_manager {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault_signer = &mut  ctx.accounts.vault_signer;
        vault_signer.bump = ctx.bumps.vault_signer;
        vault_signer.token_mint = ctx.accounts.token_mint.key();
        Ok(())
    }

    pub fn harvest<'info>(ctx: Context<'_, '_, 'info, 'info, Harvest<'info>>) -> Result<()> {
        process_harvest(ctx)
    }

    pub fn get_balance(ctx: Context<GetBalance>) -> Result<()> {
        let token_vault = &mut ctx.accounts.token_vault.to_account_info();
        msg!("{:?}", token_vault);
        Ok(())
    }
}










