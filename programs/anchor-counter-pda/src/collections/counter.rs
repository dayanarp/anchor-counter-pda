use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // automatically calculate the space required for the struct
pub struct Counter {
    pub count: u64, // almacena el valor actual del counter
    pub authority: Pubkey, // autoridad sobre el counter
    pub bump: u8, // almacena el bump de la PDA
}

// implementa funciones o estructuras asociadas al tipo "Counter"
impl Counter {
    // semilla para la PDA
    pub const COUNTER_SEED: &'static str = "counter";
}
