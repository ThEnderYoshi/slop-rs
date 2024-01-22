//! Defines [slop_rs](crate)'s error types.

use std::io;

use thiserror::Error;

/// Alias of [Result] where [Err] holds a [SlopError].
pub type SlopResult<T> = Result<T, SlopError>;

/// The possible errors returned by the SLOP API.
/// 
/// See also: [SlopResult]
#[derive(Debug, Error)]
pub enum SlopError {
    /// While parsing, the line was not a valid string KV or list KV starter.
    /// 
    /// Holds the 0-based index and contents if the line in question.
    /// The index is written as 1-based when displayed.
    #[error("(in line {}) `{1}` is not a valid kv", .0 + 1)]
    InvalidLine(usize, String),

    /// While parsing, the list KV was never closed.
    /// 
    /// Holds the 0-based index and contents of the line that starts the KV.
    /// The index is written as 1-based when displayed.
    #[error("(in line {}) `{1}` is not closed", .0 + 1)]
    UnclosedList(usize, String),

    /// Returned during [Slop::insert](crate::Slop::insert) if the key contains
    /// `=` or ends in `{`.
    #[error("the key `{0}` contains invalid characters")]
    InvalidKey(String),

    /// Wrapper for [io::Error]s.
    #[error("io error: {0}")]
    Io(#[from] io::Error),
}
