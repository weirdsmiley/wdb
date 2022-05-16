//! This module defines all the supported commands for the debugger. It
//! also contains all the methods associated to perform operations
//! according to the command after parsing it.
use crate::debugger::Context;
use std::error::Error;

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
    // Help,
    // Print,
    Unknown,
}

// A trait implementation for each supported command. This trait allows commands
// to implement their own processing functions. For example, a process() for
// breakpoint will contain assigning the breakpoint to a particular program
// point.
// TODO: Redesign this trait. This should not take Context as parameter, but
// rather infer parent structs from inside of CmdTy. Then should it return
// Result<(), Error>?
pub(crate) trait CmdTy {
    fn process<'a>(&mut self) -> Result<(), Box<dyn Error>>;
}
