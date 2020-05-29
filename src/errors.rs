use hypothesis::AnnotationID;
use thiserror::Error;

/// Errors which can be caused by normal gooseberry operation.
/// Those caused by external libraries throw their own errors when possible
#[derive(Debug, Error)]
pub enum Apologize {
    /// Thrown when trying to access an unrecorded tag
    #[error("You haven't tagged anything as {tag:?} yet.")]
    TagNotFound { tag: String },
    /// Thrown when trying annotation ID doesn't match any recorded annotations
    #[error("Couldn't find an annotation with ID {id:?}")]
    AnnotationNotFound { id: AnnotationID },
    /// Thrown when explicit Y not received from user for destructive things
    #[error("I'm a coward. Doing nothing.")]
    DoingNothing,
    /// Thrown when $HOME is not set
    #[error("Homeless: $HOME not set")]
    Homeless,
    #[error("SearchError: Search failed")]
    SearchError,
    /// Errors related to changing the configuration file
    #[error("ConfigError: {message:?}")]
    ConfigError { message: String },
    /// Catch-all for stuff that should never happen
    #[error("OutOfCheeseError: {message:?}\nRedo from start.")]
    OutOfCheeseError { message: String },
}
