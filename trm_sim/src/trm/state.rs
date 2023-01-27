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
pub(crate) struct State<'a> {
    /// the name of the state
    name: String,
    /// is this state the start state
    is_start: bool,
    /// is this state a final state
    is_final: bool,
    /// the transitions of the state
    transitions: Vec<Transition<'a>>,
    /// the raw transitions
    trans_raw: Option<Vec<TransitionSerde>>,
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
pub(crate) struct Transition<'a> {
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
    /// the transition consume symbol is not match produce symbol
    TransitionConsumeProduceNotMatch,
    /// the transition direction is not found
    TransitionDirectionNotFound,
    /// the transition next state is not found
    TransitionNextStateNotFound,
}

/// error struct for syntax errors
#[derive(Debug, Clone)]
pub struct SyntaxError {
    /// the type of the error
    pub error_type: SyntaxErrorType,
    /// the error message
    pub message: String,
}

impl<'a> State<'a> {
    /// create new state from serde state
    pub fn from_serde(state: StateSerde) -> Self {
        state.into_state()
    }

    /// get serde state
    pub fn to_serde(&self) -> StateSerde {
        StateSerde::from_state(self)
    }

    /// initialize the transitions
    pub fn init_trans(&mut self, states: &'a [State<'a>]) -> Result<(), SyntaxError> {
        self.transitions = self
            .trans_raw
            .take()
            .unwrap()
            .into_iter()
            .map(|t| t.into_transition(states))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }
}

impl StateSerde {
    /// into state
    pub(crate) fn into_state(self) -> State<'static> {
        State {
            name: self.name,
            is_start: self.is_start,
            is_final: self.is_final,
            transitions: vec![],
            trans_raw: Some(self.trans),
        }
    }
    /// from state
    pub(crate) fn from_state(state: &State) -> Self {
        let trans = state
            .transitions
            .iter()
            .map(TransitionSerde::from_transition)
            .collect();
        Self {
            name: state.name.clone(),
            is_start: state.is_start,
            is_final: state.is_final,
            trans,
        }
    }
}

impl<'a> Transition<'a> {
    /// create new transition from serde transition
    pub fn from_serde(trans: TransitionSerde, states: &'a [State]) -> Result<Self, SyntaxError> {
        trans.into_transition(states)
    }

    /// get serde transition
    pub fn to_serde(&self) -> TransitionSerde {
        TransitionSerde::from_transition(self)
    }
}

impl Drop for Transition<'_> {
    /// drop skip next state
    fn drop(&mut self) {
        self.next_state = None;
    }
}

impl TransitionSerde {
    /// into transition
    pub(crate) fn into_transition<'a>(
        self,
        states: &'a [State],
    ) -> Result<Transition<'a>, SyntaxError> {
        let (consume, produce) = self.get_consume_produce()?;
        let direction = self.get_direction()?;
        let next_state = self.get_next_state(states)?;
        Ok(Transition {
            consume,
            produce,
            direction,
            next_state: Some(next_state),
        })
    }

    /// get next state by name
    fn get_next_state<'a>(&'_ self, states: &'a [State<'a>]) -> Result<&'a State<'a>, SyntaxError> {
        let next_state = states.iter().find(|s| s.name == self.next_state_name);
        match next_state {
            Some(s) => Ok(s),
            None => Err(SyntaxError {
                error_type: SyntaxErrorType::TransitionNextStateNotFound,
                message: format!(
                    "Transition `{}` -> `{}` next state named `{}` not found",
                    self.consume, self.produce, self.next_state_name
                ),
            }),
        }
    }

    /// get move directions
    fn get_direction(&self) -> Result<Vec<Direction>, SyntaxError> {
        let next_direction = self
            .next_direction
            .to_uppercase()
            .chars()
            .map(|c| match c {
                'L' => Ok(Direction::Left),
                'R' => Ok(Direction::Right),
                'S' => Ok(Direction::Stay),
                _ => Err(SyntaxError {
                    error_type: SyntaxErrorType::TransitionDirectionNotFound,
                    message: format!(
                        "Transition `{}` -> `{}` direction `{}` not found",
                        self.consume, self.produce, c
                    ),
                }),
            })
            .collect::<Result<Vec<Direction>, SyntaxError>>();
        next_direction
    }

    /// get pair of consume and produce symbols
    fn get_consume_produce(&self) -> Result<(Vec<char>, Vec<char>), SyntaxError> {
        let consume = self.consume.chars().collect::<Vec<char>>();
        let produce = self.produce.chars().collect::<Vec<char>>();
        if consume.len() != produce.len() {
            Err(SyntaxError {
                error_type: SyntaxErrorType::TransitionConsumeProduceNotMatch,
                message: format!(
                    "Transition `{}` -> `{}` consume and produce symbols not match",
                    self.consume, self.produce
                ),
            })
        } else {
            Ok((consume, produce))
        }
    }

    /// create serializable transition from transition
    pub(crate) fn from_transition(transition: &Transition) -> Self {
        // get the direction from direction
        let next_direction = transition
            .direction
            .iter()
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
