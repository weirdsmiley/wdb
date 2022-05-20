//! rdb is a debugger ideally suited for any ELF binary which can be
//! executed on an x86_64 machine.
#![allow(
    non_snake_case,
    dead_code,
    unused_imports,
    unused_variables,
    non_camel_case_types,
    unused_macros
)]
use crate::utils::rdbError;
use object::Object;
use std::process;
use std::{env, fs};

mod commands;
mod debugee;
mod debugger;
mod parse;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        // TODO: 1. Dump error gracefully.
        //       2. What about returning Box?
        let err = rdbError("file not found".into());
        eprintln!("{}", err);
        return Err(Box::new(err));
    }
    let bin = fs::read(&args[1])?;
    let obj = object::File::parse(&*bin)?;

    if obj.architecture() == object::Architecture::X86_64 {
        debugger::init_debugger(&bin, &obj)?;
    } else {
        eprintln!("file format not supported");
        process::exit(1);
    }

    Ok(())
}
