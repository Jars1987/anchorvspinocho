pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
pub use instructions::*;

declare_id!("F8yLrrV5YTSboq7xUppAMA59f4jp4z9xkYpCRn6gfDUH");

#[program]
pub mod vault {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount, &ctx.bumps)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        ctx.accounts.withdraw()
    }
}
