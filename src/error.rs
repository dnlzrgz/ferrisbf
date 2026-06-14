use std::error;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum RuntimeError {
    PointerOutOfBounds,
    IoError,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::PointerOutOfBounds => {
                write!(f, "pointer moved out of bound of the tape")
            }
            RuntimeError::IoError => {
                write!(f, "failed to read or write data")
            }
        }
    }
}

impl error::Error for RuntimeError {}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnmatchedOpenBracket,
    UnmatchedCloseBracket,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnmatchedOpenBracket => write!(f, "unmatched '[' in source"),
            ParseError::UnmatchedCloseBracket => write!(f, "unmatched ']' in source"),
        }
    }
}

impl error::Error for ParseError {}
