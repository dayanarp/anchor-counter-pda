use anchor_lang::prelude::*;
use crate::instructions::*; // import del módulo de instrucciones

// Esta declaración buscará los archivos llamados `collections.rs` y
// `instructions.rs` e inserta su contenido dentro de su respectivo 
// módulo bajo este alcance (scope) 
mod collections;
mod instructions;

declare_id!("5k8NgBspoknB7x88XvPHdhWGQgVMdQPaSdbpQM7BNf5H");

#[program]
pub mod anchor_counter_pda {

    use super::*;
    // instrucción create_counter
    pub fn create_counter(ctx: Context<CreateCounter>) -> Result<()> {
        instructions::create::create_counter(ctx)
    }
    // instrucción increment_counter
    pub fn increment_counter(ctx: Context<IncrementCounter>) -> Result<()> {
        instructions::increment::increment_counter(ctx)
    }
    // instrucción delete_counter
    pub fn delete_counter(ctx: Context<DeleteCounter>) -> Result<()>{
        instructions::delete::delete_counter(ctx)
    }
}

