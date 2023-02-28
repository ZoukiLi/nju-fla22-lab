use std::error::Error;
use std::fmt::{Display, Formatter};

/// Machine running error
#[derive(Debug, Clone)]
pub enum MachineRunningError {
    /// the transition next state is not found
    NextStateNotFound,
}

impl Display for MachineRunningError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MachineRunningError::NextStateNotFound => write!(f, "Next state not found."),
        }
    }
}

impl Error for MachineRunningError {}
