use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWqk1zXg7Z4jzq7N4Qx6gsJx");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.authority = ctx.accounts.authority.key();
        counter.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<ModifyCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter.count.checked_add(1).ok_or(CounterError::Overflow)?;
        Ok(())
    }

    pub fn decrement(ctx: Context<ModifyCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter.count.checked_sub(1).ok_or(CounterError::Underflow)?;
        Ok(())
    }

    pub fn reset(ctx: Context<ModifyCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = CounterAccount::SPACE,
        seeds = [b"counter", authority.key().as_ref()],
        bump
    )]
    pub counter: Account<'info, CounterAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyCounter<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        seeds = [b"counter", authority.key().as_ref()],
        bump
    )]
    pub counter: Account<'info, CounterAccount>,
}

#[account]
pub struct CounterAccount {
    pub authority: Pubkey,
    pub count: u64,
}

impl CounterAccount {
    pub const SPACE: usize = 8 + 32 + 8;
}

#[error_code]
pub enum CounterError {
    #[msg("counter overflow")]
    Overflow,
    #[msg("counter underflow")]
    Underflow,
}
