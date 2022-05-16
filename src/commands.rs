//! This module defines all the supported commands for the debugger. It
//! also contains all the methods associated to perform operations
//! according to the command after parsing it.
use crate::debugger::Context;

pub mod breakpoint;
pub mod file;
pub mod module;
pub mod run;

// List of all supported commands in the debugger.
#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) enum Cmd {
    BreakPoint,
    Run,
    Quit,
    // TODO:
    // Help,
    // Print,
    Unknown,
}

// A trait implementation for each supported command. This trait allows commands
// to implement their own processing functions. For example, a process() for
// breakpoint will contain assigning the breakpoint to a particular program
// point.
pub(crate) trait CmdTy {
    fn process(self, Ctx: &mut Context);
}
