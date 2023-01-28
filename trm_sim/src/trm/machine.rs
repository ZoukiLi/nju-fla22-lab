//! This module contains the turing machine struct and its methods.

use super::state::*;
use super::tape::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Display, Formatter};

/// a turing machine struct
/// # Example
/// ```
/// use trm_sim::trm::Machine;
/// let model = r#"
/// {
///     "states": [
///         {
///             "name": "q0",
///             "start": true,
///             "transitions": [
///                 {
///                     "cons": ["0"],
///                     "pros": ["1"],
///                     "dirs": ["R"],
///                     "next": "q1"
///                 },
///                 {
///                     "cons": ["1"],
///                     "pros": ["0"],
///                     "dirs": ["R"],
///                     "next": "q1"
///                 }
///             ]
///         },
///         {
///             "name": "q1",
///             "final": true,
///             "transitions": [
///                 {
///                     "cons": ["0"],
///                     "pros": ["1"],
///                     "dirs": ["R"],
///                     "next": "q1"
///                 },
///                 {
///                     "cons": ["1"],
///                     "pros": ["0"],
///                     "dirs": ["R"],
///                     "next": "q1"
///                 }
///             ]
///         }
///     ]
/// }
/// "#;
/// let machine = Machine::new(model, "json")?;
///
/// ```
#[derive(Debug, Clone)]
pub struct Machine {
    /// the states of the machine
    states: HashMap<String, State>,
    /// the start state of the machine
    start_state: String,
    /// the final states of the machine
    final_states: HashSet<String>,
    /// the current state
    current_state: String,
    /// the tapes of the machine
    tape: Vec<Tape>,
    /// the number of tapes
    tape_num: usize,
    /// wildcard for notnull char
    not_null_wc: char,
    /// wildcard for nullable char
    null_wc: char,
    /// blank char
    blank: char,
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

/// machine running error
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

impl Machine {
    /// creates a new machine from a model,
    /// with given model format.
    /// # Arguments
    /// * `model` - the model of the machine
    /// * `fmt` - the format of the model
    /// # Errors
    /// * `SyntaxError` - if the model is not valid
    pub fn new(model: &str, fmt: &str) -> Result<Self, SyntaxError> {
        // deserialize model
        let model = MachineModel::from_str(model, fmt)?;
        // create states
        let states: HashMap<_, _> = model
            .state
            .into_iter()
            .map(|state| state.into_state())
            .map(|state| state.map(|s| (s.name.clone(), s)))
            .collect::<Result<_, _>>()?;
        // filter start state and final states
        let start_state = states
            .iter()
            .filter(|(_, state)| state.is_start)
            .map(|(name, _)| name.clone())
            .collect::<Vec<String>>();
        let final_states = states
            .iter()
            .filter(|(_, state)| state.is_final)
            .map(|(name, _)| name.clone())
            .collect::<HashSet<String>>();

        // check start state
        if start_state.len() != 1 {
            return Err(SyntaxError {
                error_type: SyntaxErrorType::StartStateError,
                message: format!("start state error: {:#?}", start_state),
            });
        }

        let machine = Machine {
            states,
            start_state: start_state[0].clone(),
            final_states,
            current_state: start_state[0].clone(),
            tape: Vec::new(),
            tape_num: 0,
            not_null_wc: '*',
            null_wc: '_',
            blank: ' ',
        };
        Ok(machine)
    }

    /// resets the machine to the start state,
    /// and clears the tapes.
    /// # Errors
    /// * `SyntaxError` - if the machine has no start state, or has more than one start state,
    pub fn reset(&mut self) {
        self.current_state = self.start_state.clone();
        self.tape.clear();
    }

    /// returns the identifier of the machine
    pub fn get_identifier(&self) -> MachineIdentifier {
        MachineIdentifier {
            tape: self.tape.iter().map(|t| t.freeze(self.blank)).collect(),
            current_state: self.current_state.clone(),
        }
    }

    /// input a string to the first tape of machine
    /// # Arguments
    /// * `input` - the input string for first tape
    pub fn input(&mut self, input: &str) {
        self.tape.push(Tape::new(input));
        // insert blank to other tapes
        for _ in 1..self.tape_num {
            self.tape.push(Tape::new(""));
        }
    }

    /// runs the machine for one step
    /// # Errors
    /// * `SyntaxError` - if one transition next state does not exist
    pub fn run(&mut self) -> Result<bool, MachineRunningError> {
        // get current state
        let state = self
            .states
            .get(&self.current_state)
            .unwrap_or_else(|| panic!("current state not found: {}", self.current_state));

        self.find_transition_index(state)
            .map(|index| {
                let t = &state.transitions[index];
                // get next state
                let next_state = self
                    .states
                    .get(&t.next_state_name)
                    .ok_or(MachineRunningError::NextStateNotFound)?;
                // write to tape
                t.produce.iter().zip(&mut self.tape).for_each(|(p, tape)| {
                    tape.write(*p);
                });
                // move tape
                t.direction
                    .iter()
                    .zip(&mut self.tape)
                    .for_each(|(m, tape)| match *m {
                        Direction::Left => tape.move_left(),
                        Direction::Right => tape.move_right(),
                        _ => {}
                    });
                // set next state
                self.current_state = next_state.name.clone();
                Ok(false)
            })
            .unwrap_or(Ok(true))
    }

    /// find which transition to use
    fn find_transition_index(&self, state: &State) -> Option<usize> {
        // get transition
        let match_all_tape = |rules: &[char]| {
            rules
                .iter()
                .zip(&self.tape)
                .all(|(rule, tape)| tape.tape_match(*rule, self.not_null_wc, self.null_wc))
        };
        let count_wc = |rules: &[char]| {
            rules
                .iter()
                .filter(|c| **c == self.not_null_wc || **c == self.null_wc)
                .count()
        };

        state
            .transitions
            .iter()
            .enumerate()
            .filter(|(_, t)| match_all_tape(&t.consume))
            .min_by_key(|(_, t)| count_wc(&t.consume))
            .map(|(i, _)| i)
    }

    /// check if the machine is in a final state
    pub fn is_final(&self) -> bool {
        self.final_states.contains(&self.current_state)
    }
}

impl MachineModel {
    /// creates a new machine model from a string,
    /// with given model format.
    /// # Arguments
    /// * `model` - the model of the machine
    /// * `fmt` - the format of the model
    /// # Errors
    /// * `SyntaxError` - if the model is not valid
    pub fn from_str(model: &str, fmt: &str) -> Result<Self, SyntaxError> {
        let model = match fmt {
            "json" => serde_json::from_str(model).map_err(|e| SyntaxError {
                error_type: SyntaxErrorType::SyntaxNotValid(e.to_string()),
                message: "json deserializer failed.".to_string(),
            })?,
            "toml" => toml::from_str(model).map_err(|e| SyntaxError {
                error_type: SyntaxErrorType::SyntaxNotValid(e.to_string()),
                message: "toml deserializer failed.".to_string(),
            })?,
            _ => {
                return Err(SyntaxError {
                    error_type: SyntaxErrorType::FormatNotProvided,
                    message: format!("not provided format: {}", fmt),
                })
            }
        };

        Ok(model)
    }
}
