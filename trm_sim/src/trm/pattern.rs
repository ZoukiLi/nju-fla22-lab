//! deal char pattern like wildcards and nullable

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum PatternAction {
    Keep,
    Replace(char),
}

impl PatternAction {
    pub fn new(keep: bool, replace: char) -> Self {
        if keep {
            PatternAction::Keep
        } else {
            PatternAction::Replace(replace)
        }
    }
}

pub trait Pattern {
    fn match_input(&self, input: Option<char>) -> bool;

    fn action(&self, cons: char, prod: char) -> PatternAction;
}

#[derive(Debug, Clone)]
pub struct CharPattern {
    pub pattern: char,
}

impl Pattern for CharPattern {
    fn match_input(&self, input: Option<char>) -> bool {
        input == Some(self.pattern)
    }

    fn action(&self, _cons: char, prod: char) -> PatternAction {
        // always replace
        PatternAction::new(false, prod)
    }
}

#[derive(Debug, Clone)]
pub struct EmptyPattern;

impl Pattern for EmptyPattern {
    fn match_input(&self, input: Option<char>) -> bool {
        input.is_none()
    }

    fn action(&self, cons: char, prod: char) -> PatternAction {
        // keep if cons == prod
        PatternAction::new(cons == prod, prod)
    }
}

#[derive(Debug, Clone)]
pub struct SomeWildcardPattern;

impl Pattern for SomeWildcardPattern {
    fn match_input(&self, input: Option<char>) -> bool {
        input.is_some()
    }

    fn action(&self, cons: char, prod: char) -> PatternAction {
        // keep if cons == prod
        PatternAction::new(cons == prod, prod)
    }
}

#[derive(Debug, Clone)]
pub struct AnyPattern;

impl Pattern for AnyPattern {
    fn match_input(&self, _input: Option<char>) -> bool {
        true
    }

    fn action(&self, cons: char, prod: char) -> PatternAction {
        // keep if cons == prod
        PatternAction::new(cons == prod, prod)
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct PatternConfig {
    #[serde(rename = "empty")]
    pub empty: char,
    #[serde(rename = "some")]
    pub some_wildcard: char,
    pub any: char,
}

impl Default for PatternConfig {
    fn default() -> Self {
        Self {
            empty: '_',
            some_wildcard: '*',
            any: '.',
        }
    }
}

impl PatternConfig {
    pub fn parse(&self, pattern: &[char]) -> Vec<Box<dyn Pattern>> {
        pattern
            .iter()
            .map(|c| match *c {
                c if c == self.empty => Box::new(EmptyPattern) as Box<dyn Pattern>,
                c if c == self.some_wildcard => Box::new(SomeWildcardPattern),
                c if c == self.any => Box::new(AnyPattern),
                c => Box::new(CharPattern { pattern: c }),
            })
            .collect()
    }
}
