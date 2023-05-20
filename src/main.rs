//! wdb is a debugger ideally suited for any ELF binary which can be
//! executed on an x86_64 machine.
#![allow(
    non_snake_case,
    dead_code,
    unused_imports,
    unused_variables,
    non_camel_case_types,
    unused_macros
)]
use crate::error::wdbError;
use object::Object;
use std::{env, fs, process};

mod commands;
mod context;
mod debugee;
mod debugger;
mod error;
mod parse;
mod utils;

fn main() -> Result<(), wdbError> {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = debugger::init_debugger(args) {
        crate::utils::edump!(e);
    }

    Ok(())
}
