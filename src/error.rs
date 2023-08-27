use crate::utils::*;
/// This provides a custom error handling for wdb. It will handle all known and
/// possible unknown or custom designed error strings when needed.
use std::fmt;

// pub(crate) trait wdbError { }

#[derive(Debug)]
struct Location {
    row: usize,
    col: usize,
}

#[derive(Debug)]
pub(crate) struct wdbError {
    pub(crate) kind: wdbErrorKind,
}

// impl wdbError {
//     pub(crate) fn new(err: &'static str) -> wdbError {
//         wdbError(err.to_string())
//     }
// }

// impl From<dyn std::error::Error> for wdbError {
//     fn from(_: dyn std::error::Error) -> Self {
//         wdbError::from(wdbErrorKind::ParseError)
//     }
// }

impl From<wdbErrorKind> for wdbError {
    fn from(kind: wdbErrorKind) -> wdbError {
        wdbError { kind }
    }
}

// TODO: Remove! we don't need this
impl fmt::Display for wdbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // FIXME: Doesn't look aesthetically pleasing.
        write!(f, "{BRIGHT}{RED}!(err){RESET} {}", self.kind)
    }
}

impl std::error::Error for wdbError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.kind)
    }
}

// TODO: Distinguishing between errors and warnings. Errors will terminate
// the debugger's loop, but warnings should help the developer, and allow
// the debugger to proceed.
/// Declare all possible kinds of errors for the debugger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum wdbErrorKind {
    ArchitectureError,
    ParseError,
    BreakPointIUError, // Incorrect usage error
    BreakPointParseError,
    BreakPointParseIntError,
    FileIUError,
    RunIUError,
    RunPCOverflowError,
    NonExistentBinary,
    UnknownCmd,
}

impl wdbErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        use wdbErrorKind::*;

        match self {
            ArchitectureError => "architecture not supported",
            ParseError => "unable to read command",
            BreakPointIUError => "usage: br[eakpoint] <file>:<line>",
            BreakPointParseError => "breakpoint: unable to parse parameter",
            BreakPointParseIntError => "breakpoint: line is not a number",
            FileIUError => "usage: f[ile] <binary>",
            RunIUError => "usage: r[un] <param>",
            RunPCOverflowError => "run: Program counter has overflowed!",
            NonExistentBinary => "no binary provided",
            UnknownCmd => "unknown command",
        }
    }
}

impl fmt::Display for wdbErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::error::Error for wdbErrorKind {}
