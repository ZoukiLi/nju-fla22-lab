//! This module is for definition of turing machine state and transition structs.

use crate::trm::syntax_error::SyntaxError;
use crate::trm::transition::{Transition, TransitionSerde};
use crate::trm::PatternConfig;
use serde::{Deserialize, Serialize};

/// a turing machine state
pub struct State {
    /// the name of the state
    pub name: String,
    /// is this state the start state
    pub is_start: bool,
    /// is this state a final state
    pub is_final: bool,
    /// the transitions of the state
    pub transitions: Vec<Transition>,
}

/// a helper struct for serde state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSerde {
    /// the name of the state
    name: String,
    /// is this state the start state
    #[serde(default, alias = "start")]
    is_start: bool,
    /// is this state a final state
    #[serde(default, alias = "final")]
    is_final: bool,

    /// the transitions of the state
    #[serde(default, alias = "transitions")]
    trans: Vec<TransitionSerde>,
}

impl State {
    /// create new state from StateSerde
    pub fn try_from_serde(state: StateSerde, config: PatternConfig) -> Result<Self, SyntaxError> {
        state.into_state(config)
    }

    /// get StateSerde
    pub fn to_serde(&self) -> StateSerde {
        StateSerde::from_state(self)
    }
}

impl StateSerde {
    /// into state with syntax check
    pub fn into_state(self, config: PatternConfig) -> Result<State, SyntaxError> {
        let transitions = self
            .trans
            .into_iter()
            .map(|t| t.into_transition(config))
            .collect::<Result<_, _>>()?;

        Ok(State {
            name: self.name,
            is_start: self.is_start,
            is_final: self.is_final,
            transitions,
        })
    }

    /// create serializable state from state reference
    pub fn from_state(state: &State) -> Self {
        Self {
            name: state.name.clone(),
            is_start: state.is_start,
            is_final: state.is_final,
            trans: state
                .transitions
                .iter()
                .map(TransitionSerde::from_transition)
                .collect(),
        }
    }
}
