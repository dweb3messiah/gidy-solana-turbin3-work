use anchor_lang::prelude::*;
declare_id!("7BPVg1FiHHL4gziRwPaiE2GJt8XSgF9XPBAu93uC2T5n");

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;

        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.withdraw()?;
        ctx.accounts.close()?;

        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw()?;
        ctx.accounts.close()?;

        Ok(())
    }
}

