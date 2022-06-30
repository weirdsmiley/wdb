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
use crate::utils::wdbError;
use object::Object;
use std::{env, fs, process};

mod commands;
mod debugee;
mod debugger;
mod parse;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        // FIXME: Why do returned Err(Box(T)) print entire T as it is? (meta)
        return Err(Box::new(wdbError(
            "no binary provided for debugging".into(),
        )));
    }

    // FIXME: It should still open the console and let dev use f command to load
    // binary dynamically.
    debugger::init_debugger(&args[1])?;

    Ok(())
}
