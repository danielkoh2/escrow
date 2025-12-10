use anchor_lang::prelude::*;

declare_id!("3XnmqxGUYJChbpCwB7MSY1FGqnjUxFei1JoDx9daArea");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
