// Este archivo expone el contenido de la carpeta "instructions"
// como un módulo que contiene lo que se declara aquí.

// Esta declaración buscará un archivo llamado `create.rs`, 
// `increment.rs` y `delete.rs`, e inserta su contenido dentro 
// los módulos respectivos bajo este alcance, volviendolos 
// submodulos de este modulo
pub mod create;
pub mod increment;
pub mod delete;

pub use create::*;
pub use increment::*;
pub use delete::*;