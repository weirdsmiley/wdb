//! Utils section provide macro to print the instructions instead of the
//! usual one address in each line.
//!
use std::fmt;

pub(crate) const RESET: &str = "\x1b[0m";
pub(crate) const BRIGHT: &str = "\x1b[1m";
pub(crate) const DIM: &str = "\x1b[2m";
pub(crate) const UNDERSCORE: &str = "\x1b[4m";
pub(crate) const BLINK: &str = "\x1b[5m";
pub(crate) const REVERSE: &str = "\x1b[7m";
pub(crate) const HIDDEN: &str = "\x1b[8m";
pub(crate) const BLACK: &str = "\x1b[30m";
pub(crate) const RED: &str = "\x1b[31m";
pub(crate) const GREEN: &str = "\x1b[32m";
pub(crate) const YELLOW: &str = "\x1b[33m";
pub(crate) const BLUE: &str = "\x1b[34m";
pub(crate) const MAGENTA: &str = "\x1b[35m";
pub(crate) const CYAN: &str = "\x1b[36m";
pub(crate) const WHITE: &str = "\x1b[37m";
pub(crate) const BGBLACK: &str = "\x1b[40m";
pub(crate) const BGRED: &str = "\x1b[41m";
pub(crate) const BGGREEN: &str = "\x1b[42m";
pub(crate) const BGYELLOW: &str = "\x1b[43m";
pub(crate) const BGBLUE: &str = "\x1b[44m";
pub(crate) const BGMAGENTA: &str = "\x1b[45m";
pub(crate) const BGCYAN: &str = "\x1b[46m";
pub(crate) const BGWHITE: &str = "\x1b[47m";

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
