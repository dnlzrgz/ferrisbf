mod error;
mod interpreter;
mod parser;

pub use error::RuntimeError;
pub use interpreter::{Machine, run};
pub use parser::{Instruction, parse};
