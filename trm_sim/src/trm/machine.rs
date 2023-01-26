//! This module contains the turing machine struct and its methods.

use super::tape::*;
use super::state::*;
use serde::{Deserialize, Serialize};

/// a turing machine struct
#[derive(Debug, Clone)]
pub struct Machine<'a> {
    /// the states of the machine
    states: Vec<State<'a>>,
    /// the current state pointer
    current_state: Option<&'a State<'a>>,
    /// the tapes of the machine
    tape: Vec<Tape>,
}

/// a helper struct of machine model for serde
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineModel {
    /// the states of the machine
    #[serde(default, alias = "states")]
    state: Vec<StateSerde>,
}

/// readonly identifier for one machine,
/// which is also serializable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineIdentifier {
    /// current state name
    current_state: String,
    /// current tape content
    tape: Vec<FrozenTape>,
}
