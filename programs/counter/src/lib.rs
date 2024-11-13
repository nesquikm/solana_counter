use anchor_lang::prelude::*;

declare_id!("jqyQHsgRPPLrfjjeuuE7wHav1gJo3YyjyfaPNR3q6cV");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
