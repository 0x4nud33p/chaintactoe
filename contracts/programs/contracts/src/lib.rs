use anchor_lang::prelude::*;

declare_id!("EVn9PXaLKVBE1SXcZjbcKd9bQiuN1LvGm1RdnT3zqb1S");

#[program]
pub mod contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
