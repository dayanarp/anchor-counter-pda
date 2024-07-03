// Este archivo expone el contenido de la carpeta "collections"
// como un módulo que contiene lo que se declara aquí.

// Esta declaración buscará un archivo llamado `counter.rs` y
// inserta su contenido dentro de un módulo llamado `counter` bajo este alcance
// volviendolo submodulos de este modulo
pub mod counter;
pub use counter::*;