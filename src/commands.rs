//! This module defines all the supported commands for the debugger. It
//! also contains all the methods associated to perform operations
//! according to the command after parsing it.
use crate::error::wdbError;
use std::error::Error;

pub mod breakpoint;
pub mod file;
pub mod module;
pub mod run;

// List of all supported commands in the debugger.
#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) enum Cmd {
    BreakPoint,
    File,
    Help,
    // Print,
    Quit,
    Run,
    // List,
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
    // declare a type which stores the current cmd (raw)
    type cmd;
    // Replace the context whenever necessary (viz file command)
    type ParentCtx;

    // The processing logic for every command will lie here.
    fn process(&mut self, c: Self::cmd) -> Result<Self::ParentCtx, wdbError>;

    // Each command can implement this function for specifically dumping their
    // help. This is different from dumping their members' information.
    fn dump_help(&self);
}
