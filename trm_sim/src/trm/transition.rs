use serde::{Deserialize, Serialize};

use crate::trm::{Pattern, PatternConfig};

use crate::trm::syntax_error::{SyntaxError, SyntaxErrorType};

/// a turing machine transition
pub struct Transition {
    /// the symbols to consume
    pub consume: Vec<char>,
    /// the pattern to consume
    pub consume_pattern: Vec<Box<dyn Pattern>>,
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

impl Transition {
    /// create new transition from serde transition
    pub fn try_from_serde(
        trans: TransitionSerde,
        config: PatternConfig,
    ) -> Result<Self, SyntaxError> {
        trans.into_transition(config)
    }

    /// get serde transition
    pub fn to_serde(&self) -> TransitionSerde {
        TransitionSerde::from_transition(self)
    }
}

impl TransitionSerde {
    /// into transition with syntax check
    pub fn into_transition(self, config: PatternConfig) -> Result<Transition, SyntaxError> {
        let (consume, produce) = self.get_consume_produce()?;
        let consume_pattern = config.parse(&consume);
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
            consume_pattern,
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

/// the direction to move
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Stay,
}
