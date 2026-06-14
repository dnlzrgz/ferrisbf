mod error;
mod interpreter;
mod parser;

pub use error::RuntimeError;
pub use interpreter::Machine;
pub use parser::Instruction;
