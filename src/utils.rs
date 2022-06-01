//! Utils section provide macro to print the instructions instead of the
//! usual one address in each line.
//!
use std::fmt;

#[macro_export]
/// Print such that each 'instruction' is on new line.
macro_rules! instprint {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{}};
}

/// Provides a primitive macro for dumping objects (if dump is
/// implemented).
macro_rules! dump {
    ($var:expr) => {
        println!("{}", $var.dump());
    };
}
pub(crate) use dump;

/// This provides a custom error handling for wdb.
#[derive(Debug)]
pub(crate) struct wdbError(pub(crate) String);

impl fmt::Display for wdbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(wdb): {}", self.0)
    }
}

impl std::error::Error for wdbError {}

// TODO:
/// Declare all possible kinds of errors for the debugger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum wdbErrorKind {
    /// Breakpoint being parsed runs into an error.
    BreakPointParseError,
}
