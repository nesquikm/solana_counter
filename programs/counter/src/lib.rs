use anchor_lang::prelude::*;

// Specify the program address
declare_id!("8C9Aq2hhTvu79DKHv5GQ1FJiY2KwKgN7Ej9r2ZEuQkYx");

// Instructions defined in program module
#[program]
pub mod counter {
    use super::*;

    // Instruction to initialize a new counter account
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Reference to the counter account from the Initialize struct
        let counter = &mut ctx.accounts.counter;
        counter.bump = ctx.bumps.counter; // store bump seed in `Counter` account
        msg!("Counter account created! Current count: {}", counter.count);
        msg!("Counter bump: {}", counter.bump);
        Ok(())
    }

    // Instruction to increment a counter account
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        // Mutable reference to the counter account from the Increment struct
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);

        // Increment the count value stored on the counter account by 1
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented! Current count: {}", counter.count);
        Ok(())
    }
}

// Accounts required by the initialize instruction
#[derive(Accounts)]
pub struct Initialize<'info> {
    // The account paying to create the counter account
    #[account(mut)]
    pub user: Signer<'info>, // specify account must be signer on the transaction

    // The counter account being created and initialized in the instruction
    #[account(
        init,         // specifies we are creating this account
        seeds = [b"counter"], // optional seeds for pda
        bump,
        payer = user, // specifies account paying for the creation of the account
        space = 8 + Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>, // specify account is 'Counter' type
    pub system_program: Program<'info, System>, // specify account must be System Program
}

// Account required by the increment instruction
#[derive(Accounts)]
pub struct Increment<'info> {
    // The address of the `Counter` account must be a PDA derived with the specified `seeds`
    #[account(
        mut,
        seeds = [b"counter"], // optional seeds for pda
        bump = counter.bump,  // bump seed for pda stored in `Counter` account
    )]
    pub counter: Account<'info, Counter>, // specify account is 'Counter' type
}

// Define structure of `Counter` account
#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64, // define count value type as u64
    pub bump: u8,   // 1 byte
}
