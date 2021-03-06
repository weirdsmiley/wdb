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

/// This primitive provides an error dumping for wdbErrorKinds.
macro_rules! edump {
    ($var:expr) => {
        eprintln!("{}", $var);
    };
}
pub(crate) use edump;

/// This provides a custom error handling for wdb.
#[derive(Debug)]
pub(crate) struct wdbError(pub(crate) String);

impl fmt::Display for wdbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: {}", self.0)
    }
}

impl std::error::Error for wdbError {}

// TODO: Distinguishing between errors and warnings. Errors will terminate
// the debugger's loop, but warnings should help the developer, and allow
// the debugger to proceed.
/// Declare all possible kinds of errors for the debugger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum wdbErrorKind {
    BreakPointIUError, // Incorrect usage
    BreakPointParseError,
    BreakPointParseIntError,
    FileIUError,
    RunIUError,
    RunPCOverflowError,
}

impl fmt::Display for wdbErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // This will handle all the kinds of errors and their
        // displays. Remember, these are only for writing and not exiting.
        // The decision of whether to exit (debugger) or not is dependant
        // on the invoker.
        return match self {
            // TODO:
            // Usage errors should be printed on console because no one wants
            // help pages in their logs. So eventually, *IUError(s) will go away
            // and there will be a dump_help for each command type. And those
            // dump_help will only be printing on console.
            wdbErrorKind::BreakPointIUError => {
                write!(f, "usage: br[eakpoint] <file>:<line>")
            }
            wdbErrorKind::BreakPointParseError => {
                write!(f, "breakpoint: unable to parse parameter")
            }
            wdbErrorKind::BreakPointParseIntError => {
                write!(f, "breakpoint: line is not a number")
            }
            wdbErrorKind::FileIUError => {
                write!(f, "usage: f[ile] <binary>")
            }
            wdbErrorKind::RunIUError => {
                write!(f, "usage: r[un] <param>")
            }
            wdbErrorKind::RunPCOverflowError => {
                write!(f, "run: Program counter has overflowed!")
            }
        };
    }
}

impl std::error::Error for wdbErrorKind {}
