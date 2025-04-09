use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};

use crate::state::VaultState;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + VaultState::INIT_SPACE,
        seeds = [b"state".as_ref(), user.key().as_ref()],
        bump
    )]
    pub state: Account<'info, VaultState>,

    #[account(seeds = [b"vault".as_ref(), state.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64, bumps: &DepositBumps) -> Result<()> {
        self.state.vault_bump = bumps.vault;
        self.state.state_bump = bumps.state;

        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        system_program::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}
