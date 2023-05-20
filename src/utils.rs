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

/// Provides a primitive macro for dumping objects if dump is implemented.
macro_rules! dump {
    ($var:expr) => {
        println!("{}", $var.dump());
    };
}
pub(crate) use dump;

/// This primitive provides an error dumping for wdbErrorKinds.
macro_rules! edump {
    ($var:expr) => {
        eprintln!("{}", $var)
    };
}
pub(crate) use edump;
