use std::error;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum RuntimeError {
    PointerOutOfBounds,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::PointerOutOfBounds => {
                write!(f, "pointer moved out of bound of the tape")
            }
        }
    }
}

impl error::Error for RuntimeError {}
