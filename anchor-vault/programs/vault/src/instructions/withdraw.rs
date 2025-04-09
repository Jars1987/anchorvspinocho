use crate::state::VaultState;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(seeds = [b"state".as_ref(), user.key().as_ref()], bump = state.state_bump)]
    pub state: Account<'info, VaultState>,

    #[account(
            mut,
            seeds = [b"vault".as_ref(), state.key().as_ref()],
            bump = state.vault_bump,
        )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.state.to_account_info().key.as_ref(),
            &[self.state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        system_program::transfer(cpi_ctx, self.vault.lamports())?;
        Ok(())
    }
}
