//! This module is for definition of turing machine state and transition structs.

use serde::{Deserialize, Serialize};

/// the direction to move
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
    Left,
    Right,
    Stay,
}

/// a turing machine state
#[derive(Debug, Clone)]
pub struct State<'a> {
    /// the name of the state
    name: String,
    /// is this state the start state
    is_start: bool,
    /// is this state a final state
    is_final: bool,
    /// the transitions of the state
    transitions: Vec<Transition<'a>>,
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

/// a turing machine transition
#[derive(Debug, Clone)]
pub struct Transition<'a> {
    /// the symbols to consume
    consume: Vec<char>,
    /// the symbols to produce
    produce: Vec<char>,
    /// the direction to move
    direction: Vec<Direction>,
    /// the next state
    next_state: Option<&'a State<'a>>,
}

/// a helper struct for serde transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionSerde {
    /// the symbols to consume
    #[serde(alias = "cons")]
    consume: String,
    /// the symbols to produce
    #[serde(alias = "prod")]
    produce: String,
    /// the direction to move
    #[serde(rename = "next")]
    next_direction: String,
    /// the next state
    #[serde(rename = "next")]
    next_state_name: String,
}

/// error type for syntax errors
#[derive(Debug, Clone)]
pub enum SyntaxErrorType {
    /// the state name is not found
    StateNameNotFound,
    /// the transition consume symbol is not found
    TransitionConsumeSymbolNotFound,
    /// the transition produce symbol is not found
    TransitionProduceSymbolNotFound,
    /// the transition direction is not found
    TransitionDirectionNotFound,
    /// the transition next state is not found
    TransitionNextStateNotFound,
}

/// error struct for syntax errors
#[derive(Debug, Clone)]
pub struct SyntaxError {
    /// the type of the error
    error_type: SyntaxErrorType,
    /// the error message
    message: String,
}

impl<'a> Transition<'a> {
    /// create new transition from serde transition
    pub fn from_serde(trans: TransitionSerde, states: &Vec<State>) -> Result<Self, SyntaxError> {

    }
}

impl TransitionSerde {
    pub fn into_transition(self, states: &Vec<State>) -> Result<Transition, SyntaxError> {

    }

    /// get next state by name
    fn get_next_state(&self, states: &Vec<State>) -> Result<&State, SyntaxError> {
        let next_state = states.iter()
            .find(|s|s.name == self.next_state_name);
        match next_state {
            Some(s) => Ok(s),
            None => Err(SyntaxError {
                error_type: SyntaxErrorType::TransitionNextStateNotFound,
                message: format!("Transition `{}` -> `{}` next state named `{}` not found",
                    self.consume, self.produce, self.next_state_name),
            }),
        }
    }

    /// get next directions
    fn get_next_direction(&self) -> Result<Vec<Direction>, SyntaxError> {
        let next_direction = self.next_direction
            .to_uppercase()
            .chars()
            .map(|c| match c {
                'L' => Ok(Direction::Left),
                'R' => Ok(Direction::Right),
                'S' => Ok(Direction::Stay),
                _ => Err(SyntaxError {
                    error_type: SyntaxErrorType::TransitionDirectionNotFound,
                    message: format!("Transition `{}` -> `{}` direction `{}` not found",
                        self.consume, self.produce, c),
                }),
            })
            .collect::<Result<Vec<Direction>, SyntaxError>>();
        next_direction
    }

    /// create new transition from serde transition
    pub fn from_transition(transition: &Transition) -> Self {
        // get the direction from direction
        let next_direction = transition.direction.iter()
            .map(|d| match d {
                Direction::Left => 'L',
                Direction::Right => 'R',
                Direction::Stay => 'S',
            })
            .collect::<String>();
        // get the next state name
        let next_state_name = transition.next_state.unwrap().name.clone();
        Self {
            consume: transition.consume.iter().collect(),
            produce: transition.produce.iter().collect(),
            next_direction,
            next_state_name,
        }
    }
}