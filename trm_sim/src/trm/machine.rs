//! This module contains the turing machine struct and its methods.

use std::collections::HashMap;
use super::tape::*;
use super::state::*;
use serde::{Deserialize, Serialize};

/// a turing machine struct
#[derive(Debug, Clone)]
pub struct Machine {
    /// the states of the machine
    states: HashMap<String, State>,
    /// the current state
    current_state: String,
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
