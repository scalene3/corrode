use std::collections::TryReserveError;

use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum StackError {
    #[error("Cannot pop empty stack")]
    EmptyStack { idx: usize, op: u8 },
    #[error("Not enough capacity on stack")]
    ReserveError { source: TryReserveError },
    #[error("Unknown operation: {idx} at index: {byte}")]
    UnknownOp { idx: usize, byte: u8 },
}

impl StackError {
    pub fn step(&self) -> Self {
        match self {
            StackError::EmptyStack { idx, op } => Self::EmptyStack { idx: *idx, op: *op },
            StackError::ReserveError { source } => Self::ReserveError {
                source: source.clone(),
            },
            StackError::UnknownOp { idx, byte } => Self::UnknownOp {
                idx: *idx + 1,
                byte: *byte,
            },
        }
    }
}
