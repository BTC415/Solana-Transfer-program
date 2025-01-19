#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, TransferChecked};
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use std::str::FromStr;

declare_id!("6GmLK8Qz9hBnCa89jcaVoX4b8nKCYc62p4hUbZ5XrsBL");
pub const MINT_ADDRESS: &str = "pcjPpFP8tJVW7C3YFiUgxgAz2DqJArTQmyooZNkpcVY";

#[program]
pub mod spl_token_transfer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initialized: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn transfer_token(ctx: Context<TransferToken>, amount: u64) -> Result<()> {
        let transfer_cpi_accounts = TransferChecked {
            from: ctx.accounts.from_ata.clone().to_account_info(),
            to: ctx.accounts.to_ata.clone().to_account_info(),
            authority: ctx.accounts.from.clone().to_account_info(),
            mint: ctx.accounts.mint.clone().to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.clone().to_account_info(),
            transfer_cpi_accounts,
        );
        token_2022::transfer_checked(cpi_ctx, amount, ctx.accounts.mint.decimals)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub from_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub to_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(address = Pubkey::from_str(MINT_ADDRESS).unwrap())]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Program<'info, Token2022>,
}
