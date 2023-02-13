//! This module is for definition of turing machine state and transition structs.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};

/// the direction to move
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Stay,
}

/// a turing machine state
#[derive(Debug, Clone)]
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

/// a turing machine transition
#[derive(Debug, Clone)]
pub struct Transition {
    /// the symbols to consume
    pub consume: Vec<char>,
    /// the symbols to produce
    pub produce: Vec<char>,
    /// the direction to move
    pub direction: Vec<Direction>,
    /// the next state
    pub next_state_name: String,
}

/// a helper struct for serde transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionSerde {
    /// the symbols to consume
    #[serde(alias = "consume")]
    cons: String,
    /// the symbols to produce
    #[serde(alias = "produce")]
    prod: String,
    /// the direction to move
    #[serde(rename = "move")]
    next_direction: String,
    /// the next state
    #[serde(rename = "next")]
    next_state_name: String,
}

/// error type for syntax errors
#[derive(Debug, Clone)]
pub enum SyntaxErrorType {
    /// the transition consume symbol is not match produce symbol
    TransitionConsumeProduceNotMatch,
    /// the transition direction is not found
    TransitionDirectionNotFound,
    /// the transition next state is not found
    TransitionNextStateNotFound,
    /// the syntax is not valid
    SyntaxNotValid(String),
    /// the format is not provided
    FormatNotProvided,
    /// start state is not found or more than one
    StartStateError,
}

/// error struct for syntax errors
#[derive(Debug, Clone)]
pub struct SyntaxError {
    /// the type of the error
    pub error_type: SyntaxErrorType,
    /// the error message
    pub message: String,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}: {}", self.error_type, self.message)
    }
}

impl Error for SyntaxError {}

impl State {
    /// create new state from StateSerde
    pub fn from_serde(state: StateSerde) -> Result<Self, SyntaxError> {
        state.into_state()
    }

    /// get StateSerde
    pub fn to_serde(&self) -> StateSerde {
        StateSerde::from_state(self)
    }
}

impl StateSerde {
    /// into state with syntax check
    pub fn into_state(self) -> Result<State, SyntaxError> {
        let transitions = self
            .trans
            .into_iter()
            .map(|t| t.into_transition())
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

impl Transition {
    /// create new transition from serde transition
    pub fn from_serde(trans: TransitionSerde) -> Result<Self, SyntaxError> {
        trans.into_transition()
    }

    /// get serde transition
    pub fn to_serde(&self) -> TransitionSerde {
        TransitionSerde::from_transition(self)
    }
}

impl TransitionSerde {
    /// into transition with syntax check
    pub fn into_transition(self) -> Result<Transition, SyntaxError> {
        let (consume, produce) = self.get_consume_produce()?;
        let direction = self.get_direction()?;
        if direction.len() != consume.len() {
            return Err(SyntaxError {
                error_type: SyntaxErrorType::TransitionConsumeProduceNotMatch,
                message: format!(
                    "Transition `{}` -> `{}` consume do not match move direction `{}`",
                    self.cons, self.prod, self.next_direction
                ),
            });
        }
        Ok(Transition {
            consume,
            produce,
            direction,
            next_state_name: self.next_state_name,
        })
    }

    /// get move directions
    fn get_direction(&self) -> Result<Vec<Direction>, SyntaxError> {
        self.next_direction
            .to_uppercase()
            .chars()
            .map(|c| match c {
                'L' => Ok(Direction::Left),
                'R' => Ok(Direction::Right),
                'S' => Ok(Direction::Stay),
                _ => Err(SyntaxError {
                    error_type: SyntaxErrorType::TransitionDirectionNotFound,
                    message: format!(
                        "Transition `{}` -> `{}` direction `{c}` not found",
                        self.cons, self.prod
                    ),
                }),
            })
            .collect()
    }

    /// get pair of consume and produce symbols
    fn get_consume_produce(&self) -> Result<(Vec<char>, Vec<char>), SyntaxError> {
        let consume = self.cons.chars().collect::<Vec<char>>();
        let produce = self.prod.chars().collect::<Vec<char>>();
        if consume.len() != produce.len() {
            Err(SyntaxError {
                error_type: SyntaxErrorType::TransitionConsumeProduceNotMatch,
                message: format!(
                    "Transition `{}` -> `{}` consume and produce symbols not match",
                    self.cons, self.prod
                ),
            })
        } else {
            Ok((consume, produce))
        }
    }

    /// create serializable transition from transition
    pub fn from_transition(transition: &Transition) -> Self {
        // get the direction from direction
        let next_direction = transition
            .direction
            .iter()
            .map(|d| match d {
                Direction::Left => 'L',
                Direction::Right => 'R',
                Direction::Stay => 'S',
            })
            .collect();
        // get the next state name
        let next_state_name = transition.next_state_name.clone();
        Self {
            cons: transition.consume.iter().collect(),
            prod: transition.produce.iter().collect(),
            next_direction,
            next_state_name,
        }
    }
}
