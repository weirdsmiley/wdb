//! Utils section provide macro to print the instructions instead of the
//! usual one address in each line.
//!
use std::fmt;

#[macro_export]
// TODO: Print such that each 'instruction' is on new line.
macro_rules! instprint {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{}};
}

macro_rules! dump {
    ($var:expr) => {
        println!("{}", $var.dump());
    };
}
pub(crate) use dump;

// This macro provides a custom error handling for rdb.
#[derive(Debug)]
pub(crate) struct rdbError(pub(crate) String);

impl fmt::Display for rdbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(rdb): {}", self.0)
    }
}

impl std::error::Error for rdbError {}
