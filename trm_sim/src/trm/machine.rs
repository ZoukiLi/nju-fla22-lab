//! This module contains the turing machine struct and its methods.

use super::state::*;
use super::tape::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::zip;

/// A turing machine struct
/// # Example
/// ```
/// # fn test_run() -> Result<(), Box<dyn std::error::Error>> {
/// use trm_sim::trm::Machine;
/// let model = r#"
/// {
///     // the states of the machine ...
/// #   "states": [
/// #       {
/// #           "name": "q0",
/// #           "start": true,
/// #           "transitions": [
/// #               {
/// #                   "cons": "0",
/// #                   "prod": "1",
/// #                   "move": "R",
/// #                   "next": "q1"
/// #               },
/// #               {
/// #                   "cons": "1",
/// #                   "prod": "0",
/// #                   "move": "R",
/// #                   "next": "q1"
/// #               }
/// #           ]
/// #       },
/// #       {
/// #           "name": "q1",
/// #           "final": true,
/// #           "transitions": [
/// #               {
/// #                   "cons": "0",
/// #                   "prod": "1",
/// #                   "move": "R",
/// #                   "next": "q1"
/// #               },
/// #               {
/// #                   "cons": "1",
/// #                   "prod": "0",
/// #                   "move": "R",
/// #                   "next": "q1"
/// #               }
/// #           ]
/// #       }
/// #   ]
/// }
/// "#;
/// let mut machine = Machine::new(model, "json")?;
/// machine.input("1101");
/// machine.run()?;
/// let id = machine.identifier();
/// assert_eq!(id.current_state, "q1");
/// assert_eq!(id.tape[0].tape, "0101");
/// # Ok(())
/// # }
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

/// A helper struct of machine model for serde
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineModel {
    /// the states of the machine
    #[serde(default, alias = "states")]
    state: Vec<StateSerde>,
    /// wildcard for notnull char
    #[serde(default, rename = "notNullWildcard")]
    not_null_wc: Option<char>,
    /// wildcard for nullable char
    #[serde(default, rename = "nullWildcard")]
    null_wc: Option<char>,
    /// blank char
    #[serde(default, rename = "blank")]
    blank: Option<char>,
}

/// Readonly identifier for one machine,
/// which is also serializable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineIdentifier {
    /// current state name
    pub current_state: String,
    /// current tape content
    pub tape: Vec<FrozenTape>,
}

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

impl Machine {
    /// Creates a new machine from a model,
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
                message: format!("start state error: {start_state:#?}"),
            });
        }

        let machine = Machine {
            states,
            start_state: start_state[0].clone(),
            final_states,
            current_state: start_state[0].clone(),
            tape: Vec::new(),
            tape_num: 0,
            not_null_wc: model.not_null_wc.unwrap_or('*'),
            null_wc: model.null_wc.unwrap_or('_'),
            blank: model.blank.unwrap_or(' '),
        };
        Ok(machine)
    }

    /// Resets the machine to the start state,
    /// and clears the tapes.
    /// # Errors
    /// * `SyntaxError` - if the machine has no start state, or has more than one start state,
    pub fn reset(&mut self) {
        self.current_state = self.start_state.clone();
        self.tape.clear();
    }

    /// returns the identifier of the machine
    pub fn identifier(&self) -> MachineIdentifier {
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
    /// * `NextStateNotFound` - if one transition next state does not exist
    /// # Returns
    ///
    pub fn run_once(&mut self) -> Result<bool, MachineRunningError> {
        // get current state
        let state = self
            .states
            .get(&self.current_state)
            .ok_or(MachineRunningError::NextStateNotFound)?;

        Machine::find_transition(state, &self.tape, self.not_null_wc, self.null_wc)
            .map(|t| {
                // get next state
                let next_state = self
                    .states
                    .get(&t.next_state_name)
                    .ok_or(MachineRunningError::NextStateNotFound)?;
                // write to tape
                zip(&t.consume, &t.produce)
                    .zip(&mut self.tape)
                    .for_each(|((c, p), tape)| {
                        // if both consume char and produce char are wildcard,
                        // then do nothing
                        if *c != *p {
                            tape.write(*p);
                        }
                    });
                // move tape
                t.direction
                    .iter()
                    .zip(&mut self.tape)
                    .for_each(|(m, tape)| tape.move_to(*m));
                // set next state
                self.current_state = next_state.name.clone();
                Ok(false)
            })
            .unwrap_or(Ok(true))
    }

    /// run until the machine stops
    /// # Errors
    /// * `NextStateNotFound` - if one transition next state does not exist
    pub fn run(&mut self) -> Result<bool, MachineRunningError> {
        loop {
            if self.run_once()? {
                break;
            }
        }
        Ok(self.final_states.contains(&self.current_state))
    }


    /// find which transition to use
    fn find_transition<'a>(
        state: &'a State,
        tape: &'_ [Tape],
        some_wc: char,
        null_wc: char,
    ) -> Option<&'a Transition> {
        // get transition
        let match_all_tape = |rules: &[char]| {
            rules
                .iter()
                .zip(tape)
                .all(|(rule, tape)| tape.tape_match(*rule, some_wc, null_wc))
        };
        let count_wc = |rules: &[char]| {
            rules
                .iter()
                .filter(|c| **c == some_wc || **c == null_wc)
                .count()
        };

        state
            .transitions
            .iter()
            .filter(|t| match_all_tape(&t.consume))
            .min_by_key(|t| count_wc(&t.consume))
    }

    /// check if the machine is in a final state
    pub fn is_final(&self) -> bool {
        self.final_states.contains(&self.current_state)
    }

    /// get the model of the machine
    pub fn model(&self) -> MachineModel {
        let states = self.states.values().map(|s| s.to_serde()).collect();
        MachineModel {
            state: states,
            not_null_wc: Some(self.not_null_wc),
            null_wc: Some(self.null_wc),
            blank: Some(self.blank),
        }
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
                    message: format!("not provided format: {fmt}"),
                })
            }
        };

        Ok(model)
    }
}
