use anchor_lang::prelude::*;
use crate::collections::Counter; // import del módulo Counter

#[derive(Accounts)]
pub struct CreateCounter<'info> {
    #[account(mut)]
    authority: Signer<'info>, // wallet que crea el counter
    #[account(
        init,
        seeds = [
            Counter::COUNTER_SEED.as_bytes(),
            authority.key().as_ref()
            ], // semillas para la PDA
        bump, // bump canonico de la PDA
        payer = authority, // wallet que crea el counter
        space = 8 + Counter::INIT_SPACE, // space calculated 
    )]
    counter: Account<'info, Counter>, // counter PDA
    system_program: Program<'info, System>,
}

// acción que se ejecutará al llamar la instrucción create_counter 
pub fn create_counter(ctx: Context<CreateCounter>) -> Result<()> {
    // asignamos la autoridad del counter
    ctx.accounts.counter.authority = ctx.accounts.authority.key();
    // inicializamos count en 0
    ctx.accounts.counter.count = 0;
    // guardamos el bump de la PDA
    ctx.accounts.counter.bump = ctx.bumps.counter;
    Ok(())
}