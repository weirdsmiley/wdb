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
        // TODO: Provide a file command which loads the debugging binary at
        // runtime (inside the wdb terminal).
        // FIXME: Why do these returned Err(Box(T)) print entire T as it is?
        // (meta)
        return Err(Box::new(wdbError(
            "no binary provided for debugging".into(),
        )));
    }
    // TODO: Read the binary properly and parse through its debugging info.
    let bin = fs::read(&args[1])?;
    // Lets focus on ELF only for now.
    let obj = object::File::parse(&*bin)?;

    if obj.architecture() == object::Architecture::X86_64 {
        debugger::init_debugger(&bin, &obj)?;
    } else {
        return Err(Box::new(wdbError("file format not supported".into())));
        // process::exit(1);
    }

    Ok(())
}
