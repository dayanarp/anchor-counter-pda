use anchor_lang::prelude::*;
use crate::collections::Counter; // import del módulo Counter

#[derive(Accounts)]
pub struct DeleteCounter<'info> {
    #[account(
        mut,
        seeds = [
            Counter::COUNTER_SEED.as_bytes(),
            authority.key().as_ref()
            ], // semillas para la PDA
        bump = counter.bump, // bump de la PDA
        constraint = counter.authority == authority.key(), // verificamos la autoridad
        close = authority // al cerrar la cuenta authority recibe la renta
    )]
    pub counter: Account<'info, Counter>, // counter PDA
    #[account(mut)]
    pub authority: Signer<'info>, // autoridad del counter
}

// acción que se ejecutará al llamar la instrucción delete_counter 
pub fn delete_counter(_ctx: Context<DeleteCounter>) -> Result<()>{
    Ok(())
}