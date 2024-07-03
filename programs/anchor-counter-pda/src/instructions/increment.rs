use anchor_lang::prelude::*;
use crate::collections::Counter; // import del módulo Counter


#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    authority: Signer<'info>, // autoridad del counter
    #[account(
        mut, 
        seeds = [
            Counter::COUNTER_SEED.as_ref(), 
            authority.key().as_ref()
            ], // semillas de la PDA
        bump = counter.bump, // bump de la PDA
        constraint = counter.authority == authority.key() // verificamos la autoridad
    )]
    counter: Account<'info, Counter>, // counter PDA
}

// acción que se ejecutará al llamar la instrucción increment_counter 
pub fn increment_counter(ctx: Context<IncrementCounter>) -> Result<()>{
    ctx.accounts.counter.count += 1; // incrementamos count en 1
    Ok(())
}