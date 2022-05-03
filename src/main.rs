use object::Object;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

mod debugee;
mod debugger;
mod parse;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("file not found");
        process::exit(1);
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
