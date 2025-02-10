use anchor_lang::prelude::*;

declare_id!("AVHeRNSzSXBZDBF9csZucf2VT8YWns12QzaFPW1ZYDEW");

#[program]
pub mod turbin3vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
