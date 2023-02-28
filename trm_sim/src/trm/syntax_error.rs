use std::error::Error;
use std::fmt::{Display, Formatter};

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
